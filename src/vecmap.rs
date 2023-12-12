use std::fmt;
use std::iter::Enumerate;
use std::iter::FusedIterator;
use std::ops::Index;

use api_model::problem::TripId;
use api_model::problem::TripIdNum;
use bitvec::bitvec;
use bitvec::order::Lsb0;
use bitvec::prelude::BitVec;
use bitvec::slice::IterOnes;

/// A map indexable by [`TripId`].
///
/// Iteration order follows the natural ordering of [`TripId`].
#[derive(Clone, Eq, PartialEq)]
pub struct VecTripMap<T> {
    data: Vec<Option<T>>,
    keys: BitVec<u64>,
    len: usize,
}

impl<T: Clone> VecTripMap<T> {
    pub fn with_capacity(n: usize) -> Self {
        let mut keys = BitVec::EMPTY;
        keys.extend(bitvec![0; n]);
        Self {
            data: vec![None; n],
            keys,
            len: 0,
        }
    }
}

impl<T> VecTripMap<T> {
    pub const fn new() -> Self {
        Self {
            data: vec![],
            keys: BitVec::EMPTY,
            len: 0,
        }
    }

    /// Inserts a key-value pair into the map.
    ///
    /// If the key is present in the map, the value is updated and the old value
    /// is returned. Otherwise, [`None`] is returned.
    pub fn insert(&mut self, key: TripId, value: T) -> Option<T> {
        let index = *key as usize;
        if index >= self.data.len() {
            self.keys.resize(index + 1, false);
            self.data
                .extend((0..(index - self.data.len() + 1)).map(|_| None));
        }

        self.keys.set(*key as usize, true);
        let existing = self.data[*key as usize].replace(value);
        if existing.is_none() {
            self.len += 1;
        }
        existing
    }

    /// Removes the key-value pair indicated by `key`.
    ///
    /// If the key was present, it is returned. Otherwise [`None`] is returned.
    pub fn remove(&mut self, key: TripId) -> Option<T> {
        let index = *key as usize;
        if index >= self.data.len() {
            None
        } else {
            self.keys.set(*key as usize, false);
            let existing = self.data[*key as usize].take();
            if existing.is_some() {
                self.len -= 1;
            }
            existing
        }
    }

    /// Get the given key's entry in the map for insertion and/or in-place
    /// manipulation.
    pub fn entry(&mut self, key: TripId) -> Entry<T> {
        let index = *key as usize;
        if index >= self.data.len() {
            Entry::Vacant(key, self)
        } else {
            if self.data[*key as usize].is_some() {
                return Entry::Occupied(self.data[*key as usize].as_mut().unwrap());
            }
            return Entry::Vacant(key, self);
        }
    }

    /// Returns a reference to the value associated with `key` if it exists,
    /// otherwise returns `None`.
    pub fn get(&self, key: TripId) -> Option<&T> {
        let index = *key as usize;
        if index >= self.data.len() {
            None
        } else {
            self.data[index].as_ref()
        }
    }

    /// Returns a mutable reference to the value associated with `key` if it
    /// exists, otherwise returns `None`.
    pub fn get_mut(&mut self, key: TripId) -> Option<&mut T> {
        let index = *key as usize;
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
    pub fn contains_key(&self, key: TripId) -> bool {
        self.get(key).is_some()
    }

    /// Returns an iterator over the key-value pairs of the map, following the
    /// natural order of the keys.
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            inner: self.keys.iter_ones(),
            values: &self.data,
            len: self.len,
        }
    }

    /// Returns an iterator over the keys of the map following the natural order
    /// of the keys.
    pub fn keys(&self) -> Keys<'_> {
        Keys {
            inner: self.keys.iter_ones(),
        }
    }

    // TODO values()

    pub fn clear(&mut self) {
        self.keys.clear();
        self.len = 0;
        self.data.clear();
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Keys<'a> {
    inner: IterOnes<'a, u64, Lsb0>,
}

impl<'a> Iterator for Keys<'a> {
    type Item = TripId;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|t| TripId::new(t as TripIdNum))
    }
}

