use indexmap::Equivalent;
use indexmap::IndexMap;
use indexmap::IndexSet;
use std::borrow::Borrow;
use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::BuildHasher;
use std::hash::Hash;
use std::iter::repeat;

use crate::multimap_base_impl;
use crate::multimap_mutators_impl;
#[derive(Debug)]
pub struct IndexVecMultimap<K, V, S = RandomState> {
    inner: IndexMap<K, Vec<V>, S>,
    len: usize,
}

impl<K, V> IndexVecMultimap<K, V> {
    multimap_base_impl! { IndexMap<K,Vec<V>>, Vec<V> }
}

impl<K, V, S> IndexVecMultimap<K, V, S>
where
    K: Hash + Eq,
    V: Hash + Eq,
    S: BuildHasher + Default,
{
    multimap_mutators_impl! {
        IndexMap<K, Vec<V>, S>,
        Vec<V>,
        vec![],
        vec,
        (Q: Hash + Equivalent<K>),
        (R: Equivalent<V>)
    }
}

#[derive(Debug)]
pub struct IndexSetMultimap<K, V, S = RandomState> {
    inner: IndexMap<K, IndexSet<V, S>, S>,
    len: usize,
}

impl<K, V> IndexSetMultimap<K, V> {
    multimap_base_impl! {IndexMap<K, IndexSet<V>>, IndexSet<V>}
}

impl<K, V, S> IndexSetMultimap<K, V, S>
where
    K: Hash + Eq,
    V: Hash + Eq,
    S: BuildHasher + Default,
{
    multimap_mutators_impl! {
        IndexMap<K, IndexSet<V,S>, S>,
        IndexSet<V,S>,
        IndexSet::with_hasher(S::default()),
        set,
        (Q: Hash + Equivalent<K>),
        (R: Hash + Equivalent<V>)
    }
}

#[derive(Debug)]
pub struct HashVecMultimap<K, V, S = RandomState> {
    inner: HashMap<K, Vec<V>, S>,
    len: usize,
}

impl<K, V> HashVecMultimap<K, V> {
    multimap_base_impl! { HashMap<K,Vec<V>>, Vec<V> }
}

impl<K, V, S> HashVecMultimap<K, V, S>
where
    K: Hash + Eq,
    V: Hash + Eq,
    S: BuildHasher + Default,
{
    multimap_mutators_impl! {
        HashMap<K, Vec<V>, S>,
        Vec<V>,
        vec![],
        vec,
        (K: Borrow<Q>, Q: Hash + Eq),
        (V: Borrow<R>, R: Equivalent<V>)
    }
}

#[derive(Debug)]
pub struct HashSetMultimap<K, V, S = RandomState> {
    inner: HashMap<K, HashSet<V, S>, S>,
    len: usize,
}

impl<K, V> HashSetMultimap<K, V> {
    multimap_base_impl! {HashMap<K, HashSet<V>>, HashSet<V>}
}

impl<K, V, S> HashSetMultimap<K, V, S>
where
    K: Hash + Eq,
    V: Hash + Eq,
    S: BuildHasher + Default,
{
    multimap_mutators_impl! {
        HashMap<K, HashSet<V,S>, S>,
        HashSet<V,S>,
        HashSet::with_hasher(S::default()),
        set,
        (K: Borrow<Q>, Q: Hash + Eq),
        (V: Borrow<R>, R: Hash + Eq)
    }
}

// TODO generate with a macro
impl<K, V, S> FromIterator<(K, V)> for IndexSetMultimap<K, V, S>
where
    K: Hash + Eq,
    V: Hash + Eq,
    S: BuildHasher + Default,
{
    fn from_iter<I: IntoIterator<Item = (K, V)>>(iterable: I) -> Self {
        let iter = iterable.into_iter();
        let (low, _) = iter.size_hint();
        let mut map = Self::with_capacity_and_hasher(low, <_>::default());
        map.extend(iter);
        map
    }
}

impl<K, V, S> Extend<(K, V)> for IndexSetMultimap<K, V, S>
where
    K: Hash + Eq,
    V: Hash + Eq,
    S: BuildHasher + Default,
{
    fn extend<I: IntoIterator<Item = (K, V)>>(&mut self, iterable: I) {
        // Using the  same reservation logic as in IndexMap
        let iter = iterable.into_iter();
        let reserve = if self.is_empty() {
            iter.size_hint().0
        } else {
            (iter.size_hint().0 + 1) / 2
        };
        self.reserve(reserve);
        iter.for_each(move |(k, v)| {
            self.insert(k, v);
        });
    }
}

