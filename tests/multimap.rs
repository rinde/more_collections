#![cfg(test)]

macro_rules! set_multimap_tests {
    ($type:tt) => {
        #[test]
        fn insert_ignores_duplicates() {
            let mut map = $type::new();
            assert_eq!(0, map.len());

            assert!(map.insert(0, "A".to_string()));
            assert_eq!(1, map.len());
            assert!(map.contains(&0, &"A".to_string()));

            assert!(!map.insert(0, "A".to_string()));
            assert_eq!(1, map.len());
            assert!(map.contains(&0, &"A".to_string()));
        }
    };
}

macro_rules! index_multimap_tests {
    ($type:tt, $multimap_macro:tt, $values_macro:tt) => {
        #[test]
        fn with_capacity_constructs_instance_with_correct_capacity() {
            let map7: $type<usize, usize> = $type::with_key_capacity(7);
            let map17: $type<usize, usize> = $type::with_key_capacity(35);
            assert_eq!(7, map7.key_capacity());
            assert_eq!(35, map17.key_capacity());
        }

        #[test]
        fn get_full_returns_correct_values() {
            let map = $multimap_macro! {
                0 => {1, 2, 3},
                2 => {2, 3},
                1 => {3}
            };
            assert_eq!(Some((0, &0, &$values_macro! { 1, 2, 3 })), map.get_full(&0));
            assert_eq!(Some((1, &2, &$values_macro! { 2, 3 })), map.get_full(&2));
            assert_eq!(None, map.get_full(&3));
        }

        #[test]
        fn get_key_index_returns_correct_value() {
            let map = $multimap_macro! {
                0 => {1, 2, 3},
                2 => {2, 3},
                1 => {3}
            };
            assert_eq!(Some(0), map.get_key_index(&0));
            assert_eq!(Some(2), map.get_key_index(&1));
            assert_eq!(Some(1), map.get_key_index(&2));
            assert_eq!(None, map.get_key_index(&3));
        }
    };
}

macro_rules! hash_multimap_tests {
    ($type:tt) => {
        #[test]
        fn with_capacity_constructs_instance_with_correct_capacity() {
            let map7: $type<usize, usize> = $type::with_key_capacity(7);
            let map17: $type<usize, usize> = $type::with_key_capacity(35);
            assert_eq!(7, map7.key_capacity());
            assert!(35 <= map17.key_capacity());
        }
    };
}

