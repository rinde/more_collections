use std::borrow::Borrow;
use std::collections::HashMap;
use std::hash::BuildHasher;
use std::marker::PhantomData;
use std::ops::Mul;

use indexmap::IndexMap;
use indexmap::IndexSet;
use std::hash::Hash;

pub type IndexSetMultimap<K, V, S> = ConcreteIndexMultimap<K, V, S, IndexSet<V>>;
pub type IndexVecMultimap<K, V, S> = ConcreteIndexMultimap<K, V, S, Vec<V>>;

// struct HashMultimap

trait Multimap<K, V> {
    fn insert(&mut self, key: K, value: V) -> bool;

    fn remove(&mut self, key: &K, value: &V) -> bool;

    fn contains_key<Q: ?Sized>(&self, key: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: Hash + Eq;

    fn contains<Q: ?Sized, R: ?Sized>(&self, key: &Q, value: &R) -> bool
    where
        K: Borrow<Q>,
        Q: Hash + Eq;

    fn reserve(&mut self, additional: usize);

    fn len(&self) -> usize;

    fn keys_len(&self) -> usize;
}
// TODO
// pub fn remove_key(&mut self, key: &K) -> Option<IndexSet<V, S>>
// pub fn get<Q: ?Sized>(&self, key: &Q) -> Option<&IndexSet<V, S>>
// fn iter(&self) -> impl Iterator<Item = (&K, &V)>;

trait IndexMultimap<K, V>: Multimap<K, V> {
    fn get_index_of_key<Q: ?Sized>(&self, key: &Q) -> Option<usize>;
}

// TODO index
// fn get_index(&self, index: usize) -> Option<(&K, &IndexSet<V, S>)>

trait InnerKeys<K, V, IV> {
    fn insert_with<F>(&mut self, key: K, value: V, constructor: F) -> bool
    where
        F: FnOnce() -> IV;

    fn get(&self, key: &K) -> Option<&IV>;

    fn get_mut(&mut self, key: &K) -> Option<&mut IV>;

    fn remove(&mut self, key: &K) -> Option<IV>;

    fn contains_key<Q: ?Sized>(&self, key: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: Hash + Eq;
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

    fn get(&self, key: &K) -> Option<&IV> {
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

    fn get(&self, key: &K) -> Option<&IV> {
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
}

struct GenericMultimap<K, V, IK, IV> {
    inner: IK,
    len: usize,
    _marker_k: PhantomData<K>,
    _marker_v: PhantomData<V>,
    _marker_iv: PhantomData<IV>,
}

impl<K, V, IK, IV> Multimap<K, V> for GenericMultimap<K, V, IK, IV>
where
    IK: InnerKeys<K, V, IV>,
    IV: InnerValues<V>,
{
    fn insert(&mut self, key: K, value: V) -> bool {
        // TODO
        // self.inner.insert_with(key, value, || )
        todo!()
    }

    fn remove(&mut self, key: &K, value: &V) -> bool {
        if let Some(values) = self.inner.get_mut(key) {
            if values.remove(value) {
                if values.is_empty() {
                    self.inner.remove(key);
                }
                self.len -= 1;
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    fn contains_key<Q: ?Sized>(&self, key: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.inner.contains_key(key)
    }

    fn contains<Q: ?Sized, R: ?Sized>(&self, key: &Q, value: &R) -> bool
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.inner.get(key).contains(value)
    }

    fn reserve(&mut self, additional: usize) {
        todo!()
    }

    fn len(&self) -> usize {
        todo!()
    }

    fn keys_len(&self) -> usize {
        todo!()
    }
}

struct ConcreteIndexMultimap<K, V, S, VS> {
    inner: IndexMap<K, VS, S>,
    len: usize,
    _marker_v: PhantomData<V>,
}

trait InnerValues<T> {
    // fn with_hasher(hash_builder: S) -> Self;

    fn insert(&mut self, value: T) -> bool;

    fn remove(&mut self, value: &T) -> bool;

    fn is_empty(&self) -> bool;

    fn contains(&self, value: &T) -> bool;
}

impl<T> InnerValues<T> for IndexSet<T>
where
    T: Hash + Eq,
{
    // fn with_hasher(hash_builder: S) -> Self {
    //     IndexSet::with_hasher(hash_builder)
    // }

    fn insert(&mut self, value: T) -> bool {
        IndexSet::insert(self, value)
    }

    fn remove(&mut self, value: &T) -> bool {
        IndexSet::remove(self, value)
    }

    fn is_empty(&self) -> bool {
        IndexSet::is_empty(self)
    }

    fn contains(&self, value: &T) -> bool {
        IndexSet::contains(&self, value)
    }
}

impl<T> InnerValues<T> for Vec<T>
where
    T: PartialEq,
{
    // fn with_hasher(_hash_builder: S) -> Self {
    //     vec![]
    // }

    fn insert(&mut self, value: T) -> bool {
        self.push(value);
        true
    }

    fn remove(&mut self, value: &T) -> bool {
        if let Some(index) = self.iter().position(|x| x == value) {
            self.remove(index);
            true
        } else {
            false
        }
    }

    fn is_empty(&self) -> bool {
        self.is_empty()
    }

    fn contains(&self, value: &T) -> bool {
        self.iter().any(|x| x == value)
    }
}

impl<K, V, S, VS> ConcreteIndexMultimap<K, V, S, VS>
where
    K: Hash + Eq,
    VS: InnerValues<V>,
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
