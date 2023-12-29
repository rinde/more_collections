use crate::collections::hash_map::RandomState;
use core::fmt;
use core::fmt::Debug;
use core::fmt::Formatter;
use core::hash::BuildHasher;
use core::iter::Chain;
use core::iter::FusedIterator;

use ::core::hash::Hash;
use indexmap::Equivalent;
use smallvec::SmallVec;

use crate::small_map;
use crate::SmallMap;

/// A set-like container that can store a specified number of elements inline.
///
/// `SmallSet` shares most of its API with, and behaves like,
/// [`IndexSet`](indexmap::IndexSet). It can store a limited amount of data
/// inline, backed by [`SmallVec`]. If the data exceeds the
/// limit `C`, `SmallSet` will move _all_ its data over to the heap in the form
/// of an `IndexSet`. For performance reasons, transitions between heap and
/// inline storage should generally be avoided.
///
/// The `SmallSet` datastructure is meant for situations where the data does not
/// exceed `C` _most of the time_ but it still needs to support cases where the
/// data _does_ exceed `C`.
///
/// # Example
///
/// ```
/// use more_collections::SmallSet;
///
/// let mut set = SmallSet::<usize, 3>::new();
/// // The set can hold up to three items inline
/// set.insert(0);
/// set.insert(1);
/// set.insert(2);
/// assert_eq!(3, set.len());
/// assert!(set.is_inline());
///
/// // Adding the fourth element will move the set to the heap
/// set.insert(3);
/// assert_eq!(4, set.len());
/// assert!(!set.is_inline());
/// ```
#[derive(Default, Clone)]
pub struct SmallSet<T, const C: usize, S = RandomState> {
    data: SmallMap<T, (), C, S>,
}

impl<T, const C: usize> SmallSet<T, C> {
    /// Create a new set.
    pub fn new() -> Self {
        Self {
            data: SmallMap::new(),
        }
    }

    // Helper method for macro, don't use directly.
    #[doc(hidden)]
    pub const fn from_const_unchecked(inline: SmallVec<[(T, ()); C]>) -> Self {
        Self {
            data: SmallMap::from_const_unchecked(inline),
        }
    }
}

impl<T, const C: usize, S> SmallSet<T, C, S> {
    /// The number of values stored in the set.
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Returns `true` if the set is empty.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// The memory capacity that will be allocated inline. If the nubmer of
    /// values exceeds the inline capacity, the set will move to the heap.
    pub fn inline_capacity(&self) -> usize {
        self.data.inline_capacity()
    }

    /// Is the data contained by this set stored inline (`true`) or on the heap
    /// (`false`).
    pub fn is_inline(&self) -> bool {
        self.data.is_inline()
    }

