//! More collection types.
//!
//! # Multimap
//!
//! A collection that maps keys to values, similar to
//! [HashMap](std::collections::HashMap), but where each key may be associated
//! with _multiple_ values. Multimaps can be visualized as a map from keys to
//! non-empty collections of values:
//! - a → 0, 1
//! - b → 2
//!
//! Or, it can be visualized as a collection of key-value pairs:
//! - a → 0
//! - a → 1
//! - b → 2
//!
//! The multimap API is based on the second form, `len() == 3` and `keys_len()
//! == 2` for the above example.
//!
//! | Name               | Behaves as |
//! | ------------------ | ----- |
//! | [HashSetMultimap]  | [HashMap](std::collections::HashMap)`<K,
//! `[HashSet](std::collections::HashSet)`<V>>` | [HashVecMultimap]  |
//! [HashMap](std::collections::HashMap)`<K, `[Vec](std::vec::Vec)`<V>>`
//! | [IndexSetMultimap] | [IndexMap](indexmap::IndexMap)`<K,
//! `[HashSet](indexmap::IndexSet)`<V>>` | [IndexVecMultimap] |
//! [IndexMap](indexmap::IndexMap)`<K, `[Vec](std::vec::Vec)`<V>>`

mod multimap;

pub use multimap::*;

// TODO follow all guidelines here https://rust-lang.github.io/api-guidelines/checklist.html
