use std::borrow::Borrow;
use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::BuildHasher;
use std::marker::PhantomData;

use indexmap::IndexMap;
use indexmap::IndexSet;
use std::hash::Hash;

use crate::keys::InnerKeys;
use crate::values::InnerValues;

// TODO hasher needs to be configurable (but there are also multimaps without hashers)

pub type IndexSetMultimap<K, V, S> =
    GenericMultimap<K, V, IndexMap<K, IndexSet<V, S>, S>, IndexSet<V, S>>;
pub type IndexVecMultimap<K, V> = GenericMultimap<K, V, IndexMap<K, Vec<V>>, Vec<V>>;
pub type HashSetMultimap<K, V> = GenericMultimap<K, V, HashMap<K, HashSet<V>>, HashSet<V>>;
pub type HashVecMultimap<K, V> = GenericMultimap<K, V, HashMap<K, Vec<V>>, Vec<V>>;

// struct HashMultimap

trait Multimap<K, V> {
    fn insert(&mut self, key: K, value: V) -> bool;

    fn remove(&mut self, key: &K, value: &V) -> bool;

    fn contains_key<Q: ?Sized>(&self, key: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: Hash + Eq;

    fn contains<Q: ?Sized>(&self, key: &Q, value: &V) -> bool
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
#[derive(Default)]
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

    fn contains<Q: ?Sized>(&self, key: &Q, value: &V) -> bool
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        if let Some(values) = self.inner.get(key) {
            values.contains(value)
        } else {
            false
        }
    }

    fn reserve(&mut self, additional: usize) {
        self.inner.reserve(additional)
    }

    fn len(&self) -> usize {
        self.len
    }

    fn keys_len(&self) -> usize {
        self.inner.len()
    }
}

impl<K, V, S> IndexSetMultimap<K, V, S>
where
    S: BuildHasher + Default,
{
    pub fn with_capacity_and_hasher(len: usize, hash_builder: S) -> Self {
        IndexSetMultimap {
            inner: IndexMap::with_capacity_and_hasher(len, hash_builder),
            len,
            _marker_k: PhantomData,
            _marker_v: PhantomData,
            _marker_iv: PhantomData,
        }
    }

    pub fn with_hasher(hash_builder: S) -> Self {
        Self::with_capacity_and_hasher(0, hash_builder)
    }

    // TODO how to call this in insert() ?
    fn new_values() -> IndexSet<V, S> {
        IndexSet::with_hasher(S::default())
    }
}

// struct ConcreteIndexMultimap<K, V, S, VS> {
//     inner: IndexMap<K, VS, S>,
//     len: usize,
//     _marker_v: PhantomData<V>,
// }

// impl<K, V, S, VS> ConcreteIndexMultimap<K, V, S, VS>
// where
//     K: Hash + Eq,
//     VS: InnerValues<V>,
//     S: BuildHasher + Default,
// {
//     pub fn new() -> Self {
//         Self {
//             inner: IndexMap::with_hasher(S::default()),
//             len: 0,
//             _marker_v: PhantomData,
//         }
//     }

//     pub fn insert(&mut self, key: K, value: V) -> bool {
//         if self
//             .inner
//             .entry(key)
//             .or_insert_with(|| VS::with_hasher(S::default()))
//             .insert(value)
//         {
//             self.len += 1;
//             true
//         } else {
//             false
//         }
//     }
// }

// impl<K, V, S> IndexVecMultimap<K, V, S> {
//     fn extra() {}
// }