    /// Returns an iterator over the values in insertion order.
    pub fn iter(&'_ self) -> Iter<'_, T> {
        Iter {
            inner: self.data.iter(),
        }
    }

    // Helper method for macro, don't use directly.
    #[doc(hidden)]
    pub const fn from_const_unchecked_with_hasher(inline: SmallVec<[(T, ()); C]>) -> Self {
        Self {
            data: SmallMap::from_const_unchecked_with_hasher(inline),
        }
    }
}

impl<T, const C: usize, S> SmallSet<T, C, S>
where
    T: Hash + Eq,
    S: BuildHasher + Default,
{
    /// Inserts the specified value into this set.
    ///
    /// If an equivalent item already exists in the set, it returns `false`
    /// leaving the original value in the set and without altering its insertion
    /// order. Otherwise, it inserts the new item and returns `true`.
    ///
    /// If a new value is added that causes the size of the `SmallSet` to exceed
    /// the inline capacity, all existing data and the new value is moved to the
    /// heap.
    ///
    /// Computational complexity:
    ///  - inline: O(n)
    ///  - heap: O(1)
    pub fn insert(&mut self, value: T) -> bool {
        self.data.insert(value, ()).is_some()
    }

    /// Inserts the specified value into this set, and get their index.
    ///
    /// If an equivalent item already exists in the set, it returns the index of
    /// the existing item and `false`, leaving the original value in the set and
    /// without altering its insertion order. Otherwise, it inserts the new
    /// item and returns the index of the inserted item and `true`.
    ///
    /// If a new value is added that causes the size of the `SmallSet` to exceed
    /// the inline capacity, all existing data and the new value is moved to the
    /// heap.
    ///
    /// Computational complexity:
    ///  - inline: O(n)
    ///  - heap: O(1)
    pub fn insert_full(&mut self, value: T) -> (usize, bool) {
        let (index, value) = self.data.insert_full(value, ());
        (index, value.is_some())
    }
}

impl<T, const C: usize, S> SmallSet<T, C, S>
where
    T: Hash + Eq,
    S: BuildHasher,
{
    pub fn from_keys(map: SmallMap<T, (), C, S>) -> Self {
        SmallSet { data: map }
    }

    /// Get a value by index, if it is present, else `None`.
    ///
    /// Computational complexity: O(1)
    pub fn get_index(&self, index: usize) -> Option<&T> {
        self.data.get_index(index).map(|(k, _v)| k)
    }

    /// Return the item index, if it exists in the set, else `None`.
    ///
    /// Computational complexity:
    ///  - inline: O(n)
    ///  - heap: O(1)
    pub fn get_index_of<Q: ?Sized>(&self, key: &Q) -> Option<usize>
    where
        Q: Hash + Equivalent<T>,
    {
        self.data.get_index_of(key)
    }

    /// Remove the key-value pair equivalent to `key` and return
    /// its value.
    ///
    /// **NOTE:** This is equivalent to `.swap_remove(key)`, if you need to
    /// preserve the order of the keys in the map, use `.shift_remove(key)`
    /// instead.
    ///
    /// Computes in **O(1)** time (average).
    pub fn remove<Q: ?Sized>(&mut self, key: &Q) -> bool
    where
        Q: Hash + Equivalent<T>,
    {
        self.data.remove(key).is_some()
    }

    /// Return an iterator over the values that are in `self` but not `other`.
    ///
    /// Values are produced in the same order that they appear in `self`.
    pub fn difference<'a, const C2: usize, S2>(
        &'a self,
        other: &'a SmallSet<T, C2, S2>,
    ) -> Difference<'a, T, C2, S2>
    where
        S2: BuildHasher,
    {
        Difference {
            iter: self.iter(),
            other,
        }
    }

    /// Return an iterator over the values that are in `self` or `other`,
    /// but not in both.
    ///
    /// Values from `self` are produced in their original order, followed by
    /// values from `other` in their original order.
    pub fn symmetric_difference<'a, const C2: usize, S2>(
        &'a self,
        other: &'a SmallSet<T, C2, S2>,
    ) -> SymmetricDifference<'a, T, C, S, C2, S2>
    where
        S2: BuildHasher,
    {
        let diff1 = self.difference(other);
        let diff2 = other.difference(self);
        SymmetricDifference {
            iter: diff1.chain(diff2),
        }
    }

    /// Return an iterator over the values that are in both `self` and `other`.
    ///
    /// Values are produced in the same order that they appear in `self`.
    pub fn intersection<'a, const C2: usize, S2>(
        &'a self,
        other: &'a SmallSet<T, C2, S2>,
    ) -> Intersection<'a, T, C2, S2>
    where
        S2: BuildHasher,
    {
        Intersection {
            iter: self.iter(),
            other,
        }
    }

    /// Return an iterator over all values that are in `self` or `other`.
    ///
    /// Values from `self` are produced in their original order, followed by
    /// values that are unique to `other` in their original order.
    pub fn union<'a, const C2: usize, S2>(
        &'a self,
        other: &'a SmallSet<T, C2, S2>,
    ) -> Union<'a, T, C, S>
    where
        S2: BuildHasher,
    {
        Union {
            iter: self.iter().chain(other.difference(self)),
        }
    }

    /// Return `true` if an equivalent to `value` exists in the set.
    ///
    /// Computational complexity:
    ///  - inline: O(n)
    ///  - heap: O(1)
    pub fn contains<Q: ?Sized>(&self, value: &Q) -> bool
    where
        Q: Hash + Equivalent<T>,
    {
        self.data.contains_key(value)
    }
}

