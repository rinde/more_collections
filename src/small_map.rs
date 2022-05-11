use std::fmt;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::mem;
use std::ops::Index;
use std::ops::IndexMut;

use ::core::hash::Hash;
use indexmap::Equivalent;
use smallvec::SmallVec;

use crate::FastIndexMap;

/// A map-like container that can store a specified number of elements inline.
///
/// `SmallMap` shares most of its API with, and behaves like
/// [IndexMap](indexmap::IndexMap). It can store a limited amount of data
/// inline, backed by [SmallVec]. If the data exceeds the limit `C`, `SmallMap`
/// will move _all_ its data over to the heap in the form of an `IndexMap`. For
/// performance reasons, transitions between heap and inline storage should
/// generally be avoided.
///
/// The `SmallMap` datastructure is meant for situations where the data does not
/// exceed `C` _most of the time_ but it still needs to support cases where the
/// data _does_ exceed `C`.
///
/// # Example
///
/// ```
/// use fast_hash_collections::SmallMap;
///
/// let mut map = SmallMap::<usize, String, 3>::new();
/// // The map can hold up to three items inline
/// map.insert(0, "zero".to_string());
/// map.insert(1, "one".to_string());
/// map.insert(2, "two".to_string());
/// assert_eq!(3, map.len());
/// assert!(map.is_inline());
///
/// // Adding the fourth item will move the map to the heap
/// map.insert(3, "three".to_string());
/// assert_eq!(4, map.len());
/// assert!(!map.is_inline());
/// ```
#[derive(Default, Clone)]
pub struct SmallMap<K, V, const C: usize> {
    data: MapData<K, V, C>,
}

#[derive(Debug, Clone)]
enum MapData<K, V, const C: usize> {
    Inline(SmallVec<[(K, V); C]>),
    Heap(FastIndexMap<K, V>),
}

impl<K, V, const C: usize> SmallMap<K, V, C> {
    /// Create a new map.
    pub fn new() -> Self {
        debug_assert!(
                C > 0,
                "Cannot instantiate SmallMap with no inline capacity, use positive capacity or use IndexMap instead",
            );
        SmallMap {
            data: MapData::Inline(SmallVec::new()),
        }
    }

    /// The number of key-values stored in the map.
    pub fn len(&self) -> usize {
        match &self.data {
            MapData::Inline(sv) => sv.len(),
            MapData::Heap(map) => map.len(),
        }
    }

    /// Returns `true` if the map is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// The memory capacity that will be allocated inline. If the nubmer of
    /// values exceeds the inline capacity, the map will move to the heap.
    pub fn inline_capacity(&self) -> usize {
        C
    }

    /// Is the data contained by this map stored inline (`true`) or on the heap
    /// (`false`).
    pub fn is_inline(&self) -> bool {
        matches!(self.data, MapData::Inline(_))
    }

    /// Returns an iterator over the key-values in insertion order.
    pub fn iter(&'_ self) -> Iter<'_, K, V> {
        match &self.data {
            MapData::Inline(vec) => Iter::Inline(vec.iter()),
            MapData::Heap(map) => Iter::Heap(map.iter()),
        }
    }

    // Helper method for macro, don't use directly.
    #[doc(hidden)]
    pub const fn from_const_unchecked(inline: SmallVec<[(K, V); C]>) -> Self {
        Self {
            data: MapData::Inline(inline),
        }
    }
}

impl<K: Hash + Eq, V, const C: usize> SmallMap<K, V, C> {
    /// Return a reference to the value stored for `key`, if it is present,
    /// else `None`.
    ///
    /// Computational complexity:
    ///  - inline: O(n)
    ///  - heap: O(1)
    pub fn get<Q: ?Sized>(&self, key: &Q) -> Option<&V>
    where
        Q: Hash + Equivalent<K>,
    {
        match &self.data {
            MapData::Inline(vec) => vec
                .iter()
                .find(|(k, _v)| key.equivalent(k))
                .map(|(_k, v)| v),
            MapData::Heap(map) => map.get(key),
        }
    }