#[derive(Clone, Copy)]
pub struct Iter<'a, T> {
    inner: IterOnes<'a, u64, Lsb0>,
    values: &'a Vec<Option<T>>,
    len: usize,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = (TripId, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|i| {
            self.len -= 1;
            (
                TripId::new(i as TripIdNum),
                self.values[i].as_ref().unwrap(),
            )
        })
    }
}

impl<'a, T> ExactSizeIterator for Iter<'a, T> {
    fn len(&self) -> usize {
        self.len
    }
}

impl<'a, T> DoubleEndedIterator for Iter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.inner.next_back().map(|i| {
            self.len -= 1;
            (
                TripId::new(i as TripIdNum),
                self.values[i].as_ref().unwrap(),
            )
        })
    }
}

impl<'a, T> FusedIterator for Iter<'a, T> {}

impl<'a, T: fmt::Debug> fmt::Debug for Iter<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let iter = Iter {
            inner: self.inner,
            values: self.values,
            len: self.len,
        };
        f.debug_list().entries(iter).finish()
    }
}

impl<T> IntoIterator for VecTripMap<T> {
    type Item = (TripId, T);

    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            inner: self.data.into_iter().enumerate(),
            len: self.len,
        }
    }
}

#[derive(Debug)]
pub struct IntoIter<T> {
    inner: Enumerate<std::vec::IntoIter<Option<T>>>,
    len: usize,
}

impl<T> Iterator for IntoIter<T> {
    type Item = (TripId, T);

    // TODO should this use the bitset when the data is less dense?
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.inner.next() {
                Some((_, None)) => continue,
                Some((i, Some(v))) => return Some((TripId::new(i as TripIdNum), v)),
                None => return None,
            }
        }
    }
}

impl<T> ExactSizeIterator for IntoIter<T> {
    fn len(&self) -> usize {
        self.len
    }
}

impl<T: Clone> Default for VecTripMap<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Index<TripId> for VecTripMap<T> {
    type Output = T;

    fn index(&self, index: TripId) -> &Self::Output {
        let index = *index as usize;
        if index >= self.data.len() {
            panic!("out of bounds");
        } else {
            self.data[index]
                .as_ref()
                .unwrap_or_else(|| panic!("doesn't exist"))
        }
    }
}

#[derive(Debug)]
pub enum Entry<'a, T> {
    Vacant(TripId, &'a mut VecTripMap<T>),
    Occupied(&'a mut T),
}

impl<'a, T> Entry<'a, T> {
    /// Inserts the given default value in the entry if it is vacant.
    ///
    /// Returns a mutable reference to the existing value if it is occupied, or
    /// a mutable reference to the newly added value if it is vacant.
    pub fn or_insert(self, default: T) -> &'a mut T {
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
    pub fn or_insert_with<F>(self, creator: F) -> &'a mut T
    where
        F: FnOnce() -> T,
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
    pub fn or_default(self) -> &'a mut T
    where
        T: Default,
    {
        match self {
            Entry::Occupied(entry) => entry,
            Entry::Vacant(key, map) => {
                map.insert(key, T::default());
                map.get_mut(key).unwrap()
            }
        }
    }

    /// Modifies the entry if it is occupied.
    pub fn and_modify<F>(self, f: F) -> Self
    where
        F: FnOnce(&mut T),
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

impl<T: Clone> FromIterator<(TripId, T)> for VecTripMap<T> {
    fn from_iter<I: IntoIterator<Item = (TripId, T)>>(iter: I) -> Self {
        let iter = iter.into_iter();
        let (lower_bound, _) = iter.size_hint();

        let mut map = VecTripMap::with_capacity(lower_bound);
        for (key, value) in iter {
            map.insert(key, value);
        }
        map
    }
}

impl<T: fmt::Debug> fmt::Debug for VecTripMap<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_map().entries(self.iter()).finish()
    }
}