impl<T, const C: usize, S> Hash for SmallSet<T, C, S>
where
    T: Hash + Eq,
{
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.data.hash(state);
    }
}
impl<T, const C: usize, S> Eq for SmallSet<T, C, S> where T: Hash + Eq {}
impl<T, const C: usize, S> PartialEq for SmallSet<T, C, S>
where
    T: Hash + Eq,
{
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

#[derive(Clone)]
pub struct Iter<'a, T> {
    inner: small_map::Iter<'a, T, ()>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|(t, _)| t)
    }
}

impl<'a, T> ExactSizeIterator for Iter<'a, T> {
    fn len(&self) -> usize {
        self.inner.len()
    }
}

impl<'a, T> DoubleEndedIterator for Iter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.inner.next_back().map(|(t, _)| t)
    }
}

impl<T, const C: usize, S> IntoIterator for SmallSet<T, C, S> {
    type Item = T;

    type IntoIter = IntoIter<T, C>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            inner: self.data.into_iter(),
        }
    }
}

pub struct IntoIter<T, const C: usize> {
    inner: small_map::IntoIter<T, (), C>,
}

impl<T, const C: usize> Iterator for IntoIter<T, C> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|(k, _)| k)
    }
}

impl<T, const C: usize> ExactSizeIterator for IntoIter<T, C> {
    fn len(&self) -> usize {
        self.inner.len()
    }
}

impl<T, const C: usize> FusedIterator for IntoIter<T, C> {}

impl<T, const C: usize, S> FromIterator<T> for SmallSet<T, C, S>
where
    T: Hash + Eq,
    S: BuildHasher + Default,
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self {
            data: SmallMap::from_iter(iter.into_iter().map(|i| (i, ()))),
        }
    }
}

impl<T, const C: usize, S> Debug for SmallSet<T, C, S>
where
    T: Hash + Eq + Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_set().entries(self.iter()).finish()
    }
}

#[derive(Clone)]
pub struct Difference<'a, T, const C: usize, S> {
    iter: Iter<'a, T>,
    other: &'a SmallSet<T, C, S>,
}

impl<'a, T, const C: usize, S> Iterator for Difference<'a, T, C, S>
where
    T: Eq + Hash,
    S: BuildHasher,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.find(|item| !self.other.contains(*item))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, self.iter.size_hint().1)
    }
}

impl<T, const C: usize, S> DoubleEndedIterator for Difference<'_, T, C, S>
where
    T: Eq + Hash,
    S: BuildHasher,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.rfind(|item| !self.other.contains(*item))
    }
}

impl<T, const C: usize, S> FusedIterator for Difference<'_, T, C, S>
where
    T: Eq + Hash,
    S: BuildHasher,
{
}

#[derive(Clone)]
pub struct SymmetricDifference<'a, T, const C1: usize, S1, const C2: usize, S2> {
    iter: Chain<Difference<'a, T, C2, S2>, Difference<'a, T, C1, S1>>,
}

impl<'a, T, const C1: usize, S1, const C2: usize, S2> Iterator
    for SymmetricDifference<'a, T, C1, S1, C2, S2>
where
    T: Eq + Hash,
    S1: BuildHasher,
    S2: BuildHasher,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }

    fn fold<B, F>(self, init: B, f: F) -> B
    where
        F: FnMut(B, Self::Item) -> B,
    {
        self.iter.fold(init, f)
    }
}

impl<'a, T, const C1: usize, S1, const C2: usize, S2> DoubleEndedIterator
    for SymmetricDifference<'a, T, C1, S1, C2, S2>
where
    T: Eq + Hash,
    S1: BuildHasher,
    S2: BuildHasher,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back()
    }

    fn rfold<B, F>(self, init: B, f: F) -> B
    where
        F: FnMut(B, Self::Item) -> B,
    {
        self.iter.rfold(init, f)
    }
}

impl<'a, T, const C1: usize, S1, const C2: usize, S2> FusedIterator
    for SymmetricDifference<'a, T, C1, S1, C2, S2>
where
    T: Eq + Hash,
    S1: BuildHasher,
    S2: BuildHasher,
{
}