    /// Return a mutable reference to the value stored for `key`, if it is
    /// present, else `None`.
    ///
    /// Computational complexity:
    ///  - inline: O(n)
    ///  - heap: O(1)
    pub fn get_mut<Q: ?Sized>(&mut self, key: &Q) -> Option<&mut V>
    where
        Q: Hash + Equivalent<K>,
    {
        match &mut self.data {
            MapData::Inline(vec) => vec
                .iter_mut()
                .find(|(k, _v)| key.equivalent(k))
                .map(|(_k, v)| v),
            MapData::Heap(map) => map.get_mut(key),
        }
    }

    /// Get a key-value pair by index, if it is present, else `None`.
    ///
    /// Computational complexity: O(1)
    pub fn get_index(&self, index: usize) -> Option<(&K, &V)> {
        match &self.data {
            MapData::Inline(vec) => {
                if index < self.len() {
                    Some(&vec[index]).map(|(k, v)| (k, v))
                } else {
                    None
                }
            }
            MapData::Heap(map) => map.get_index(index),
        }
    }

    /// Get a mutable key-value pair by index, if it is present, else `None`.
    ///
    /// Computational complexity: O(1)
    pub fn get_index_mut(&mut self, index: usize) -> Option<(&K, &mut V)> {
        // This is a slight deviation from the current IndexMap API which also
        // returns a mutable key. As is stated here ([1]) however, that was a
        // mistake and will be corrected in a future release.
        // [1] https://github.com/bluss/indexmap/issues/174.
        match &mut self.data {
            MapData::Inline(vec) => {
                if index < vec.len() {
                    Some(&mut vec[index]).map(|(k, v)| (&*k, v))
                } else {
                    None
                }
            }
            MapData::Heap(map) => map.get_index_mut(index).map(|(k, v)| (&*k, v)),
        }
    }

    /// Return the item index, if it exists in the map, else `None`.
    ///
    /// Computational complexity:
    ///  - inline: O(n)
    ///  - heap: O(1)
    pub fn get_index_of<Q: ?Sized>(&self, key: &Q) -> Option<usize>
    where
        Q: Hash + Equivalent<K>,
    {
        match &self.data {
            MapData::Inline(vec) => vec.iter().position(|(k, _v)| key.equivalent(k)),
            MapData::Heap(map) => map.get_index_of(key),
        }
    }

    /// Get the given key's corresponding entry in the map for insertion and/or
    /// in-place manipulation.
    ///
    /// Computational complexity:
    ///  - inline: O(n)
    ///  - heap: O(1)
    pub fn entry(&mut self, key: K) -> Entry<'_, K, V, C> {
        let index = self.get_index_of(&key);
        match index {
            Some(index) => Entry::Occupied(self, index),
            None => Entry::Vacant(self, key),
        }
    }

    /// Convert the specified map and turn it into a `SmallMap`.
    ///
    /// If the map len is smaller or equal the inline capacity, the data will be
    /// moved inline.
    pub fn from_map(map: FastIndexMap<K, V>) -> Self {
        if map.len() <= C {
            Self {
                data: MapData::Inline(SmallVec::from_vec(map.into_iter().collect())),
            }
        } else {
            Self {
                data: MapData::Heap(map),
            }
        }
    }

    /// Inserts the specified key-value pair into this map.
    ///
    /// If a value for the specified `key` already exists, the new value will
    /// overwrite the existing value. The iteration order of the key-value pair
    /// will remain in the original position.
    ///
    /// If a new key is added that causes the size of the `SmallMap` to exceed
    /// the inline capacity, all existing data and the new key-value pair is
    /// moved to the heap.
    ///
    /// Computational complexity:
    ///  - inline: O(n)
    ///  - heap: O(1)
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        match &mut self.data {
            MapData::Inline(sv) => {
                let existing_index = sv.iter().position(|(k, _v)| &key == k);
                if let Some(existing_index) = existing_index {
                    let ret = mem::replace(&mut sv[existing_index], (key, value));
                    Some(ret.1)
                } else if sv.len() + 1 > C {
                    // Move to heap
                    let mut map = sv.drain(0..sv.len()).collect::<FastIndexMap<_, _>>();
                    let ret = map.insert(key, value);
                    self.data = MapData::Heap(map);
                    ret
                } else {
                    sv.push((key, value));
                    None
                }
            }
            MapData::Heap(map) => map.insert(key, value),
        }
    }
}

