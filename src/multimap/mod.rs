use std::borrow::Borrow;
use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::BuildHasher;
use std::hash::Hash;
use std::iter::FusedIterator;

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

/// An owning iterator over the entries of a multimap.
pub struct IntoIter<K, V, S> {
    outer: indexmap::map::IntoIter<K, IndexSet<V, S>>,
    inner: Option<(K, indexmap::set::IntoIter<V>)>,
    len: usize,
}

impl<K, V, S> Iterator for IntoIter<K, V, S>
where
    K: Clone,
{
    type Item = (K, V);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((current_key, inner_iter)) = &mut self.inner {
            let next = inner_iter.next();

            if let Some(next_value) = next {
                Some((current_key.clone(), next_value))
            } else {
                if let Some((key, values)) = self.outer.next() {
                    let mut new_inner_iter = values.into_iter();
                    let v = new_inner_iter.next().unwrap();
                    self.inner = Some((key.clone(), new_inner_iter));

                    Some((key, v))
                } else {
                    None
                }
            }
        } else {
            None
        }
    }
}

impl<K: Clone, V, S> ExactSizeIterator for IntoIter<K, V, S> {
    fn len(&self) -> usize {
        self.len
    }
}

impl<K: Clone, V, S> FusedIterator for IntoIter<K, V, S> {}

impl<K: Clone, V, S> IntoIterator for IndexSetMultimap<K, V, S> {
    type Item = (K, V);

    type IntoIter = IntoIter<K, V, S>;

    fn into_iter(self) -> Self::IntoIter {
        let mut iter = self.inner.into_iter();
        let inner = iter.next().map(|(k, v)| (k, v.into_iter()));
        IntoIter {
            outer: iter,
            inner,
            len: self.len,
        }
    }
}
