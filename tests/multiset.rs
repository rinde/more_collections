#![cfg(test)]

macro_rules! general_multiset_tests {
    ($type:tt, $map_macro:tt) => {
        #[test]
        fn insert_works_correctly() {
            let mut ms = $type::new();
            assert_eq!(1, ms.insert("A"));
            assert_eq!(1, ms.len());
            assert_eq!(2, ms.insert("A"));
            assert_eq!(2, ms.count("A"));
            assert_eq!(1, ms.unique_len());
            let expected = $map_macro! {
                "A" => 2
            };
            assert_eq!(&expected, ms.as_map());
        }

        #[test]
        fn from_iter_t() {
            let elements = vec![0, 1, 7, 0, 7, 9, 7];
            let multiset = elements.iter().copied().collect::<$type<_>>();
            assert_eq!(7, multiset.len());
            assert_eq!(4, multiset.unique_len());
            let expected = $map_macro! {
                0 => 2,
                1 => 1,
                7 => 3,
                9 => 1,
            };
            assert_eq!(&expected, multiset.as_map());
        }

        // #[test]
        // fn from_tuples() {
        //     let elements = vec![("A", 3u8), ("B", 2), ("A", 1), ("C", 7)];
        //     let multiset = $type::from_tuples(elements);
        //     println!("{multiset:?}");
        //     assert_eq!(10, multiset.len());
        //     assert_eq!(3, multiset.unique_len());
        //     // let expected = $map_macro! {
        //     //     0 => 2,
        //     //     1 => 1,
        //     //     7 => 3,
        //     //     9 => 1,
        //     // };
        //     // assert_eq!(&expected, multiset.as_map());
        // }
    };
}

mod hash_multiset {
    use maplit::hashmap;
    use more_collections::HashMultiset;

    general_multiset_tests!(HashMultiset, hashmap);
}

mod index_multiset {
    use indexmap::indexmap;
    use more_collections::IndexMultiset;

    general_multiset_tests!(IndexMultiset, indexmap);
}
