use std::borrow::Borrow;
use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::BuildHasher;
use std::hash::Hash;

/// Multimap implementation that behaves like `HashMap<K, HashSet<V>>`.
#[derive(Debug, Clone)]
pub struct HashSetMultimap<K, V, S = RandomState> {
    inner: HashMap<K, HashSet<V, S>, S>,
    len: usize,
}

impl<K, V> HashSetMultimap<K, V, RandomState> {
    multimap_base_impl! {HashMap<K, HashSet<V>>}
}

impl<K, V, S> HashSetMultimap<K, V, S> {
    multimap_base2_impl! {HashMap}
}

impl<K, V, S> HashSetMultimap<K, V, S>
where
    K: Hash + Eq,
    V: Hash + Eq,
    S: BuildHasher + Default,
{
    multimap_mutators_impl! {
        HashMap<K, HashSet<V,S>, S>,
        HashSet<V,S>,
        HashSet::with_hasher(S::default()),
        set,
        (K: Borrow<Q>, Q: Hash + Eq),
        (V: Borrow<R>, R: Hash + Eq)
    }
}

multimap_extend! {
    HashSetMultimap,
    (K, V, S),
    HashMap,
    HashSet<V,S>,
    (K: Hash + Eq),
    (V: Hash + Eq),
    (K: Hash + Eq + Borrow<Q>, Q: Hash + Eq)
}
multimap_eq! { HashSetMultimap, (Hash + Eq)}