macro_rules! general_multimap_tests {
    ($type:tt, $multimap_macro:tt, $keys_macro:tt, $values_macro:tt) => {
        #[test]
        fn remove_removes_key_when_needed() {
            let data = vec![(0, "A1".to_string()), (0, "A2".to_string())];
            let mut map = data.into_iter().collect::<$type<_, _>>();
            assert_eq!(2, map.len());
            assert_eq!(1, map.keys_len());
            assert!(!map.is_empty());

            assert_eq!(Some("A2".to_string()), map.remove(&0, &"A2".to_string()));
            assert!(!map.contains(&0, &"A2".to_string()));
            assert_eq!(1, map.len());
            assert_eq!(1, map.keys_len());
            assert!(!map.is_empty());

            let result = map.get(&0);
            assert!(result.is_some());
            assert_eq!(1, result.unwrap().len());
            assert_eq!(
                vec![&"A1".to_string()],
                result.unwrap().iter().collect::<Vec<_>>()
            );

            assert_eq!(Some("A1".to_string()), map.remove(&0, &"A1".to_string()));
            assert!(!map.contains(&0, &"A1".to_string()));
            assert_eq!(0, map.len());
            assert_eq!(0, map.keys_len());
            assert!(map.is_empty());
            assert_eq!(None, map.get(&0));
        }

        #[test]
        fn remove_key_returns_entire_value_collection_when_present() {
            let mut map = vec![(0, "A1".to_string()), (0, "A2".to_string())]
                .into_iter()
                .collect::<$type<_, _>>();
            assert_eq!(2, map.len());
            assert_eq!(1, map.keys_len());
            assert!(!map.is_empty());

            let expected = Some(maplit::hashset!["A1".to_string(), "A2".to_string()]);
            assert_eq!(
                expected,
                map.remove_key(&0)
                    .map(|r| r.into_iter().collect::<std::collections::HashSet<_>>())
            );
            assert_eq!(0, map.len());
            assert_eq!(0, map.keys_len());
            assert!(map.is_empty());
            let empty: $type<usize, String> = $multimap_macro! {};
            assert_eq!(empty, map);

            assert_eq!(None, map.remove_key(&0));
        }

        #[test]
        fn remove_key_entry_returns_entire_value_collection_when_present() {
            let mut map = $multimap_macro! {
                0 => {"A".to_string() }
            };
            let actual = map.remove_key_entry(&0);
            let expected = Some((0, $values_macro! { "A".to_string() }));
            assert_eq!(expected, actual);
            assert!(map.is_empty());
            assert_eq!(None, map.remove_key_entry(&0));
        }

        #[test]
        fn retain_on_key_filter() {
            let mut map = $multimap_macro! {
                0 => {1, 2, 3 },
                4 => {5},
                7 => {3, 9}
            };
            map.retain(|k, _v| k == &4);
            assert_eq!(1, map.len());
            let expected = $multimap_macro! {
                4 => {5}
            };
            assert_eq!(expected, map);
        }

        #[test]
        fn retain_on_value_filter() {
            let mut map = $multimap_macro! {
                0 => {1, 2, 3 },
                4 => {5},
                7 => {3, 9}
            };
            map.retain(|_k, v| v == &3);
            assert_eq!(2, map.len());
            let expected = $multimap_macro! {
                0 => {3},
                7 => {3}
            };
            assert_eq!(expected, map);
        }

        #[test]
        fn retain_on_key_value_filter() {
            let mut map = $multimap_macro! {
                0 => {1, 2, 3 },
                4 => {5},
                7 => {3, 9}
            };
            map.retain(|k, v| (k + v) % 2 == 0);
            assert_eq!(3, map.len());
            let expected = $multimap_macro! {
                0 => {2},
                7 => {3, 9}
            };
            assert_eq!(expected, map);
        }

        #[test]
        fn remove_is_noop_when_key_value_is_not_there() {
            let data = vec![(0, "A1".to_string()), (0, "A2".to_string())];
            let mut map = data.into_iter().collect::<$type<_, _>>();
            assert_eq!(None, map.remove(&0, &"A3".to_string()));
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
            let mut map = $type::new();
            for (i, (k, v)) in data.iter().enumerate() {
                assert_eq!(map.len(), i);
                map.insert(*k, v.to_string());
                assert_eq!(map.len(), i + 1);
            }
            let map = data.into_iter().collect::<$type<_, _>>();
            assert_eq!(7, map.len());
            assert_eq!(5, map.keys_len());
        }

        #[test]
        fn shrink_keys_to_fit_sets_capacity_at_most_len() {
            // empty sets it to zero
            let mut map7: $type<usize, usize> = $type::with_key_capacity(7);
            map7.shrink_keys_to_fit();
            assert_eq!(0, map7.key_capacity());

            // occupied sets it to len
            let mut map: $type<usize, usize> = $type::with_key_capacity(7);
            map.insert(0, 1);
            map.insert(0, 2);
            map.insert(1, 7);
            map.shrink_keys_to_fit();
            assert!(map.key_capacity() >= 2);
        }

        #[test]
        fn shrink_values_to_fit_sets_capacity_at_most_len() {
            let mut map: $type<usize, usize> = $type::with_key_capacity(7);
            map.insert(0, 1);
            map.insert(0, 2);
            map.insert(0, 3);
            map.insert(1, 7);
            map.shrink_values_to_fit();
            assert!(map[&0].capacity() >= 3);
            assert!(map[&1].capacity() >= 1);
        }

        #[test]
        fn contains_key_returns_correct_value() {
            let map = $multimap_macro! {
                0 => { 1, 2, 3 },
                9 => { 2, 3 },
                333 => { 3 }
            };

            assert!(map.contains_key(&0));
            assert!(map.contains_key(&9));
            assert!(map.contains_key(&333));

            assert!(!map.contains_key(&1));
            assert!(!map.contains_key(&456));
            assert!(!map.contains_key(&7));
        }

        #[test]
        fn as_map_returns_borrowed_inner() {
            let mm = $multimap_macro! {
                0 => { 1, 2, 3 },
                9 => { 2, 3 }
            };
            let actual = mm.as_map();
            let expected = &$keys_macro! {
                0 => $values_macro! { 1, 2, 3},
                9 => $values_macro! {2, 3},
            };
            assert_eq!(expected, actual);
        }

        #[test]
        fn into_map_returns_owned_inner() {
            let mm = $multimap_macro! {
                0 => { 1, 2, 3 },
                9 => { 2, 3 }
            };
            let actual = mm.into_map();
            let expected = $keys_macro! {
                0 => $values_macro! { 1, 2, 3},
                9 => $values_macro! {2, 3},
            };
            assert_eq!(expected, actual);
        }

        #[test]
        fn extend_works_with_empty_multimap() {
            let mut actual = $multimap_macro! {};
            actual.extend(vec![(0, 1), (2, 3)]);

            let expected = $multimap_macro! {
                0 => { 1 },
                2 => { 3 }
            };
            assert_eq!(expected, actual);
        }

        #[test]
        fn extend_works_with_non_empty_multimap() {
            let mut actual = $multimap_macro! {
                0 => { 1 },
                2 => { 3 }
            };
            actual.extend(vec![(0, 2), (2, 4), (4, 5)]);
            let expected = $multimap_macro! {
                0 => { 1, 2 },
                2 => { 3, 4 },
                4 => { 5 }
            };
            assert_eq!(expected, actual);
        }

        #[test]
        fn extend_works_with_copy_iter() {
            let mut actual = $multimap_macro! {};
            // these values get copied
            actual.extend(vec![(&0, &1), (&2, &3)]);
            let expected = $multimap_macro! {
                0 => { 1 },
                2 => { 3 }
            };
            assert_eq!(expected, actual);
        }

        #[test]
        fn from_ignores_empty_sets() {
            let map = $multimap_macro! {
                0 => { 1, 2, 3 },
                9 => { },
                333 => { 3 }
            };

            assert_eq!(2, map.keys_len());
            assert_eq!(4, map.len());
            assert!(!map.contains_key(&9));

            let expected = $multimap_macro! {
                0 => { 1, 2, 3 },
                333 => { 3 }
            };
            assert_eq!(expected, map);
        }

        #[test]
        fn equality_test_fails_on_different_len() {
            let a = $multimap_macro! {0 => { 0 }};
            let b = $multimap_macro! {0 => { 0 }, 1 => { 1 }};
            assert!(!a.eq(&b))
        }

        #[test]
        fn equality_test_fails_on_same_len_but_distinct_elem_count() {
            let a = $multimap_macro! {0 => { 0 }};
            let b = $multimap_macro! {0 => { 0, 1 }};
            assert!(!a.eq(&b))
        }

        #[test]
        fn equality_test_succeeds_on_inversely_ordered_sets() {
            let a = $multimap_macro! {
                0 => { 1, 0 },
                1 => { 2, 3 }
            };
            let b = $multimap_macro! {
                1 => { 3, 2 },
                0 => { 0, 1 }
            };
            assert!(a.eq(&b))
        }

        #[test]
        fn get_and_index_give_same_result() {
            let a = $multimap_macro! {
                0 => { 1, 0, 7 },
                1 => { 2, 3 }
            };
            let index = &a[&0];
            let get = a.get(&0).unwrap();
            assert_eq!(index, &$values_macro! { 1, 0, 7});
            assert_eq!(index, get);
        }

        #[test]
        #[should_panic(expected = "no entry found for key")]
        fn index_panics_for_unknown_key() {
            let a = $multimap_macro! {
                0 => { 1, 0, 7 }
            };
            let _ = &a[&1];
        }
    };
}

