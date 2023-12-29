use crate::collections::hash_map::RandomState;
use crate::collections::HashMap;
use alloc::vec::Vec;
use core::borrow::Borrow;
use core::hash::BuildHasher;
use core::hash::Hash;

/// Multimap implementation that behaves like `HashMap<K, Vec<V>>`.
#[derive(Debug, Clone)]
pub struct HashVecMultimap<K, V, S = RandomState> {
    inner: HashMap<K, Vec<V>, S>,
    len: usize,
}

#[cfg(feature = "std")]
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
        vec_equal,
        (K: Borrow<Q>, Q: Hash + Eq),
        (V: Borrow<R>, R: Eq)
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

impl_iter! {
    HashVecMultimap,
    (K,V),
    crate::collections::hash_map::Iter<'a, K, Vec<V>>,
    core::slice::Iter<'a, V>
}
impl_keys! {HashVecMultimap, (K, V), crate::collections::hash_map::Keys<'a, K, Vec<V>>}
impl_into_iterator! {
    HashVecMultimap,
    (K,V),
    crate::collections::hash_map::IntoIter<K, Vec<V>>,
    alloc::vec::IntoIter<V>
}
impl_into_keys! {HashVecMultimap, (K, V), crate::collections::hash_map::IntoKeys<K, Vec<V>>}

#[macro_export]
macro_rules! hashvecmultimap {
    (@single $($x:tt)*) => (());
    (@count $($rest:expr),*) => (<[()]>::len(&[$(hashvecmultimap!(@single $rest)),*]));

    ($($key:expr => {$($value:expr),* },)+) => { hashvecmultimap!($($key => $($value,)* ),+) };
    ($($key:expr => {$($value:expr),* }),*) => {
        {
            let _cap = hashvecmultimap!(@count $($key),*);
            let mut _map = $crate::collections::HashMap::with_capacity(_cap);
            $(
                let _ = _map.insert($key, vec!{$( $value, )*});
            )*
            HashVecMultimap::from(_map)
        }
    };
}
