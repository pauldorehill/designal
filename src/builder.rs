// TODO: Handle generics / lifetime / where if removed
use crate::attributes::*;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    AngleBracketedGenericArguments, Attribute, DataStruct, DeriveInput, Error, Field, Generics,
    Ident, Path, PathArguments, Result, Visibility,
};

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
            syn::GenericArgument::Type(t) => match t {
                syn::Type::Path(p) => remove_type_wrappers(&p.path, atts, naming),
                _ => Ok(quote! {#args}),
            },
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
                        // I think for good hygine this must always be the full path
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
                        // I think for good hygine this must always be the full path
                        // .map(|args| match atts.hashmap {
                        //     Some(_) => quote! { std::collections::HashMap<#args> },
                        //     None => quote! { std::collections::BTreeMap<#args> },
                        // })
                    }
                    _ => unreachable!(),
                }
            // This is the final path it comes down after recursion
            } else {
                // Ok(quote! { #path })
                match &atts.renamer {
                    Some(renamer) => {
                        let final_ty_name =
                            renamer.make_new_name(&s.ident, AttributeLocation::Field(naming))?;

                        // Need to add back any further types in <T> after the name. eg. Option<i32>
                        let args = &s.arguments;
                        Ok(quote! { #final_ty_name#args})
                        // match &s.arguments {
                        //     PathArguments::AngleBracketed(angle_args) => {
                        //         Ok(quote! { #final_ty_name#angle_args})
                        //     }
                        //     _ => Ok(quote! { #final_ty_name }),
                        // }
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
    let new_atts = &atts.others_to_keep;
    let vis = &field.vis;
    let default_ty = &field.ty;
    let ty = final_type.unwrap_or_else(|| Ok(quote! { #default_ty }))?;
    match &field.ident {
        Some(name) => Ok(quote! { #(#new_atts)* #vis #name: #ty }),
        None => Ok(quote! { #(#new_atts)* #vis #ty }),
    }
}

fn map_field(
    field: &Field,
    naming: Naming,
    struct_level_atts: &AttributeOptions,
) -> Option<Result<TokenStream>> {
    let atts = match AttributeOptions::new(&field.attrs, AttributeLocation::Field(naming)) {
        Ok(atts) => atts.add_struct_level_to_field_level(struct_level_atts),
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

#[derive(Copy, Clone)]
pub(crate) enum Naming {
    Named,
    Unnamed,
}

impl Naming {
    pub(crate) fn is_unnamed(&self) -> bool {
        match self {
            Naming::Named => false,
            Naming::Unnamed => true,
        }
    }
}

//TODO: Tidy up these?
pub(crate) struct ReturnType<'a> {
    vis: &'a Visibility,
    name: Ident,
    fields: TokenStream,
    attributes: &'a Vec<&'a Attribute>,
    derives: Option<TokenStream>,
    cfg: Option<TokenStream>,
    naming: Naming,
    generics: &'a Generics,
}

impl<'a> ReturnType<'a> {
    fn new(
        name: Ident,
        data_struct: &DataStruct,
        input: &'a DeriveInput,
        naming: Naming,
        struct_level_atts: &'a AttributeOptions,
    ) -> Result<Self> {
        let fields = {
            let xs = data_struct
                .fields
                .iter()
                .filter_map(|field| map_field(field, naming, &struct_level_atts))
                .collect::<Result<Vec<TokenStream>>>()?;
            quote! { #(#xs),* }
        };
        let derives = struct_level_atts.derives.as_ref().map(|xs| {
            let tokens: Result<Vec<TokenStream>> = xs
                .iter()
                .map(|v| match syn::parse_str::<Ident>(&v.0) {
                    Ok(ident) => Ok(quote! {#ident}),
                    Err(_) => match syn::parse_str::<Path>(&v.0) {
                        Ok(path) => Ok(quote! {#path}),
                        Err(_) => Err(Error::new(
                            v.1,
                            format!("Could not parse `{}` as a `Path` or `Ident`", v.0),
                        )),
                    },
                })
                .collect();
            tokens.map(|t| quote! { #[derive(#(#t),*)] })
        });
        let cfg = struct_level_atts
            .cfg_feature
            .as_ref()
            .map(|xs| xs.iter().map(|v| quote! { #[cfg(feature = #v)] }).collect());

        match derives {
            Some(Ok(derives)) => {
                let s = Self {
                    vis: &input.vis,
                    name,
                    fields,
                    attributes: &struct_level_atts.others_to_keep,
                    derives: Some(derives),
                    cfg,
                    naming,
                    generics: &input.generics,
                };
                Ok(s)
            }
            Some(Err(e)) => Err(e),
            None => {
                let s = Self {
                    vis: &input.vis,
                    name,
                    fields,
                    attributes: &struct_level_atts.others_to_keep,
                    derives: None,
                    cfg,
                    naming,
                    generics: &input.generics,
                };
                Ok(s)
            }
        }
    }

    fn build(&self) -> Result<TokenStream> {
        let name = &self.name;
        let fields = &self.fields;
        let vis = self.vis;
        let atts = self.attributes;
        let derives = &self.derives;
        let cfg = &self.cfg;
        let generics = self.generics;
        let wher = &self.generics.where_clause;
        let tokens = match self.naming {
            Naming::Named => {
                quote! {
                    #cfg
                    #derives
                    #(#atts)*
                    #vis struct #name #generics
                    #wher {
                        #fields
                    }
                }
            }
            Naming::Unnamed => {
                quote! {
                    #cfg
                    #derives
                    #(#atts)*
                    #vis struct #name #generics (#fields)
                    #wher;
                }
            }
        };
        Ok(tokens)
    }

    fn rename(ident: &Ident, attr: &AttributeOptions) -> Result<Ident> {
        // Safe to unwrap since is checked in validation of attributes
        let renamer = attr.renamer.as_ref().unwrap();
        let name = renamer.make_new_name(&ident, AttributeLocation::Struct(ident.span()))?;
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
        let struct_atts =
            AttributeOptions::new(&input.attrs, AttributeLocation::Struct(input.ident.span()))?;
        let name = Self::rename(&input.ident, &struct_atts)?;
        let ty = match &input.data {
            syn::Data::Struct(s) => match &s.fields {
                syn::Fields::Named(_) => {
                    ReturnType::new(name, s, &input, Naming::Named, &struct_atts)
                }
                syn::Fields::Unnamed(_) => {
                    ReturnType::new(name, s, &input, Naming::Unnamed, &struct_atts)
                }
                syn::Fields::Unit => Err(Error::new(
                    input.ident.span(),
                    "Unit structs are not supported",
                )),
            },
            syn::Data::Enum(_) => Err(Error::new(
                input.ident.span(),
                "Enums are not yet supported",
            )),
            syn::Data::Union(_) => Err(Error::new(
                input.ident.span(),
                "Unions are not yet supported",
            )),
        };
        ty?.build()
    }
}
