use smallvec::SmallVec;

use crate::FastIndexMap;
use ::core::hash::Hash;
use std::fmt::Debug;
use std::mem;

struct SmallMap<K, V, const C: usize> {
    data: MapData<K, V, C>,
}

enum MapData<K, V, const C: usize> {
    Inline(SmallVec<[(K, V); C]>),
    Heap(FastIndexMap<K, V>),
}

impl<K, V, const C: usize> SmallMap<K, V, C>
where
    K: Hash + Eq + Debug,
    V: Debug,
{
    pub fn new() -> Self {
        debug_assert!(
            C > 0,
            "Cannot instantiate SmallMap with 0 capacity, raise capacity or use IndexMap instead",
        );
        SmallMap {
            data: MapData::Inline(SmallVec::new()),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn len(&self) -> usize {
        match &self.data {
            MapData::Inline(sv) => sv.len(),
            MapData::Heap(map) => map.len(),
        }
    }

    pub fn insert(mut self, key: K, value: V) -> Option<V> {
        match &self.data {
            MapData::Inline(sv) => {
                if sv.len() + 1 > C {
                    let inner = sv.into_inner().unwrap();
                    let mut map = sv.into_iter().collect::<FastIndexMap<K, V>>();
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
    }
}