impl<'a, K, V, S> Extend<(&'a K, &'a V)> for IndexSetMultimap<K, V, S>
where
    K: Hash + Eq + Copy,
    V: Hash + Eq + Copy,
    S: BuildHasher + Default,
{
    fn extend<I: IntoIterator<Item = (&'a K, &'a V)>>(&mut self, iterable: I) {
        self.extend(iterable.into_iter().map(|(&key, &value)| (key, value)));
    }
}

impl<K, V, S> From<IndexMap<K, IndexSet<V, S>, S>> for IndexSetMultimap<K, V, S>
where
    K: Hash + Eq + Copy,
    V: Hash + Eq + Copy,
    S: BuildHasher + Default,
{
    fn from(mut map: IndexMap<K, IndexSet<V, S>, S>) -> Self {
        map.retain(|_k, v| !v.is_empty());
        let len = map.iter().map(|(_k, v)| v.len()).sum();
        IndexSetMultimap { inner: map, len }
    }
}

impl<K, V1, S1, V2, S2> PartialEq<IndexSetMultimap<K, V2, S2>> for IndexSetMultimap<K, V1, S1>
where
    K: Hash + Eq,
    V1: Hash + Eq + PartialEq<V2> + Borrow<V2>,
    V2: Hash + Eq + PartialEq<V1> + Borrow<V1>,
    S1: BuildHasher + Default,
    S2: BuildHasher + Default,
{
    fn eq(&self, other: &IndexSetMultimap<K, V2, S2>) -> bool {
        if self.len() != other.len() {
            return false;
        }
        self.iter().all(|(key, value)| other.contains(key, value))
    }
}

impl<K, V, S> Eq for IndexSetMultimap<K, V, S>
where
    K: Eq + Hash,
    V: Eq + Hash,
    S: BuildHasher + Default,
{
}

#[cfg(test)]
mod tests {
    use indexmap::indexmap;
    use indexmap::indexset;

    use super::*;

    #[test]
    fn with_capacity_constructs_instance_with_correct_capacity() {
        let map7: IndexSetMultimap<usize, usize> = IndexSetMultimap::with_key_capacity(7);
        let map17: IndexSetMultimap<usize, usize> = IndexSetMultimap::with_key_capacity(17);
        assert_eq!(7, map7.key_capacity());
        assert_eq!(17, map17.key_capacity());
    }

    #[test]
    fn insert_ignores_duplicates() {
        let mut map = IndexSetMultimap::new();
        assert_eq!(0, map.len());

        assert!(map.insert(0, "A".to_string()));
        assert_eq!(1, map.len());
        assert!(map.contains(&0, &"A".to_string()));

        assert!(!map.insert(0, "A".to_string()));
        assert_eq!(1, map.len());
        assert!(map.contains(&0, &"A".to_string()));
    }

    #[test]
    fn remove_removes_key_when_needed() {
        let data = vec![(0, "A1".to_string()), (0, "A2".to_string())];
        let mut map = data.into_iter().collect::<IndexSetMultimap<_, _>>();
        assert_eq!(2, map.len());
        assert_eq!(1, map.keys_len());
        assert!(!map.is_empty());

        assert!(map.remove(&0, &"A2".to_string()));
        assert!(!map.contains(&0, &"A2".to_string()));
        assert_eq!(1, map.len());
        assert_eq!(1, map.keys_len());
        assert!(!map.is_empty());
        assert_eq!(Some(&indexset! {"A1".to_string()}), map.get(&0));

        assert!(map.remove(&0, &"A1".to_string()));
        assert!(!map.contains(&0, &"A1".to_string()));
        assert_eq!(0, map.len());
        assert_eq!(0, map.keys_len());
        assert!(map.is_empty());
        assert_eq!(None, map.get(&0));
    }

    #[test]
    fn remove_key_returns_entire_value_set_when_present() {
        let mut map = vec![(0, "A1".to_string()), (0, "A2".to_string())]
            .into_iter()
            .collect::<IndexSetMultimap<_, _>>();
        assert_eq!(2, map.len());
        assert_eq!(1, map.keys_len());
        assert!(!map.is_empty());

        let expected = Some(indexset! {"A1".to_string(), "A2".to_string()});
        assert_eq!(expected, map.remove_key(&0));
        assert_eq!(0, map.len());
        assert_eq!(0, map.keys_len());
        assert!(map.is_empty());

        assert_eq!(None, map.remove_key(&0));
    }

    #[test]
    fn remove_is_noop_when_key_value_is_not_there() {
        let data = vec![(0, "A1".to_string()), (0, "A2".to_string())];
        let mut map = data.into_iter().collect::<IndexSetMultimap<_, _>>();
        assert!(!map.remove(&0, &"A3".to_string()));
        assert_eq!(2, map.len());
        assert_eq!(1, map.keys_len());
    }

