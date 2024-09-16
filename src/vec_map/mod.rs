#![warn(missing_docs, missing_debug_implementations)]
//! [`VecMap`] is a [`Vec`]-backed map, for faster random access.
mod iter;

use std::cmp::Ordering;
use std::fmt;
use std::marker::PhantomData;
use std::ops::Index;
use std::ops::IndexMut;

pub use crate::vec_map::iter::*;

/// A key that can be used in a map without needing a hasher.
///
/// There needs to be a 1:1 correspondence between `IndexKey` and it's index.
/// Typically this is used with a newtype. A blanket implementation exists for
/// types that implement `From<usize>`, `Into<usize>`, and `Copy`. By using a
/// crate such as [derive_more](https://docs.rs/derive_more/latest/derive_more/)
/// these traits can be derived.
pub trait IndexKey: Copy {
    /// Returns the unique index that this key is associated with.
    fn as_index(&self) -> usize;

    /// Converts the index back to the key.
    fn from_index(index: usize) -> Self;
}

impl<T> IndexKey for T
where
    T: From<usize> + Into<usize> + Copy,
{
    fn as_index(&self) -> usize {
        (*self).into()
    }

    fn from_index(index: usize) -> Self {
        index.into()
    }
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
///
/// # Serialization and deserialization
///
/// An optional feature that can be unlocked with the `serde` feature. `VecMap`s
/// are serialized and deserialized as `Vec<Option<V>>`.
#[derive(Clone)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize),
    serde(from = "Vec<Option<V>>")
)]
pub struct VecMap<K, V> {
    data: Vec<Option<V>>,
    len: usize,
    _marker: PhantomData<K>,
}

impl<K: IndexKey, V> VecMap<K, V> {
    /// Initializes an empty [`VecMap`].
    ///
    /// For performance reasons it's almost always better to avoid dynamic
    /// resizing by using [`Self::with_capacity()`] instead.
    #[must_use]
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
    #[must_use]
    pub fn capacity(&self) -> usize {
        self.data.len()
    }

    /// Initializes [`VecMap`] with capacity to hold exactly `n` elements in the
    /// index range of `0..n`.
    #[must_use]
    pub fn with_capacity(n: usize) -> Self {
        let mut data = Vec::with_capacity(n);
        data.resize_with(n, || None);
        Self {
            data,
            len: 0,
            _marker: PhantomData,
        }
    }

    /// Initializes [`VecMap`] with `n` occurences of `elem`.
    pub fn from_elem(elem: V, n: usize) -> Self
    where
        V: Clone,
    {
        Self {
            data: vec![Some(elem); n],
            len: n,
            _marker: PhantomData,
        }
    }

    /// Clears all data from the [`VecMap`] without changing the capacity.
    pub fn clear(&mut self) {
        self.len = 0;
        let capacity = self.data.len();
        self.data.clear();
        self.data.resize_with(capacity, || None);
    }

    /// Reserve capacity for `additional` key-value pairs.
    pub fn reserve(&mut self, additional: usize) {
        self.data.resize_with(self.data.len() + additional, || None);
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

    /// Removes the last key-value pair.
    ///
    /// Worst case performance is O(n) in case the value is at the first index,
    /// where n = the capacity.
    pub fn pop(&mut self) -> Option<(K, V)> {
        if self.is_empty() {
            None
        } else {
            self.data.iter_mut().enumerate().rev().find_map(|(i, x)| {
                x.take().map(|x| {
                    self.len -= 1;
                    (K::from_index(i), x)
                })
            })
        }
    }

    /// Iterates over each key-value pair in the map and keep those where the
    /// closure `keep` returns `true`.
    ///
    /// The elements are visited in order.
    pub fn retain<F>(&mut self, mut keep: F)
    where
        F: FnMut(K, &mut V) -> bool,
    {
        if !self.is_empty() {
            self.data
                .iter_mut()
                .enumerate()
                .for_each(|(i, value_option)| {
                    if let Some(value) = value_option.as_mut() {
                        if !keep(K::from_index(i), value) {
                            self.len -= 1;
                            value_option.take();
                        }
                    }
                });
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
    #[must_use]
    pub const fn len(&self) -> usize {
        self.len
    }

    /// Returns `true` if the map contains no elements.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns `true` if the map contains an item with the specified `key`.
    pub fn contains_key(&self, key: K) -> bool {
        self.get(key).is_some()
    }

    /// Returns an iterator over the key-value pairs of the map, following the
    /// natural order of the keys.
    #[must_use]
    pub fn iter(&self) -> Iter<'_, K, V> {
        Iter {
            inner: self.data.iter().enumerate(),
            len: self.len,
            _marker: PhantomData,
        }
    }

    /// Returns an iterator over the key-value pairs of the map, with the values
    /// being mutable, following the natural order of the keys.
    pub fn iter_mut(&mut self) -> IterMut<'_, K, V> {
        IterMut {
            inner: self.data.iter_mut().enumerate(),
            len: self.len,
            _marker: PhantomData,
        }
    }

    /// Returns an iterator over the keys of the map following the natural order
    /// of the keys.
    #[must_use]
    pub fn keys(&self) -> Keys<'_, K, V> {
        Keys {
            inner: self.data.iter().enumerate(),
            len: self.len,
            _marker: PhantomData,
        }
    }

