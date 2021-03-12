//! This is a `Derive` macro that trys to help reduce code duplication between the front and backend
//! when using [futures-signals](https:docs.rs/futures-signals) and [dominator](https:docs.rs/dominator/).
//! When using signals you have to wrap a lot of types in a `Mutable`, `MutableVec`, and `MutableBTreeMap` which you likely don't want to have on your backend code. It will recursively trim away the following types from `struct` fields:
//!
//! - `Mutable<T>` -> `T`
//! - `MutableVec<T>` -> `Vec<T>`
//! - `MutableBTreeMap<K, V>` -> `BTreeMap<K, V>`
//! - `MutableBTreeMap<K, ()>` -> `BTreeSet<K>`
//! - `Rc<T>` -> `T`
//! - `Arc<T>` -> `T`
//!
//! See the [Container Attributes](#container-attributes) and [Field Attributes](#field-attributes) section for some configuration options.
//!
//! ```
//! use designal::Designal;
//! use futures_signals::signal::Mutable;
//! use futures_signals::signal_vec::MutableVec;
//! use std::rc::Rc;
//!
//! #[derive(Designal)]
//! #[designal(trim_end = "Signal")]
//! #[designal(attribute = #[derive(Debug)])]
//! struct FlavoursSignal(MutableVec<String>);
//!
//! #[derive(Designal)]
//! #[designal(trim_end = "Signal")]
//! #[designal(attribute = #[derive(Debug)])]
//! struct TasteSignal {
//!     salt: Mutable<u32>,
//!     sweet: Mutable<bool>,
//!     sour: Mutable<Rc<i8>>,
//!     #[designal(trim_end = "Signal")]
//!     flavours: FlavoursSignal,
//! }
//!
//! #[derive(Designal)]
//! #[designal(trim_end = "Signal")]
//! #[designal(attribute = #[derive(Debug)])]
//! struct HumanSignal {
//!     #[designal(trim_end = "Signal")]
//!     taste: Rc<TasteSignal>,
//!     name: Mutable<(String, String)>,
//!     #[designal(remove)]
//!     editing: Mutable<bool>,
//! }
//! ```
//!
//! Generates this code:
//! ```rust
//! #[derive(Debug)]
//! struct Flavours(Vec<String>);
//!
//! #[derive(Debug)]
//! struct Taste {
//!     salt: u32,
//!     sweet: bool,
//!     sour: i8,
//!     flavours: Flavours,
//! }
//!
//! #[derive(Debug)]
//! struct Human {
//!     taste: Taste,
//!     name: (String, String),
//! }
//! ```
//!
//! ## Container Attributes
//! Every struct will need to have one of the renaming attributes `rename`, `add_start`, `add_end`, `trim_start`, `trim_start_all`, `trim_end`, or `trim_end_all`.
//! #### `#[designal(rename = "NewName")]`
//! Renames the struct completely.
//! #### `#[designal(add_start = "Prefix")]`
//! Renames the struct by adding the string to the start of the struct identifier.
//! #### `#[designal(add_end = "Postfix")]`
//! Renames the struct by adding the string to the end of the struct identifier.
//! #### `#[designal(trim_start = "Prefix")]`
//! Renames the struct by removing the string from the start of the struct identifier.
//! #### `#[designal(trim_start_all = "Prefix")]`
//! Renames the struct by removing the string from the start of the struct identifier and also renames any field types that start with the same prefix. If a field doesn't start with the prefix it is left as is; if the field has its own renamer that will take precedence.
//! #### `#[designal(trim_end = "Postfix")]`
//! Renames the struct by removing the string from the end of the struct identifier.
//! #### `#[designal(trim_end_all = "Postfix")]`
//! Renames the struct by removing the string from the end of the struct identifier and also renames any field types that end with the same postfix. If a field doesn't end with the postfix it is left as is; if the field has its own renamer that will take precedence.
// #### `#[designal(derive = "Debug")]`
// Adds a derive attribute to the generated struct. Can accept a list of csv values `#[designal(derive = "Serialize, Deserialize, Debug, Default")]`; be used multiple times; or like`#[designal(derive = "Debug", derive = "PartialEq")]`.
//! #### `#[designal(cfg_feature = "your_feature")]`
//! Adds a `#[cfg(feature = "your_feature")]` attribute to the generated struct.
//! #### `#[designal(keep_rc)]`
//! Keeps any `Rc`'s used on any fields.
//! #### `#[designal(keep_arc)]`
//! Keeps any `Arc`'s used on any fields.
//! #### `#[designal(hashmap)]`
//! If any field is a `MutableBTreeMap<K, V>` returns it as a `HashMap<K, V>` rather than the default of `BTreeMap<K, V>`. If any field is `MutableBTreeMap<K, ()>` returns it as a `HashSet<K>`.
//!
//! ## Field Attributes
//! #### `#[designal(rename = "NewName")]`
//! Renames the field's declared type completely.
//! #### `#[designal(add_start = "Prefix")]`
//! Renames the field's declared type by adding the string to the start of the field's declared type identifier.
//! #### `#[designal(add_end = "Postfix")]`
//! Renames the field's declared type by adding the string to the end of the field's declared type identifier.
//! #### `#[designal(trim_start = "Prefix")]`
//! Renames the field's declared type by removing the string from the start of the field's declared type identifier.
//! #### `#[designal(trim_end = "Postfix")]`
//! Renames the field's declared type by removing the string from the end of the field's declared type identifier.
//! #### `#[designal(remove)]`
//! Removes the field from the generated struct.
//! #### `#[designal(ignore)]`
//! Tells `designal` to leave the field alone and return it as is.
//! #### `#[designal(keep_rc)]`
//! Keeps any `Rc`'s used in the field.
//! #### `#[designal(keep_arc)]`
//! Keeps any `Arc`'s used in the field.
//! #### `#[designal(hashmap)]`
//! If the field is a `MutableBTreeMap<K, V>` returns it as a `HashMap<K, V>` rather than the default of `BTreeMap<K, V>`. If it is `MutableBTreeMap<K, ()>` returns it as a `HashSet<K>`.

