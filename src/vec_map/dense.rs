use std::marker::PhantomData;

use crate::vecmap_base2_impl;
use crate::vecmap_base_impl;
use crate::IndexKey;

pub struct DenseVecMap<K, V> {
    data: Vec<V>,
    len: usize,
    _marker: PhantomData<K>,
}

#[inline(always)]
fn identity<T>(t: T) -> T {
    t
}

vecmap_base_impl!(DenseVecMap, (V: Clone + Default), identity);
vecmap_base2_impl!(DenseVecMap, (V: Clone + Default), identity);

impl<K: IndexKey, V: Clone + Default> DenseVecMap<K, V> {
    // TODO what is expected here? Should it panic if out of bounds? Should it fill up the gaps with default values?
    // / Inserts a key-value pair into the map.
    // /
    // / If the key is present in the map, the value is updated and the old value
    // / is returned. Otherwise, [`None`] is returned.
    // pub fn insert(&mut self, key: K, value: V) -> Option<V> {
    //     let index = key.as_index();
    //     if index >= self.capacity() {
    //         self.data
    //             .extend((0..=(index - self.data.len())).map(|_| Default::default()));
    //     }

    //     let existing = self.data[index].replace(value);
    //     if existing.is_none() {
    //         self.len += 1;
    //     }
    //     existing
    // }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let map: DenseVecMap<usize, usize> = DenseVecMap::with_capacity(7);
    }
}
