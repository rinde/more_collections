use indexmap::Equivalent;
use indexmap::IndexMap;
use std::collections::hash_map::RandomState;
use std::hash::BuildHasher;
use std::hash::Hash;

/// Multiset implementation that behaves like `HashMap<T, usize>`.
#[derive(Debug, Clone)]
pub struct IndexMultiset<T, S = RandomState> {
    inner: IndexMap<T, usize, S>,
    len: usize,
}

impl<T> IndexMultiset<T, RandomState> {
    multiset_base_impl! {IndexMap<T, usize>}
}

impl<T, S> IndexMultiset<T, S> {
    multiset_base2_impl! {IndexMap}
}

impl<T, S> IndexMultiset<T, S>
where
    T: Hash + Eq,
    S: BuildHasher + Default,
{
    multiset_mutators_impl! {
        IndexMap<T, usize, S>,
        (Q: Hash + Equivalent<T>)
    }
}
