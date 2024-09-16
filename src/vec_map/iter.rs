use std::fmt;
use std::iter::Enumerate;
use std::iter::FusedIterator;
use std::marker::PhantomData;

use crate::IndexKey;
use crate::VecMap;

impl<'a, K: IndexKey, V> IntoIterator for &'a VecMap<K, V> {
    type Item = (K, &'a V);
    type IntoIter = Iter<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, K: IndexKey, V> IntoIterator for &'a mut VecMap<K, V> {
    type Item = (K, &'a mut V);
    type IntoIter = IterMut<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<K: IndexKey, V> IntoIterator for VecMap<K, V> {
    type Item = (K, V);

    type IntoIter = IntoIter<K, V>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            inner: self.data.into_iter().enumerate(),
            len: self.len,
            _marker: PhantomData,
        }
    }
}

/// An iterator that iterates over the key-value pairs following the key
/// ordering.
#[derive(Clone)]
pub struct Iter<'a, K, V> {
    pub(super) inner: Enumerate<core::slice::Iter<'a, Option<V>>>,
    pub(super) len: usize,
    pub(super) _marker: PhantomData<K>,
}

impl<'a, K: IndexKey, V> Iterator for Iter<'a, K, V> {
    type Item = (K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        if self.len == 0 {
            return None;
        }
        self.inner.by_ref().find_map(|(i, v)| {
            v.as_ref().map(|v| {
                self.len -= 1;
                (K::from_index(i), v)
            })
        })
    }
}

impl<K: IndexKey, V> DoubleEndedIterator for Iter<'_, K, V> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.len == 0 {
            return None;
        }
        self.inner
            .by_ref()
            .filter_map(|(i, v)| {
                v.as_ref().map(|v| {
                    self.len -= 1;
                    (K::from_index(i), v)
                })
            })
            .next_back()
    }
}

impl<K: IndexKey, V> ExactSizeIterator for Iter<'_, K, V> {
    fn len(&self) -> usize {
        self.len
    }
}

impl<K: IndexKey, V> FusedIterator for Iter<'_, K, V> {}

impl<'a, K: IndexKey + fmt::Debug, V: fmt::Debug> fmt::Debug for Iter<'a, K, V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO why can't we use self.clone()
        let iter: Iter<'a, K, V> = Iter {
            inner: self.inner.clone(),
            len: self.len,
            _marker: PhantomData,
        };
        f.debug_list().entries(iter).finish()
    }
}

/// An iterator that iterates over the key-value pairs following the key
/// ordering.
#[derive(Debug)] // TODO figure out a way to implement Debug cleanly but without cloning
pub struct IterMut<'a, K, V> {
    pub(super) inner: Enumerate<core::slice::IterMut<'a, Option<V>>>,
    pub(super) len: usize,
    pub(super) _marker: PhantomData<K>,
}

impl<'a, K: IndexKey, V> Iterator for IterMut<'a, K, V> {
    type Item = (K, &'a mut V);

    fn next(&mut self) -> Option<Self::Item> {
        if self.len == 0 {
            return None;
        }
        self.inner.by_ref().find_map(|(i, v)| {
            v.as_mut().map(|v| {
                self.len -= 1;
                (K::from_index(i), v)
            })
        })
    }
}

impl<K: IndexKey, V> DoubleEndedIterator for IterMut<'_, K, V> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.len == 0 {
            return None;
        }
        self.inner
            .by_ref()
            .filter_map(|(i, v)| {
                v.as_mut().map(|v| {
                    self.len -= 1;
                    (K::from_index(i), v)
                })
            })
            .next_back()
    }
}

impl<K: IndexKey, V> ExactSizeIterator for IterMut<'_, K, V> {
    fn len(&self) -> usize {
        self.len
    }
}

impl<K: IndexKey, V> FusedIterator for IterMut<'_, K, V> {}

/// An owned iterator that iterates over the key-value pairs following the key
/// ordering.
#[derive(Clone, Debug)] // TODO figure out a way to implement Debug cleanly but without cloning
pub struct IntoIter<K, V> {
    pub(super) inner: Enumerate<std::vec::IntoIter<Option<V>>>,
    pub(super) len: usize,
    pub(super) _marker: PhantomData<K>,
}

impl<K: IndexKey, V> Iterator for IntoIter<K, V> {
    type Item = (K, V);

    fn next(&mut self) -> Option<Self::Item> {
        if self.len == 0 {
            return None;
        }
        self.inner.by_ref().find_map(|(i, v)| {
            v.map(|v| {
                self.len -= 1;
                (K::from_index(i), v)
            })
        })
    }
}

impl<K: IndexKey, V> DoubleEndedIterator for IntoIter<K, V> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.len == 0 {
            return None;
        }
        self.inner
            .by_ref()
            .filter_map(|(i, v)| {
                v.map(|v| {
                    self.len -= 1;
                    (K::from_index(i), v)
                })
            })
            .next_back()
    }
}

