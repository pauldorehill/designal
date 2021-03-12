use crate::attributes::AttributeType;
use proc_macro2::Ident;
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Attribute, Error, Meta, Result, Token,
};
struct AttributesAttribute {
    _name: Ident,
    _equals: Token!(=),
    attribute: Punctuated<Vec<Attribute>, Token!(,)>,
}

impl Parse for AttributesAttribute {
    fn parse(input: ParseStream) -> Result<AttributesAttribute> {
        let name = input.parse::<Ident>().and_then(|x| {
            if x == AttributeType::ATTRIBUTE {
                Ok(x)
            } else {
                Err(syn::Error::new(
                    x.span(),
                    format!(
                        "Designal only expects tokens in this form for use with `{} = #[atts]`",
                        AttributeType::ATTRIBUTE
                    ),
                ))
            }
        });
        Ok(AttributesAttribute {
            _name: name?,
            _equals: input.parse()?,
            attribute: input.parse_terminated(Attribute::parse_outer)?,
        })
    }
}

// TODO: Allow combining of two types
// At the moment this can't be done:
// #[designal(trim_end = "Signal", attribute = #[derive(Debug)])]
// Also / switch to use of string?
// #[designal(attribute = "#[derive(Debug)]", trim_end = "Bean")]
pub(crate) fn parse(att: &Attribute) -> Vec<Result<AttributeType>> {
    match att.parse_meta() {
        Ok(meta) => match meta {
            syn::Meta::List(meta_list) => meta_list.nested.iter().map(AttributeType::new).collect(),
            Meta::Path(p) => vec![Err(Error::new(
                p.segments[0].ident.span(),
                "Unsupported attribute type",
            ))],
            Meta::NameValue(nv) => {
                vec![Err(Error::new(nv.lit.span(), "Unsupported attribute type"))]
            }
        },
        Err(orig) => match att.parse_args::<AttributesAttribute>() {
            Ok(t) => t
                .attribute
                .iter()
                .map(|a| Ok(AttributeType::Attributes(quote::quote! { #(#a) *})))
                .collect(),
            Err(_) => {
                vec![Err(Error::new(
                    orig.span(),
                    "Could not parse Designal the attributes",
                ))]
            }
        },
    }
}
