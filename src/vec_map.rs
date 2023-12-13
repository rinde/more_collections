use std::fmt;
use std::iter::Enumerate;
use std::iter::FusedIterator;
use std::marker::PhantomData;
use std::ops::Index;

use bitvec::bitvec;
use bitvec::order::Lsb0;
use bitvec::prelude::BitVec;
use bitvec::slice::IterOnes;

/// A key that can be used in a map.
pub trait CopyKey: Copy {
    fn as_index(&self) -> usize;

    fn from_index(index: usize) -> Self;
}

/// A [`Vec`]-backed map.
///
/// Iteration order follows the natural ordering of [`Indexable`].
#[derive(Clone, Eq, PartialEq)]
pub struct VecMap<K, V> {
    data: Vec<Option<V>>,
    keys: BitVec<u64>,
    len: usize,
    _marker: PhantomData<K>,
}

impl<K, V: Clone> VecMap<K, V> {
    pub fn with_capacity(n: usize) -> Self {
        let mut keys = BitVec::EMPTY;
        keys.extend(bitvec![0; n]);
        Self {
            data: vec![None; n],
            keys,
            len: 0,
            _marker: PhantomData,
        }
    }
}

impl<K: CopyKey, V> VecMap<K, V> {
    pub const fn new() -> Self {
        Self {
            data: vec![],
            keys: BitVec::EMPTY,
            len: 0,
            _marker: PhantomData,
        }
    }

    /// Inserts a key-value pair into the map.
    ///
    /// If the key is present in the map, the value is updated and the old value
    /// is returned. Otherwise, [`None`] is returned.
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        let index = key.as_index();
        if index >= self.data.len() {
            self.keys.resize(index + 1, false);
            self.data
                .extend((0..(index - self.data.len() + 1)).map(|_| None));
        }

        self.keys.set(index, true);
        let existing = self.data[index].replace(value);
        if existing.is_none() {
            self.len += 1;
        }
        existing
    }

    /// Removes the key-value pair indicated by `key`.
    ///
    /// If the key was present, it is returned. Otherwise [`None`] is returned.
    pub fn remove(&mut self, key: K) -> Option<V> {
        let index = key.as_index();
        if index >= self.data.len() {
            None
        } else {
            self.keys.set(index, false);
            let existing = self.data[index].take();
            if existing.is_some() {
                self.len -= 1;
            }
            existing
        }
    }

    /// Get the given key's entry in the map for insertion and/or in-place
    /// manipulation.
    pub fn entry(&mut self, key: K) -> Entry<K, V> {
        let index = key.as_index();
        if index >= self.data.len() {
            Entry::Vacant(key, self)
        } else {
            if self.data[index].is_some() {
                return Entry::Occupied(self.data[index].as_mut().unwrap());
            }
            return Entry::Vacant(key, self);
        }
    }

    /// Returns a reference to the value associated with `key` if it exists,
    /// otherwise returns `None`.
    pub fn get(&self, key: K) -> Option<&V> {
        let index = key.as_index();
        if index >= self.data.len() {
            None
        } else {
            self.data[index].as_ref()
        }
    }

    /// Returns a mutable reference to the value associated with `key` if it
    /// exists, otherwise returns `None`.
    pub fn get_mut(&mut self, key: K) -> Option<&mut V> {
        let index = key.as_index();
        if index >= self.data.len() {
            None
        } else {
            self.data[index].as_mut()
        }
    }

    /// Return the number of key-value pairs in the map.
    pub fn len(&self) -> usize {
        self.len
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
        Iter {
            inner: self.keys.iter_ones(),
            values: &self.data,
            len: self.len,
            _marker: PhantomData,
        }
    }

    pub fn iter2(&self) -> Iter2<'_, K, V> {
        Iter2 {
            inner: self.data.iter().enumerate(),
            _marker: PhantomData,
        }
    }

    /// Returns an iterator over the keys of the map following the natural order
    /// of the keys.
    pub fn keys(&self) -> Keys<'_, K> {
        Keys {
            inner: self.keys.iter_ones(),
            _marker: PhantomData,
        }
    }

    // TODO values()

    pub fn clear(&mut self) {
        self.keys.clear();
        self.len = 0;
        self.data.clear();
    }
}

impl<K: CopyKey, V> Default for VecMap<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K: CopyKey, V> Index<K> for VecMap<K, V> {
    type Output = V;

    fn index(&self, key: K) -> &Self::Output {
        let index = key.as_index();
        if index >= self.data.len() {
            panic!("out of bounds");
        } else {
            self.data[index]
                .as_ref()
                .unwrap_or_else(|| panic!("doesn't exist"))
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Keys<'a, K> {
    inner: IterOnes<'a, u64, Lsb0>,
    _marker: PhantomData<K>,
}

impl<'a, K: CopyKey> Iterator for Keys<'a, K> {
    type Item = K;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|t| K::from_index(t))
    }
}

