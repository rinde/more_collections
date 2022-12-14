//! More collection types.
//!
//! # Multimap
//!
//! A collection that maps keys to values, similar to [`HashMap`], but where
//! each key may be associated with _multiple_ values. Multimaps can be
//! visualized as a map from keys to non-empty collections of values:
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
//! | Name               | Behaves as                          | Keys order          | Values order        | May contain duplicates |
//! | ------------------ | ----------------------------------- | ------------------- | ------------------- | ---------------------- |
//! | [HashSetMultimap]  | [`HashMap`]`<K,`[`HashSet`]`<V>>`   | Arbitrary order     | Arbitrary order     | No                     |
//! | [HashVecMultimap]  | [`HashMap`]`<K,`[`Vec`]`<V>>`       | Arbitrary order     | Insertion order[^1] | Yes                    |
//! | [IndexSetMultimap] | [`IndexMap`]`<K,`[`IndexSet`]`<V>>` | Insertion order[^1] | Insertion order[^1] | No                     |
//! | [IndexVecMultimap] | [`IndexMap`]`<K, `[`Vec`]`<V>>`     | Insertion order[^1] | Insertion order[^1] | Yes                    |
//!
//! [^1]: Insertion order is preserved, unless `remove()` or `swap_remove()`
//! is called. See more in the [IndexMap](https://docs.rs/indexmap/1.7.0/indexmap/map/struct.IndexMap.html#order) documentation.
//!
//! # Crate features
//! All features are _disabled_ by default. The options are:
//! - `hashsetmultimap`
//! - `hashvecmultimap`
//! - `indexsetmultimap`
//! - `indexvecmultimap`
//!
//! [`HashMap`]: std::collections::HashMap
//! [`HashSet`]: std::collections::HashSet
//! [`IndexMap`]: indexmap::IndexMap
//! [`IndexSet`]: indexmap::IndexSet
//! [`Vec`]: std::vec::Vec

mod multimap;
#[cfg(all(feature = "indexmap", feature = "small_vec", feature = "smallmap"))]
mod small_map;
#[cfg(all(
    feature = "indexmap",
    feature = "small_vec",
    feature = "smallmap",
    feature = "smallset"
))]
mod small_set;

pub use multimap::*;

#[cfg(all(feature = "indexmap", feature = "small_vec", feature = "smallmap"))]
pub use small_map::*;

#[cfg(all(
    feature = "indexmap",
    feature = "small_vec",
    feature = "smallmap",
    feature = "smallset"
))]
pub use small_set::*;

// TODO follow all guidelines here https://rust-lang.github.io/api-guidelines/checklist.html
