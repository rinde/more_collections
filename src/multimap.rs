use indexmap::Equivalent;
use indexmap::IndexMap;
use indexmap::IndexSet;
use std::borrow::Borrow;
use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::BuildHasher;
use std::hash::Hash;
use std::iter::repeat;
use std::iter::FromIterator;

use crate::multimap_base2_impl;
use crate::multimap_base_impl;
use crate::multimap_eq;
use crate::multimap_extend;
use crate::multimap_mutators_impl;

/// Multimap implementation that behaves like `HashMap<K, HashSet<V>>`.
#[derive(Debug)]
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
    HashMap<K, HashSet<V,S>, S>,
    (K: Hash + Eq),
    (V: Hash + Eq)
}
multimap_eq! { HashSetMultimap, (Hash + Eq)}

/// Multimap implementation that behaves like `HashMap<K, Vec<V>>`.
#[derive(Debug)]
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
        vec![],
        vec,
        (K: Borrow<Q>, Q: Hash + Eq),
        (V: Borrow<R>, R: Equivalent<V>)
    }
}

multimap_extend! {
    HashVecMultimap,
    (K, V, S),
    HashMap<K, Vec<V>, S>,
    (K: Hash + Eq),
    (V: Eq)
}
multimap_eq! { HashVecMultimap, (Eq)}

/// Multimap implementation that behaves like `IndexMap<K, Vec<V>>`.
#[derive(Debug)]
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
        vec![],
        vec,
        (Q: Hash + Equivalent<K>),
        (R: Equivalent<V>)
    }
}

multimap_extend! {
    IndexVecMultimap,
    (K, V, S),
    IndexMap<K, Vec<V>, S>,
    (K: Hash + Eq),
    (V: Eq)
}
multimap_eq! { IndexVecMultimap, (Eq)}

/// Multimap implementation that behaves like `IndexMap<K, IndexSet<V>>`.
#[derive(Debug)]
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
}

multimap_extend! {
    IndexSetMultimap,
    (K, V, S),
    IndexMap<K, IndexSet<V,S>, S>,
    (K: Hash + Eq),
    (V: Hash + Eq)
}
multimap_eq! { IndexSetMultimap, (Hash + Eq)}

#[macro_export]
macro_rules! indexsetmultimap {
    (@single $($x:tt)*) => (());
    (@count $($rest:expr),*) => (<[()]>::len(&[$(indexsetmultimap!(@single $rest)),*]));

    ($($key:expr => {$($value:expr),* },)+) => { indexsetmultimap!($($key => $($value,)* ),+) };
    ($($key:expr => {$($value:expr),* }),*) => {
        {
            let _cap = indexsetmultimap!(@count $($key),*);
            let mut _map = indexmap::IndexMap::with_capacity(_cap);
            $(
                let _ = _map.insert($key, indexmap::indexset!{$( $value, )*});
            )*
            IndexSetMultimap::from(_map)
        }
    };
}

#[macro_export]
macro_rules! indexvecmultimap {
    (@single $($x:tt)*) => (());
    (@count $($rest:expr),*) => (<[()]>::len(&[$(indexvecmultimap!(@single $rest)),*]));

    ($($key:expr => {$($value:expr),* },)+) => { indexvecmultimap!($($key => $($value,)* ),+) };
    ($($key:expr => {$($value:expr),* }),*) => {
        {
            let _cap = indexvecmultimap!(@count $($key),*);
            let mut _map = indexmap::IndexMap::with_capacity(_cap);
            $(
                let _ = _map.insert($key, vec!{$( $value, )*});
            )*
            IndexVecMultimap::from(_map)
        }
    };
}

#[macro_export]
macro_rules! hashvecmultimap {
    (@single $($x:tt)*) => (());
    (@count $($rest:expr),*) => (<[()]>::len(&[$(hashvecmultimap!(@single $rest)),*]));

    ($($key:expr => {$($value:expr),* },)+) => { hashvecmultimap!($($key => $($value,)* ),+) };
    ($($key:expr => {$($value:expr),* }),*) => {
        {
            let _cap = hashvecmultimap!(@count $($key),*);
            let mut _map = std::collections::HashMap::with_capacity(_cap);
            $(
                let _ = _map.insert($key, vec!{$( $value, )*});
            )*
            HashVecMultimap::from(_map)
        }
    };
}

#[macro_export]
macro_rules! hashsetmultimap {
    (@single $($x:tt)*) => (());
    (@count $($rest:expr),*) => (<[()]>::len(&[$(hashsetmultimap!(@single $rest)),*]));

    ($($key:expr => {$($value:expr),* },)+) => { hashsetmultimap!($($key => $($value,)* ),+) };
    ($($key:expr => {$($value:expr),* }),*) => {
        {
            let _cap = hashsetmultimap!(@count $($key),*);
            let mut _map = std::collections::HashMap::with_capacity(_cap);
            $(
                let _ = _map.insert($key, maplit::hashset!{$( $value, )*});
            )*
            HashSetMultimap::from(_map)
        }
    };
}
