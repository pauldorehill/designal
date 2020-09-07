// TODO: Handle generics / lifetime / where if removed
// TODO: Nested types? Add in a derive clause to get name etc?

use crate::attributes::*;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{
    AngleBracketedGenericArguments, Error, Field, Generics, Ident, ItemStruct, Path,
    PathArguments, Result, Visibility, NestedMeta,
};

const MUTABLE_VEC: &str = "MutableVec";
const MUTABLE: &str = "Mutable";
const SIGNAL_LOWER: &str = "signal";
const MUTABLE_LOWER: &str = "mutable";

fn remover(
    angle_args: &AngleBracketedGenericArguments,
    attrs: &AttributeOptions,
) -> Result<TokenStream> {
    let args = &angle_args.args;
    match args.first() {
        Some(arg) => match arg {
            syn::GenericArgument::Type(t) => match t {
                syn::Type::Path(p) => remove_wrappers(&p.path, attrs),
                _ => Ok(quote! {#args}),
            },
            _ => Ok(quote! {#args}),
        },
        None => unreachable!(),
    }
}

fn remove_wrappers(path: &Path, attrs: &AttributeOptions) -> Result<TokenStream> {
    match path.segments.last() {
        Some(s) => {
            if s.ident == MUTABLE
                || (s.ident == "Rc" && attrs.keep_rc.is_none())
                || (s.ident == "Arc" && attrs.keep_arc.is_none())
            {
                match &s.arguments {
                    PathArguments::AngleBracketed(angle_args) => remover(angle_args, attrs),
                    _ => unreachable!(),
                }
            } else if s.ident == MUTABLE_VEC {
                match &s.arguments {
                    PathArguments::AngleBracketed(angle_args) => {
                        remover(angle_args, attrs).map(|args| quote! { Vec<#args> })
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
    attrs: &AttributeOptions,
    final_type: Option<Result<TokenStream>>,
) -> Result<TokenStream> {
    // TODO: Do I need to add back the atts?
    let atts = field.attrs.iter().filter(|a|AttributeOptions::is_not_designal_att(a));
    let vis = &field.vis;
    let default_ty = &field.ty;
    let ty = final_type.unwrap_or_else(|| Ok(quote! { #default_ty }))?;
    match (&field.ident, &attrs.renamer) {
        (Some(current), Some(renamer)) => renamer
            .make_new_name(&current)
            .map(|name| quote! { #(#atts)* #vis #name: #ty }),
        (Some(name), None) => Ok(quote! { #(#atts)* #vis #name: #ty }),
        _ => Ok(quote! { #(#atts)* #vis #ty }),
    }
}

fn map_field(
    field: &Field,
    naming: Naming,
    struct_level_atts: &AttributeOptions,
) -> Option<Result<TokenStream>> {
    let attrs = match AttributeOptions::new(AttributeLocation::Field(naming, &field.attrs)) {
        Ok(attrs) => attrs.add_struct_level_to_field_level(struct_level_atts),
        Err(e) => return Some(Err(e)),
    };
    if attrs.remove.is_some() {
        None
    } else if attrs.ignore.is_some() {
        Some(clean_field(&field, &attrs, None))
    } else {
        let tokens = if let syn::Type::Path(p) = &field.ty {
            let final_type = Some(remove_wrappers(&p.path, &attrs));
            clean_field(&field, &attrs, final_type)
        } else {
            clean_field(&field, &attrs, None)
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
pub(crate) struct ReturnStruct<'a> {
    vis: &'a Visibility,
    name: Ident,
    fields: TokenStream,
    derives: Option<TokenStream>,
    naming: Naming,
    generics: &'a Generics,
}

impl<'a> ReturnStruct<'a> {
    fn new(
        name: Ident,
        data_struct: &'a ItemStruct,
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
            vis: &data_struct.vis,
            name,
            fields,
            derives,
            naming,
            generics: &data_struct.generics,
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
        let derives = &self.derives;
        let generics = self.generics;
        let wher = &self.generics.where_clause;

        let tokens = match self.naming {
            Naming::Named => {
                quote! {
                    #derives
                    #vis struct #name #generics
                    #wher {
                        #fields
                    }
                }
            }
            Naming::Unnamed => {
                quote! {
                    #derives
                    #vis struct #name #generics (#fields)
                    #wher;
                }
            }
        };
        Ok(tokens)
    }

    pub(crate) fn parse_input(input: ItemStruct, attrs: Vec<NestedMeta>) -> Result<TokenStream> {
        // TODO: No need to parse these attrs? Since they are separate to this marcro so need to just parse through
        let struct_atts = AttributeOptions::new(AttributeLocation::Struct(attrs))?;
        let name = Self::make_name(&input.ident, &struct_atts)?;

        let ty = match &input.fields {
            syn::Fields::Named(_) => ReturnStruct::new(name, &input, Naming::Named, &struct_atts),
            syn::Fields::Unnamed(_) => {
                ReturnStruct::new(name, &input, Naming::Unnamed, &struct_atts)
            }
            syn::Fields::Unit => Err(Error::new(
                input.ident.span(),
                "Unit structs are not supported",
            )),
        };
        ty?.build()
    }
}
