#[doc(hidden)]
#[macro_export]
macro_rules! multiset_base_impl {
    ($inner_ty:ty) => {
        /// Creates an empty multiset.
        ///
        /// The multiset is initially created with a capacity of 0, so it will
        /// not allocate until it is first inserted into.
        #[must_use]
        pub fn new() -> Self {
            Self {
                inner: <$inner_ty>::new(),
                len: 0,
            }
        }

        /// Creates an empty multiset with the specified capacity.
        ///
        /// The multiset will be able to hold at least `capacity` _unique_ elements
        /// without reallocating. If `capacity` is 0, the multiset will not allocate.
        #[must_use]
        pub fn with_capacity(capacity: usize) -> Self {
            Self {
                inner: <$inner_ty>::with_capacity(capacity),
                len: 0,
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! multiset_base2_impl {
    ($inner_ty:tt) => {
        /// Creates an empty multiset which will use the given hash builder to hash
        /// elements.
        #[inline]
        pub fn with_hasher(hash_builder: S) -> Self {
            Self::with_capacity_and_hasher(0, hash_builder)
        }

        /// Creates an empty multiset with the specified capacity, using `hash_builder`
        /// to hash the elements.
        #[inline]
        pub fn with_capacity_and_hasher(n: usize, hash_builder: S) -> Self {
            Self {
                inner: $inner_ty::with_capacity_and_hasher(n, hash_builder),
                len: 0,
            }
        }

        /// Returns the number of unique elements the multiset can hold without
        /// reallocating.
        #[inline]
        pub fn capacity(&self) -> usize {
            self.inner.capacity()
        }

        // TODO iter_mut()

        /// Returns the number of elements in the multiset.
        ///
        /// Note that the number of elements in the multiset may not be the
        /// same as the number of _unqiue_ elements in the multiset. See
        /// [`Self::unique_len()`].
        pub const fn len(&self) -> usize {
            self.len
        }

        /// Returns `true` if the multiset contains no elements.
        pub const fn is_empty(&self) -> bool {
            self.len == 0
        }

        /// Returns the number of elements in the multiset.
        ///
        /// Note that the number of _unique_ elements in the multiset may not be the
        /// same as the total number of elements in the multiset. See
        /// [`Self::len()`].
        pub fn unique_len(&self) -> usize {
            self.inner.len()
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! multiset_mutators_impl {
    ($type:ty, $inner_ty_full:ty, $inner_ty:tt, ($($elements_ref:tt)*)) => {

        // TODO this method will have to be split out as it won't be needed in all implementations
        /// Reserve capacity for `additional` more unique elements.
        #[inline]
        pub fn reserve(&mut self, additional: usize) {
            self.inner.reserve(additional);
        }

        /// Shrinks the capacity of the multiset as much as possible.
        /// It will drop down as much as possible while maintaining the
        /// internal rules and possibly leaving some space in accordance with
        /// the resize policy.
        #[inline]
        pub fn shrink_to_fit(&mut self) {
            self.inner.shrink_to_fit();
        }

        // TODO add shrink_to for Hash*Multisets

        // TODO add entry()

        /// Count the number of occurences of an element that has been added
        /// to this multiset.
        #[inline]
        pub fn count<Q: ?Sized>(&self, element: &Q) -> usize
        where
            $($elements_ref)*,
        {
            self.inner.get(element).copied().unwrap_or_default()
        }

        /// Returns `true` if the multiset contains an element for the
        /// specified element.
        #[inline]
        pub fn contains<Q: ?Sized>(&self, element: &Q) -> bool
        where
            $($elements_ref)*,
        {
            self.inner.get(element).is_some()
        }

        // TODO add get_mut() --> only if it is possible to keep internal `len`
        // consistent

        /// Inserts the element in the multiset. Returns the number of
        /// occurences of this element *including* this newly inserted element.
        pub fn insert(&mut self, element: T) -> usize {
            self.insert_n(element, 1)
        }

        /// Inserts the element in the multiset `n` times. Returns the number
        /// of occurences of this element *including* the newly inserted
        /// elements.
        pub fn insert_n(&mut self, element: T, occurences: usize) -> usize {
            if occurences > 0 {
                self.len += occurences;
                *self.inner.entry(element)
                .and_modify(|counter| *counter += occurences)
                .or_insert(occurences)
            } else {
                self.count(&element)
            }
        }

        /// Remove a single occurence of the element from the multiset.
        ///
        /// If the multiset contains the element, one occurence will be
        /// removed. The original number of occurences is returned and if this
        /// is the last occurence of `element`, the `element` is also returned.
        #[inline]
        pub fn remove<Q: ?Sized>(&mut self, element: &Q) -> Option<(usize, Option<T>)>
        where
            $($elements_ref)*
        {
            self.remove_n( element, 1)
        }

        /// Remove the element from the multiset.
        ///
        /// If the multiset contains the element, it will be removed `n` times
        /// where `n` is the smallest value of the specified `occurrences` and
        /// the number of occurrences of `element` in the multiset. The
        /// original number of occurences is returned and if all occurrences of
        /// `element` are removed, the `element` is also returned.
        #[inline]
        pub fn remove_n<Q: ?Sized>(&mut self, element: &Q, occurrences:usize ) -> Option<(usize, Option<T>)>
        where
            $($elements_ref)*
        {
            if let Some(count) = self.inner.get_mut(element) {
               if *count <= occurrences {
                    let subtracted = std::cmp::min(*count,occurrences);
                    self.len -= subtracted;
                    Some((*count, self.inner.remove_entry(element).map(|(k,_v)|k)))
                } else {
                    let original_count = *count;
                    (*count) -= occurrences;
                    self.len -= occurrences;
                   Some((original_count, None))
                }
            } else {
                None
            }
        }

        /// Retains only the elements specified by the predicate.
        ///
        /// In other words, remove all pairs `(k, v)` such that `f(&k, &v)`
        /// returns `false`.
        // #[inline]
        // pub fn retain<F>(&mut self, f: F)
        // where
        //     F: Fn(&T) -> bool
        // {
        //     self.inner.retain(|k,elements| {
        //         elements.retain(|x| {
        //             let retain = f(k,x);
        //             if !retain {
        //                 self.len -= 1;
        //             }
        //             retain
        //         });
        //         !elements.is_empty()
        //     });
        // }

        /////////////////////////////////////
        // Multiset specific methods
        /////////////////////////////////////

        /// Create a new multiset from an iterator of tuples containing the
        /// item counts. If there are duplicate items, the _last_ entry will be
        /// used for the count.
        ///
        /// # Example
        /// ```
        /// # use std::collections::hash_map::RandomState;
        ///
        #[doc=concat!("# use more_collections::", stringify!($type), ";")]
        #[doc=concat!("let multiset: ",
            stringify!($type),
            "<_, RandomState> = ",
            stringify!($type),
            "::from_tuples(vec!["
        )]
        ///     ("A", 3u8), ("B", 2), ("A", 1), ("C", 7)
        /// ]);
        ///
        /// assert_eq!(1, multiset.count("A"));
        /// ```
        pub fn from_tuples<I, C>(iterable: I) -> Self
        where
            I: IntoIterator<Item = (T, C)>,
            C: Into<usize>,
        {
            let map = iterable
                .into_iter()
                .map(|(element, count)| (element, count.into()))
                .collect::<$inner_ty<T, usize, S>>();
            Self {
                len: map.iter().map(|(_, c)| c).sum(),
                inner: map,
            }
        }

        /// Return a borrow of the underlying map.
        pub const fn as_map(&self) -> &$inner_ty_full {
            &self.inner
        }

        /// Return the underlying map, the multiset cannot be used after
        /// calling this.
        pub fn into_map(self) -> $inner_ty_full {
            self.inner
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! multiset_common_traits_impl {
    ($type:tt, $inner_ty:tt, ($($elements:tt)*)) => {

        impl<T, S> Extend<T> for $type<T, S>
        where
            $($elements)*,
            S: BuildHasher + Default
        {
            fn extend<I: IntoIterator<Item = T>>(&mut self, iterable: I) {
                let iter = iterable.into_iter();
                let reserve = (iter.size_hint().0 + 1) / 2;
                self.reserve(reserve);
                iter.for_each(move |element| {
                    self.insert(element);
                })
            }
        }

        // impl<T, C> Extend<(T,C)> for $type<T>
        // where
        //     $($elements)*,
        //     C: Into<usize>
        // {
        //     fn extend<I: IntoIterator<Item = (T,C)>>(&mut self, iterable: I) {
        //         let iter = iterable.into_iter();
        //         let reserve = (iter.size_hint().0 + 1) / 2;
        //         self.reserve(reserve);
        //         iter.for_each(move |element| {
        //             self.insert(element);
        //         })
        //     }
        // }

        impl<T, S> FromIterator<T> for $type<T, S>
        where
            $($elements)*,
            S: BuildHasher + Default
        {
            fn from_iter<I: IntoIterator<Item = T>>(iterable: I) -> Self {
                let iter = iterable.into_iter();
                let (low, _) = iter.size_hint();
                // Expecting that about 50% of the incoming values are
                // duplicates and reserving that amount of capacity.
                let mut multiset = Self::with_capacity_and_hasher(low / 2, <_>::default());
                multiset.extend(iter);
                multiset
            }
        }

        impl<T, S> Default for $type<T, S>
        where
            $($elements)*,
            S: BuildHasher + Default
        {
            fn default() -> Self {
                Self {
                    inner: Default::default(),
                    len: 0,
                }
            }

        }
    };
}
