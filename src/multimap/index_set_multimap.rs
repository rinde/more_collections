use std::borrow::Borrow;
use std::collections::hash_map::RandomState;
use std::hash::BuildHasher;
use std::hash::Hash;

use indexmap::Equivalent;
use indexmap::IndexMap;
use indexmap::IndexSet;

/// Multimap implementation that behaves like `IndexMap<K, IndexSet<V>>`.
#[derive(Debug, Clone)]
pub struct IndexSetMultimap<K, V, S = RandomState> {
    inner: IndexMap<K, IndexSet<V, S>, S>,
    len: usize,
}

impl<K, V> IndexSetMultimap<K, V> {
    multimap_base_impl! {IndexMap<K, IndexSet<V>>}
}

impl<K, V, S> IndexSetMultimap<K, V, S> {
    multimap_base2_impl! {IndexMap}
}

impl<K, V, S> IndexSetMultimap<K, V, S>
where
    K: Hash + Eq,
    V: Hash + Eq,
    S: BuildHasher + Default,
{
    multimap_mutators_impl! {
        IndexMap<K, IndexSet<V,S>, S>,
        IndexSet<V,S>,
        IndexSet::with_hasher(S::default()),
        set,
        (Q: Hash + Equivalent<K>),
        (R: Hash + Equivalent<V>)
    }
    index_multimap_impl! {
        IndexMap<K, IndexSet<V,S>, S>,
        IndexSet<V,S>,
        IndexSet::with_hasher(S::default()),
        set,
        (Q: Hash + Equivalent<K>),
        (R: Hash + Equivalent<V>)
    }
}

multimap_extend! {
    IndexSetMultimap,
    (K, V, S),
    IndexMap,
    IndexSet<V,S>,
    (K: Hash + Eq),
    (V: Hash + Eq),
    (K: Hash + Eq, Q: Hash + Equivalent<K>)
}
multimap_eq! { IndexSetMultimap, (Hash + Eq)}

impl_into_iterator! {
    IndexSetMultimap,
    (K,V,S),
    indexmap::map::IntoIter<K, IndexSet<V, S>>,
    indexmap::set::IntoIter<V>
}
impl_into_keys! {IndexSetMultimap, (K, V, S), indexmap::map::IntoKeys<K, IndexSet<V, S>>}
