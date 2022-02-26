use std::borrow::Borrow;
use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use std::hash::BuildHasher;
use std::hash::Hash;

use indexmap::Equivalent;

/// Multimap implementation that behaves like `HashMap<K, Vec<V>>`.
#[derive(Debug, Clone)]
pub struct HashVecMultimap<K, V, S = RandomState> {
    inner: HashMap<K, Vec<V>, S>,
    len: usize,
}

impl<K, V> HashVecMultimap<K, V> {
    multimap_base_impl! { HashMap<K,Vec<V>>}
}

impl<K, V, S> HashVecMultimap<K, V, S> {
    multimap_base2_impl! {HashMap}
}

impl<K, V, S> HashVecMultimap<K, V, S>
where
    K: Hash + Eq,
    V: Eq,
    S: BuildHasher + Default,
{
    multimap_mutators_impl! {
        HashMap<K, Vec<V>, S>,
        Vec<V>,
        Vec::new(),
        vec,
        (K: Borrow<Q>, Q: Hash + Eq),
        (V: Borrow<R>, R: Equivalent<V>)
    }
}

multimap_extend! {
    HashVecMultimap,
    (K, V, S),
    HashMap,
    Vec<V>,
    (K: Hash + Eq),
    (V: Eq),
    (K: Hash + Eq + Borrow<Q>, Q: Hash + Eq)
}
multimap_eq! { HashVecMultimap, (Eq)}
