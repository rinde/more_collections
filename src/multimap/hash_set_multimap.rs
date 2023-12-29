use crate::collections::hash_map::RandomState;
use crate::collections::HashMap;
use crate::collections::HashSet;
use core::borrow::Borrow;
use core::hash::BuildHasher;
use core::hash::Hash;

/// Multimap implementation that behaves like `HashMap<K, HashSet<V>>`.
#[derive(Debug, Clone)]
pub struct HashSetMultimap<K, V, S = RandomState> {
    inner: HashMap<K, HashSet<V, S>, S>,
    len: usize,
}

#[cfg(feature = "std")]
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

impl_iter! {
    HashSetMultimap,
    (K, V, S),
    crate::collections::hash_map::Iter<'a, K, HashSet<V, S>>,
    crate::collections::hash_set::Iter<'a, V>
}
impl_keys! {HashSetMultimap, (K, V, S), crate::collections::hash_map::Keys<'a, K, HashSet<V, S>>}
impl_into_iterator! {
    HashSetMultimap,
    (K, V, S),
    crate::collections::hash_map::IntoIter<K, HashSet<V, S>>,
    crate::collections::hash_set::IntoIter<V>
}
impl_into_keys! {HashSetMultimap, (K, V, S), crate::collections::hash_map::IntoKeys<K, HashSet<V, S>>}

#[macro_export]
macro_rules! hashsetmultimap {
    (@single $($x:tt)*) => (());
    (@count $($rest:expr),*) => (<[()]>::len(&[$(hashsetmultimap!(@single $rest)),*]));

    ($($key:expr => {$($value:expr),* },)+) => { hashsetmultimap!($($key => $($value,)* ),+) };
    ($($key:expr => {$($value:expr),* }),*) => {
        {
            let _mapcap = hashsetmultimap!(@count $($key),*);
            let mut _map = $crate::collections::HashMap::with_capacity(_mapcap);
            $(
                let _setcap = hashsetmultimap!(@count $($value),*);
                let mut _set = $crate::collections::HashSet::with_capacity(_setcap);
                $(
                    let _ = _set.insert($value);
                )*
                let _ = _map.insert($key, _set);
            )*
            HashSetMultimap::from(_map)
        }
    };
}