impl<K: IndexKey, V> ExactSizeIterator for IntoIter<K, V> {
    fn len(&self) -> usize {
        self.len
    }
}

impl<K: IndexKey, V> FusedIterator for IntoIter<K, V> {}

/// An iterator over the keys following the key natural order.
#[derive(Clone)]
pub struct Keys<'a, K, V> {
    pub(super) inner: Enumerate<core::slice::Iter<'a, Option<V>>>,
    pub(super) len: usize,
    pub(super) _marker: PhantomData<K>,
}

impl<K: IndexKey, V> Iterator for Keys<'_, K, V> {
    type Item = K;

    fn next(&mut self) -> Option<Self::Item> {
        if self.len == 0 {
            return None;
        }
        self.inner.find_map(|(i, value)| {
            value.as_ref().map(|_| {
                self.len -= 1;
                K::from_index(i)
            })
        })
    }
}

impl<K: IndexKey, V> DoubleEndedIterator for Keys<'_, K, V> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.len == 0 {
            return None;
        }
        self.inner
            .by_ref()
            .filter_map(|(i, v)| {
                v.as_ref().map(|_| {
                    self.len -= 1;
                    K::from_index(i)
                })
            })
            .next_back()
    }
}

impl<K: IndexKey, V> ExactSizeIterator for Keys<'_, K, V> {
    fn len(&self) -> usize {
        self.len
    }
}

impl<K: IndexKey, V> FusedIterator for Keys<'_, K, V> {}

impl<'a, K, V> fmt::Debug for Keys<'a, K, V>
where
    K: IndexKey + fmt::Debug,
    V: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO why can't we use self.clone()
        let iter: Keys<'a, K, V> = Keys {
            inner: self.inner.clone(),
            len: self.len,
            _marker: PhantomData,
        };
        f.debug_list().entries(iter).finish()
    }
}

/// An iterator over the values following the key natural order.
#[derive(Clone)]
pub struct Values<'a, V> {
    pub(super) inner: core::slice::Iter<'a, Option<V>>,
    pub(super) len: usize,
}

impl<'a, V> Iterator for Values<'a, V> {
    type Item = &'a V;

    fn next(&mut self) -> Option<Self::Item> {
        if self.len == 0 {
            return None;
        }
        self.inner.find_map(|value| value.as_ref()).map(|v| {
            self.len -= 1;
            v
        })
    }
}

impl<V> DoubleEndedIterator for Values<'_, V> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.len == 0 {
            return None;
        }
        self.inner
            .by_ref()
            .rev()
            .find_map(|value| value.as_ref())
            .map(|v| {
                self.len -= 1;
                v
            })
    }
}

impl<V> ExactSizeIterator for Values<'_, V> {
    fn len(&self) -> usize {
        self.len
    }
}

impl<V> FusedIterator for Values<'_, V> {}

impl<'a, V> fmt::Debug for Values<'a, V>
where
    V: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO why can't we use self.clone()
        let iter: Values<'a, V> = Values {
            inner: self.inner.clone(),
            len: self.len,
        };
        f.debug_list().entries(iter).finish()
    }
}

#[cfg(test)]
mod test {
    use crate::vec_map::test::MyKey;
    use crate::vecmap;
    use crate::VecMap;

    #[test]
    fn test_iter() {
        let map = vecmap! { 9 => "nine", 17 => "seventeen", 2 => "two"};

        // forward
        let mut iter = map.iter();
        assert_eq!(3, iter.len());
        assert_eq!(Some((2, &"two")), iter.next());
        assert_eq!(2, iter.len());
        assert_eq!(Some((9, &"nine")), iter.next());
        assert_eq!(1, iter.len());
        assert_eq!(Some((17, &"seventeen")), iter.next());
        assert_eq!(0, iter.len());
        assert_eq!(None, iter.next());

        // back, forward, back
        let mut iter = map.iter();
        assert_eq!(3, iter.len());
        assert_eq!(Some((17, &"seventeen")), iter.next_back());
        assert_eq!(2, iter.len());
        assert_eq!(Some((2, &"two")), iter.next());
        assert_eq!(1, iter.len());
        assert_eq!(Some((9, &"nine")), iter.next_back());
        assert_eq!(0, iter.len());
        assert_eq!(None, iter.next_back());

        let map: VecMap<usize, usize> = VecMap::with_capacity(40);
        let mut iter = map.iter();
        assert_eq!(0, iter.len());
        assert_eq!(None, iter.next());
        assert_eq!(0, iter.len());
    }

