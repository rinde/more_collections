use indexmap::IndexMap;
use std::borrow::Borrow;
use std::collections::HashMap;
use std::hash::BuildHasher;
use std::hash::Hash;

use crate::values::InnerValues;

pub trait InnerKeys<K, V, IV> {
    fn insert_with<F>(&mut self, key: K, value: V, constructor: F) -> bool
    where
        F: FnOnce() -> IV;

    fn get<Q: ?Sized>(&self, key: &Q) -> Option<&IV>
    where
        K: Borrow<Q>,
        Q: Hash + Eq;

    fn get_mut(&mut self, key: &K) -> Option<&mut IV>;

    fn remove(&mut self, key: &K) -> Option<IV>;

    fn contains_key<Q: ?Sized>(&self, key: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: Hash + Eq;

    fn reserve(&mut self, additional: usize);

    fn len(&self) -> usize;

    fn is_empty(&self) -> bool;
}

impl<K, V, IV, S> InnerKeys<K, V, IV> for HashMap<K, IV, S>
where
    K: Eq + Hash,
    IV: InnerValues<V>,
    S: BuildHasher,
{
    fn insert_with<F>(&mut self, key: K, value: V, constructor: F) -> bool
    where
        F: FnOnce() -> IV,
    {
        self.entry(key).or_insert_with(constructor).insert(value)
    }

    fn get<Q: ?Sized>(&self, key: &Q) -> Option<&IV>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        HashMap::get(self, key)
    }

    fn get_mut(&mut self, key: &K) -> Option<&mut IV> {
        HashMap::get_mut(&mut self, key)
    }

    fn remove(&mut self, key: &K) -> Option<IV> {
        HashMap::remove(&mut self, key)
    }

    fn contains_key<Q: ?Sized>(&self, key: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        HashMap::contains_key(&self, key)
    }

    fn reserve(&mut self, additional: usize) {
        HashMap::reserve(&mut self, additional)
    }

    fn len(&self) -> usize {
        HashMap::len(&self)
    }

    fn is_empty(&self) -> bool {
        HashMap::is_empty(&self)
    }
}

impl<K, V, IV, S> InnerKeys<K, V, IV> for IndexMap<K, IV, S>
where
    K: Eq + Hash,
    IV: InnerValues<V>,
    S: BuildHasher,
{
    fn insert_with<F>(&mut self, key: K, value: V, constructor: F) -> bool
    where
        F: FnOnce() -> IV,
    {
        self.entry(key).or_insert_with(constructor).insert(value)
    }

    fn get<Q: ?Sized>(&self, key: &Q) -> Option<&IV>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        IndexMap::get(self, key)
    }

    fn get_mut(&mut self, key: &K) -> Option<&mut IV> {
        IndexMap::get_mut(self, key)
    }

    fn remove(&mut self, key: &K) -> Option<IV> {
        IndexMap::remove(&mut self, key)
    }

    fn contains_key<Q: ?Sized>(&self, key: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        IndexMap::contains_key(&self, key)
    }

    fn reserve(&mut self, additional: usize) {
        IndexMap::reserve(&mut self, additional)
    }

    fn len(&self) -> usize {
        IndexMap::len(&self)
    }

    fn is_empty(&self) -> bool {
        IndexMap::is_empty(&self)
    }
}