impl<K, V, const C: usize> Eq for SmallMap<K, V, C>
where
    K: Hash + Eq,
    V: Eq,
{
}
impl<K, V, const C: usize> PartialEq for SmallMap<K, V, C>
where
    K: Hash + Eq,
    V: Eq,
{
    fn eq(&self, other: &Self) -> bool {
        self.iter().eq(other.iter())
    }
}

impl<K, V, const C: usize> Default for MapData<K, V, C> {
    fn default() -> Self {
        MapData::Inline(SmallVec::new())
    }
}

impl<K, V, const C: usize> Index<usize> for SmallMap<K, V, C>
where
    K: Eq + Hash,
    V: Eq,
{
    type Output = V;

    fn index(&self, index: usize) -> &Self::Output {
        self.get_index(index)
            .expect("SmallMap: index out of bounds")
            .1
    }
}

impl<K, V, const C: usize> IndexMut<usize> for SmallMap<K, V, C>
where
    K: Eq + Hash,
    V: Eq,
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.get_index_mut(index)
            .expect("SmallMap: index out of bounds")
            .1
    }
}

pub enum Iter<'a, K, V> {
    Inline(std::slice::Iter<'a, (K, V)>),
    Heap(indexmap::map::Iter<'a, K, V>),
}

impl<'a, K, V> Iterator for Iter<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Iter::Inline(iter) => iter.next().map(|(k, v)| (k, v)),
            Iter::Heap(iter) => iter.next(),
        }
    }
}

impl<'a, K, V> ExactSizeIterator for Iter<'a, K, V> {
    fn len(&self) -> usize {
        match self {
            Iter::Inline(iter) => iter.len(),
            Iter::Heap(iter) => iter.len(),
        }
    }
}

impl<K, V, const C: usize> IntoIterator for SmallMap<K, V, C> {
    type Item = (K, V);

    type IntoIter = IntoIter<K, V, C>;

    fn into_iter(self) -> Self::IntoIter {
        match self.data {
            MapData::Inline(vec) => IntoIter::Inline(vec.into_iter()),
            MapData::Heap(map) => IntoIter::Heap(map.into_iter()),
        }
    }
}

pub enum IntoIter<K, V, const C: usize> {
    Inline(smallvec::IntoIter<[(K, V); C]>),
    Heap(indexmap::map::IntoIter<K, V>),
}

impl<K, V, const C: usize> Iterator for IntoIter<K, V, C> {
    type Item = (K, V);

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            IntoIter::Inline(iter) => iter.next().map(|(k, v)| (k, v)),
            IntoIter::Heap(iter) => iter.next(),
        }
    }
}

impl<K, V, const C: usize> ExactSizeIterator for IntoIter<K, V, C> {
    fn len(&self) -> usize {
        match self {
            IntoIter::Inline(iter) => iter.len(),
            IntoIter::Heap(iter) => iter.len(),
        }
    }
}

impl<K, V, const C: usize> FromIterator<(K, V)> for SmallMap<K, V, C>
where
    K: Hash + Eq,
{
    fn from_iter<I: IntoIterator<Item = (K, V)>>(iterable: I) -> Self {
        let iter = iterable.into_iter();
        let (lower_bound, _) = iter.size_hint();
        if lower_bound <= C {
            let vec = SmallVec::<[(K, V); C]>::from_iter(iter);
            // If the lower bound of the size hint is off, such that the actual vector
            // length exceeds the inline capacity, then the data will be moved to an
            // IndexMap _after_ it was moved into the SmallVec.
            if vec.len() > C {
                Self {
                    data: MapData::Heap(FastIndexMap::from_iter(vec)),
                }
            } else {
                Self {
                    data: MapData::Inline(vec),
                }
            }
        } else {
            Self {
                data: MapData::Heap(FastIndexMap::from_iter(iter)),
            }
        }
    }
}

