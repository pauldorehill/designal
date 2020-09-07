//! ```
//! use designal::Designal;
//! use futures_signals::signal::Mutable;
//! use futures_signals::signal_vec::MutableVec;
//! use std::rc::Rc;
//!
//! #[derive(Designal, Debug)]
//! #[designal(derive = "Debug")]
//! struct HumanBean<'a, T> {
//!     taste: Mutable<Rc<T>>,
//!     crunch: Mutable<u32>,
//!     flavours: MutableVec<&'a str>,
//!     #[designal(remove)]
//!     editing: bool,
//! }
//!
//! let bean = HumanBeanDesig {
//!     taste: String::from("turkey"),
//!     crunch: 10,
//!     flavours: vec!["snozz"],
//! };
//! ```

mod attributes;
mod builder;
use builder::ReturnStruct;
use quote::quote;
use syn::{parse_macro_input, spanned::Spanned, AttributeArgs, Error, Item};

// TODO: Should this be an attribute macro? Since its not really a trait
// this would also allow parsing of other derives onto the child and attaching to mod
// TODO: Remove features when working
#[proc_macro_attribute]
pub fn designal(
    atts: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let item = parse_macro_input!(item as Item);
    let atts = parse_macro_input!(atts as AttributeArgs);
    let existing = quote! { #item };

    let new_tokens = match item {
        Item::Mod(m) => Err(Error::new(m.span(), "Not done for modules yet")),
        Item::Struct(s) => {
            ReturnStruct::parse_input(s, atts)
        }
        _ => Err(Error::new(
            item.span(),
            "designal only works for structs and modules (wip)",
        )),
    }
    .unwrap_or_else(|err| err.to_compile_error());
    
    let tokens = quote! {
        #existing
        #new_tokens
    };
    tokens.into()
}
