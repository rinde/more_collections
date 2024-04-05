use crate::IndexKey;
use std::fmt::Debug;
use std::iter::Enumerate;
use std::iter::Map;
use std::marker::PhantomData;

/// An iterator that iterates over the key-value pairs following the key
/// ordering.
pub struct Iter<'a, K, V> {
    pub(super) inner: Map<Enumerate<core::slice::Iter<'a, V>>, fn((usize, &V)) -> (K, &V)>,
    pub(super) _marker: PhantomData<K>,
}

impl<'a, K: IndexKey, V> Iterator for Iter<'a, K, V> {
    type Item = (K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

// Use derive(Clone) as soon as bug https://github.com/rust-lang/rust/issues/26925 is fixed
impl<K, V> Clone for Iter<'_, K, V> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            _marker: self._marker.clone(),
        }
    }
}

impl<K: IndexKey + Debug, V: Debug> std::fmt::Debug for Iter<'_, K, V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.clone()).finish()
    }
}