#[macro_export]
macro_rules! vectripmap {
    (@single $($x:tt)*) => (());
    (@count $($rest:expr),*) => (<[()]>::len(&[$($crate::vectripmap!(@single $rest)),*]));

    ($($key:expr => $value:expr,)+) => { $crate::vectripmap!($($key => $value),+) };
    ($($key:expr => $value:expr),*) => {
        {
            let _cap = $crate::vectripmap!(@count $($key),*);
            let mut _map = $crate::vec_trip_map::VecTripMap::with_capacity(_cap);
            $(
                let _ = _map.insert($key, $value);
            )*
            _map
        }
    };
}

#[cfg(test)]
mod test {
    use super::*;

    const fn t(id: TripIdNum) -> TripId {
        TripId::new(id)
    }

    #[test]
    fn test_with_capacity() {
        let map: VecTripMap<usize> = VecTripMap::with_capacity(3);

        assert_eq!(3, map.data.len());
        assert_eq!(vec![None, None, None], map.data);
        assert!(map.is_empty());

        assert_eq!(0, map.keys.count_ones());
        assert_eq!(3, map.keys.count_zeros());
    }

    #[test]
    fn test_new() {
        let map: VecTripMap<usize> = VecTripMap::new();
        assert!(map.is_empty());
        assert!(map.data.is_empty());
        assert!(map.keys.is_empty());
    }

    #[test]
    fn test_insert() {
        let mut map = VecTripMap::new();

        // insert in unallocated space
        assert_eq!(None, map.insert(t(3), "hi"));
        assert_eq!(vec![None, None, None, Some("hi")], map.data);
        assert_eq!(bitvec![0, 0, 0, 1], map.keys);

        // insert in allocated space
        assert_eq!(None, map.insert(t(1), "hello"));
        assert_eq!(vec![None, Some("hello"), None, Some("hi")], map.data);
        assert_eq!(bitvec![0, 1, 0, 1], map.keys);

        // overwrite existing item
        assert_eq!(Some("hi"), map.insert(t(3), "bye"));
        assert_eq!(vec![None, Some("hello"), None, Some("bye")], map.data);
        assert_eq!(bitvec![0, 1, 0, 1], map.keys);
    }

    #[test]
    fn test_remove() {
        let mut map = vectripmap! { t(9) => "nine", t(17) => "seventeen", t(2) => "two"};
        assert_eq!(vec![t(2), t(9), t(17)], map.keys().collect::<Vec<_>>());
        assert_eq!(3, map.len());

        // removing a non-existent key-value pair has no effect
        assert_eq!(None, map.remove(t(10)));
        assert_eq!(3, map.len());
        assert_eq!(vec![t(2), t(9), t(17)], map.keys().collect::<Vec<_>>());

        // removing an existing key-value pair correctly updates the map
        assert_eq!(Some("seventeen"), map.remove(t(17)));
        assert_eq!(2, map.len());
        assert_eq!(vec![t(2), t(9)], map.keys().collect::<Vec<_>>());
    }

    #[test]
    fn test_entry_or_insert() {
        let mut map = VecTripMap::new();

        // non existing
        let return_value = map.entry(t(2)).or_insert("hello");
        assert_eq!("hello", *return_value);
        assert_eq!(vectripmap! { t(2) => "hello" }, map);

        // already existing
        let return_value = map.entry(t(2)).or_insert("bye");
        assert_eq!("hello", *return_value);
        assert_eq!(vectripmap! { t(2) => "hello" }, map);

        // overwrite through reference
        let result = map.entry(t(2)).or_insert("this is ignored");
        *result = "bye";
        assert_eq!(vectripmap! { t(2) => "bye" }, map);
    }

    #[test]
    fn test_entry_or_insert_with() {
        let mut map = VecTripMap::new();

        // non existing
        let return_value = map.entry(t(2)).or_insert_with(|| "hello");
        assert_eq!("hello", *return_value);
        assert_eq!(vectripmap! { t(2) => "hello" }, map);

        // already existing
        let return_value = map.entry(t(2)).or_insert_with(|| "bye");
        assert_eq!("hello", *return_value);
        assert_eq!(vectripmap! { t(2) => "hello" }, map);

        // overwrite through reference
        let result = map.entry(t(2)).or_insert_with(|| "this is ignored");
        *result = "bye";
        assert_eq!(vectripmap! { t(2) => "bye" }, map);
    }