#[derive(Clone)]
pub struct Intersection<'a, T, const C: usize, S> {
    iter: Iter<'a, T>,
    other: &'a SmallSet<T, C, S>,
}

impl<'a, T, const C: usize, S> Iterator for Intersection<'a, T, C, S>
where
    T: Eq + Hash,
    S: BuildHasher,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.find(|item| self.other.contains(*item))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, self.iter.size_hint().1)
    }
}

impl<'a, T, const C: usize, S> DoubleEndedIterator for Intersection<'a, T, C, S>
where
    T: Eq + Hash,
    S: BuildHasher,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.rfind(|item| self.other.contains(*item))
    }
}

impl<'a, T, const C: usize, S> FusedIterator for Intersection<'a, T, C, S>
where
    T: Eq + Hash,
    S: BuildHasher,
{
}

#[derive(Clone)]
pub struct Union<'a, T, const C: usize, S> {
    iter: Chain<Iter<'a, T>, Difference<'a, T, C, S>>,
}

impl<'a, T, const C: usize, S> Iterator for Union<'a, T, C, S>
where
    T: Eq + Hash,
    S: BuildHasher,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }

    fn fold<B, F>(self, init: B, f: F) -> B
    where
        F: FnMut(B, Self::Item) -> B,
    {
        self.iter.fold(init, f)
    }
}

impl<'a, T, const C: usize, S> DoubleEndedIterator for Union<'a, T, C, S>
where
    T: Eq + Hash,
    S: BuildHasher,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back()
    }

    fn rfold<B, F>(self, init: B, f: F) -> B
    where
        F: FnMut(B, Self::Item) -> B,
    {
        self.iter.rfold(init, f)
    }
}

impl<'a, T, const C: usize, S> FusedIterator for Union<'a, T, C, S>
where
    T: Eq + Hash,
    S: BuildHasher,
{
}

/// Create a [`SmallSet`] with with the specified values.
#[macro_export]
macro_rules! smallset {
    ($($x:expr),*$(,)*) => ({
        let map = $crate::smallmap!( $($x => (),)* );
        $crate::SmallSet::from_keys(map)
    });
}

/// Create a [`SmallSet`] with inline capacity equal to the number of values.
#[macro_export]
macro_rules! smallset_inline {
    ($($key:expr),*$(,)*) => ({
        let vec = smallvec::smallvec_inline!( $(($key, ()),)*);
        debug_assert_eq!(
            vec.len(),
            vec
                .iter()
                .map(|(k, _v)| k)
                .collect::<$crate::collections::HashSet<_>>()
                .len(),
            "smallset_inline! cannot be initialized with duplicate keys"
        );
        $crate::SmallSet::from_const_unchecked(vec)
    });
}

#[cfg(test)]
mod test {
    use super::*;
    use alloc::string::ToString;

    #[test]
    fn test_len_and_inline_capacity() {
        let mut set = SmallSet::<usize, 1>::new();
        assert_eq!(0, set.len());
        assert_eq!(0, set.iter().len());
        set.insert(0);
        assert_eq!(1, set.len());
        assert_eq!(1, set.iter().len());

        let set: SmallSet<_, 10> = smallset! {0, 1, 4};
        assert_eq!(3, set.len());
        assert_eq!(10, set.inline_capacity());

        let set = smallset_inline! {0, 1, 4 };
        assert_eq!(3, set.len());
        assert_eq!(3, set.iter().len());
        assert_eq!(3, set.inline_capacity());
    }

    #[test]
    fn smallset_macro_removes_duplicates() {
        let set: SmallSet<_, 10> = smallset! { 0 , 0};
        assert_eq!(1, set.len());
    }

    #[test]
    #[should_panic(expected = "smallset_inline! cannot be initialized with duplicate keys")]
    fn smallset_inline_macro_fails_on_duplicates() {
        smallset_inline! { 0 , 0 };
    }

