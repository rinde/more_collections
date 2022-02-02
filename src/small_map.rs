use std::collections::HashSet;
use std::fmt::Debug;
use std::mem;
use std::ops::Index;
use std::ops::IndexMut;

use ::core::hash::Hash;
use indexmap::Equivalent;
use smallvec::SmallVec;

use crate::FastIndexMap;

/// A map-like container that can store a specified number of elements inline.
///
/// `SmallMap` acts like an [IndexMap](indexmap::IndexMap). It can store a
/// limited amount of data inline, backed by [SmallVec]. If the data exceeds
/// the limit `C`, `SmallMap` will move _all_ its data over to the heap in the
/// form of an `IndexMap`. For performance reasons, transitions between heap and
/// inline storage should generally be avoided. This datastructure is meant
/// for situations where the data does not exceed `C` _most of the time_ but it
/// still needs to support cases where the data _does_ exceed `C`.
#[derive(Debug, Default)]
pub struct SmallMap<K, V, const C: usize> {
    data: MapData<K, V, C>,
}

#[derive(Debug)]
enum MapData<K, V, const C: usize> {
    Inline(SmallVec<[(K, V); C]>),
    Heap(FastIndexMap<K, V>),
}

impl<K, V, const C: usize> SmallMap<K, V, C> {
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn len(&self) -> usize {
        match &self.data {
            MapData::Inline(sv) => sv.len(),
            MapData::Heap(map) => map.len(),
        }
    }

    pub fn iter(&'_ self) -> Iter<'_, K, V> {
        match &self.data {
            MapData::Inline(vec) => Iter::Inline(vec.iter()),
            MapData::Heap(map) => Iter::Heap(map.iter()),
        }
    }

    pub const fn from_const(inline: SmallVec<[(K, V); C]>) -> Self {
        Self {
            data: MapData::Inline(inline),
        }
    }
}

impl<K, V, const C: usize> SmallMap<K, V, C>
where
    K: Hash + Eq,
    V: Eq,
{
    pub fn new() -> Self {
        debug_assert!(
            C > 0,
            "Cannot instantiate SmallMap with 0 capacity, use positive capacity or use IndexMap instead",
        );
        SmallMap {
            data: MapData::Inline(SmallVec::new()),
        }
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        match &self.data {
            MapData::Inline(vec) => vec.iter().find(|(k, _v)| k == key).map(|(_k, v)| v),
            MapData::Heap(map) => map.get(key),
        }
    }

    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        match &mut self.data {
            MapData::Inline(vec) => vec.iter_mut().find(|(k, _v)| k == key).map(|(_k, v)| v),
            MapData::Heap(map) => map.get_mut(key),
        }
    }

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

    pub fn get_index_mut(&mut self, index: usize) -> Option<(&mut K, &mut V)> {
        match &mut self.data {
            MapData::Inline(vec) => {
                if index < vec.len() {
                    Some(&mut vec[index]).map(|(k, v)| (k, v))
                } else {
                    None
                }
            }
            MapData::Heap(map) => map.get_index_mut(index),
        }
    }

    pub fn get_index_of<Q: ?Sized>(&self, key: &Q) -> Option<usize>
    where
        Q: Hash + Equivalent<K>,
    {
        match &self.data {
            MapData::Inline(vec) => vec.iter().position(|(k, _v)| key.equivalent(k)),
            MapData::Heap(map) => map.get_index_of(key),
        }
    }

    pub fn entry(&mut self, key: K) -> Entry<'_, K, V, C> {
        let index = self.get_index_of(&key);
        match index {
            Some(index) => Entry::Occupied(self, index),
            None => Entry::Vacant(self, key),
        }
    }

    pub fn inline_size(&self) -> usize {
        C
    }

    pub fn from_map(map: FastIndexMap<K, V>) -> Self {
        if map.capacity() <= C {
            Self {
                data: MapData::Inline(SmallVec::from_vec(map.into_iter().collect())),
            }
        } else {
            Self {
                data: MapData::Heap(map),
            }
        }
    }
}

