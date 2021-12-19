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
    ($type:tt) => {
        #[test]
        fn with_capacity_constructs_instance_with_correct_capacity() {
            let map7: $type<usize, usize> = $type::with_key_capacity(7);
            let map17: $type<usize, usize> = $type::with_key_capacity(35);
            assert_eq!(7, map7.key_capacity());
            assert_eq!(35, map17.key_capacity());
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
    ($type:tt, $multimap_macro:tt) => {
        #[test]
        fn remove_removes_key_when_needed() {
            let data = vec![(0, "A1".to_string()), (0, "A2".to_string())];
            let mut map = data.into_iter().collect::<$type<_, _>>();
            assert_eq!(2, map.len());
            assert_eq!(1, map.keys_len());
            assert!(!map.is_empty());

            assert!(map.remove(&0, &"A2".to_string()));
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

            assert!(map.remove(&0, &"A1".to_string()));
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
        fn remove_is_noop_when_key_value_is_not_there() {
            let data = vec![(0, "A1".to_string()), (0, "A2".to_string())];
            let mut map = data.into_iter().collect::<$type<_, _>>();
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
    };
}

mod hash_set_multimap {
    use collections::hashsetmultimap;
    use collections::multimap::HashSetMultimap;

    general_multimap_tests! {HashSetMultimap, hashsetmultimap}
    hash_multimap_tests! {HashSetMultimap}
    set_multimap_tests! {HashSetMultimap}
}

mod hash_vec_multimap {
    use collections::hashvecmultimap;
    use collections::multimap::HashVecMultimap;

    general_multimap_tests! {HashVecMultimap, hashvecmultimap}
    hash_multimap_tests! {HashVecMultimap}
}

mod index_set_multimap {
    use collections::indexsetmultimap;
    use collections::multimap::IndexSetMultimap;

    general_multimap_tests! {IndexSetMultimap, indexsetmultimap}
    set_multimap_tests! {IndexSetMultimap}
    index_multimap_tests! {IndexSetMultimap}
}

mod index_vec_multimap {
    use collections::indexvecmultimap;
    use collections::multimap::IndexVecMultimap;

    general_multimap_tests! {IndexVecMultimap, indexvecmultimap}
    index_multimap_tests! {IndexVecMultimap}
}
