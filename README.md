 This is a `Derive` macro that trys to help reduce code duplication between the front and backend
 when using [futures-signals](https:docs.rs/futures-signals) and [dominator](https:docs.rs/dominator/).
 When using signals you have to wrap a lot of types in a `Mutable`, `MutableVec`, and `MutableBTreeMap` which you likely don't want
 to have on your backend code.

 It will recursively trim away the following types from `struct` fields:
 - `Mutable<T>` -> `T`
 - `MutableVec<T>` -> `Vec<T>`
 - `MutableBTreeMap<K, V>` -> `BTreeMap<K, V>`
 - `Rc<T>` -> `T`
 - `Arc<T>` -> `T`

 See the [Container Attributes](#container-attributes) and [Field Attributes](#field-attributes) section for some configuration options.

 ```rust
 use designal::Designal;
 use futures_signals::signal::Mutable;
 use futures_signals::signal_vec::MutableVec;
 use std::rc::Rc;

 #[derive(Designal)]
 #[designal(trim_end = "Signal", derive = "Debug")]
 struct FlavoursSignal(MutableVec<String>);

 #[derive(Designal)]
 #[designal(trim_end = "Signal", derive = "Debug")]
 struct TasteSignal {
     salt: Mutable<u32>,
     sweet: Mutable<bool>,
     sour: Mutable<Rc<i8>>,
     #[designal(trim_end = "Signal")]
     flavours: FlavoursSignal,
 }

 #[derive(Designal)]
 #[designal(trim_end = "Signal", derive = "Debug")]
 struct HumanSignal {
     #[designal(trim_end = "Signal")]
     taste: Rc<TasteSignal>,
     name: Mutable<(String, String)>,
     #[designal(remove)]
     editing: Mutable<bool>,
 }
 ```

 Generates this code:
 ```rust
 #[derive(Debug)]
 struct Flavours(Vec<String>);

 #[derive(Debug)]
 struct Taste {
     salt: u32,
     sweet: bool,
     sour: i8,
     flavours: Flavours,
 }

 #[derive(Debug)]
 struct Human {
     taste: Taste,
     name: (String, String),
 }
 ```

 # Container Attributes
 Every struct will need to have one of the renaming attributes `rename`, `add_start`, `add_end`, `trim_start`, or `trim_end`.
 ### `#[designal(rename = "NewName")]`
 Renames the struct completely.
 ### `#[designal(add_start = "Prefix")]`
 Renames the struct by adding the string to the start of the struct identifier. Will throw a compile error if the
 string isn't found.
 ### `#[designal(add_end = "Postfix")]`
 Renames the struct by adding the string to the end of the struct identifier. Will throw a compile error if the
 string isn't found.
 ### `#[designal(trim_start = "Prefix")]`
 Renames the struct by removing the string from the start of the struct identifier. Will throw a compile error if the
 string isn't found.
 ### `#[designal(trim_end = "Postfix")]`
 Renames the struct by removing the string from the end of the struct identifier. Will throw a compile error if the
 string isn't found.
 ### `#[designal(derive = "Debug")]`
 Adds a derive attribute to the generated struct.
 ### `#[designal(cfg_feature = "your_feature")]`
 Adds a `#[cfg(feature = "your_feature")]` attribute to the generated struct.
 ### `#[designal(keep_rc)]`
 Keeps any `Rc`'s used on any fields.
 ### `#[designal(keep_arc)]`
 Keeps any `Arc`'s used on any fields.
 ### `#[designal(hashmap)]`
 If any field is a `MutableBTreeMap<K, V>` returns it as a `HashMap<K, V>` rather than the default of `BTreeMap<K, V>`.

 # Field Attributes
 ### `#[designal(rename = "NewName")]`
 Renames the field's declared type completely.
 ### `#[designal(add_start = "Prefix")]`
 Renames the field's declared type by adding the string to the start of the field's declared type identifier. Will throw a compile error if the
 string isn't found.
 ### `#[designal(add_end = "Postfix")]`
 Renames the field's declared type by adding the string to the end of the field's declared type identifier. Will throw a compile error if the
 string isn't found.
 ### `#[designal(trim_start = "Prefix")]`
 Renames the field's declared type by removing the string from the start of the field's declared type identifier. Will throw a compile error if the
 string isn't found.
 ### `#[designal(trim_end = "Postfix")]`
 Renames the field's declared type by removing the string from the end of the field's declared type identifier. Will throw a compile error if the
 string isn't found.
 Instructs the `designal` macro not to touch this field and leave it as it is.
 ### `#[designal(remove)]`
 Removes the field from the generated struct.
 ### `#[designal(keep_rc)]`
 Keeps any `Rc`'s used in the field.
 ### `#[designal(keep_arc)]`
 Keeps any `Arc`'s used in the field.
 ### `#[designal(hashmap)]`
 If the field is a `MutableBTreeMap<K, V>` returns it as a `HashMap<K, V>` rather than the default of `BTreeMap<K, V>`.
