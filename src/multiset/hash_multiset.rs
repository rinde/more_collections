use std::borrow::Borrow;
use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use std::hash::BuildHasher;
use std::hash::Hash;

/// Multiset implementation that behaves like `HashMap<T, usize>`.
#[derive(Debug, Clone)]
pub struct HashMultiset<T, S = RandomState> {
    inner: HashMap<T, usize, S>,
    len: usize,
}

impl<T> HashMultiset<T, RandomState> {
    multiset_base_impl! {HashMap<T, usize>}
}

impl<T, S> HashMultiset<T, S> {
    multiset_base2_impl! {HashMap}
}

impl<T, S> HashMultiset<T, S>
where
    T: Hash + Eq,
    S: BuildHasher + Default,
{
    multiset_mutators_impl! {
        HashMultiset,
        HashMap<T, usize, S>,
        HashMap,
        (T: Borrow<Q>, Q: Hash + Eq)
    }
}

multiset_common_traits_impl!(HashMultiset, HashMap, (T: Hash + Eq));