    #[test]
    fn iter_iterates_in_insertion_order() {
        fn test<const C: usize>(inline: bool) {
            let inline_map: SmallSet<_, C> = smallset! {
                1, 0, 4
            };
            assert_eq!(inline, inline_map.is_inline());
            assert_eq!(
                alloc::vec![&1, &0, &4],
                inline_map.iter().collect::<alloc::vec::Vec<_>>(),
                "iter() does not return values in the correct order"
            );
            assert_eq!(
                alloc::vec![1, 0, 4],
                inline_map.into_iter().collect::<alloc::vec::Vec<_>>(),
                "into_iter() does not return values in the correct order"
            );
        }
        test::<1>(false);
        test::<3>(true);
    }

    #[test]
    fn insert_and_insert_full_tests() {
        // Test cases:
        // | Value               | Memory       | Insertion position |
        // | ------------------- | ------------ | ------------------ |
        // | new                 | Stay inline  | Last               |
        // | new                 | Move to heap | Last               |
        // | new                 | Stay on heap | Last               |
        // | already existing    | Stay inline  | Same as existing   |
        // | already existing    | Stay on heap | Same as existing   |

        let values = [10, 5, 86, 93];
        struct TestCase {
            name: &'static str,
            initial_values: alloc::vec::Vec<usize>,
            insert_value: usize,
            expected_inline_before: bool,
            expected_inline_after: bool,
            expected_values: alloc::vec::Vec<usize>,
            expected_return: (usize, bool),
        }
        let test_cases = [
            TestCase {
                name: "new key/value, stay inline",
                initial_values: values[0..2].to_vec(),
                insert_value: 7,
                expected_inline_before: true,
                expected_inline_after: true,
                expected_values: alloc::vec![10, 5, 7],
                expected_return: (2, false),
            },
            TestCase {
                name: "new key/value, move to heap",
                initial_values: values[0..3].to_vec(),
                insert_value: 7,
                expected_inline_before: true,
                expected_inline_after: false,
                expected_values: alloc::vec![10, 5, 86, 7],
                expected_return: (3, false),
            },
            TestCase {
                name: "new key/value, stay on heap",
                initial_values: values[0..4].to_vec(),
                insert_value: 7,
                expected_inline_before: false,
                expected_inline_after: false,
                expected_values: alloc::vec![10, 5, 86, 93, 7],
                expected_return: (4, false),
            },
            TestCase {
                name: "overwrite existing key/value, stay inline",
                initial_values: values[0..3].to_vec(),
                insert_value: 5,
                expected_inline_before: true,
                expected_inline_after: true,
                expected_values: alloc::vec![10, 5, 86],
                expected_return: (1, true),
            },
            TestCase {
                name: "overwrite existing key/value, stay on heap",
                initial_values: values[0..4].to_vec(),
                insert_value: 10,
                expected_inline_before: false,
                expected_inline_after: false,
                expected_values: alloc::vec![10, 5, 86, 93],
                expected_return: (0, true),
            },
        ];

        for test_case in test_cases {
            let mut small_set_1 = SmallSet::<usize, 3>::new();
            for v in test_case.initial_values {
                small_set_1.insert(v);
            }
            let mut small_set_2 = small_set_1.clone();

            for set in [&small_set_1, &small_set_2] {
                assert_eq!(
                    test_case.expected_inline_before,
                    set.is_inline(),
                    "inline state before insertion in SmallSet does not match expected in test '{}'",
                    test_case.name
                );
            }

            let actual_return_1 = small_set_1.insert(test_case.insert_value);
            let actual_return_2 = small_set_2.insert_full(test_case.insert_value);
            assert_eq!(
                test_case.expected_return.1, actual_return_1,
                "return of insertion in SmallMap does not match expected return in test '{}'",
                test_case.name
            );
            assert_eq!(
                test_case.expected_return, actual_return_2,
                "return of insertion_full in SmallMap does not match expected return in test '{}'",
                test_case.name
            );

            for set in [small_set_1, small_set_2] {
                assert_eq!(
                    test_case.expected_inline_after,
                    set.is_inline(),
                    "inline state after insertion in SmallSet does not match expected in test '{}'",
                    test_case.name
                );
                assert_eq!(
                    test_case.expected_values,
                    set.iter().copied().collect::<alloc::vec::Vec<_>>(),
                    "values in SmallSet do not match expected values in test '{}'",
                    test_case.name
                );
            }
        }
    }