    #[test]
    fn test_iter_mut() {
        let mut map = vecmap! { 9 => "nine", 17 => "seventeen", 2 => "two"};

        // forward
        let mut iter = map.iter_mut();
        assert_eq!(3, iter.len());
        let v = iter.next();
        assert_eq!(Some((2, &mut "two")), v);
        *v.unwrap().1 = "22222";
        assert_eq!(2, iter.len());
        assert_eq!(Some((9, &mut "nine")), iter.next());
        assert_eq!(1, iter.len());
        assert_eq!(Some((17, &mut "seventeen")), iter.next());
        assert_eq!(0, iter.len());
        assert_eq!(None, iter.next());

        // back, forward, back, back
        let mut iter = map.iter_mut();
        assert_eq!(3, iter.len());
        let v = iter.next_back();
        assert_eq!(Some((17, &mut "seventeen")), v);
        *v.unwrap().1 = "17171717";
        assert_eq!(2, iter.len());
        assert_eq!(Some((2, &mut "22222")), iter.next());
        assert_eq!(1, iter.len());
        assert_eq!(Some((9, &mut "nine")), iter.next_back());
        assert_eq!(0, iter.len());
        assert_eq!(None, iter.next_back());

        assert_eq!("17171717", map[17]);

        let mut map: VecMap<usize, usize> = VecMap::with_capacity(40);
        let mut iter = map.iter_mut();
        assert_eq!(0, iter.len());
        assert_eq!(None, iter.next());
        assert_eq!(0, iter.len());
    }

    #[test]
    fn test_into_iter() {
        let map = vecmap! { 9 => "nine", 17 => "seventeen", 2 => "two"};

        // forward
        let mut iter = map.into_iter();
        assert_eq!(3, iter.len());
        assert_eq!(Some((2, "two")), iter.next());
        assert_eq!(2, iter.len());
        assert_eq!(Some((9, "nine")), iter.next());
        assert_eq!(1, iter.len());
        assert_eq!(Some((17, "seventeen")), iter.next());
        assert_eq!(0, iter.len());
        assert_eq!(None, iter.next());

        let map = vecmap! { 9 => "nine", 17 => "seventeen", 2 => "two"};
        // back, forward, back
        let mut iter = map.into_iter();
        assert_eq!(3, iter.len());
        assert_eq!(Some((17, "seventeen")), iter.next_back());
        assert_eq!(2, iter.len());
        assert_eq!(Some((2, "two")), iter.next());
        assert_eq!(1, iter.len());
        assert_eq!(Some((9, "nine")), iter.next_back());
        assert_eq!(0, iter.len());
        assert_eq!(None, iter.next_back());

        let map: VecMap<usize, usize> = VecMap::with_capacity(40);
        let mut iter = map.into_iter();
        assert_eq!(0, iter.len());
        assert_eq!(None, iter.next());
        assert_eq!(0, iter.len());
    }

    #[test]
    fn test_keys() {
        let map = vecmap! { 9 => "nine", 17 => "seventeen", 2 => "two"};

        // forward
        let mut iter = map.keys();
        assert_eq!(3, iter.len());
        assert_eq!(Some(2), iter.next());
        assert_eq!(2, iter.len());
        assert_eq!(Some(9), iter.next());
        assert_eq!(1, iter.len());
        assert_eq!(Some(17), iter.next());
        assert_eq!(0, iter.len());
        assert_eq!(None, iter.next());

        // back, forward, back
        let mut iter = map.keys();
        assert_eq!(3, iter.len());
        assert_eq!(Some(17), iter.next_back());
        assert_eq!(2, iter.len());
        assert_eq!(Some(2), iter.next());
        assert_eq!(1, iter.len());
        assert_eq!(Some(9), iter.next_back());
        assert_eq!(0, iter.len());
        assert_eq!(None, iter.next_back());

        let map: VecMap<usize, usize> = VecMap::with_capacity(40);
        let mut iter = map.keys();
        assert_eq!(0, iter.len());
        assert_eq!(None, iter.next());
        assert_eq!(0, iter.len());
    }

    #[test]
    fn test_values() {
        let map = vecmap! { MyKey(9) => "nine", MyKey(17) => "seventeen", MyKey(2) => "two"};

        // forward
        let mut iter = map.values();
        assert_eq!(3, iter.len());
        assert_eq!(Some(&"two"), iter.next());
        assert_eq!(2, iter.len());
        assert_eq!(Some(&"nine"), iter.next());
        assert_eq!(1, iter.len());
        assert_eq!(Some(&"seventeen"), iter.next());
        assert_eq!(0, iter.len());
        assert_eq!(None, iter.next());

        // back, forward, back
        let mut iter = map.values();
        assert_eq!(3, iter.len());
        assert_eq!(Some(&"seventeen"), iter.next_back());
        assert_eq!(2, iter.len());
        assert_eq!(Some(&"two"), iter.next());
        assert_eq!(1, iter.len());
        assert_eq!(Some(&"nine"), iter.next_back());
        assert_eq!(0, iter.len());
        assert_eq!(None, iter.next_back());

        let map: VecMap<usize, usize> = VecMap::with_capacity(40);
        let mut iter = map.values();
        assert_eq!(0, iter.len());
        assert_eq!(None, iter.next());
        assert_eq!(0, iter.len());
    }
}
