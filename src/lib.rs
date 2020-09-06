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
use builder::ReturnType;
use syn::{parse_macro_input, DeriveInput};

// TODO: Should this be an attribute macro? Since its not really a trait
// this would also allow parsing of other derives onto the child and attaching to mod
#[proc_macro_derive(Designal, attributes(designal))]
pub fn designal(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let tokens = ReturnType::parse_input(input).unwrap_or_else(|err| err.to_compile_error());
    tokens.into()
}