#[derive(Clone)]
pub struct Iter2<'a, K, V> {
    inner: Enumerate<core::slice::Iter<'a, Option<V>>>,
    _marker: PhantomData<K>,
}

impl<'a, K: CopyKey, V> Iterator for Iter2<'a, K, V> {
    type Item = (K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        while let Some((i, v)) = self.inner.next() {
            if let Some(v) = v {
                return Some((K::from_index(i), v));
            }
        }
        None
        // // .skip_while(|(_i, x)| x.is_none())
        // .next()
        // .map(|(i, v)| (K::from_index(i), v.as_ref().unwrap()))
    }
}

#[derive(Clone, Copy)]
pub struct Iter<'a, K, V> {
    inner: IterOnes<'a, u64, Lsb0>,
    values: &'a Vec<Option<V>>,
    len: usize,
    _marker: PhantomData<K>,
}

impl<'a, K: CopyKey, V> Iterator for Iter<'a, K, V> {
    type Item = (K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|i| {
            self.len -= 1;
            (K::from_index(i), self.values[i].as_ref().unwrap())
        })
    }
}

impl<'a, K: CopyKey, V> ExactSizeIterator for Iter<'a, K, V> {
    fn len(&self) -> usize {
        self.len
    }
}

impl<'a, K: CopyKey, V> DoubleEndedIterator for Iter<'a, K, V> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.inner.next_back().map(|i| {
            self.len -= 1;
            (K::from_index(i), self.values[i].as_ref().unwrap())
        })
    }
}

impl<'a, K: CopyKey, V> FusedIterator for Iter<'a, K, V> {}

impl<'a, K: CopyKey + fmt::Debug, V: fmt::Debug> fmt::Debug for Iter<'a, K, V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO why can't we use iter.clone()
        let iter: Iter<'a, K, V> = Iter {
            inner: self.inner,
            values: self.values,
            len: self.len,
            _marker: PhantomData,
        };
        f.debug_list().entries(iter).finish()
    }
}

impl<K: CopyKey, V> IntoIterator for VecMap<K, V> {
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

#[derive(Debug)]
pub struct IntoIter<K, V> {
    inner: Enumerate<std::vec::IntoIter<Option<V>>>,
    len: usize,
    _marker: PhantomData<K>,
}

impl<K: CopyKey, V> Iterator for IntoIter<K, V> {
    type Item = (K, V);

    // TODO should this use the bitset when the data is less dense?
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.inner.next() {
                Some((_, None)) => continue,
                Some((i, Some(v))) => return Some((K::from_index(i), v)),
                None => return None,
            }
        }
    }
}

impl<K: CopyKey, V> ExactSizeIterator for IntoIter<K, V> {
    fn len(&self) -> usize {
        self.len
    }
}

#[derive(Debug)]
pub enum Entry<'a, K: CopyKey, V> {
    Vacant(K, &'a mut VecMap<K, V>),
    Occupied(&'a mut V),
}

impl<'a, K: CopyKey, V> Entry<'a, K, V> {
    /// Inserts the given default value in the entry if it is vacant.
    ///
    /// Returns a mutable reference to the existing value if it is occupied, or
    /// a mutable reference to the newly added value if it is vacant.
    pub fn or_insert(self, default: V) -> &'a mut V {
        match self {
            Entry::Occupied(entry) => entry,
            Entry::Vacant(key, map) => {
                map.insert(key, default);
                map.get_mut(key).unwrap()
            }
        }
    }

    /// Inserts the result of the `creator` function in the entry if it is
    /// vacant.
    ///
    /// Returns a mutable reference to the existing value if it is occupied, or
    /// a mutable reference to the newly added value if it is vacant.
    pub fn or_insert_with<F>(self, creator: F) -> &'a mut V
    where
        F: FnOnce() -> V,
    {
        match self {
            Entry::Occupied(entry) => entry,
            Entry::Vacant(key, map) => {
                map.insert(key, creator());
                map.get_mut(key).unwrap()
            }
        }
    }

    /// Inserts the default value in the entry if it is vacant.
    ///
    /// Returns a mutable reference to the existing value if it is occupied, or
    /// a mutable reference to the newly added value if it is vacant.
    pub fn or_default(self) -> &'a mut V
    where
        V: Default,
    {
        match self {
            Entry::Occupied(entry) => entry,
            Entry::Vacant(key, map) => {
                map.insert(key, V::default());
                map.get_mut(key).unwrap()
            }
        }
    }

    /// Modifies the entry if it is occupied.
    pub fn and_modify<F>(self, f: F) -> Self
    where
        F: FnOnce(&mut V),
    {
        match self {
            Entry::Occupied(o) => {
                f(o);
                Entry::Occupied(o)
            }
            x => x,
        }
    }
}

