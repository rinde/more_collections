use std::borrow::Borrow;
use std::collections::hash_map::RandomState;
use std::hash::BuildHasher;
use std::hash::Hash;

use indexmap::Equivalent;
use indexmap::IndexMap;

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
        vec_equivalent,
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

impl_iter! {
    IndexVecMultimap,
    (K,V),
    indexmap::map::Iter<'a, K, Vec<V>>,
    std::slice::Iter<'a, V>
}
impl_keys! {IndexVecMultimap, (K,V), indexmap::map::Keys<'a, K, Vec<V>>}
impl_into_iterator! {
    IndexVecMultimap,
    (K,V),
    indexmap::map::IntoIter<K, Vec<V>>,
    std::vec::IntoIter<V>
}

impl_into_keys! {IndexVecMultimap, (K,V), indexmap::map::IntoKeys<K, Vec<V>>}

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