mod hash_set_multimap {
    use maplit::hashmap;
    use maplit::hashset;
    use more_collections::hashsetmultimap;
    use more_collections::HashSetMultimap;

    general_multimap_tests! {HashSetMultimap, hashsetmultimap, hashmap, hashset}
    hash_multimap_tests! {HashSetMultimap}
    set_multimap_tests! {HashSetMultimap}
}

mod hash_vec_multimap {
    use maplit::hashmap;
    use more_collections::hashvecmultimap;
    use more_collections::HashVecMultimap;

    general_multimap_tests! {HashVecMultimap, hashvecmultimap, hashmap, vec}
    hash_multimap_tests! {HashVecMultimap}
}

mod index_set_multimap {
    use indexmap::indexmap;
    use indexmap::indexset;
    use more_collections::indexsetmultimap;
    use more_collections::IndexSetMultimap;

    general_multimap_tests! {IndexSetMultimap, indexsetmultimap, indexmap, indexset}
    set_multimap_tests! {IndexSetMultimap}
    index_multimap_tests! {IndexSetMultimap, indexsetmultimap, indexset}

    #[test]
    fn insert_full_returns_correct_values() {
        let mut map = indexsetmultimap! {
            0 => {1, 2, 3},
            2 => {2, 3},
            1 => {3}
        };

        // new key, new value
        assert_eq!((3, 0, true), map.insert_full(7, 2));
        assert_eq!(7, map.len());

        // existing key, new value
        assert_eq!((2, 1, true), map.insert_full(1, 2));
        assert_eq!(8, map.len());

        // existing key and value
        assert_eq!((0, 1, false), map.insert_full(0, 2));
        assert_eq!(8, map.len());
    }
}

mod index_vec_multimap {
    use indexmap::indexmap;
    use more_collections::indexvecmultimap;
    use more_collections::IndexVecMultimap;

    general_multimap_tests! {IndexVecMultimap, indexvecmultimap, indexmap, vec}
    index_multimap_tests! {IndexVecMultimap, indexvecmultimap, vec}

    #[test]
    fn insert_full_returns_correct_values() {
        let mut map = indexvecmultimap! {
            0 => {1, 2, 3},
            2 => {2, 3},
            1 => {3}
        };

        // new key, new value
        assert_eq!((3, 0), map.insert_full(7, 2));
        assert_eq!(7, map.len());

        // existing key, new value
        assert_eq!((2, 1), map.insert_full(1, 2));
        assert_eq!(8, map.len());

        // existing key and value
        assert_eq!((0, 3), map.insert_full(0, 2));
        assert_eq!(9, map.len());
    }
}
