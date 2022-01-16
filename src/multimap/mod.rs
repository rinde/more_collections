use std::borrow::Borrow;
use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::BuildHasher;
use std::hash::Hash;

use indexmap::Equivalent;
use indexmap::IndexMap;
use indexmap::IndexSet;

#[macro_use]
mod create_macros;

#[macro_use]
mod gen_macros;

use crate::multimap_base2_impl;
use crate::multimap_base_impl;
use crate::multimap_eq;
use crate::multimap_extend;
use crate::multimap_mutators_impl;

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

/// Multimap implementation that behaves like `IndexMap<K, Vec<V>>`.
#[derive(Debug, Clone)]
pub struct IndexVecMultimap<K, V, S = RandomState> {
    inner: IndexMap<K, Vec<V>, S>,
    len: usize,
}

impl<K, V> IndexVecMultimap<K, V> {
    multimap_base_impl! { IndexMap<K,Vec<V>>}
}

impl<K, V, S> IndexVecMultimap<K, V, S> {
    multimap_base2_impl! {IndexMap}
}

impl<K, V, S> IndexVecMultimap<K, V, S>
where
    K: Hash + Eq,
    V: Eq,
    S: BuildHasher + Default,
{
    multimap_mutators_impl! {
        IndexMap<K, Vec<V>, S>,
        Vec<V>,
        Vec::new(),
        vec,
        (Q: Hash + Equivalent<K>),
        (R: Equivalent<V>)
    }

    index_multimap_impl! {
        IndexMap<K, Vec<V>, S>,
        Vec<V>,
        Vec::new(),
        vec,
        (Q: Hash + Equivalent<K>),
        (R: Equivalent<V>)
    }
}

multimap_extend! {
    IndexVecMultimap,
    (K, V, S),
    IndexMap,
    Vec<V>,
    (K: Hash + Eq),
    (V: Eq),
    (K: Hash + Eq, Q: Hash + Equivalent<K>)
}
multimap_eq! { IndexVecMultimap, (Eq)}

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