    /// Returns an iterator over the values of the map following the natural
    /// order of the keys.
    #[must_use]
    pub fn values(&self) -> Values<'_, V> {
        Values {
            inner: self.data.iter(),
            len: self.len,
        }
    }
}

impl<K: IndexKey, V> Default for VecMap<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K, V: PartialEq> PartialEq for VecMap<K, V> {
    fn eq(&self, other: &Self) -> bool {
        if self.len != other.len {
            return false;
        }

        let shared_capacity = self.data.len().min(other.data.len());
        if self.data[..shared_capacity] != other.data[..shared_capacity] {
            return false;
        }

        match self.data.len().cmp(&other.data.len()) {
            Ordering::Less => other.data[shared_capacity..].iter().all(Option::is_none),
            Ordering::Equal => true,
            Ordering::Greater => self.data[shared_capacity..].iter().all(Option::is_none),
        }
    }
}

impl<K, V: Eq> Eq for VecMap<K, V> {}

impl<K: IndexKey, V> Index<K> for VecMap<K, V> {
    type Output = V;

    fn index(&self, key: K) -> &Self::Output {
        let index = key.as_index();
        if index >= self.data.len() {
            panic!("{index} is out of bounds");
        } else {
            self.data[index]
                .as_ref()
                .unwrap_or_else(|| panic!("There is no item at index {index}"))
        }
    }
}

impl<K: IndexKey, V> IndexMut<K> for VecMap<K, V> {
    fn index_mut(&mut self, key: K) -> &mut Self::Output {
        let index = key.as_index();
        if index >= self.data.len() {
            panic!("{index} is out of bounds");
        } else {
            self.data[index]
                .as_mut()
                .unwrap_or_else(|| panic!("There is no item at index {index}"))
        }
    }
}

/// Entry for an existing key-value pair or a vacant location to insert one.
pub enum Entry<'a, K: IndexKey, V> {
    /// Vacant slot (i.e. the key does not exist in the map).
    Vacant(K, &'a mut VecMap<K, V>),
    /// Occupied slot.
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
    #[allow(clippy::return_self_not_must_use)] // no need to use entry after this
    pub fn and_modify<F>(self, f: F) -> Self
    where
        F: FnOnce(&mut V),
    {
        match self {
            Entry::Occupied(o) => {
                f(o);
                Entry::Occupied(o)
            }
            x @ Entry::Vacant(_, _) => x,
        }
    }
}

impl<K: IndexKey + fmt::Debug, V: fmt::Debug> fmt::Debug for Entry<'_, K, V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Entry::Occupied(ref value) => f.debug_tuple(stringify!(Entry)).field(value).finish(),
            Entry::Vacant(ref key, _) => f.debug_tuple(stringify!(Entry)).field(key).finish(),
        }
    }
}

impl<K: IndexKey, V> FromIterator<(K, V)> for VecMap<K, V> {
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

impl<K: IndexKey, V> Extend<(K, V)> for VecMap<K, V> {
    fn extend<T: IntoIterator<Item = (K, V)>>(&mut self, iter: T) {
        // extend does not attempt to reserve additional space because the space needed
        // is dependent on the keys that are added
        iter.into_iter().for_each(|(key, value)| {
            self.insert(key, value);
        });
    }
}

impl<K: IndexKey + fmt::Debug, V: fmt::Debug> fmt::Debug for VecMap<K, V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_map().entries(self.iter()).finish()
    }
}

impl<K, V> From<Vec<Option<V>>> for VecMap<K, V> {
    fn from(value: Vec<Option<V>>) -> Self {
        Self {
            len: value.iter().filter(|x| x.is_some()).count(),
            data: value,
            _marker: PhantomData,
        }
    }
}

