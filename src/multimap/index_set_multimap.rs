use core::borrow::Borrow;
use crate::collections::hash_map::RandomState;
use core::hash::BuildHasher;
use core::hash::Hash;

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
    multimap_base_impl! {IndexMap<K, IndexSet<V, RandomState>, RandomState>}
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

impl_iter! {
    IndexSetMultimap,
    (K,V,S),
    indexmap::map::Iter<'a, K, IndexSet<V, S>>,
    indexmap::set::Iter<'a, V>
}
impl_keys! {IndexSetMultimap, (K, V, S), indexmap::map::Keys<'a, K, IndexSet<V, S>>}
impl_into_iterator! {
    IndexSetMultimap,
    (K,V,S),
    indexmap::map::IntoIter<K, IndexSet<V, S>>,
    indexmap::set::IntoIter<V>
}
impl_into_keys! {IndexSetMultimap, (K, V, S), indexmap::map::IntoKeys<K, IndexSet<V, S>>}

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
