// TODO: Handle generics / lifetime / where if removed
use crate::attributes::*;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    spanned::Spanned, AngleBracketedGenericArguments, DataEnum, DataStruct, DeriveInput, Error,
    Field, Ident, Path, PathArguments, Result, Variant,
};

#[derive(Copy, Clone)]
pub(crate) enum Naming {
    Named,
    Unnamed,
}

impl Naming {
    pub(crate) fn is_unnamed(&self) -> bool {
        matches!(self, Self::Unnamed)
    }
}

fn make_final_type(
    angle_args: &AngleBracketedGenericArguments,
    atts: &AttributeOptions,
    naming: Naming,
) -> Result<TokenStream> {
    let args = &angle_args.args;
    // Rc, Arc, Mutable, MutableVec
    if args.len() == 1 {
        if let Some(span) = atts.hashmap {
            return Err(Error::new(
                span,
                "Use of `hashmap` on a non `MutableBTreeMap<K, V>`",
            ));
        }
        match args.first().unwrap() {
            syn::GenericArgument::Type(syn::Type::Path(p)) => {
                remove_type_wrappers(&p.path, atts, naming)
            }
            _ => Ok(quote! {#args}),
        }
    // MutableBTreeMap
    } else if args.len() == 2 {
        match (args.first().unwrap(), args.last().unwrap()) {
            (syn::GenericArgument::Type(key), syn::GenericArgument::Type(value)) => {
                match (key, value) {
                    (syn::Type::Path(key), syn::Type::Path(value)) => {
                        let key = remove_type_wrappers(&key.path, atts, naming)?;
                        let value = remove_type_wrappers(&value.path, atts, naming)?;
                        // For hygine this must always be the full path
                        match atts.hashmap {
                            Some(_) => Ok(quote! { std::collections::HashMap<#key, #value> }),
                            None => Ok(quote! { std::collections::BTreeMap<#key, #value> }),
                        }
                    }
                    // This is when the value is unit -> Map to a HashSet / BTreeSet
                    (syn::Type::Path(key), syn::Type::Tuple(value)) if value.elems.is_empty() => {
                        match atts.hashmap {
                            Some(_) => Ok(quote! { std::collections::HashSet<#key> }),
                            None => Ok(quote! { std::collections::BTreeSet<#key> }),
                        }
                    }
                    _ => Ok(quote! {#args}),
                }
            }
            _ => Ok(quote! {#args}),
        }
    } else {
        unreachable!()
    }
}

fn remove_type_wrappers(
    path: &Path,
    atts: &AttributeOptions,
    naming: Naming,
) -> Result<TokenStream> {
    match path.segments.last() {
        Some(s) => {
            if s.ident == "Mutable"
                || (s.ident == "Rc" && atts.keep_rc.is_none())
                || (s.ident == "Arc" && atts.keep_arc.is_none())
            {
                match &s.arguments {
                    PathArguments::AngleBracketed(angle_args) => {
                        make_final_type(angle_args, atts, naming)
                    }
                    _ => unreachable!(),
                }
            } else if s.ident == "MutableVec" {
                match &s.arguments {
                    PathArguments::AngleBracketed(angle_args) => {
                        make_final_type(angle_args, atts, naming).map(|args| quote! { Vec<#args> })
                    }
                    _ => unreachable!(),
                }
            } else if s.ident == "MutableBTreeMap" {
                match &s.arguments {
                    PathArguments::AngleBracketed(angle_args) => {
                        make_final_type(angle_args, atts, naming)
                    }
                    _ => unreachable!(),
                }
            // This is the final path it comes down after recursion
            } else {
                match &atts.renamer {
                    Some(renamer) => {
                        let final_ty_name =
                            renamer.make_new_name(&s.ident, AttributeLocation::Field(naming))?;
                        // Need to add back any further types in <T> after the name. eg. Option<i32>
                        let args = &s.arguments;
                        Ok(quote! { #final_ty_name#args})
                    }
                    None => Ok(quote! { #path }),
                }
            }
        }
        None => unreachable!(),
    }
}

fn clean_field(
    field: &Field,
    atts: &AttributeOptions,
    final_type: Option<Result<TokenStream>>,
) -> Result<TokenStream> {
    let current_atts = &atts.current_attributes;
    let vis = &field.vis;
    let default_ty = &field.ty;
    let ty = final_type.unwrap_or_else(|| Ok(quote! { #default_ty }))?;
    let designal_atts = &atts.designal_attributes;
    match &field.ident {
        Some(name) => Ok(quote! {
            #(#current_atts)*
            #(#designal_atts)*
            #vis #name: #ty
        }),
        None => Ok(quote! {
            #(#current_atts)*
            #(#designal_atts)*
            #vis #ty
        }),
    }
}

fn map_field(
    field: &Field,
    naming: Naming,
    type_atts: &AttributeOptions,
) -> Option<Result<TokenStream>> {
    let atts = match AttributeOptions::new(&field.attrs, AttributeLocation::Field(naming)) {
        Ok(atts) => atts.add_type_level_to_field_level(type_atts),
        Err(e) => return Some(Err(e)),
    };
    if atts.remove.is_some() {
        None
    } else if atts.ignore.is_some() {
        Some(clean_field(&field, &atts, None))
    } else {
        let tokens = if let syn::Type::Path(p) = &field.ty {
            let final_type = Some(remove_type_wrappers(&p.path, &atts, naming));
            clean_field(&field, &atts, final_type)
        } else {
            clean_field(&field, &atts, None)
        };
        Some(tokens)
    }
}

fn build_struct(
    name: Ident,
    data: &DataStruct,
    input: &DeriveInput,
    type_atts: &AttributeOptions,
) -> Result<TokenStream> {
    let naming = match data.fields {
        syn::Fields::Named(_) => Naming::Named,
        syn::Fields::Unnamed(_) | syn::Fields::Unit => Naming::Unnamed,
    };
    let vis = &input.vis;
    let generics = &input.generics;
    let wher = &input.generics.where_clause;
    let atts = &type_atts.current_attributes;
    let designal_atts = &type_atts.designal_attributes;
    let fields = {
        let xs = data
            .fields
            .iter()
            .filter_map(|field| map_field(field, naming, &type_atts))
            .collect::<Result<Vec<TokenStream>>>()?;
        quote! { #(#xs),* }
    };
    Ok(match naming {
        Naming::Named => {
            quote! {
                #(#atts)*
                #(#designal_atts)*
                #vis struct #name #generics
                #wher {
                    #fields
                }
            }
        }
        Naming::Unnamed => {
            quote! {
                #(#atts)*
                #(#designal_atts)*
                #vis struct #name #generics (#fields)
                #wher;
            }
        }
    })
}

fn map_enum_variant(variant: &Variant, type_atts: &AttributeOptions) -> Result<TokenStream> {
    let ident = &variant.ident;
    let disc = &variant.discriminant;
    let atts = &variant.attrs;
    let fields = if variant.fields.is_empty() {
        quote! {}
    } else {
        let xs = variant
            .fields
            .iter()
            // Enums must always be treated as Named for map_field
            .filter_map(|field| map_field(field, Naming::Named, &type_atts))
            .collect::<Result<Vec<TokenStream>>>()?;
        match &variant.fields {
            syn::Fields::Named(_) => {
                quote! { {#(#xs),*} }
            }
            syn::Fields::Unnamed(_) | syn::Fields::Unit => {
                quote! { (#(#xs),*) }
            }
        }
    };

    match disc {
        Some(_) => Err(Error::new(
            variant.span(),
            "Discriminated variants are not yet implemented",
        )),
        None => Ok(quote! {
            #(#atts)*
            #ident
            #fields,
        }),
    }
}

fn build_enum(
    name: Ident,
    data: &DataEnum,
    input: &DeriveInput,
    type_atts: &AttributeOptions,
) -> Result<TokenStream> {
    let vis = &input.vis;
    let generics = &input.generics;
    let wher = &input.generics.where_clause;
    let atts = &type_atts.current_attributes;
    let designal_atts = &type_atts.designal_attributes;
    let variants = {
        let xs = data
            .variants
            .iter()
            .map(|variant| map_enum_variant(variant, type_atts))
            .collect::<Result<Vec<TokenStream>>>()?;
        quote! {
            #(#xs)*
        }
    };
    Ok(quote! {
        #(#atts)*
        #(#designal_atts)*
        #vis enum #name #generics
        #wher {
            #variants
        }
    })
}

fn rename_type(ident: &Ident, attr: &AttributeOptions) -> Result<Ident> {
    // Safe to unwrap since is checked in validation of attributes
    let renamer = attr.renamer.as_ref().unwrap();
    let name = renamer.make_new_name(&ident, AttributeLocation::Type(ident.span()))?;
    if name != *ident {
        Ok(name)
    } else {
        Err(Error::new(
            *renamer.span(),
            "Can't rename to the same name as the struct",
        ))
    }
}

pub(crate) fn parse_input(input: DeriveInput) -> Result<TokenStream> {
    let type_atts =
        AttributeOptions::new(&input.attrs, AttributeLocation::Type(input.ident.span()))?;
    let name = rename_type(&input.ident, &type_atts)?;
    let tokens = match &input.data {
        syn::Data::Struct(data) => build_struct(name, data, &input, &type_atts),
        syn::Data::Enum(data) => build_enum(name, data, &input, &type_atts),
        syn::Data::Union(_) => Err(Error::new(
            input.ident.span(),
            "Unions are not yet supported",
        )),
    }?;
    Ok(tokens)
}