impl<K, V, const C: usize> SmallMap<K, V, C>
where
    K: Hash + Eq + Debug + Clone,
    V: Eq + Debug + Clone,
{
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        match &mut self.data {
            MapData::Inline(sv) => {
                if sv.len() + 1 > C {
                    // TODO can this be done without cloning
                    let mut map = sv.iter().cloned().collect::<FastIndexMap<_, _>>();
                    let ret = map.insert(key, value);
                    self.data = MapData::Heap(map);
                    ret
                } else {
                    let existing_index = sv.iter().position(|(k, _v)| &key == k);
                    if let Some(existing_index) = existing_index {
                        let ret = mem::replace(&mut sv[existing_index], (key, value));
                        Some(ret.1)
                    } else {
                        sv.push((key, value));
                        None
                    }
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
        self.data == other.data
    }
}

impl<K, V, const C: usize> Eq for MapData<K, V, C>
where
    K: Hash + Eq,
    V: Eq,
{
}
impl<K, V, const C: usize> PartialEq for MapData<K, V, C>
where
    K: Hash + Eq,
    V: Eq,
{
    // TODO consider comparing on iterators?
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Inline(l0), Self::Inline(r0)) => l0 == r0,
            (Self::Heap(l0), Self::Heap(r0)) => l0 == r0,
            (_, _) => false,
        }
    }
}

impl<K, V, const C: usize> Default for MapData<K, V, C> {
    fn default() -> Self {
        MapData::Inline(SmallVec::new())
    }
}

impl<K, V, const C: usize> From<SmallVec<[(K, V); C]>> for SmallMap<K, V, C>
where
    K: Eq + Hash,
    V: Eq,
{
    // TODO also add a 'safe' method to convert SmallVec to map
    fn from(vec: SmallVec<[(K, V); C]>) -> Self {
        debug_assert_eq!(
            vec.iter().map(|(k, _)| k).collect::<HashSet<_>>().len(),
            vec.len(),
            "Duplicate keys are not allowed"
        );
        SmallMap {
            data: MapData::Inline(vec),
        }
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

pub enum Entry<'a, K, V, const C: usize>
where
    K: Hash + Eq,
    V: Eq,
{
    Occupied(&'a mut SmallMap<K, V, C>, usize),
    Vacant(&'a mut SmallMap<K, V, C>, K),
}

impl<'a, K, V, const C: usize> Entry<'a, K, V, C>
where
    K: Hash + Eq + Debug + Clone,
    V: Eq + Debug + Clone,
{
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

// TODO to make smallmap! more efficient it could be considered to directly
// create a smallvec internally, and check for duplicate keys using an
// debug_assert
#[macro_export]
macro_rules! smallmap {
    // count helper: transform any expression into 1
    (@one $x:expr) => (1usize);
    ($($key:expr => $value:expr),*$(,)*) => ({
        let count = 0usize $(+ $crate::smallmap!(@one $key))*;
        #[allow(unused_mut)]
        let mut map = $crate::SmallMap::new();
        if count <= map.inline_size() {
            $(map.insert($key, $value);)*
            map
        } else {
            $crate::SmallMap::from_map($crate::fastindexmap![$($key => $value,)*])
        }
    });
}

/// Creates [`SmallMap`] with capacity equal to the number of values.
#[macro_export]
macro_rules! smallmap_inline {
    // count helper: transform any expression into 1
    (@one $x:expr) => (1usize);
    ($($key:expr => $value:expr),*$(,)*) => ({
        let vec = smallvec::smallvec_inline!( $(($key, $value),)*);
        $crate::SmallMap::from_const(vec)
    });
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn map() {
        let mut map: SmallMap<usize, usize, 1> = SmallMap::new();

        assert_eq!(0, map.len());
        map.insert(0, 1);
        assert_eq!(1, map.len());

        println!("{}", map.len());

        let map: SmallMap<_, _, 10> = smallmap! {
            0 => 1,
            1 => 7,
            4 => 9
        };

        assert_eq!(3, map.len());
        assert_eq!(10, map.inline_size());

        let map = smallmap_inline! {
            0 => 1,
            1 => 7,
            4 => 9
        };
        assert_eq!(3, map.len());
        assert_eq!(3, map.inline_size());
    }
}
