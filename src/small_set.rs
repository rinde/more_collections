use std::fmt::Debug;

use ::core::hash::Hash;

use crate::small_map;
use crate::SmallMap;

#[derive(Debug, Default)]
pub struct SmallSet<T, const C: usize> {
    data: SmallMap<T, (), C>,
}

impl<T, const C: usize> SmallSet<T, C>
where
    T: Hash + Eq,
{
    pub fn new() -> Self {
        Self {
            data: SmallMap::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn from_keys(map: SmallMap<T, (), C>) -> SmallSet<T, C> {
        SmallSet { data: map }
    }

    pub fn iter(&'_ self) -> Iter<'_, T> {
        Iter {
            inner: self.data.iter(),
        }
    }
}

impl<T, const C: usize> SmallSet<T, C>
where
    T: Hash + Eq + Debug + Clone,
{
    pub fn insert(&mut self, value: T) {
        self.data.insert(value, ());
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
    T: Hash + Eq + Debug + Clone,
{
    // TODO this is not efficient if the size is already known
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut set = SmallSet::new();
        iter.into_iter().for_each(|i| set.insert(i));
        set
    }
}

// TODO to make smallset! more efficient it could be considered to directly
// create a smallvec internally, and check for duplicate keys using an
// debug_assert
#[macro_export]
macro_rules! smallset {
    ($($x:expr),*$(,)*) => ({
        let map = $crate::smallmap!( $($x => (),)* );
        $crate::SmallSet::from_keys(map)
    });
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        let set: SmallSet<_, 5> = smallset! { 0, 1, 2, 5, 2};
        println!("{:?}", set);
        // assert_eq!(set.iter())
    }
}