    #[test]
    fn len_is_consistent() {
        let data = vec![
            (0, "A".to_string()),
            (1, "B".to_string()),
            (2, "C".to_string()),
            (3, "D".to_string()),
            (4, "E".to_string()),
            (4, "E2".to_string()),
            (0, "A2".to_string()),
        ];
        let mut map = IndexSetMultimap::new();
        for (i, (k, v)) in data.iter().enumerate() {
            assert_eq!(map.len(), i);
            map.insert(*k, v.to_string());
            assert_eq!(map.len(), i + 1);
        }
        let map = data.into_iter().collect::<IndexSetMultimap<_, _>>();
        assert_eq!(7, map.len());
        assert_eq!(5, map.keys_len());
    }

    #[test]
    fn equality_test_fails_on_different_len() {
        let a = IndexSetMultimap::from(indexmap! {0 => indexset!{ 0 }});
        let b = IndexSetMultimap::from(indexmap! {0 => indexset!{ 0 }, 1 => indexset!{ 1 }});
        assert!(!a.eq(&b))
    }

    #[test]
    fn equality_test_fails_on_same_len_but_distinct_elem_count() {
        let a = IndexSetMultimap::from(indexmap! {0 => indexset!{ 0 }});
        let b = IndexSetMultimap::from(indexmap! {0 => indexset!{ 0, 1 }});
        assert!(!a.eq(&b))
    }
    #[test]
    fn equality_test_succeeds_on_inversely_ordered_sets() {
        let a = IndexSetMultimap::from(indexmap! {
            0 => indexset!{ 1, 0 },
            1 => indexset!{ 2, 3 },
        });
        let b = IndexSetMultimap::from(indexmap! {
            1 => indexset!{ 3, 2 },
            0 => indexset!{ 0, 1 },
        });
        assert!(a.eq(&b))
    }

    // #[test]
    // fn get_index_returns_correct_value() {
    //     let map = IndexSetMultimap::from(indexmap! {
    //         0 => indexset!{ 1, 2, 3 },
    //         2 => indexset!{ 2, 3 },
    //         1 => indexset!{ 3 },
    //     });

    //     assert_eq!(map.get_index(0), Some((&0, &indexset! {1,2,3})));
    //     assert_eq!(map.get_index(1), Some((&2, &indexset! {2,3})));
    //     assert_eq!(map.get_index(2), Some((&1, &indexset! {3})));
    //     assert_eq!(map.get_index(3), None);
    // }
    #[test]
    fn contains_key_returns_correct_value() {
        let map = IndexSetMultimap::from(indexmap! {
            0 => indexset!{ 1, 2, 3 },
            9 => indexset!{ 2, 3 },
            333 => indexset!{ 3 },
        });

        assert!(map.contains_key(&0));
        assert!(map.contains_key(&9));
        assert!(map.contains_key(&333));

        assert!(!map.contains_key(&1));
        assert!(!map.contains_key(&456));
        assert!(!map.contains_key(&7));
    }

    #[test]
    fn extend_works_with_empty_multimap() {
        let mut actual = IndexSetMultimap::from(indexmap! {});
        actual.extend(vec![(0, 1), (2, 3)]);

        let expected = IndexSetMultimap::from(indexmap! {
            0 => indexset!{ 1 },
            2 => indexset!{ 3 },
        });
        assert_eq!(expected, actual);
    }

    #[test]
    fn extend_works_with_non_empty_multimap() {
        let mut actual = IndexSetMultimap::from(indexmap! {
            0 => indexset!{ 1 },
            2 => indexset!{ 3 },
        });
        actual.extend(vec![(0, 2), (2, 3), (4, 5)]);
        let expected = IndexSetMultimap::from(indexmap! {
            0 => indexset!{ 1, 2 },
            2 => indexset!{ 3 },
            4 => indexset!{ 5 },
        });
        assert_eq!(expected, actual);
    }

    #[test]
    fn extend_works_with_copy_iter() {
        let mut actual = IndexSetMultimap::from(indexmap! {});
        // these values get copied
        actual.extend(vec![(&0, &1), (&2, &3)]);
        let expected = IndexSetMultimap::from(indexmap! {
            0 => indexset!{ 1 },
            2 => indexset!{ 3 },
        });
        assert_eq!(expected, actual);
    }

    #[test]
    fn from_ignores_empty_sets() {
        let map = IndexSetMultimap::from(indexmap! {
            0 => indexset!{ 1, 2, 3 },
            9 => indexset!{ },
            333 => indexset!{ 3 },
        });

        assert_eq!(2, map.keys_len());
        assert_eq!(4, map.len());
        assert!(!map.contains_key(&9));

        let actual = map.iter().collect::<Vec<_>>();
        let expected = vec![(&0, &1), (&0, &2), (&0, &3), (&333, &3)];
        assert_eq!(expected, actual);
    }
}