impl<K: CopyKey, V: Clone> FromIterator<(K, V)> for VecMap<K, V> {
    fn from_iter<I: IntoIterator<Item = (K, V)>>(iter: I) -> Self {
        let iter = iter.into_iter();
        let (lower_bound, _) = iter.size_hint();

        let mut map = VecMap::with_capacity(lower_bound);
        for (key, value) in iter {
            map.insert(key, value);
        }
        map
    }
}

impl<K: CopyKey + fmt::Debug, V: fmt::Debug> fmt::Debug for VecMap<K, V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_map().entries(self.iter()).finish()
    }
}

#[macro_export]
macro_rules! vecmap {
    (@single $($x:tt)*) => (());
    (@count $($rest:expr),*) => (<[()]>::len(&[$($crate::vecmap!(@single $rest)),*]));

    ($($key:expr => $value:expr,)+) => { $crate::vecmap!($($key => $value),+) };
    ($($key:expr => $value:expr),*) => {
        {
            let _cap = $crate::vecmap!(@count $($key),*);
            let mut _map = $crate::vec_map::VecMap::with_capacity(_cap);
            $(
                let _ = _map.insert($key, $value);
            )*
            _map
        }
    };
}

#[macro_export]
macro_rules! impl_indexable{
    ( $( $Int: ty )+ ) => {
        $(
            impl CopyKey for $Int {
                #[inline]
                fn as_index(&self) -> usize {
                    *self as usize
                }

                #[inline]
                fn from_index(index:usize) -> $Int {
                    index as $Int
                }
            }
        )+
    }
}