    #[test]
    fn test_entry_or_default() {
        let mut map = VecTripMap::new();

        // non existing
        let return_value = map.entry(t(2)).or_default();
        assert_eq!("", *return_value);
        assert_eq!(vectripmap! { t(2) => "" }, map);

        // already existing
        map.insert(t(4), "hello");
        let return_value = map.entry(t(4)).or_default();
        assert_eq!("hello", *return_value);
        assert_eq!(vectripmap! { t(2) => "", t(4) => "hello" }, map);

        // overwrite through reference
        let result = map.entry(t(4)).or_default();
        *result = "bye";
        assert_eq!(vectripmap! {t(2) => "", t(4) => "bye"}, map);
    }

    #[test]
    fn test_entry_and_modify() {
        let mut map: VecTripMap<usize> = VecTripMap::new();

        // empty entry, closure should not get called
        map.entry(t(2))
            .and_modify(|_| panic!("should not be called"))
            .or_default();

        // occupied entry, closure should get called
        map.entry(t(2)).and_modify(|num| {
            *num = 10;
        });
        assert_eq!(vectripmap! {t(2)=> 10}, map);
    }

    #[test]
    fn test_get() {
        let map = vectripmap! { t(9) => "nine", t(17) => "seventeen", t(2) => "two"};
        assert_eq!(Some(&"nine"), map.get(t(9)));
        assert_eq!(None, map.get(t(10)));
        assert_eq!(None, map.get(t(10000)));
    }

    #[test]
    fn test_get_mut() {
        let mut map = vectripmap! { t(9) => "nine", t(17) => "seventeen", t(2) => "two"};
        assert_eq!(Some(&mut "nine"), map.get_mut(t(9)));
        *map.get_mut(t(9)).unwrap() = "negen";
        assert_eq!(Some(&"negen"), map.get(t(9)));

        assert_eq!(None, map.get_mut(t(10)));
        assert_eq!(None, map.get_mut(t(10000)));
    }

    #[test]
    fn test_len_and_is_empty() {
        let numbers = [3, 9, 0, 15, 24, 2, 17, 7, 4];
        let mut map = vectripmap! {};
        assert_eq!(0, map.len());
        assert!(map.is_empty());
        for (i, num) in numbers.into_iter().enumerate() {
            map.insert(TripId::new(num), format!("number {num}"));
            assert_eq!(i + 1, map.len());
            assert!(!map.is_empty());
        }
    }

    #[test]
    fn test_contains_key() {
        let map = vectripmap! { t(9) => "nine", t(17) => "seventeen", t(2) => "two"};

        assert!(!map.contains_key(t(3)));
        assert!(!map.contains_key(t(300)));

        assert!(map.contains_key(t(9)));
        assert!(map.contains_key(t(17)));
        assert!(map.contains_key(t(2)));
    }

    #[test]
    fn test_iter() {
        let map = vectripmap! { t(9) => "nine", t(17) => "seventeen", t(2) => "two"};

        // forward
        let mut iter = map.iter();
        assert_eq!(3, iter.len());
        assert_eq!(Some((t(2), &"two")), iter.next());
        assert_eq!(2, iter.len());
        assert_eq!(Some((t(9), &"nine")), iter.next());
        assert_eq!(1, iter.len());
        assert_eq!(Some((t(17), &"seventeen")), iter.next());
        assert_eq!(0, iter.len());
        assert_eq!(None, iter.next());

        // back, forward, back
        let mut iter = map.iter();
        assert_eq!(3, iter.len());
        assert_eq!(Some((t(17), &"seventeen")), iter.next_back());
        assert_eq!(2, iter.len());
        assert_eq!(Some((t(2), &"two")), iter.next());
        assert_eq!(1, iter.len());
        assert_eq!(Some((t(9), &"nine")), iter.next_back());
        assert_eq!(0, iter.len());
        assert_eq!(None, iter.next_back());
    }
}