#[cfg(feature = "serde")]
impl<K, V> serde::Serialize for VecMap<K, V>
where
    K: IndexKey,
    V: serde::Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.collect_seq(self.data.iter())
    }
}

/// Create a `VecMap` containing the arguments.
///
/// There are two forms of this macro:
///
/// - Create a [`VecMap`] containing a give list of key-value pairs:
///
/// ```
/// # use more_collections::vecmap;
/// let map = vecmap! {
///     1usize => "a",
///     2 => "b",
/// };
/// assert_eq!(map[1], "a");
/// assert_eq!(map[2], "b");
/// assert_eq!(map.get(3), None);
///
/// // 1 is the first key
/// assert_eq!(map.keys().next(), Some(1));
/// ```
/// - Create a [`VecMap`] from a given element and size:
/// ```
/// # use more_collections::vecmap;
/// # use more_collections::VecMap;
/// let counters: VecMap<usize, usize> = vecmap! { 0; 3 };
///
/// assert_eq!(
///     vec![0, 0, 0],
///     counters.values().copied().collect::<Vec<_>>()
/// );
/// assert_eq!(3, counters.len());
/// assert_eq!(3, counters.capacity());
/// ```
#[macro_export]
macro_rules! vecmap {
    (@single $($x:tt)*) => (());
    (@count $($rest:expr),*) => (<[()]>::len(&[$($crate::vecmap!(@single $rest)),*]));

    ($elem:expr; $n:expr) => (
        $crate::vec_map::VecMap::from_elem($elem, $n);
    );

    ($($key:expr => $value:expr,)+) => { $crate::vecmap!($($key => $value),+) };
    ($($key:expr => $value:expr),*) => {
        {
            let _cap = $crate::vecmap!(@count $($key),*);
            let mut _map = $crate::vec_map::VecMap::with_capacity(_cap);
            $(
                #[allow(let_underscore_drop)]
                let _ = _map.insert($key, $value);
            )*
            _map
        }
    };
}

#[cfg(test)]
mod test {
    use derive_more::From;
    use derive_more::Into;

    use super::*;

    #[derive(Into, From, Copy, Clone, Debug)]
    pub(in crate::vec_map) struct MyKey(pub(crate) usize);

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
    fn test_pop() {
        let mut map = vecmap! { 9usize => "nine", 17 => "seventeen", 2 => "two"};
        assert_eq!(18, map.capacity());
        assert_eq!(3, map.len());
        assert_eq!(Some((17, "seventeen")), map.pop());
        assert_eq!(18, map.capacity());
        assert_eq!(2, map.len());
        assert_eq!(Some((9, "nine")), map.pop());
        assert_eq!(18, map.capacity());
        assert_eq!(1, map.len());
        assert_eq!(Some((2, "two")), map.pop());
        assert_eq!(18, map.capacity());
        assert_eq!(0, map.len());
        assert_eq!(None, map.pop());
        assert_eq!(18, map.capacity());
        assert_eq!(0, map.len());
    }

    #[test]
    fn test_retain_by_key() {
        let mut map = vecmap! { 9usize => "nine".to_string(), 17 => "seventeen".to_string(), 2 => "two".to_string()};
        map.retain(|k, _| k < 9);
        assert_eq!(1, map.len());
        assert_eq!(
            vec![(2, "two".to_string())],
            map.into_iter().collect::<Vec<_>>()
        );
    }

    #[test]
    fn test_retain_by_value() {
        let mut map = vecmap! { 9usize => "nine".to_string(), 17 => "seventeen".to_string(), 2 => "two".to_string()};
        map.retain(|_, s| s.len() > 8);
        assert_eq!(1, map.len());
        assert_eq!(
            vec![(17, "seventeen".to_string())],
            map.into_iter().collect::<Vec<_>>()
        );
    }

    #[test]
    fn test_retain_mut_value() {
        let mut map = vecmap! { 9usize => "nine".to_string(), 17 => "seventeen".to_string(), 2 => "two".to_string()};
        map.retain(|_, s| {
            if s.len() < 8 {
                s.push_str("-yes");
            }
            true
        });
        assert_eq!(3, map.len());
        assert_eq!(
            vec![
                (2, "two-yes".to_string()),
                (9, "nine-yes".to_string()),
                (17, "seventeen".to_string())
            ],
            map.into_iter().collect::<Vec<_>>()
        );
    }

