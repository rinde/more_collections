mod iter2;
use std::fmt::Debug;
use std::marker::PhantomData;

use crate::vec_map::index_vec::iter2::Iter;
use crate::IndexKey;

pub struct IndexVec<K, V> {
    data: Vec<V>,
    _marker: PhantomData<K>,
}

impl<K: IndexKey, V> IndexVec<K, V> {
    /// Initializes an empty [`IndexVec`].
    ///
    /// For performance reasons it's almost always better to avoid dynamic
    /// resizing by using [`Self::with_capacity()`] instead.
    pub const fn new() -> Self {
        Self {
            data: vec![],
            _marker: PhantomData,
        }
    }

    /// Returns the number of elements the collection can hold without reallocating.
    ///
    /// The index range of items that the collection can hold without reallocating is
    /// `0..capacity`.
    pub fn capacity(&self) -> usize {
        self.data.capacity()
    }

    /// Initializes [`IndexVec`] with capacity to hold exactly `n` elements in the
    /// index range of `0..n`.
    pub fn with_capacity(n: usize) -> Self {
        Self {
            data: Vec::with_capacity(n),
            _marker: PhantomData,
        }
    }

    /// Clears all data from the [`IndexVec`] without changing the capacity.
    pub fn clear(&mut self) {
        self.data.clear();
    }

    /// Reserve capacity for `additional` key-value pairs.
    pub fn reserve(&mut self, additional: usize) {
        self.data.reserve(additional);
    }

    /// Removes the last key-value pair.
    ///
    /// Performance: O(1)
    pub fn pop(&mut self) -> Option<(K, V)> {
        self.data
            .pop()
            .map(|value| (K::from_index(self.len()), value))
    }

    // TODO entry?

    /// Returns a reference to the value associated with `key` if it exists,
    /// otherwise returns `None`.
    pub fn get(&self, key: K) -> Option<&V> {
        self.data.get(key.as_index())
    }

    /// Returns a mutable reference to the value associated with `key` if it
    /// exists, otherwise returns `None`.
    pub fn get_mut(&mut self, key: K) -> Option<&mut V> {
        self.data.get_mut(key.as_index())
    }

    /// Return the number of key-value pairs in the map.
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Returns `true` if the map contains no elements.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns `true` if the map contains an item with the specified `key`.
    pub fn contains_key(&self, key: K) -> bool {
        self.get(key).is_some()
    }

    /// Returns an iterator over the key-value pairs of the map, following the
    /// natural order of the keys.
    pub fn iter(&self) -> Iter<'_, K, V> {
        iter2::Iter {
            inner: self
                .data
                .iter()
                .enumerate()
                .map(|(i, v)| (K::from_index(i), v)),
            _marker: PhantomData,
        }
    }
}

impl<K: IndexKey, V: Clone> IndexVec<K, V> {
    /// Initializes [`IndexVec`] with `n` occurences of `elem`.
    pub fn from_elem(elem: V, n: usize) -> Self {
        Self {
            data: vec![elem; n],
            _marker: PhantomData,
        }
    }
}

impl<K: IndexKey + Debug, V: Debug> std::fmt::Debug for IndexVec<K, V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_map().entries(self.iter()).finish()
    }
}
