use std::fmt;
use std::fmt::Debug;
use std::fmt::Formatter;

use ::core::hash::Hash;
use smallvec::SmallVec;

use crate::small_map;
use crate::SmallMap;

/// A set-like container that can store a specified number of elements inline.
///
/// `SmallSet` shares most of its API with, and behaves like,
/// [IndexSet](indexmap::IndexSet). It can store a limited amount of data
/// inline, backed by [SmallVec](smallvec::SmallVec). If the data exceeds the
/// limit `C`, `SmallSet` will move _all_ its data over to the heap in the form
/// of an `IndexSet`. For performance reasons, transitions between heap and
/// inline storage should generally be avoided.
///
/// The `SmallSet` datastructure is meant for situations where the data does not
/// exceed `C` _most of the time_ but it still needs to support cases where the
/// data _does_ exceed `C`.
#[derive(Default, Clone)]
pub struct SmallSet<T, const C: usize> {
    data: SmallMap<T, (), C>,
}

impl<T, const C: usize> SmallSet<T, C> {
    /// Create a new set.
    pub fn new() -> Self {
        Self {
            data: SmallMap::new(),
        }
    }

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
    pub const fn from_const_unchecked(inline: SmallVec<[(T, ()); C]>) -> Self {
        Self {
            data: SmallMap::from_const_unchecked(inline),
        }
    }
}

impl<T, const C: usize> SmallSet<T, C>
where
    T: Hash + Eq,
{
    /// Inserts the specified value into this set.
    ///
    /// If the value already exists, this is a no-op.
    ///
    /// If a new value is added that causes the size of the `SmallSet` to exceed
    /// the inline capacity, all existing data and the new value is moved to the
    /// heap.
    ///
    /// Computational complexity:
    ///  - inline: O(n)
    ///  - heap: O(1)
    pub fn insert(&mut self, value: T) {
        self.data.insert(value, ());
    }

    pub fn from_keys(map: SmallMap<T, (), C>) -> SmallSet<T, C> {
        SmallSet { data: map }
    }
}

impl<T, const C: usize> Eq for SmallSet<T, C> where T: Hash + Eq {}
impl<T, const C: usize> PartialEq for SmallSet<T, C>
where
    T: Hash + Eq,
{
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

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

impl<T, const C: usize> FromIterator<T> for SmallSet<T, C>
where
    T: Hash + Eq,
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self {
            data: SmallMap::from_iter(iter.into_iter().map(|i| (i, ()))),
        }
    }
}

impl<T, const C: usize> Debug for SmallSet<T, C>
where
    T: Hash + Eq + Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_set().entries(self.iter()).finish()
    }
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
                .collect::<$crate::FastHashSet<_>>()
                .len(),
            "smallset_inline! cannot be initialized with duplicate keys"
        );
        $crate::SmallSet::from_const_unchecked(vec)
    });
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn iter_order_follows_insertion_order() {
        let set: SmallSet<_, 5> = smallset! { 0, 1, 2, 5, 2};
        assert_eq!(4, set.len());
        let actual = set.iter().copied().collect::<Vec<_>>();
        let expected = vec![0, 1, 2, 5];
        assert_eq!(expected, actual);
    }

    #[test]
    fn debug_string_test() {
        let actual = format!("{:?}", smallset_inline! {0, 1, 2});
        let expected = "{0, 1, 2}";
        assert_eq!(expected, actual);
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
}
