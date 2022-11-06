#[doc(hidden)]
#[macro_export]
macro_rules! multiset_base_impl {
    ($inner_ty:ty) => {
        /// Creates an empty multiset.
        ///
        /// The multiset is initially created with a capacity of 0, so it will
        /// not allocate until it is first inserted into.
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

        /// Returns the number of unique elements the multiset can hold without reallocating.
        #[inline]
        pub fn capacity(&self) -> usize {
            self.inner.capacity()
        }

        // TODO iter_mut()

        /// Returns the number of elements in the multiset.
        ///
        /// Note that the number of elements in the multiset may not be the
        /// same as the number of _unqiue_ elements in the multiset. See
        /// [Self::unique_len()].
        pub fn len(&self) -> usize {
            self.len
        }

        /// Returns `true` if the multiset contains no elements.
        pub fn is_empty(&self) -> bool {
            self.len == 0
        }

        /// Returns the number of elements in the multiset.
        ///
        /// Note that the number of _unique_ elements in the multiset may not be the
        /// same as the total number of elements in the multiset. See
        /// [Self::len()].
        pub fn unique_len(&self) -> usize {
            self.inner.len()
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! multiset_mutators_impl {
    ($inner_ty:ty, ($($elements_ref:tt)*)) => {

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

        /// Returns `true` if the multiset contains anelement for the specified element.
        #[inline]
        pub fn contains<Q: ?Sized>(&self, element: &Q) -> bool
        where
            $($elements_ref)*,
        {
            self.inner.get(element).is_some()
        }

        // TODO add get_mut() --> only if it is possible to keep internal `len` consistent

        pub fn insert() {
            // TODO implement insert
        }

        /// Remove the element from the multiset.
        ///
        /// If the multiset contains the element, it will be removed `n` times
        /// where `n` is the smallest value of the specified `occurrences` and
        /// the number of occurrences of `element` in the multiset. The
        /// original number of occurences is returned and if all occurrences of
        /// `element` are removed, the `element` is also returned.
        #[inline]
        pub fn remove<Q: ?Sized>(&mut self, element: &Q, occurrences:usize ) -> Option<(usize, Option<T>)>
        where
            $($elements_ref)*
        {
            if let Some(count) = self.inner.get_mut(element) {
               if *count == occurrences {
                    self.len -= *count;
                    Some((occurrences, self.inner.remove_entry(element).map(|(k,_v)|k)))
                } else {
                    let subtracted = std::cmp::min(*count,occurrences);
                    let original_count = *count;
                    (*count) -= subtracted;
                    self.len -= subtracted;
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

        //////////////////////////////////////
        /// Multiset specific methods
        //////////////////////////////////////

        /// Return a borrow of the underlying map.
        pub fn as_map(&self) -> &$inner_ty {
            &self.inner
        }

        /// Return the underlying map, the multiset cannot be used after
        /// calling this.
        pub fn into_map(self) -> $inner_ty {
            self.inner
        }
    };
}
