#![warn(missing_docs, missing_debug_implementations)]
//! [`VecMap`] is a [`Vec`]-backed map, for faster random access.
mod iter;

use std::fmt;
use std::marker::PhantomData;
use std::ops::Index;

pub use crate::vec_map::iter::IntoIter;
pub use crate::vec_map::iter::Iter;
pub use crate::vec_map::iter::Keys;

/// A key that can be used in a map without needing a hasher.
///
/// There needs to be a 1:1 correspondence between `IndexKey` and it's index.
/// Typically this is used with a newtype. Default implementations exist for all
/// unsigned integer types.
pub trait IndexKey: Copy {
    /// Returns the unique index that this key is associated with.
    fn as_index(&self) -> usize;

    /// Converts the index back to the key.
    fn from_index(index: usize) -> Self;
}

/// A [`Vec`]-backed map.
///
/// It has faster random access performance and slower iteration speed compared
/// to other maps. Makes most sense for relatively dense maps or if iteration is
/// needed significantly less than random access. In case of doubt, benchmark it
/// for your usecase.
///
/// # Performance
///
/// `VecMap` outperforms `HashMap`, `IndexMap`, and `BTreeMap` for random access
/// (such as `get()`) and random modifications (such as `insert()`). For
/// modifications this is only true **iff `VecMap` does not need to do any
/// resizing**. Therefore, if performance is essential, it is strongly
/// recommended to initialize `VecMap` with `with_capacity()`.
///
/// Iteration order follows the natural ordering of [`IndexKey::as_index()`].
#[derive(Clone, Eq, PartialEq)]
pub struct VecMap<K, V> {
    data: Vec<Option<V>>,
    len: usize,
    _marker: PhantomData<K>,
}

impl<K, V: Clone> VecMap<K, V> {
    /// Initializes [`VecMap`] with capacity to hold exactly `n` elements in the
    /// index range of `0..n`.
    pub fn with_capacity(n: usize) -> Self {
        Self {
            data: vec![None; n],
            len: 0,
            _marker: PhantomData,
        }
    }
}

impl<K: IndexKey, V> VecMap<K, V> {
    /// Initializes an empty [`VecMap`].
    ///
    /// For performance reasons it's almost always better to avoid dynamic
    /// resizing by using [`Self::with_capacity()`] instead.
    pub const fn new() -> Self {
        Self {
            data: vec![],
            len: 0,
            _marker: PhantomData,
        }
    }

    /// Returns the number of elements the map can hold without reallocating.
    ///
    /// The index range of items that the map can hold without reallocating is
    /// `0..capacity`.
    pub fn capacity(&self) -> usize {
        self.data.len()
    }

    /// Inserts a key-value pair into the map.
    ///
    /// If the key is present in the map, the value is updated and the old value
    /// is returned. Otherwise, [`None`] is returned.
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        let index = key.as_index();
        if index >= self.capacity() {
            self.data
                .extend((0..=(index - self.data.len())).map(|_| None));
        }

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
            inner: self.data.iter().enumerate(),
            len: self.len,
            _marker: PhantomData,
        }
    }

    /// Returns an iterator over the keys of the map following the natural order
    /// of the keys.
    pub fn keys(&self) -> Keys<'_, K, V> {
        Keys {
            inner: self.data.iter().enumerate(),
            len: self.len,
            _marker: PhantomData,
        }
    }

    // TODO values()

    /// Clears all data from the [`VecMap`] without changing the capacity.
    pub fn clear(&mut self) {
        self.len = 0;
        self.data.clear();
    }
}

impl<K: IndexKey, V> Default for VecMap<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K: IndexKey, V> Index<K> for VecMap<K, V> {
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

#[derive(Debug)]
pub enum Entry<'a, K: IndexKey, V> {
    Vacant(K, &'a mut VecMap<K, V>),
    Occupied(&'a mut V),
}

impl<'a, K: IndexKey, V> Entry<'a, K, V> {
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

impl<K: IndexKey, V: Clone> FromIterator<(K, V)> for VecMap<K, V> {
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

impl<K: IndexKey + fmt::Debug, V: fmt::Debug> fmt::Debug for VecMap<K, V> {
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
            impl IndexKey for $Int {
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
    }

    #[test]
    fn test_new() {
        let map: VecMap<usize, usize> = VecMap::new();
        assert!(map.is_empty());
        assert!(map.data.is_empty());
    }

    #[test]
    fn test_insert() {
        let mut map = VecMap::new();

        // insert in unallocated space
        assert_eq!(None, map.insert(3usize, "hi"));
        assert_eq!(vec![None, None, None, Some("hi")], map.data);

        // insert in allocated space
        assert_eq!(None, map.insert(1, "hello"));
        assert_eq!(vec![None, Some("hello"), None, Some("hi")], map.data);

        // overwrite existing item
        assert_eq!(Some("hi"), map.insert(3, "bye"));
        assert_eq!(vec![None, Some("hello"), None, Some("bye")], map.data);
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
    fn test_enum() {
        #[derive(Copy, Clone, Eq, PartialEq, Debug)]
        enum TestEnum {
            A,
            B,
        }

        impl IndexKey for TestEnum {
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
        use TestEnum::A;
        use TestEnum::B;
        let mut map: VecMap<TestEnum, usize> = VecMap::with_capacity(40);
        map.insert(B, 20);
        map.insert(A, 17);

        assert_eq!(vec![(A, &17), (B, &20)], map.iter().collect::<Vec<_>>());
    }

    #[test]
    fn test_new_type() {
        #[derive(Copy, Clone, Eq, PartialEq, Debug)]
        struct MyNewType(u8);

        impl IndexKey for MyNewType {
            fn as_index(&self) -> usize {
                self.0 as usize
            }

            fn from_index(index: usize) -> Self {
                Self(u8::try_from(index).unwrap())
            }
        }
        let mut map: VecMap<MyNewType, ()> = VecMap::with_capacity(40);
        map.insert(MyNewType(39), ());
        map.insert(MyNewType(20), ());
        map.insert(MyNewType(10), ());

        assert_eq!(
            vec![MyNewType(10), MyNewType(20), MyNewType(39)],
            map.keys().collect::<Vec<_>>()
        );
    }

    #[test]
    fn test_capacity() {
        let mut map: VecMap<usize, ()> = VecMap::new();
        assert_eq!(0, map.capacity());
        assert!(map.is_empty());

        map.insert(0, ());
        assert_eq!(1, map.len());
        assert_eq!(1, map.capacity());

        map.insert(100, ());
        assert_eq!(2, map.len());
        assert_eq!(101, map.capacity());

        let mut map: VecMap<usize, ()> = VecMap::with_capacity(20);
        assert_eq!(20, map.capacity());
        assert!(map.is_empty());

        map.insert(0, ());
        assert_eq!(1, map.len());
        assert_eq!(20, map.capacity());

        map.insert(100, ());
        assert_eq!(2, map.len());
        assert_eq!(101, map.capacity());
    }
}