impl_indexable!(u8 u16 u32 u64 u128 usize);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_with_capacity() {
        let map: VecMap<usize, usize> = VecMap::with_capacity(3);

        assert_eq!(3, map.data.len());
        assert_eq!(vec![None, None, None], map.data);
        assert!(map.is_empty());

        assert_eq!(0, map.keys.count_ones());
        assert_eq!(3, map.keys.count_zeros());
    }

    #[test]
    fn test_new() {
        let map: VecMap<usize, usize> = VecMap::new();
        assert!(map.is_empty());
        assert!(map.data.is_empty());
        assert!(map.keys.is_empty());
    }

    #[test]
    fn test_insert() {
        let mut map = VecMap::new();

        // insert in unallocated space
        assert_eq!(None, map.insert(3usize, "hi"));
        assert_eq!(vec![None, None, None, Some("hi")], map.data);
        assert_eq!(bitvec![0, 0, 0, 1], map.keys);

        // insert in allocated space
        assert_eq!(None, map.insert(1, "hello"));
        assert_eq!(vec![None, Some("hello"), None, Some("hi")], map.data);
        assert_eq!(bitvec![0, 1, 0, 1], map.keys);

        // overwrite existing item
        assert_eq!(Some("hi"), map.insert(3, "bye"));
        assert_eq!(vec![None, Some("hello"), None, Some("bye")], map.data);
        assert_eq!(bitvec![0, 1, 0, 1], map.keys);
    }

    #[test]
    fn test_remove() {
        let mut map = vecmap! { 9usize => "nine", 17 => "seventeen", 2 => "two"};
        assert_eq!(vec![2, 9, 17], map.keys().collect::<Vec<_>>());
        assert_eq!(3, map.len());

        // removing a non-existent key-value pair has no effect
        assert_eq!(None, map.remove(10));
        assert_eq!(3, map.len());
        assert_eq!(vec![2, 9, 17], map.keys().collect::<Vec<_>>());

        // removing an existing key-value pair correctly updates the map
        assert_eq!(Some("seventeen"), map.remove(17));
        assert_eq!(2, map.len());
        assert_eq!(vec![2, 9], map.keys().collect::<Vec<_>>());
    }

    #[test]
    fn test_entry_or_insert() {
        let mut map = VecMap::new();

        // non existing
        let return_value = map.entry(2u8).or_insert("hello");
        assert_eq!("hello", *return_value);
        assert_eq!(vecmap! { 2 => "hello" }, map);

        // already existing
        let return_value = map.entry(2).or_insert("bye");
        assert_eq!("hello", *return_value);
        assert_eq!(vecmap! { 2 => "hello" }, map);

        // overwrite through reference
        let result = map.entry(2).or_insert("this is ignored");
        *result = "bye";
        assert_eq!(vecmap! { 2 => "bye" }, map);
    }

    #[test]
    fn test_entry_or_insert_with() {
        let mut map = VecMap::new();

        // non existing
        let return_value = map.entry(2u16).or_insert_with(|| "hello");
        assert_eq!("hello", *return_value);
        assert_eq!(vecmap! { 2 => "hello" }, map);

        // already existing
        let return_value = map.entry(2).or_insert_with(|| "bye");
        assert_eq!("hello", *return_value);
        assert_eq!(vecmap! { 2 => "hello" }, map);

        // overwrite through reference
        let result = map.entry(2).or_insert_with(|| "this is ignored");
        *result = "bye";
        assert_eq!(vecmap! { 2 => "bye" }, map);
    }

    #[test]
    fn test_entry_or_default() {
        let mut map = VecMap::new();

        // non existing
        let return_value = map.entry(2u32).or_default();
        assert_eq!("", *return_value);
        assert_eq!(vecmap! { 2 => "" }, map);

        // already existing
        map.insert(4, "hello");
        let return_value = map.entry(4).or_default();
        assert_eq!("hello", *return_value);
        assert_eq!(vecmap! { 2 => "", 4 => "hello" }, map);

        // overwrite through reference
        let result = map.entry(4).or_default();
        *result = "bye";
        assert_eq!(vecmap! {2 => "", 4 => "bye"}, map);
    }

    #[test]
    fn test_entry_and_modify() {
        let mut map: VecMap<usize, usize> = VecMap::new();

        // empty entry, closure should not get called
        map.entry(2)
            .and_modify(|_| panic!("should not be called"))
            .or_default();

        // occupied entry, closure should get called
        map.entry(2).and_modify(|num| {
            *num = 10;
        });
        assert_eq!(vecmap! {2=> 10}, map);
    }

    #[test]
    fn test_get() {
        let map = vecmap! { 9u128 => "nine", 17 => "seventeen", 2 => "two"};
        assert_eq!(Some(&"nine"), map.get(9));
        assert_eq!(None, map.get(10));
        assert_eq!(None, map.get(10000));
    }

    #[test]
    fn test_get_mut() {
        let mut map = vecmap! { 9u16 => "nine", 17 => "seventeen", 2 => "two"};
        assert_eq!(Some(&mut "nine"), map.get_mut(9));
        *map.get_mut(9).unwrap() = "negen";
        assert_eq!(Some(&"negen"), map.get(9));

        assert_eq!(None, map.get_mut(10));
        assert_eq!(None, map.get_mut(10000));
    }

    #[test]
    fn test_len_and_is_empty() {
        let numbers = [3u64, 9, 0, 15, 24, 2, 17, 7, 4];
        let mut map = vecmap! {};
        assert_eq!(0, map.len());
        assert!(map.is_empty());
        for (i, num) in numbers.into_iter().enumerate() {
            map.insert(num, format!("number {num}"));
            assert_eq!(i + 1, map.len());
            assert!(!map.is_empty());
        }
    }

    #[test]
    fn test_contains_key() {
        let map = vecmap! { 9u128 => "nine", 17 => "seventeen", 2 => "two"};

        assert!(!map.contains_key(3));
        assert!(!map.contains_key(300));

        assert!(map.contains_key(9));
        assert!(map.contains_key(17));
        assert!(map.contains_key(2));
    }

    #[test]
    fn test_iter() {
        let map = vecmap! { 9u16 => "nine", 17 => "seventeen", 2 => "two"};

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
    }

    #[test]
    fn test_iter2() {
        let map = vecmap! { 9u16 => "nine", 17 => "seventeen", 2 => "two"};

        // forward
        let mut iter = map.iter2();
        // assert_eq!(3, iter.len());
        assert_eq!(Some((2, &"two")), iter.next());
        // assert_eq!(2, iter.len());
        assert_eq!(Some((9, &"nine")), iter.next());
        // assert_eq!(1, iter.len());
        assert_eq!(Some((17, &"seventeen")), iter.next());
        // assert_eq!(0, iter.len());
        assert_eq!(None, iter.next());

        // back, forward, back
        // let mut iter = map.iter2();
        // // assert_eq!(3, iter.len());
        // assert_eq!(Some((17, &"seventeen")), iter.next_back());
        // // assert_eq!(2, iter.len());
        // assert_eq!(Some((2, &"two")), iter.next());
        // // assert_eq!(1, iter.len());
        // assert_eq!(Some((9, &"nine")), iter.next_back());
        // assert_eq!(0, iter.len());
        // assert_eq!(None, iter.next_back());
    }

    #[derive(Copy, Clone)]
    enum TestEnum {
        A,
        B,
    }

    impl CopyKey for TestEnum {
        fn as_index(&self) -> usize {
            match self {
                Self::A => 0,
                Self::B => 1,
            }
        }

        fn from_index(index: usize) -> Self {
            match index {
                0 => Self::A,
                1 => Self::B,
                _ => panic!(),
            }
        }
    }
}