    #[test]
    fn test_entry_or_insert() {
        let mut map = VecMap::new();

        // non existing
        let return_value = map.entry(MyKey(2)).or_insert("hello");
        assert_eq!("hello", *return_value);
        assert_eq!(vecmap! { MyKey(2) => "hello" }, map);

        // already existing
        let return_value = map.entry(MyKey(2)).or_insert("bye");
        assert_eq!("hello", *return_value);
        assert_eq!(vecmap! { MyKey(2) => "hello" }, map);

        // overwrite through reference
        let result = map.entry(MyKey(2)).or_insert("this is ignored");
        *result = "bye";
        assert_eq!(vecmap! { MyKey(2) => "bye" }, map);
    }

    #[test]
    fn test_entry_or_insert_with() {
        let mut map = VecMap::new();

        // non existing
        let return_value = map.entry(MyKey(2)).or_insert_with(|| "hello");
        assert_eq!("hello", *return_value);
        assert_eq!(vecmap! { MyKey(2) => "hello" }, map);

        // already existing
        let return_value = map.entry(MyKey(2)).or_insert_with(|| "bye");
        assert_eq!("hello", *return_value);
        assert_eq!(vecmap! { MyKey(2) => "hello" }, map);

        // overwrite through reference
        let result = map.entry(MyKey(2)).or_insert_with(|| "this is ignored");
        *result = "bye";
        assert_eq!(vecmap! { MyKey(2) => "bye" }, map);
    }

    #[test]
    fn test_entry_or_default() {
        let mut map = VecMap::new();

        // non existing
        let return_value = map.entry(MyKey(2)).or_default();
        assert_eq!("", *return_value);
        assert_eq!(vecmap! { MyKey(2) => "" }, map);

        // already existing
        map.insert(MyKey(4), "hello");
        let return_value = map.entry(MyKey(4)).or_default();
        assert_eq!("hello", *return_value);
        assert_eq!(vecmap! { MyKey(2) => "", MyKey(4) => "hello" }, map);

        // overwrite through reference
        let result = map.entry(MyKey(4)).or_default();
        *result = "bye";
        assert_eq!(vecmap! {MyKey(2) => "", MyKey(4) => "bye"}, map);
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
        let map = vecmap! { MyKey(9) => "nine", MyKey(17) => "seventeen", MyKey(2) => "two"};
        assert_eq!(Some(&"nine"), map.get(MyKey(9)));
        assert_eq!(None, map.get(MyKey(10)));
        assert_eq!(None, map.get(MyKey(10000)));
    }

    #[test]
    fn test_get_mut() {
        let mut map = vecmap! { MyKey(9) => "nine", MyKey(17) => "seventeen", MyKey(2) => "two"};
        assert_eq!(Some(&mut "nine"), map.get_mut(MyKey(9)));
        *map.get_mut(MyKey(9)).unwrap() = "negen";
        assert_eq!(Some(&"negen"), map.get(MyKey(9)));

        assert_eq!(None, map.get_mut(MyKey(10)));
        assert_eq!(None, map.get_mut(MyKey(10000)));
    }

    #[test]
    fn test_len_and_is_empty() {
        let numbers = [3, 9, 0, 15, 24, 2, 17, 7, 4];
        let mut map = vecmap! {};
        assert_eq!(0, map.len());
        assert!(map.is_empty());
        for (i, num) in numbers.into_iter().enumerate() {
            map.insert(MyKey(num), format!("number {num}"));
            assert_eq!(i + 1, map.len());
            assert!(!map.is_empty());
        }
    }

