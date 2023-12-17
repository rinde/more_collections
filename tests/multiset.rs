#![cfg(test)]

macro_rules! general_multiset_tests {
    ($type:tt, $map_macro:tt) => {
        use std::collections::hash_map::RandomState;

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
        fn insert_n_works_correctly() {
            let mut ms = $type::new();
            assert_eq!(0, ms.insert_n("A", 0));
            assert!(ms.is_empty());
            assert_eq!(0, ms.count("A"));
            assert_eq!(0, ms.len());
            assert_eq!(0, ms.unique_len());

            assert_eq!(7, ms.insert_n("A", 7));
            assert_eq!(7, ms.count("A"));
            assert_eq!(7, ms.len());
            assert_eq!(1, ms.unique_len());

            assert_eq!(3, ms.insert_n("B", 3));
            assert_eq!(3, ms.count("B"));
            assert_eq!(10, ms.len());
            assert_eq!(2, ms.unique_len());

            // adding "A" again adds to previously added "A"
            assert_eq!(16, ms.insert_n("A", 9));
            assert_eq!(16, ms.count("A"));
            assert_eq!(19, ms.len());
            assert_eq!(2, ms.unique_len());

            let expected = $map_macro! {
                "A" => 16,
                "B" => 3
            };
            assert_eq!(&expected, ms.as_map());
        }

        #[test]
        fn remove_works_correctly() {
            let mut ms: $type<_, RandomState> = $type::from_tuples($map_macro! {
                "A" => 5u8,
                "B" => 7,
                "C" => 2,
            });
            assert_eq!(14, ms.len());
            assert_eq!(3, ms.unique_len());

            assert_eq!(Some((5, None)), ms.remove("A"));
            assert_eq!(4, ms.count("A"));
            assert_eq!(13, ms.len());
            assert_eq!(3, ms.unique_len());

            // remove non-existent is no-op
            assert_eq!(None, ms.remove("D"));
            assert_eq!(13, ms.len());
            assert_eq!(3, ms.unique_len());

            // remove completely returns original object
            assert_eq!(Some((2, None)), ms.remove("C"));
            assert_eq!(Some((1, Some("C"))), ms.remove("C"));
            assert_eq!(0, ms.count("C"));
            assert_eq!(11, ms.len());
            assert_eq!(2, ms.unique_len());
        }

        #[test]
        fn remove_n_works_correctly() {
            let mut ms: $type<_, RandomState> = $type::from_tuples($map_macro! {
                "A" => 5u8,
                "B" => 7,
                "C" => 2,
            });
            assert_eq!(14, ms.len());
            assert_eq!(3, ms.unique_len());

            assert_eq!(Some((5, None)), ms.remove_n("A", 3));
            assert_eq!(2, ms.count("A"));
            assert_eq!(11, ms.len());
            assert_eq!(3, ms.unique_len());

            // remove non-existent is no-op
            assert_eq!(None, ms.remove_n("D", 99));
            assert_eq!(11, ms.len());
            assert_eq!(3, ms.unique_len());

            // remove completely returns original object
            assert_eq!(Some((2, Some("C"))), ms.remove_n("C", 123));
            assert_eq!(0, ms.count("C"));
            assert_eq!(9, ms.len());
            assert_eq!(2, ms.unique_len());
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

        #[test]
        fn from_tuples() {
            let elements = vec![("A", 3u8), ("B", 2), ("A", 1), ("C", 7)];
            let multiset: $type<_, RandomState> = $type::from_tuples(elements);
            assert_eq!(10, multiset.len());
            assert_eq!(3, multiset.unique_len());
            let expected = $map_macro! {
                "A" => 1,
                "B" => 2,
                "C" => 7,
            };
            assert_eq!(&expected, multiset.as_map());
        }
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
