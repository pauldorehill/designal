// TODO: Handle generics / lifetime / where if removed
// TODO: Nested types? Add in a derive clause to get name etc?

use crate::attributes::*;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{
    AngleBracketedGenericArguments, Attribute, DataStruct, DeriveInput, Error, Field, Generics,
    Ident, Path, PathArguments, Result, Visibility,
};

const MUTABLE_VEC: &str = "MutableVec";
const MUTABLE: &str = "Mutable";
const SIGNAL_LOWER: &str = "signal";
const MUTABLE_LOWER: &str = "mutable";

fn remover(
    angle_args: &AngleBracketedGenericArguments,
    atts: &AttributeOptions,
) -> Result<TokenStream> {
    let args = &angle_args.args;
    match args.first() {
        Some(arg) => match arg {
            syn::GenericArgument::Type(t) => match t {
                syn::Type::Path(p) => remove_wrappers(&p.path, atts),
                _ => Ok(quote! {#args}),
            },
            _ => Ok(quote! {#args}),
        },
        None => unreachable!(),
    }
}

fn remove_wrappers(path: &Path, atts: &AttributeOptions) -> Result<TokenStream> {
    match path.segments.last() {
        Some(s) => {
            if s.ident == MUTABLE
                || (s.ident == "Rc" && atts.keep_rc.is_none())
                || (s.ident == "Arc" && atts.keep_arc.is_none())
            {
                match &s.arguments {
                    PathArguments::AngleBracketed(angle_args) => remover(angle_args, atts),
                    _ => unreachable!(),
                }
            } else if s.ident == MUTABLE_VEC {
                match &s.arguments {
                    PathArguments::AngleBracketed(angle_args) => {
                        remover(angle_args, atts).map(|args| quote! { Vec<#args> })
                    }
                    _ => unreachable!(),
                }
            } else {
                Ok(quote! {#path})
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
    match (&field.ident, &atts.renamer) {
        (Some(current), Some(renamer)) => renamer
            .make_new_name(&current)
            .map(|name| quote! { #(#new_atts)* #vis #name: #ty }),
        (Some(name), None) => Ok(quote! { #(#new_atts)* #vis #name: #ty }),
        _ => Ok(quote! { #(#new_atts)* #vis #ty }),
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
            let final_type = Some(remove_wrappers(&p.path, &atts));
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
        let fields = data_struct
            .fields
            .iter()
            .filter_map(|field| map_field(field, naming, &struct_level_atts))
            .collect::<Result<Vec<TokenStream>>>()?;

        let fields = quote! { #(#fields),* };
        let derives = struct_level_atts.derives.as_ref().map(|xs| {
            quote! { #[derive(#(#xs),*)] }
        });
        let s = Self {
            vis: &input.vis,
            name,
            fields,
            attributes: &struct_level_atts.others_to_keep,
            derives,
            naming,
            generics: &input.generics,
        };
        Ok(s)
    }

    fn make_name(ident: &Ident, attr: &AttributeOptions) -> Result<Ident> {
        if let Some(renamer) = &attr.renamer {
            let name = renamer.make_new_name(&ident)?;
            if name != ident.to_string() {
                Ok(format_ident!("{}", name))
            } else {
                Err(Error::new(
                    *renamer.span(),
                    "Can't rename to the same name as the struct",
                ))
            }
        } else {
            let name = ident.to_string();
            let lower_name = name.to_lowercase();
            if lower_name == MUTABLE_LOWER || lower_name == SIGNAL_LOWER {
                Ok(format_ident!("{}Designal", ident))
            } else if lower_name.starts_with(MUTABLE_LOWER) {
                Ok(format_ident!("{}", &name[MUTABLE.len()..]))
            } else if lower_name.ends_with(MUTABLE_LOWER) {
                Ok(format_ident!("{}", &name[..name.len() - MUTABLE.len()]))
            } else if lower_name.starts_with(SIGNAL_LOWER) {
                Ok(format_ident!("{}", &name[SIGNAL_LOWER.len()..]))
            } else if lower_name.ends_with(SIGNAL_LOWER) {
                Ok(format_ident!(
                    "{}",
                    &name[..name.len() - SIGNAL_LOWER.len()]
                ))
            } else {
                Ok(format_ident!("{}Desig", ident))
            }
        }
    }

    fn build(&self) -> Result<TokenStream> {
        let name = &self.name;
        let fields = &self.fields;
        let vis = self.vis;
        let atts = self.attributes;
        let derives = &self.derives;
        let generics = self.generics;
        let wher = &self.generics.where_clause;

        let tokens = match self.naming {
            Naming::Named => {
                quote! {
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
                    #derives
                    #(#atts)*
                    #vis struct #name #generics (#fields)
                    #wher;
                }
            }
        };
        Ok(tokens)
    }

    pub(crate) fn parse_input(input: DeriveInput) -> Result<TokenStream> {
        let struct_atts = AttributeOptions::new(&input.attrs, AttributeLocation::Struct)?;
        let name = Self::make_name(&input.ident, &struct_atts)?;
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