mod attribute_parser;
mod attributes;
mod builder;
mod capture;
use std::{
    fs::File,
    io::{Read, Write},
};
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Designal, attributes(designal))]
pub fn designal(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let tokens = builder::parse_input(input).unwrap_or_else(|err| err.to_compile_error());
    // TODO: Split into its own macro / option not to write?
    let action = |mut file: File| file.write_all(tokens.to_string().as_bytes()).unwrap();
    capture::edit_file(action);
    tokens.into()
}

// This is an experimental feature to enabling writing the output to a file since can't know order of compilation
// OUT_DIR not set, but CARGO_MANIFEST_DIR is. `include!()` could be used to load generated files...
// It looks like modules are processed by the order of import
// Can't do as a attribute marco:
// non-inline modules in proc macro input are unstable see issue #54727 <https://github.com/rust-lang/rust/issues/54727
// Can't yet find the current file from the derive macro:
// https://docs.rs/proc-macro2/1.0.24/proc_macro2/struct.Span.html#method.source_file

/// Highly experimental and will change
/// Creates a file `target/designal/out.rs` that has all the generated code in. Whenever this macro is
/// called it truncates the output file and then allows any subsequent calls to `#[derive(Disignal)]` to
/// write to the `out.rs` file. For example using in `lib.rs` like this
/// ```compile_fail
/// designal::write_to_file!();
/// mod mod1;
/// mod mod2;
/// ```
/// Both `mod1` & `mod2` would be written to the output.
/// ```compile_fail
/// mod mod1;
/// designal::write_to_file!();
/// mod mod2;
/// ```
/// Would mean only `mod2` was written to the output.
/// ```compile_fail
/// designal::write_to_file!();
/// mod mod1;
/// designal::write_to_file!();
/// mod mod2;
/// ```
/// Would mean only `mod1` was written to the output and then truncated; then finally `mod2` would be
/// written to the output.
#[proc_macro]
pub fn start_write_to_file(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    if let Some(path) = capture::output_path(true) {
        let mut file = File::create(&path).unwrap_or_else(|_| {
            panic!(
                "Failed to create file for designal output at path: {:?}",
                path
            )
        });
        file.write_all(capture::FILE_MESSAGE).unwrap()
    }
    item
}

/// Manual flag to stop any further writes to the 'out.rs' file
#[proc_macro]
pub fn stop_write_to_file(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let action = |mut file: File| {
        let mut buf = String::new();
        file.read_to_string(&mut buf).unwrap();
        file.set_len(0).unwrap();
        file.write_all(buf.as_bytes()).unwrap();
    };
    capture::edit_file(action);
    item
}
