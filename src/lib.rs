//! More collection types.
//!
//! # Small* collections
//!
//! Built on top of the excellent [smallvec](https://github.com/servo/rust-smallvec)
//! crate, [`SmallMap`] and [`SmallSet`] are a `Map` and `Set` respectively that
//! are inlined if they contain fewer values than a (statically chosen)
//! capacity `C`, otherwise they are heap allocated and backed by an
//! `IndexMap`.
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
//! | Name               | Behaves as                          | Keys order
//! | Values order        | May contain duplicates | | ------------------ |
//! ----------------------------------- | ------------------- |
//! ------------------- | ---------------------- | | [`HashSetMultimap`]  |
//! [`HashMap`]`<K,`[`HashSet`]`<V>>`   | Arbitrary order     | Arbitrary order
//! | No                     | | [`HashVecMultimap`]  |
//! [`HashMap`]`<K,`[`Vec`]`<V>>`       | Arbitrary order     | Insertion
//! order[^1] | Yes                    | | [`IndexSetMultimap`] |
//! [`IndexMap`]`<K,`[`IndexSet`]`<V>>` | Insertion order[^1] | Insertion
//! order[^1] | No                     | | [`IndexVecMultimap`] |
//! [`IndexMap`]`<K, `[`Vec`]`<V>>`     | Insertion order[^1] | Insertion
//! order[^1] | Yes |
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
#[cfg(all(feature = "indexmap", feature = "smallvec", feature = "smallmap"))]
pub mod small_map;
#[cfg(all(
    feature = "indexmap",
    feature = "smallvec",
    feature = "smallmap",
    feature = "smallset"
))]
pub mod small_set;

pub mod vec_map;

// TODO add feature
pub use multimap::*;
#[cfg(all(feature = "indexmap", feature = "smallvec", feature = "smallmap"))]
pub use small_map::SmallMap;
#[cfg(all(
    feature = "indexmap",
    feature = "smallvec",
    feature = "smallmap",
    feature = "smallset"
))]
pub use small_set::SmallSet;
pub use vec_map::IndexKey;
pub use vec_map::VecMap;

// TODO follow all guidelines here https://rust-lang.github.io/api-guidelines/checklist.html
