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

macro_rules! general_multimap_tests {
    ($type:tt) => {
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
            // TODO make this generic
            // assert_eq!(Some(&indexset! {"A1".to_string()}), map.get(&0));

            assert!(map.remove(&0, &"A1".to_string()));
            assert!(!map.contains(&0, &"A1".to_string()));
            assert_eq!(0, map.len());
            assert_eq!(0, map.keys_len());
            assert!(map.is_empty());
            assert_eq!(None, map.get(&0));
        }
    };
}

mod hash_set_multimap {
    use collections::multimap::HashSetMultimap;

    set_multimap_tests! {HashSetMultimap}

    general_multimap_tests! {HashSetMultimap}
}

mod hash_vec_multimap {
    use collections::multimap::HashVecMultimap;

    general_multimap_tests! {HashVecMultimap}
}

mod index_set_multimap {
    use collections::multimap::IndexSetMultimap;

    set_multimap_tests! {IndexSetMultimap}

    general_multimap_tests! {IndexSetMultimap}
}

mod index_vec_multimap {
    use collections::multimap::IndexVecMultimap;

    general_multimap_tests! {IndexVecMultimap}
}
