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
//! ```rust
//! use designal::Designal;
//! use futures_signals::signal::Mutable;
//! use futures_signals::signal_vec::MutableVec;
//! use std::rc::Rc;
//!
//! #[derive(Designal)]
//! #[designal(trim_end = "Signal", derive = "Debug")]
//! struct FlavoursSignal(MutableVec<String>);
//!
//! #[derive(Designal)]
//! #[designal(trim_end = "Signal", derive = "Debug")]
//! struct TasteSignal {
//!     salt: Mutable<u32>,
//!     sweet: Mutable<bool>,
//!     sour: Mutable<Rc<i8>>,
//!     #[designal(trim_end = "Signal")]
//!     flavours: FlavoursSignal,
//! }
//!
//! #[derive(Designal)]
//! #[designal(trim_end = "Signal", derive = "Debug")]
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

mod attributes;
mod builder;
use std::{
    fs::OpenOptions,
    io::{Read, Write},
    path::PathBuf,
};
use syn::{parse_macro_input, DeriveInput};

/// This is used to check if the files should be generated from the derive calls
const FILE_MESSAGE: &[u8] = "// Generated\n".as_bytes();
const FILE_NAME: &str = "out.rs";

/// Finds the output path & creates it if required
fn output_path(make_dir_path: bool) -> Option<PathBuf> {
    match std::env::var("CARGO_MANIFEST_DIR") {
        Ok(root) => {
            let mut path = PathBuf::from(root);
            path.push("target/designal");
            if make_dir_path {
                std::fs::create_dir_all(&path).unwrap();
            }
            path.push(FILE_NAME);
            Some(path)
        }
        Err(_) => None,
    }
}

fn write_derive_to_file(tokens: &proc_macro2::TokenStream) {
    if let Some(path) = output_path(false) {
        if let Ok(mut file) = OpenOptions::new()
            .write(true)
            .append(true)
            .read(true)
            .open(&path)
        {
            let mut buf = [0; FILE_MESSAGE.len()];
            match file.read_exact(&mut buf) {
                Ok(_) => {
                    if buf == FILE_MESSAGE {
                        file.write_all(tokens.to_string().as_bytes()).unwrap();
                    }
                }
                Err(_) => {
                    if let Some(path) = path.parent() {
                        std::fs::remove_dir_all(path).unwrap_or(());
                    }
                }
            };
        }
    }
}

// This is an experimental feature to enabling writing the output to a file since can't know order of compilation
// OUT_DIR not set, but CARGO_MANIFEST_DIR is
// include!() can used to load generated files
// It looks like modules are processed by the order of import
// Can't do as a attribute marco:
// non-inline modules in proc macro input are unstable see issue #54727 <https://github.com/rust-lang/rust/issues/54727
// This is feature gated https://docs.rs/proc-macro2/1.0.24/proc_macro2/struct.Span.html#method.source_file so
// can't find the current file from the derive macro

/// This must be in the root `lib.rs` file and appear before any module imports that you want to write to the
/// file. It creates a file `target/designal/out.rs` that has all the generated code in
#[proc_macro]
pub fn write_to_file(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    if let Some(path) = output_path(true) {
        let mut file = std::fs::File::create(&path).unwrap_or_else(|_| {
            panic!(
                "Failed to create file for designal output at path: {:?}",
                path
            )
        });
        file.write_all(FILE_MESSAGE).unwrap()
    }
    item
}

#[proc_macro_derive(Designal, attributes(designal))]
pub fn designal(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let tokens = builder::parse_input(input).unwrap_or_else(|err| err.to_compile_error());
    // TODO: Return if want to write from the builder? / Split into its own macro?
    write_derive_to_file(&tokens);
    tokens.into()
}