    #[test]
    fn test_contains_key() {
        let map = vecmap! { MyKey(9) => "nine", MyKey(17) => "seventeen", MyKey(2) => "two"};

        assert!(!map.contains_key(MyKey(3)));
        assert!(!map.contains_key(MyKey(300)));

        assert!(map.contains_key(MyKey(9)));
        assert!(map.contains_key(MyKey(17)));
        assert!(map.contains_key(MyKey(2)));
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

    #[test]
    fn test_index_and_index_mut() {
        let immutable_map =
            vecmap! { MyKey(8) => "august", MyKey(13) => "thirteen", MyKey(22) => "twentytwo"};
        assert_eq!("august", immutable_map[MyKey(8)]);
        assert_eq!("thirteen", immutable_map[MyKey(13)]);
        assert_eq!("twentytwo", immutable_map[MyKey(22)]);

        let mut map =
            vecmap! { MyKey(8) => "august", MyKey(13) => "thirteen", MyKey(22) => "twentytwo"};
        assert_eq!("august", map[MyKey(8)]);
        assert_eq!("thirteen", map[MyKey(13)]);
        assert_eq!("twentytwo", map[MyKey(22)]);

        map[MyKey(8)] = "eight";
        assert_eq!("eight", map[MyKey(8)]);
    }

    #[test]
    #[should_panic(expected = "100 is out of bounds")]
    fn test_index_out_of_bounds_panics() {
        let immutable_map =
            vecmap! { MyKey(8) => "august", MyKey(13) => "thirteen", MyKey(22) => "twentytwo"};
        let _ = immutable_map[MyKey(100)];
    }

    #[test]
    #[should_panic(expected = "There is no item at index 1")]
    fn test_index_non_existing_panics() {
        let immutable_map =
            vecmap! { MyKey(8) => "august", MyKey(13) => "thirteen", MyKey(22) => "twentytwo"};
        let _ = immutable_map[MyKey(1)];
    }

    #[test]
    #[should_panic(expected = "100 is out of bounds")]
    #[allow(unused_must_use)]
    fn test_index_mut_out_of_bounds_panics() {
        let mut map =
            vecmap! { MyKey(8) => "august", MyKey(13) => "thirteen", MyKey(22) => "twentytwo"};
        let _ = &mut map[MyKey(100)];
    }

    #[test]
    #[should_panic(expected = "There is no item at index 1")]
    #[allow(unused_must_use)]
    fn test_index_mut_non_existing_panics() {
        // #[allow("unused-mut")]
        let mut map =
            vecmap! { MyKey(8) => "august", MyKey(13) => "thirteen", MyKey(22) => "twentytwo"};
        let _ = &mut map[MyKey(1)];
    }

    #[test]
    fn test_clear() {
        let mut map =
            vecmap! { MyKey(8) => "august", MyKey(13) => "thirteen", MyKey(22) => "twentytwo"};
        assert_eq!(23, map.capacity());
        assert_eq!(3, map.len());
        map.clear();
        assert_eq!(23, map.capacity());
        assert_eq!(0, map.len());
    }

    #[test]
    fn test_reserve() {
        let mut map: VecMap<MyKey, ()> = vecmap! {};
        assert_eq!(0, map.capacity());
        assert!(map.is_empty());

        map.reserve(7);
        assert_eq!(7, map.capacity());
        assert!(map.is_empty());

        map.reserve(7);
        assert_eq!(14, map.capacity());
        assert!(map.is_empty());
    }

    #[test]
    fn test_extend() {
        let mut map: VecMap<MyKey, ()> = vecmap! {};
        assert_eq!(0, map.capacity());
        assert!(map.is_empty());

        map.extend([(MyKey(7), ()), (MyKey(2), ())]);
        assert_eq!(8, map.capacity());
        assert_eq!(2, map.len());
    }

    #[test]
    fn test_eq_with_different_capacities() {
        let map1 = VecMap {
            data: vec![None, Some(1)],
            len: 1,
            _marker: PhantomData::<MyKey>,
        };
        let map2 = VecMap {
            data: vec![None, Some(1), None, None],
            len: 1,
            _marker: PhantomData::<MyKey>,
        };
        assert_eq!(map1, map2);
    }

    #[cfg(feature = "serde")]
    mod serde {
        use super::*;

        #[test]
        fn test_serde() {
            let input = vecmap! {MyKey(2)=> "hi", MyKey(4) => "four"};

            let serialized_str = serde_json::to_string(&input).unwrap();
            assert_eq!("[null,null,\"hi\",null,\"four\"]", serialized_str);

            let deserialized =
                serde_json::from_str::<VecMap<MyKey, &str>>(&serialized_str).unwrap();
            assert_eq!(input, deserialized);
            assert_eq!(2, deserialized.len());
            assert_eq!(5, deserialized.capacity());

            // trailing null is ignored
            let deserialized =
                serde_json::from_str::<VecMap<MyKey, &str>>("[\"test\",null]").unwrap();
            assert_eq!(vecmap! {MyKey(0)=> "test"}, deserialized);
            assert_eq!(1, deserialized.len());
            assert_eq!(2, deserialized.capacity());

            // empty
            let input: VecMap<MyKey, &str> = VecMap::new();
            let serialized_str = serde_json::to_string(&input).unwrap();
            assert_eq!("[]", serialized_str);

            let deserialized =
                serde_json::from_str::<VecMap<MyKey, &str>>(&serialized_str).unwrap();
            assert!(deserialized.is_empty());
            assert_eq!(0, deserialized.capacity());
        }
    }
}
