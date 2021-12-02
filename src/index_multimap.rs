use std::hash::BuildHasher;
use std::marker::PhantomData;

use indexmap::IndexMap;
use indexmap::IndexSet;
use std::hash::Hash;

pub type IndexSetMultimap<K, V, S> = IndexMultimap<K, V, S, IndexSet<V>>;
pub type IndexVecMultimap<K, V, S> = IndexMultimap<K, V, S, Vec<V>>;

struct IndexMultimap<K, V, S, VS> {
    inner: IndexMap<K, VS, S>,
    len: usize,
    _marker_v: PhantomData<V>,
}

trait InnerValues<T, S> {
    fn with_hasher(hash_builder: S) -> Self;

    fn insert(&mut self, value: T) -> bool;
}

impl<T, S> InnerValues<T, S> for IndexSet<T, S>
where
    T: Hash + Eq,
    S: BuildHasher,
{
    fn with_hasher(hash_builder: S) -> Self {
        IndexSet::with_hasher(hash_builder)
    }

    fn insert(&mut self, value: T) -> bool {
        IndexSet::insert(self, value)
    }
}

impl<T, S> InnerValues<T, S> for Vec<T> {
    fn with_hasher(_hash_builder: S) -> Self {
        vec![]
    }

    fn insert(&mut self, value: T) -> bool {
        self.push(value);
        true
    }
}

impl<K, V, S, VS> IndexMultimap<K, V, S, VS>
where
    K: Hash + Eq,
    VS: InnerValues<V, S>,
    S: BuildHasher + Default,
{
    pub fn new() -> Self {
        Self {
            inner: IndexMap::with_hasher(S::default()),
            len: 0,
            _marker_v: PhantomData,
        }
    }

    pub fn insert(&mut self, key: K, value: V) -> bool {
        if self
            .inner
            .entry(key)
            .or_insert_with(|| VS::with_hasher(S::default()))
            .insert(value)
        {
            self.len += 1;
            true
        } else {
            false
        }
    }
}

impl<K, V, S> IndexVecMultimap<K, V, S> {
    fn extra() {}
}
