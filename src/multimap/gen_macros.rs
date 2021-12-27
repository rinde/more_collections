#[doc(hidden)]
#[macro_export]
macro_rules! multimap_base_impl {
    ($keys:ty) => {
        /// Creates an empty multimap.
        ///
        /// The multimap is initially created with a capacity of 0, so it will
        /// not allocate until it is first inserted into.
        pub fn new() -> Self {
            Self {
                inner: <$keys>::new(),
                len: 0,
            }
        }

        /// Creates an empty multimap` with the specified key capacity.
        ///
        /// The multimap will be able to hold at least `capacity` keys without
        /// reallocating. If `capacity` is 0, the multimap will not allocate.
        pub fn with_key_capacity(capacity: usize) -> Self {
            Self {
                inner: <$keys>::with_capacity(capacity),
                len: 0,
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! multimap_base2_impl {
    ($keys:tt) => {
        /// Creates an empty multimap which will use the given hash builder to hash
        /// keys.
        #[inline]
        pub fn with_hasher(hash_builder: S) -> Self {
            Self::with_key_capacity_and_hasher(0, hash_builder)
        }

        /// Creates an empty multimap with the specified capacity, using `hash_builder`
        /// to hash the keys.
        #[inline]
        pub fn with_key_capacity_and_hasher(n: usize, hash_builder: S) -> Self {
            Self {
                inner: $keys::with_capacity_and_hasher(n, hash_builder),
                len: 0,
            }
        }

        /// Returns the number of keys the multimap can hold without reallocating.
        #[inline]
        pub fn key_capacity(&self) -> usize {
            self.inner.capacity()
        }

        // TODO keys()
        // TODO values()
        // TODO values_mut()

        pub fn iter(&self) -> impl Iterator<Item = (&K, &V)> {
            self.inner
                .iter()
                .flat_map(|(k, v)| std::iter::repeat(k).zip(v.iter()))
        }

        // TODO iter_mut()

        /// Returns the number of elements in the multimap.
        ///
        /// Note that the number of elements in the multimap may not be the
        /// same as the number of keys in the multimap. See
        /// [Self::keys_len()].
        pub fn len(&self) -> usize {
            self.len
        }

        /// Returns `true` if the multimap contains no elements.
        pub fn is_empty(&self) -> bool {
            self.len == 0
        }

        /// Returns the number of keys in the multimap.
        ///
        /// Note that the number of keys in the multimap may not be the
        /// same as the number of elements in the multimap. See
        /// [Self::len()].
        pub fn keys_len(&self) -> usize {
            self.inner.len()
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! multimap_mutators_impl {
    ($keys:ty, $values:ty, $values_ctx:expr, $values_class:tt, ($($keys_ref:tt)*), ($($values_ref:tt)*)) => {

        // TODO this method will have to be split out as it won't be needed in all implementations
        /// Reserve capacity for `additional` more keys.
        #[inline]
        pub fn reserve(&mut self, additional: usize) {
            self.inner.reserve(additional);
        }

        // TODO add try_reserve()
        // TODO add shrink_to_fit()
        // TODO add shrink_to()
        // TODO add entry()

        #[doc = concat!("Return a reference to the ", stringify!($values_class), " stored for `key`, if it is present, else `None`.")]
        #[inline]
        pub fn get<Q: ?Sized>(&self, key: &Q) -> Option<&$values>
        where
            $($keys_ref)*,
        {
            self.inner.get(key)
        }

        // TODO add get_key_values()

        /// Returns `true` if the map contains a value for the specified key.
        #[inline]
        pub fn contains_key<Q: ?Sized>(&self, key: &Q) -> bool
        where
            $($keys_ref)*,
        {
            self.get(key).is_some()
        }

        // TODO add get_mut() --> only if it is possible to keep internal `len` consistent

        crate::insert!($values_class $values_ctx);

        /// Remove the key and all associated values from the multimap.
        ///
        /// Returns values if at least one value is associated to `key`,
        /// returns `None` otherwise.
        #[inline]
        pub fn remove_key<Q: ?Sized>(&mut self, key: &Q) -> Option<$values>
        where
            $($keys_ref)*
        {
            if let Some(values) = self.inner.remove(key) {
                self.len -= values.len();
                Some(values)
            } else {
                None
            }
        }

        // TODO add remove_entry()
        // TODO add retain()
        // TODO add into_keys()
        // TODO add into_values()

        //////////////////////////////////////
        /// Multimap specific methods
        //////////////////////////////////////

        /// Remove the entry from the multimap, and return `true` if it was present.
        pub fn remove<Q: ?Sized, R: ?Sized>(&mut self, key: &Q, value: &R) -> bool
        where
            $($keys_ref)*,
            $($values_ref)*,
        {
            if let Some(values) = self.inner.get_mut(key) {
                if crate::values_remove!($values_class, values, value) {
                    if values.is_empty() {
                        self.inner.remove(key);
                    }
                    self.len -= 1;
                    true
                } else {
                    false
                }
            } else {
                false
            }
        }

        /// Return `true` if an equivalent `key` and `value` combination exists in
        /// the multimap.
        pub fn contains<Q: ?Sized, R:?Sized>(&self, key: &Q, value: &R) -> bool
        where
            $($keys_ref)*,
            $($values_ref)*,
        {
            if let Some(values) = self.inner.get(key) {
                crate::values_contains!($values_class, values, value)
            } else {
                false
            }
        }
    };
}

//////////////////////////////////////
/// IndexMap keys specific methods
//////////////////////////////////////

// TODO add insert_full()
// TODO add get_full()
// TODO add get_index_of()
// TODO add get_full_mut()
// TODO add swap_remove()
// TODO add swap_remove_entry()
// TODO add swap_remove_full()
// TODO add shift_remove()
// TODO add shift_remove_entry()
// TODO add shift_remove_full()
// TODO add pop()
// TODO add sort_keys()
// TODO add sort_by()
// TODO add sorted_by()
// TODO add reverse()

// TODO add get_index()
// TODO add get_index_mut()
// TODO add first()
// TODO add first_mut()
// TODO add last()
// TODO add last_mut()
// TODO add swap_remove_index()
// TODO add shift_remove_index()
// TODO add swap_indices()

//////////////////////////////////////
/// *Set values specific methods
//////////////////////////////////////

// TODO add difference()
// TODO add symmetric_difference()
// TODO add intersection()
// TODO add union()
// TODO add is_disjoint()
// TODO add is_subset()
// TODO add is_superset()

//////////////////////////////////////
/// Vec / IndexSet values specific methods
//////////////////////////////////////

// TODO add sort_values()
// TODO add sort_values_by()
// TODO consider adding more mutators

#[doc(hidden)]
#[macro_export]
macro_rules! insert {
    (set $values_ctx:expr) => {
        /// Insert the value into the multimap.
        ///
        /// If an equivalent entry already exists in the multimap, it returns
        /// `false` leaving the original value in the set and without altering its
        /// insertion order. Otherwise, it inserts the new entry and returns `true`.
        #[inline]
        pub fn insert(&mut self, key: K, value: V) -> bool {
            if self
                .inner
                .entry(key)
                .or_insert_with(|| $values_ctx)
                .insert(value)
            {
                self.len += 1;
                true
            } else {
                false
            }
        }
    };

    (vec $values_ctx:expr) => {
        /// Insert the value into the multimap.
        ///
        /// Allows duplicates.
        #[inline]
        pub fn insert(&mut self, key: K, value: V) {
            self.inner
                .entry(key)
                .or_insert_with(|| $values_ctx)
                .push(value);
            self.len += 1;
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! values_contains {
    (set, $values:ident, $value:ident) => {
        $values.contains($value)
    };

    (vec, $values:ident, $value:ident) => {
        $values.iter().find(|x| $value.equivalent(x)).is_some()
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! values_remove {
    (set, $values:ident, $value:ident) => {
        $values.remove($value)
    };

    (vec, $values:ident, $value:ident) => {
        if let Some(index) = $values.iter().position(|x| $value.equivalent(x)) {
            $values.remove(index);
            true
        } else {
            false
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! multimap_extend {
    ($type:tt, ($($generic_ids:tt)*), $inner_keys_type:tt, $inner_values_type:ty, ($($keys:tt)*), ($($values:tt)*), ($($keys_get:tt)*) )=> {
        impl<$($generic_ids)*> Extend<(K, V)> for $type<$($generic_ids)*>
        where
            $($keys)*,
            $($values)*,
            S: BuildHasher + Default,
        {
            fn extend<I: IntoIterator<Item = (K, V)>>(&mut self, iterable: I) {
                let iter = iterable.into_iter();
                let reserve = (iter.size_hint().0 + 1) / 2;
                self.reserve(reserve);
                iter.for_each(move |(k, v)| {
                    self.insert(k, v);
                });
            }
        }

        impl<'a, $($generic_ids)*> Extend<(&'a K, &'a V)> for $type<$($generic_ids)*>
        where
            $($keys)* + Copy,
            $($values)* + Copy,
            S: BuildHasher + Default,
        {
            fn extend<I: IntoIterator<Item = (&'a K, &'a V)>>(&mut self, iterable: I) {
                self.extend(iterable.into_iter().map(|(&key, &value)| (key, value)));
            }
        }

        impl<$($generic_ids)*> std::iter::FromIterator<(K, V)> for $type<$($generic_ids)*>
        where
            $($keys)*,
            $($values)*,
            S: BuildHasher + Default,
        {
            fn from_iter<I: IntoIterator<Item = (K, V)>>(iterable: I) -> Self {
                let iter = iterable.into_iter();
                let (low, _) = iter.size_hint();
                // TODO this resizing has a high chance of over provisioning
                let mut map = Self::with_key_capacity_and_hasher(low, <_>::default());
                map.extend(iter);
                map
            }
        }

        impl<$($generic_ids)*> From<$inner_keys_type<K,$inner_values_type,S>> for $type<$($generic_ids)*>
        where
            $($keys)*,
            $($values)*,
            S: BuildHasher + Default,
        {
            fn from(mut map: $inner_keys_type<K,$inner_values_type,S>) -> Self {
                map.retain(|_k, v| !v.is_empty());
                let len = map.iter().map(|(_k, v)| v.len()).sum();
                $type { inner: map, len }
            }
        }

        impl<$($generic_ids)*> Default for $type<$($generic_ids)*>
        where
            S: Default,
        {
            /// Creates an empty multimap, with the `Default` value for the hasher.
            #[inline]
            fn default() -> $type<$($generic_ids)*> {
                $type::with_hasher(Default::default())
            }
        }

        impl<K, Q: ?Sized, V, S> std::ops::Index<&Q> for $type<$($generic_ids)*>
        where
            $($keys_get)*,
            $($values)*,
            S: BuildHasher + Default,
        {
            type Output = $inner_values_type;

            /// Returns a reference to the values container corresponding to the supplied key.
            ///
            /// # Panics
            ///
            /// Panics if the key is not present in the multimap.
            #[inline]
            fn index(&self, key: &Q) -> &$inner_values_type {
                self.get(key).expect("no entry found for key")
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! multimap_eq {
    ($type:tt, ($($values_generics:tt)*)) => {
        impl<K, V1, S1, V2, S2> PartialEq<$type<K, V2, S2>> for $type<K, V1, S1>
        where
            K: Hash + Eq,
            V1: $($values_generics)* + PartialEq<V2> + Borrow<V2>,
            V2: $($values_generics)* + PartialEq<V1> + Borrow<V1>,
            S1: BuildHasher + Default,
            S2: BuildHasher + Default,
        {
            fn eq(&self, other: &$type<K, V2, S2>) -> bool {
                if self.len() != other.len() {
                    return false;
                }
                self.iter().all(|(key, value)| other.contains(key, value))
            }
        }

        impl<K, V, S> Eq for $type<K, V, S>
        where
            K: Eq + Hash,
            V: $($values_generics)*,
            S: BuildHasher + Default,
        {
        }
    };
}

// TODO create macro for IndexMap specific functions:
// Return item index, if it exists in the map.
//  pub fn get_index_of_key<Q: ?Sized>(&self, key: &Q) -> Option<usize>
//  where
//      Q: Hash + Equivalent<K>,
//  {
//      if self.is_empty() {
//          None
//      } else {
//          self.inner.get_index_of(key)
//      }
//  }