    #[test]
    fn equality_is_consistent() {
        let set1: SmallSet<_, 3> = smallset! {0, 1, 4 };
        let set2 = smallset_inline! {0, 1, 4 };
        let set3 = SmallSet::<_, 3>::from_iter(alloc::vec![0, 1, 4]);
        let mut set4 = SmallSet::<_, 3>::new();
        set4.insert(0);
        set4.insert(1);
        set4.insert(4);

        assert_eq!(set1, set2);
        assert_eq!(set1, set3);
        assert_eq!(set1, set4);

        assert_eq!(set2, set3);
        assert_eq!(set2, set4);

        assert_eq!(set3, set4);
    }

    #[test]
    fn empty_small_maps_are_equal() {
        let set1: SmallSet<usize, 3> = smallset! {};
        let set2: SmallSet<usize, 3> = smallset! {};
        assert_eq!(set1, set2);
    }

    #[test]
    fn debug_string_test() {
        let actual = alloc::format!("{:?}", smallset_inline! {0, 1, 2});
        let expected = "{0, 1, 2}";
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_difference() {
        fn test<const C1: usize, const C2: usize>(inline_a: bool, inline_b: bool) {
            let set_a: SmallSet<&'static str, C1> = smallset! {"2", "1", "3", "b"};
            let set_b: SmallSet<&'static str, C2> = smallset! {"2", "d", "1", "3", "a"};
            assert_eq!(inline_a, set_a.is_inline());
            assert_eq!(inline_b, set_b.is_inline());

            let diff_a = set_a.difference(&set_b).copied().collect::<alloc::vec::Vec<_>>();
            assert_eq!(alloc::vec!["b"], diff_a);
            let diff_b = set_b.difference(&set_a).copied().collect::<alloc::vec::Vec<_>>();
            assert_eq!(alloc::vec!["d", "a"], diff_b);
            let diff_b_reverse = set_b.difference(&set_a).copied().rev().collect::<alloc::vec::Vec<_>>();
            assert_eq!(alloc::vec!["a", "d"], diff_b_reverse);

            assert_eq!(0, set_a.difference(&set_a).count());
        }
        test::<1, 1>(false, false);
        test::<1, 5>(false, true);
        test::<4, 5>(true, true);
        test::<4, 4>(true, false);
    }

    #[test]
    fn test_symmetric_difference() {
        fn test<const C1: usize, const C2: usize>(inline_a: bool, inline_b: bool) {
            let set_a: SmallSet<&'static str, C1> = smallset! {"2", "1", "3", "b"};
            let set_b: SmallSet<&'static str, C2> = smallset! {"2", "d", "1", "3", "a"};
            assert_eq!(inline_a, set_a.is_inline());
            assert_eq!(inline_b, set_b.is_inline());

            let diff_a = set_a
                .symmetric_difference(&set_b)
                .copied()
                .collect::<alloc::vec::Vec<_>>();
            assert_eq!(alloc::vec!["b", "d", "a"], diff_a);
            let diff_b = set_b
                .symmetric_difference(&set_a)
                .copied()
                .collect::<alloc::vec::Vec<_>>();
            assert_eq!(alloc::vec!["d", "a", "b"], diff_b);
            let diff_b_reverse = set_b
                .symmetric_difference(&set_a)
                .copied()
                .rev()
                .collect::<alloc::vec::Vec<_>>();
            assert_eq!(alloc::vec!["b", "a", "d"], diff_b_reverse);

            assert_eq!(0, set_a.symmetric_difference(&set_a).count());
        }
        test::<1, 1>(false, false);
        test::<1, 5>(false, true);
        test::<4, 5>(true, true);
        test::<4, 4>(true, false);
    }

    #[test]
    fn test_intersection() {
        fn test<const C1: usize, const C2: usize>(inline_a: bool, inline_b: bool) {
            let set_a: SmallSet<&'static str, C1> = smallset! {"2", "1", "3", "b"};
            let set_b: SmallSet<&'static str, C2> = smallset! {"1", "d", "3", "2", "a"};
            assert_eq!(inline_a, set_a.is_inline());
            assert_eq!(inline_b, set_b.is_inline());

            let diff_a = set_a.intersection(&set_b).copied().collect::<alloc::vec::Vec<_>>();
            assert_eq!(alloc::vec!["2", "1", "3"], diff_a);
            let diff_b = set_b.intersection(&set_a).copied().collect::<alloc::vec::Vec<_>>();
            assert_eq!(alloc::vec!["1", "3", "2"], diff_b);
            let diff_b_reverse = set_b
                .intersection(&set_a)
                .copied()
                .rev()
                .collect::<alloc::vec::Vec<_>>();
            assert_eq!(alloc::vec!["2", "3", "1"], diff_b_reverse);

            assert_eq!(
                alloc::vec!["2", "1", "3", "b"],
                set_a.intersection(&set_a).copied().collect::<alloc::vec::Vec<_>>()
            );
        }
        test::<1, 1>(false, false);
        test::<1, 5>(false, true);
        test::<4, 5>(true, true);
        test::<4, 4>(true, false);
    }

    #[test]
    fn test_union() {
        fn test<const C1: usize, const C2: usize>(inline_a: bool, inline_b: bool) {
            let set_a: SmallSet<&'static str, C1> = smallset! {"2", "1", "3", "b"};
            let set_b: SmallSet<&'static str, C2> = smallset! {"1", "d", "3", "2", "a"};
            assert_eq!(inline_a, set_a.is_inline());
            assert_eq!(inline_b, set_b.is_inline());

            let diff_a = set_a.union(&set_b).copied().collect::<alloc::vec::Vec<_>>();
            assert_eq!(alloc::vec!["2", "1", "3", "b", "d", "a"], diff_a);
            let diff_b = set_b.union(&set_a).copied().collect::<alloc::vec::Vec<_>>();
            assert_eq!(alloc::vec!["1", "d", "3", "2", "a", "b"], diff_b);
            let diff_b_reverse = set_b.union(&set_a).copied().rev().collect::<alloc::vec::Vec<_>>();
            assert_eq!(alloc::vec!["b", "a", "2", "3", "d", "1"], diff_b_reverse);

            assert_eq!(
                alloc::vec!["2", "1", "3", "b"],
                set_a.union(&set_a).copied().collect::<alloc::vec::Vec<_>>()
            );
        }
        test::<1, 1>(false, false);
        test::<1, 5>(false, true);
        test::<4, 5>(true, true);
        test::<4, 4>(true, false);
    }

    #[test]
    fn get_index_of_and_contains_test() {
        fn test<const C: usize>(inline: bool) {
            let set: SmallSet<&'static str, C> = smallset! {"2", "1", "3"};
            assert_eq!(inline, set.is_inline());

            assert_eq!(None, set.get_index_of(&"0"));
            assert!(!set.contains(&"0"));
            assert_eq!(None, set.get_index_of(&MyType(0)));
            assert!(!set.contains(&MyType(0)));

            assert_eq!(Some(1), set.get_index_of(&"1"));
            assert!(set.contains(&"1"));
            assert_eq!(Some(1), set.get_index_of(&MyType(1)));
            assert!(set.contains(&MyType(1)));
            assert_eq!(Some(0), set.get_index_of(&"2"));
            assert!(set.contains(&"2"));
            assert_eq!(Some(0), set.get_index_of(&MyType(2)));
            assert!(set.contains(&MyType(2)));
            assert_eq!(Some(2), set.get_index_of(&"3"));
            assert!(set.contains(&"3"));
            assert_eq!(Some(2), set.get_index_of(&MyType(3)));
            assert!(set.contains(&MyType(3)));
        }
        test::<1>(false);
        test::<3>(true);
    }

    // Type for testing equivalence to String
    struct MyType(usize);

    // Hash needs to be equivalent to String::hash
    impl Hash for MyType {
        fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
            self.0.to_string().hash(state);
        }
    }

    impl Equivalent<&'static str> for MyType {
        fn equivalent(&self, key: &&'static str) -> bool {
            &self.0.to_string() == key
        }
    }
}