pub enum Entry<'a, K, V, const C: usize> {
    Occupied(&'a mut SmallMap<K, V, C>, usize),
    Vacant(&'a mut SmallMap<K, V, C>, K),
}

impl<'a, K: Hash + Eq, V, const C: usize> Entry<'a, K, V, C> {
    /// Modifies the entry if it is occupied. Otherwise this is a no-op.
    pub fn and_modify<F>(self, f: F) -> Self
    where
        F: FnOnce(&mut V),
    {
        match self {
            Entry::Occupied(map, index) => {
                f(map.get_index_mut(index).map(|(_k, v)| v).unwrap());
                Entry::Occupied(map, index)
            }
            x => x,
        }
    }

    /// Inserts the given default value in the entry if it is vacant. Otherwise
    /// this is a no-op.
    pub fn or_insert(self, default: V) {
        if let Entry::Vacant(map, key) = self {
            map.insert(key, default);
        };
    }
}

impl<K, V, const C: usize> Debug for SmallMap<K, V, C>
where
    K: Debug,
    V: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_map().entries(self.iter()).finish()
    }
}

#[macro_export]
macro_rules! smallmap {
    // count helper: transform any expression into 1
    (@one $x:expr) => (1usize);
    ($($key:expr => $value:expr),*$(,)*) => ({
        let count = 0usize $(+ $crate::smallmap!(@one $key))*;
        #[allow(unused_mut)]
        let mut map = $crate::SmallMap::new();
        if count <= map.inline_capacity() {
            $(map.insert($key, $value);)*
            map
        } else {
            $crate::SmallMap::from_map($crate::fastindexmap![$($key => $value,)*])
        }
    });
}

/// Creates [`SmallMap`] with inline capacity equal to the number of values.
#[macro_export]
macro_rules! smallmap_inline {
    ($($key:expr => $value:expr),*$(,)*) => ({
        let vec = $crate::smallvec_inline!( $(($key, $value),)*);
        debug_assert_eq!(
            vec.len(),
            vec
                .iter()
                .map(|(k, _v)| k)
                .collect::<$crate::FastHashSet<_>>()
                .len(),
            "smallmap_inline! cannot be initialized with duplicate keys"
        );
        $crate::SmallMap::from_const_unchecked(vec)
    });
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::fastindexmap;

    #[test]
    fn test_len_and_inline_capacity() {
        let mut map: SmallMap<usize, usize, 1> = SmallMap::new();
        assert_eq!(0, map.len());
        map.insert(0, 1);
        assert_eq!(1, map.len());

        let map: SmallMap<_, _, 10> = smallmap! {
            0 => 1,
            1 => 7,
            4 => 9
        };
        assert_eq!(3, map.len());
        assert_eq!(10, map.inline_capacity());

        let map = smallmap_inline! {
            0 => 1,
            1 => 7,
            4 => 9
        };
        assert_eq!(3, map.len());
        assert_eq!(3, map.inline_capacity());
    }

    #[test]
    fn smallmap_macro_removes_duplicates() {
        let map: SmallMap<_, _, 10> = smallmap! { 0 => 1, 0 => 2};
        assert_eq!(1, map.len());
    }

    #[test]
    #[should_panic(expected = "smallmap_inline! cannot be initialized with duplicate keys")]
    fn smallmap_inline_macro_fails_on_duplicates() {
        smallmap_inline! { 0 => 1, 0 => 2};
    }

    #[test]
    fn iter_iterates_in_insertion_order() {
        fn test<const C: usize>(inline: bool) {
            let inline_map: SmallMap<_, _, C> = smallmap! {
                1 => 7,
                0 => 1,
                4 => 9
            };
            assert_eq!(inline, inline_map.is_inline());
            assert_eq!(
                vec![(&1, &7), (&0, &1), (&4, &9)],
                inline_map.iter().collect::<Vec<_>>(),
                "iter() does not return values in the correct order"
            );
            assert_eq!(
                vec![(1, 7), (0, 1), (4, 9)],
                inline_map.into_iter().collect::<Vec<_>>(),
                "into_iter() does not return values in the correct order"
            );
        }
        test::<1>(false);
        test::<3>(true);
    }

    #[test]
    fn from_map_stores_data_inline_or_on_heap_depending_on_c_and_input_len() {
        let input = fastindexmap! { 0 => "zero", 3 => "three",  900 => "nine-hundred"};

        let heap_map = SmallMap::<_, _, 2>::from_map(input.clone());
        assert!(!heap_map.is_inline());

        let inline_map = SmallMap::<_, _, 3>::from_map(input);
        assert!(inline_map.is_inline());

        assert_eq!(
            vec![(0, "zero"), (3, "three"), (900, "nine-hundred")],
            heap_map.into_iter().collect::<Vec<_>>()
        );
        assert_eq!(
            vec![(0, "zero"), (3, "three"), (900, "nine-hundred")],
            inline_map.into_iter().collect::<Vec<_>>()
        );
    }

    #[test]
    fn insert_tests() {
        // Test cases:
        // | Key/Value           | Memory       | Insertion position |
        // | ------------------- | ------------ | ------------------ |
        // | new                 | Stay inline  | Last               |
        // | new                 | Move to heap | Last               |
        // | new                 | Stay on heap | Last               |
        // | overwrites existing | Stay inline  | Same as existing   |
        // | overwrites existing | Stay on heap | Same as existing   |

        let values = vec![
            (10, "ten"),
            (5, "five"),
            (86, "eighty-six"),
            (93, "ninety-three"),
        ];
        struct TestCase {
            name: &'static str,
            initial_values: Vec<(usize, &'static str)>,
            insert_key_value: (usize, &'static str),
            expected_inline_before: bool,
            expected_inline_after: bool,
            expected_values: Vec<(usize, &'static str)>,
            expected_return: Option<&'static str>,
        }
        let test_cases = vec![
            TestCase {
                name: "new key/value, stay inline",
                initial_values: values[0..2].to_vec(),
                insert_key_value: (7, "seven"),
                expected_inline_before: true,
                expected_inline_after: true,
                expected_values: vec![(10, "ten"), (5, "five"), (7, "seven")],
                expected_return: None,
            },
            TestCase {
                name: "new key/value, move to heap",
                initial_values: values[0..3].to_vec(),
                insert_key_value: (7, "seven"),
                expected_inline_before: true,
                expected_inline_after: false,
                expected_values: vec![(10, "ten"), (5, "five"), (86, "eighty-six"), (7, "seven")],
                expected_return: None,
            },
            TestCase {
                name: "new key/value, stay on heap",
                initial_values: values[0..4].to_vec(),
                insert_key_value: (7, "seven"),
                expected_inline_before: false,
                expected_inline_after: false,
                expected_values: vec![
                    (10, "ten"),
                    (5, "five"),
                    (86, "eighty-six"),
                    (93, "ninety-three"),
                    (7, "seven"),
                ],
                expected_return: None,
            },
            TestCase {
                name: "overwrite existing key/value, stay inline",
                initial_values: values[0..3].to_vec(),
                insert_key_value: (5, "fivefivefive"),
                expected_inline_before: true,
                expected_inline_after: true,
                expected_values: vec![(10, "ten"), (5, "fivefivefive"), (86, "eighty-six")],
                expected_return: Some("five"),
            },
            TestCase {
                name: "overwrite existing key/value, stay on heap",
                initial_values: values[0..4].to_vec(),
                insert_key_value: (10, "tententen"),
                expected_inline_before: false,
                expected_inline_after: false,
                expected_values: vec![
                    (10, "tententen"),
                    (5, "five"),
                    (86, "eighty-six"),
                    (93, "ninety-three"),
                ],
                expected_return: Some("ten"),
            },
        ];

        for test_case in test_cases {
            let mut small_map = SmallMap::<usize, &str, 3>::new();

            for (k, v) in test_case.initial_values {
                small_map.insert(k, v);
            }
            assert_eq!(
                test_case.expected_inline_before,
                small_map.is_inline(),
                "inline state before insertion in SmallMap does not match expected in test '{}'",
                test_case.name
            );

            let actual_return =
                small_map.insert(test_case.insert_key_value.0, test_case.insert_key_value.1);

            assert_eq!(
                test_case.expected_inline_after,
                small_map.is_inline(),
                "inline state after insertion in SmallMap does not match expected in test '{}'",
                test_case.name
            );
            assert_eq!(
                test_case.expected_return, actual_return,
                "return of insertion in SmallMap does not match expected return in test '{}'",
                test_case.name
            );
            assert_eq!(
                test_case.expected_values,
                small_map.into_iter().collect::<Vec<_>>(),
                "values in SmallMap do not match expected values in test '{}'",
                test_case.name
            );
        }
    }

    #[test]
    fn equality_is_consistent() {
        let map1: SmallMap<_, _, 3> = smallmap! {
            0 => 1,
            1 => 7,
            4 => 9
        };
        let map2 = smallmap_inline! {
            0 => 1,
            1 => 7,
            4 => 9
        };
        let map3 = SmallMap::<_, _, 3>::from_iter(vec![(0, 1), (1, 7), (4, 9)]);
        let mut map4 = SmallMap::<_, _, 3>::new();
        map4.insert(0, 1);
        map4.insert(1, 7);
        map4.insert(4, 9);

        assert_eq!(map1, map2);
        assert_eq!(map1, map3);
        assert_eq!(map1, map4);

        assert_eq!(map2, map3);
        assert_eq!(map2, map4);

        assert_eq!(map3, map4);
    }

    #[test]
    fn empty_small_maps_are_equal() {
        let map1: SmallMap<usize, usize, 3> = smallmap! {};
        let map2: SmallMap<usize, usize, 3> = smallmap! {};
        assert_eq!(map1, map2);
    }

    // Type for testing equivalence to String
    struct MyType(usize);

    // Hash needs to be equivalent to String::hash
    impl Hash for MyType {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
            self.0.to_string().hash(state);
        }
    }

    impl Equivalent<&'static str> for MyType {
        fn equivalent(&self, key: &&'static str) -> bool {
            &self.0.to_string() == key
        }
    }

    #[test]
    fn get_works_with_equal_and_equivalent_keys() {
        fn test<const C: usize>(inline: bool) {
            let map: SmallMap<&'static str, usize, C> =
                smallmap! {"2" => 222, "1" => 111, "3" => 333};
            assert_eq!(inline, map.is_inline());

            assert_eq!(Some(&111), map.get(&MyType(1)));
            assert_eq!(Some(&111), map.get(&"1"));
            assert_eq!(Some(&333), map.get(&MyType(3)));
            assert_eq!(None, map.get(&MyType(7)));
            assert_eq!(None, map.get(&"7"));
        }
        test::<1>(false);
        test::<3>(true);
    }

    #[test]
    fn get_mut_works_with_equal_and_equivalent_keys() {
        fn test<const C: usize>(inline: bool) {
            let mut map: SmallMap<&'static str, usize, C> =
                smallmap! {"2" => 222, "1" => 111, "3" => 333};
            assert_eq!(inline, map.is_inline());

            // present
            assert_eq!(Some(&mut 111), map.get_mut(&MyType(1)));
            assert_eq!(Some(&mut 111), map.get_mut(&"1"));

            // not present
            assert_eq!(None, map.get_mut(&MyType(7)));
            assert_eq!(None, map.get_mut(&"7"));

            // change using equivalent key
            let m = map.get_mut(&MyType(1)).unwrap();
            *m = 1;
            assert_eq!(&1, map.get(&"1").unwrap());
            assert_eq!(&1, map.get(&MyType(1)).unwrap());

            // change using equal key
            let m = map.get_mut(&"1").unwrap();
            *m = 11;
            assert_eq!(&11, map.get(&"1").unwrap());
            assert_eq!(&11, map.get(&MyType(1)).unwrap());
        }
        test::<1>(false);
        test::<3>(true);
    }

    #[test]
    fn get_index_test() {
        fn test<const C: usize>(inline: bool) {
            let map: SmallMap<&'static str, usize, C> =
                smallmap! {"2" => 222, "1" => 111, "3" => 333};
            assert_eq!(inline, map.is_inline());

            assert_eq!(Some((&"2", &222)), map.get_index(0));
            assert_eq!(Some((&"1", &111)), map.get_index(1));
            assert_eq!(Some((&"3", &333)), map.get_index(2));
            assert_eq!(None, map.get_index(3));
        }
        test::<1>(false);
        test::<3>(true);
    }

    #[test]
    fn get_index_trait_test() {
        fn test<const C: usize>(inline: bool) {
            let map: SmallMap<&'static str, usize, C> =
                smallmap! {"2" => 222, "1" => 111, "3" => 333};
            assert_eq!(inline, map.is_inline());

            assert_eq!(222, map[0]);
            assert_eq!(111, map[1]);
            assert_eq!(333, map[2]);
        }
        test::<1>(false);
        test::<3>(true);
    }

    #[test]
    #[should_panic(expected = "SmallMap: index out of bounds")]
    fn get_index_trait_panics_on_out_of_bounds_inline() {
        let map: SmallMap<&'static str, usize, 3> = smallmap! {"2" => 222, "1" => 111, "3" => 333};
        assert!(map.is_inline());
        let _ = map[5];
    }

    #[test]
    #[should_panic(expected = "SmallMap: index out of bounds")]
    fn get_index_trait_panics_on_out_of_bounds_heap() {
        let map: SmallMap<&'static str, usize, 1> = smallmap! {"2" => 222, "1" => 111, "3" => 333};
        assert!(!map.is_inline());
        let _ = map[5];
    }

    #[test]
    fn get_index_mut_test() {
        fn test<const C: usize>(inline: bool) {
            let mut map: SmallMap<&'static str, usize, C> =
                smallmap! {"2" => 222, "1" => 111, "3" => 333};
            assert_eq!(inline, map.is_inline());

            assert_eq!(Some((&"2", &mut 222)), map.get_index_mut(0));
            assert_eq!(Some((&"1", &mut 111)), map.get_index_mut(1));
            assert_eq!(Some((&"3", &mut 333)), map.get_index_mut(2));
            assert_eq!(None, map.get_index_mut(3));

            let (_k, v) = map.get_index_mut(1).unwrap();
            *v = 2;
            assert_eq!(Some((&"1", &mut 2)), map.get_index_mut(1));
        }
        test::<1>(false);
        test::<3>(true);
    }

    #[test]
    fn get_index_mut_trait_test() {
        fn test<const C: usize>(inline: bool) {
            let mut map: SmallMap<&'static str, usize, C> =
                smallmap! {"2" => 222, "1" => 111, "3" => 333};
            assert_eq!(inline, map.is_inline());

            assert_eq!(&mut 222, &mut map[0]);
            assert_eq!(&mut 111, &mut map[1]);
            assert_eq!(&mut 333, &mut map[2]);

            map[1] = 2;
            assert_eq!(&mut 2, &mut map[1]);
        }
        test::<1>(false);
        test::<3>(true);
    }

    #[test]
    #[should_panic(expected = "SmallMap: index out of bounds")]
    fn get_index_mut_trait_panics_on_out_of_bounds_inline() {
        let mut map: SmallMap<&'static str, usize, 3> =
            smallmap! {"2" => 222, "1" => 111, "3" => 333};
        assert!(map.is_inline());
        let _ = &mut map[4];
    }

    #[test]
    #[should_panic(expected = "SmallMap: index out of bounds")]
    fn get_index_mut_trait_panics_on_out_of_bounds_heap() {
        let mut map: SmallMap<&'static str, usize, 1> =
            smallmap! {"2" => 222, "1" => 111, "3" => 333};
        assert!(!map.is_inline());
        let _ = &mut map[4];
    }

    #[test]
    fn get_index_of_test() {
        fn test<const C: usize>(inline: bool) {
            let map: SmallMap<&'static str, usize, C> =
                smallmap! {"2" => 222, "1" => 111, "3" => 333};
            assert_eq!(inline, map.is_inline());

            assert_eq!(None, map.get_index_of(&"0"));
            assert_eq!(None, map.get_index_of(&MyType(0)));

            assert_eq!(Some(1), map.get_index_of(&"1"));
            assert_eq!(Some(1), map.get_index_of(&MyType(1)));
            assert_eq!(Some(0), map.get_index_of(&"2"));
            assert_eq!(Some(0), map.get_index_of(&MyType(2)));
            assert_eq!(Some(2), map.get_index_of(&"3"));
            assert_eq!(Some(2), map.get_index_of(&MyType(3)));
        }
        test::<1>(false);
        test::<3>(true);
    }

    #[test]
    fn entry_and_modify_test() {
        fn test<const C: usize>(inline: bool) {
            let mut map: SmallMap<&'static str, usize, C> =
                smallmap! {"2" => 222, "1" => 111, "3" => 333};
            assert_eq!(inline, map.is_inline());

            // not existing -> no-op
            map.entry("0").and_modify(|x| *x = 100);
            assert_eq!(None, map.get(&"0"));

            // existing -> multiply 111 x 2 = 222
            map.entry("1").and_modify(|x| *x *= 2);
            assert_eq!(Some(&222), map.get(&"1"));
        }
        test::<1>(false);
        test::<3>(true);
    }

    #[test]
    fn entry_or_insert_test() {
        fn test<const C: usize>(inline: bool) {
            let mut map: SmallMap<&'static str, usize, C> =
                smallmap! {"2" => 222, "1" => 111, "3" => 333};
            assert_eq!(inline, map.is_inline());

            // not existing -> insert new
            map.entry("0").or_insert(777);
            assert_eq!(Some(&777), map.get(&"0"));

            // existing -> no-op
            map.entry("1").or_insert(777);
            assert_eq!(Some(&111), map.get(&"1"));
        }
        test::<1>(false);
        test::<3>(true);
    }

    #[test]
    fn exact_size_iterator_test() {
        fn test<const C: usize>(inline: bool) {
            let mut map = SmallMap::<&'static str, usize, C>::new();
            assert_eq!(0, map.iter().len());
            map.insert("a", 0);
            assert!(map.is_inline()); // a map of len <= 1 is always stored inline
            assert_eq!(1, map.iter().len());
            map.insert("b", 0);
            assert_eq!(inline, map.is_inline());
            assert_eq!(2, map.iter().len());
            map.insert("c", 0);
            assert_eq!(3, map.iter().len());
            assert_eq!(inline, map.is_inline());
        }
        test::<1>(false);
        test::<3>(true);
    }

    #[test]
    fn exact_size_into_iterator_test() {
        fn test<const C: usize>(inline: bool) {
            let mut map = SmallMap::<&'static str, usize, C>::new();
            assert_eq!(0, map.clone().into_iter().len());
            map.insert("a", 0);
            assert!(map.is_inline()); // a map of len <= 1 is always stored inline
            assert_eq!(1, map.clone().into_iter().len());
            map.insert("b", 0);
            assert_eq!(inline, map.is_inline());
            assert_eq!(2, map.clone().into_iter().len());
            map.insert("c", 0);
            assert_eq!(3, map.clone().into_iter().len());
            assert_eq!(inline, map.is_inline());
        }
        test::<1>(false);
        test::<3>(true);
    }

    #[test]
    fn from_iterator_test() {
        fn test<const C: usize>(inline: bool) {
            let data = vec![("hi", 2), ("hello", 5), ("hamburg", 7), ("berlin", 6)];
            let map = SmallMap::<&'static str, usize, C>::from_iter(data.clone());
            assert_eq!(inline, map.is_inline());

            let output = map.into_iter().collect::<Vec<_>>();
            assert_eq!(data, output);
        }
        test::<1>(false);
        test::<4>(true);
    }

    #[test]
    fn from_iterator_wrong_size_hint_test() {
        struct FaultyIter<T> {
            data: Vec<T>,
            index: usize,
            len: usize,
        }
        impl<T: Clone> Iterator for FaultyIter<T> {
            type Item = T;

            fn next(&mut self) -> Option<Self::Item> {
                let ret = self.data.get(self.index).cloned();
                self.index += 1;
                ret
            }
        }
        impl<T: Clone> ExactSizeIterator for FaultyIter<T> {
            fn len(&self) -> usize {
                self.len
            }
        }

        let data = vec![("hi", 2), ("hello", 5), ("hamburg", 7), ("berlin", 6)];
        let iter = FaultyIter::<(&'static str, usize)> {
            data: data.clone(),
            index: 0,
            len: 1,
        };
        // Even though the iterator says that it's len is 1, which would fit inline.
        // The actual len is 4 which does not fit inline. This test checks whether the
        // data is correctly allocated on the heap.
        let map = SmallMap::<_, _, 3>::from_iter(iter);
        assert!(!map.is_inline());

        let output = map.into_iter().collect::<Vec<_>>();
        assert_eq!(data, output);
    }

    #[test]
    fn debug_string_test() {
        let actual = format!("{:?}", smallmap_inline! {0=>6, 1=>5, 2=>4});
        let expected = "{0: 6, 1: 5, 2: 4}";
        assert_eq!(expected, actual);
    }

    #[test]
    #[should_panic(
        expected = "Cannot instantiate SmallMap with no inline capacity, use positive capacity or use IndexMap instead"
    )]
    fn new_fails_on_zero_capacity() {
        SmallMap::<usize, usize, 0>::new();
    }
}
