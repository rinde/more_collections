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

        /// Creates an empty multimap with the specified key capacity.
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

        // TODO values_mut()

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

        /// Shrinks the capacity of the multimap's keys as much as possible.
        /// It will drop down as much as possible while maintaining the
        /// internal rules and possibly leaving some space in accordance with
        /// the resize policy.
        #[inline]
        pub fn shrink_keys_to_fit(&mut self) {
            self.inner.shrink_to_fit();
        }

        /// Shrinks the capacity of the multimap's values as much as possible.
        /// It will drop down as much as possible while maintaining the
        /// internal rules and possibly leaving some space in accordance with
        /// the resize policy.
        pub fn shrink_values_to_fit(&mut self) {
            self.inner.iter_mut().for_each(|(_,values)| values.shrink_to_fit());
        }

        // TODO add shrink_keys_to for Hash*Multimaps
        // TODO add shrink_values_to for Vec, HashSet

        // TODO add entry()

        #[doc = concat!("Return a reference to the ", stringify!($values_class), " stored for `key`, if it is present, else `None`.")]
        #[inline]
        pub fn get<Q: ?Sized>(&self, key: &Q) -> Option<&$values>
        where
            $($keys_ref)*,
        {
            self.inner.get(key)
        }

        /// Return references to the key-values pair stored for `key`, if it is
        /// present, else `None`.
        pub fn get_key_values<Q: ?Sized>(&self, key: &Q) -> Option<(&K, &$values)>
        where
            $($keys_ref)*,
        {
            self.inner.get_key_value(key)
        }

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

        /// Removes the key and all associated values from the multimap.
        ///
        /// Returns the entry (key and all associated values) if at least one
        /// value is associated to `key`, returns `None` otherwise.
        #[inline]
        pub fn remove_key_entry<Q: ?Sized>(&mut self, key: &Q) -> Option<(K, $values)>
        where
            $($keys_ref)*
        {
            if let Some((key, values)) = self.inner.remove_entry(key) {
                self.len -= values.len();
                Some((key, values))
            } else {
                None
            }
        }

        /// Retains only the elements specified by the predicate.
        ///
        /// In other words, remove all pairs `(k, v)` such that `f(&k, &v)`
        /// returns `false`.
        #[inline]
        pub fn retain<F>(&mut self, f: F)
        where
            F: Fn(&K, &V) -> bool
        {
            self.inner.retain(|k,values| {
                values.retain(|x| {
                    let retain = f(k,x);
                    if !retain {
                        self.len -= 1;
                    }
                    retain
                });
                !values.is_empty()
            });
        }

        //////////////////////////////////////
        /// Multimap specific methods
        //////////////////////////////////////

        /// Remove the entry from the multimap, and return it if it was present.
        pub fn remove<Q: ?Sized, R: ?Sized>(&mut self, key: &Q, value: &R) -> Option<V>
        where
            $($keys_ref)*,
            $($values_ref)*,
        {
            if let Some(values) = self.inner.get_mut(key) {
                let value = crate::values_remove!($values_class, values, value);
                if value.is_some() {
                    if values.is_empty() {
                        self.inner.remove(key);
                    }
                    self.len -= 1;
                }
                value
            } else {
                None
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

        /// Return a borrow of the underlying map.
        pub fn as_map(&self) -> &$keys {
            &self.inner
        }

        /// Return the underlying map, the multimap cannot be used after
        /// calling this.
        pub fn into_map(self) -> $keys {
            self.inner
        }
    };
}

//////////////////////////////////////
// IndexMap keys specific methods
//////////////////////////////////////

#[doc(hidden)]
#[macro_export]
macro_rules! index_multimap_impl {
    ($keys:ty, $values:ty, $values_ctx:expr, $values_class:tt, ($($keys_ref:tt)*), ($($values_ref:tt)*)) => {

        insert_full!($values_class $values_ctx);

        /// Return item index, key, and values.
        pub fn get_full<Q: ?Sized>(&self, key: &Q) -> Option<(usize, &K, &$values)>
        where
            $($keys_ref)*,
        {
            self.inner.get_full(key)
        }

        /// Return key index if it exists in the map.
        pub fn get_key_index<Q: ?Sized>(&self, key: &Q) -> Option<usize>
        where
            $($keys_ref)*,
        {
            if self.is_empty() {
                None
            } else {
                self.inner.get_index_of(key)
            }
        }
    };
}

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
macro_rules! insert_full {
    (set $values_ctx:expr) => {
        /// Insert a key-value pair in the multimap, and get its indices.
        ///
        /// If an equivalent key already exists in the multimap, the key
        /// remains and retains its place in the order. Additionally, if an
        /// equivalent value already exists for that particular key the value
        /// remains and retains its place in the order, and `false` is
        /// returned.
        ///
        /// If no equivalent key existed in the multimap the new key is
        /// inserted last in order. If no equivalent value existed in the
        /// multimap for this key, the value is inserted last in order.
        ///
        /// Returns `(key index, values index, success)` where success is
        /// `true` if the multimap changes as a result of calling this method.
        pub fn insert_full(&mut self, key: K, value: V) -> (usize, usize, bool) {
            match self.inner.get_full_mut(&key) {
                Some((keys_index, _, values)) => {
                    let (values_index, success) = values.insert_full(value);
                    if success {
                        self.len += 1;
                    }
                    (keys_index, values_index, success)
                }
                None => {
                    let mut values = $values_ctx;
                    values.insert(value);
                    let (keys_index, _) = self.inner.insert_full(key, values);
                    self.len += 1;
                    (keys_index, 0, true)
                }
            }
        }
    };
    (vec $values_ctx:expr) => {
        /// Insert a key-value pair in the multimap, and get its indices.
        ///
        /// If an equivalent key already exists in the multimap, the key
        /// remains and retains its place in the order. If no equivalent key
        /// existed in the multimap the new key is inserted last in order.
        ///
        /// The value is inserted last in order in the values for this
        /// particular key (duplicates are allowed).
        ///
        /// Returns `(key index, values index)`
        pub fn insert_full(&mut self, key: K, value: V) -> (usize, usize) {
            match self.inner.get_full_mut(&key) {
                Some((keys_index, _, values)) => {
                    let values_index = values.len();
                    values.push(value);
                    self.len += 1;
                    (keys_index, values_index)
                }
                None => {
                    let values = vec![value];
                    let (keys_index, _) = self.inner.insert_full(key, values);
                    self.len += 1;
                    (keys_index, 0)
                }
            }
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
        $values.take($value)
    };

    (vec, $values:ident, $value:ident) => {
        $values
            .iter()
            .position(|x| $value.equivalent(x))
            .map(|index| $values.remove(index))
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

#[doc(hidden)]
#[macro_export]
macro_rules! impl_iter {
    ($type:tt, ($($generic_ids:tt)*), $outer_iter:ty, $inner_iter:ty) => {
        /// An iterator over the entries of a multimap.
        ///
        /// This struct is created by the `iter` method on multimap.
        pub struct Iter<'a, $($generic_ids)*> where K: 'a, V: 'a{
            outer: $outer_iter,
            inner: Option<(&'a K, $inner_iter)>,
            len: usize,
        }

        impl<'a, $($generic_ids)*> Iterator for Iter<'a, $($generic_ids)*> {
            type Item = (&'a K, &'a V);

            fn next(&mut self) -> Option<Self::Item> {
                if let Some((current_key, inner_iter)) = &mut self.inner {
                    let next = inner_iter.next();

                    if let Some(next_value) = next {
                        Some((current_key, next_value))
                    } else {
                        if let Some((key, values)) = self.outer.next() {
                            let mut new_inner_iter = values.iter();
                            let v = new_inner_iter.next().unwrap();
                            self.inner = Some((key, new_inner_iter));

                            Some((key, v))
                        } else {
                            None
                        }
                    }
                } else {
                    None
                }
            }
        }

        impl<'a, $($generic_ids)*> ExactSizeIterator for Iter<'a, $($generic_ids)*> {
            fn len(&self) -> usize {
                self.len
            }
        }

        impl<'a, $($generic_ids)*> std::iter::FusedIterator for Iter<'a, $($generic_ids)*> {}

        /// An iterator over the values of a multimap.
        ///
        /// This `struct` is created by the `values` method on multimap.
        pub struct Values<'a, $($generic_ids)*>{
            inner: Iter<'a, $($generic_ids)*>,
        }

        impl<'a, $($generic_ids)*> Iterator for Values<'a, $($generic_ids)*>
        {
            type Item = &'a V;

            fn next(&mut self) -> Option<Self::Item> {
                self.inner.next().map( |(_,v)| v)
            }
        }

        impl<'a, $($generic_ids)*> ExactSizeIterator for Values<'a, $($generic_ids)*> {
            fn len(&self) -> usize {
                self.inner.len()
            }
        }

        impl<'a, $($generic_ids)*> std::iter::FusedIterator for Values<'a, $($generic_ids)*> {}

        impl<K, V, S> $type<K, V, S> {
            /// Return an iterator over the key-value pairs of the multimap.
            pub fn iter(&self) -> Iter<'_, $($generic_ids)*> {
                let mut iter = self.inner.iter();
                let inner = iter.next().map(|(k, v)| (k, v.iter()));
                Iter {
                    outer: iter,
                    inner,
                    len: self.len,
                }
            }

            /// Return an iterator over the values of the multimap.
            pub fn values(&self) -> Values<'_, $($generic_ids)*> {
                Values {
                    inner: self.iter(),
                }
            }
        }
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! impl_keys {
    ($type:tt, ($($generic_ids:tt)*), $inner_iter:ty) => {
        /// An iterator over the keys of a multimap.
        ///
        /// This `struct` is created by the `keys` method on multimap.
        pub struct Keys<'a, $($generic_ids)*> {
            inner: $inner_iter,
        }

        impl<'a, $($generic_ids)*> Iterator for Keys<'a, $($generic_ids)*> {
            type Item = &'a K;

            fn next(&mut self) -> Option<Self::Item> {
                self.inner.next()
            }
        }

        impl<'a, $($generic_ids)*> ExactSizeIterator for Keys<'a, $($generic_ids)*> {
            fn len(&self) -> usize {
                self.inner.len()
            }
        }

        impl<'a, $($generic_ids)*> std::iter::FusedIterator for Keys<'a, $($generic_ids)*> {}

        impl<K, V, S> $type<K, V, S> {
            /// Return an iterator over the keys of the multimap.
            pub fn keys(&self) -> Keys<'_, $($generic_ids)*> {
                Keys {
                    inner: self.inner.keys(),
                }
            }
        }
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! impl_into_iterator {
    ($type:tt, ($($generic_ids:tt)*), $outer_iter:ty, $inner_iter:ty) => {
        /// An owning iterator over the entries of a multimap.
        pub struct IntoIter<$($generic_ids)*> {
            outer: $outer_iter,
            inner: Option<(K, $inner_iter)>,
            len: usize,
        }

        impl<$($generic_ids)*> Iterator for IntoIter<$($generic_ids)*>
        where
            K: Clone,
        {
            type Item = (K, V);

            fn next(&mut self) -> Option<Self::Item> {
                if let Some((current_key, inner_iter)) = &mut self.inner {
                    let next = inner_iter.next();

                    if let Some(next_value) = next {
                        Some((current_key.clone(), next_value))
                    } else {
                        if let Some((key, values)) = self.outer.next() {
                            let mut new_inner_iter = values.into_iter();
                            let v = new_inner_iter.next().unwrap();
                            self.inner = Some((key.clone(), new_inner_iter));

                            Some((key, v))
                        } else {
                            None
                        }
                    }
                } else {
                    None
                }
            }
        }

        impl<$($generic_ids)*> ExactSizeIterator for IntoIter<$($generic_ids)*>
        where
            K: Clone,
        {
            fn len(&self) -> usize {
                self.len
            }
        }

        impl<$($generic_ids)*> std::iter::FusedIterator for IntoIter<$($generic_ids)*>
        where
           K: Clone,
        {}

        impl<K, V, S> IntoIterator for $type<K, V, S>
        where
            K: Clone,
        {
            type Item = (K, V);

            type IntoIter = IntoIter<$($generic_ids)*>;

            fn into_iter(self) -> Self::IntoIter {
                let mut iter = self.inner.into_iter();
                let inner = iter.next().map(|(k, v)| (k, v.into_iter()));
                IntoIter {
                    outer: iter,
                    inner,
                    len: self.len,
                }
            }
        }

        /// An owning iterator over the values of a multimap.
        ///
        /// This `struct` is created by the `into_values` method on multimap.
        pub struct IntoValues<$($generic_ids)*> {
            outer: $outer_iter,
            inner: Option<$inner_iter>,
            len: usize,
        }

        impl<$($generic_ids)*> Iterator for IntoValues<$($generic_ids)*>        {
            type Item = V;

            fn next(&mut self) -> Option<Self::Item> {
                if let Some(inner_iter) = &mut self.inner {
                    let next = inner_iter.next();

                    if let Some(next_value) = next {
                        Some(next_value)
                    } else {
                        if let Some((_key, values)) = self.outer.next() {
                            let mut new_inner_iter = values.into_iter();
                            let v = new_inner_iter.next().unwrap();
                            self.inner = Some(new_inner_iter);
                            Some(v)
                        } else {
                            None
                        }
                    }
                } else {
                    None
                }
            }
        }

        impl<$($generic_ids)*> ExactSizeIterator for IntoValues<$($generic_ids)*> {
            fn len(&self) -> usize {
                self.len
            }
        }

        impl<$($generic_ids)*> std::iter::FusedIterator for IntoValues<$($generic_ids)*> {}

        impl<K, V, S> $type<K, V, S> {
            /// Return an iterator over the values of the multimap.
            pub fn into_values(self) -> IntoValues<$($generic_ids)*> {
                let mut iter = self.inner.into_iter();
                let inner = iter.next().map(|(_k, v)| v.into_iter());
                IntoValues {
                    outer: iter,
                    inner,
                    len: self.len,
                }
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! impl_into_keys {
    ($type:tt, ($($generic_ids:tt)*), $inner_iter:ty) => {
        /// An owning iterator over the keys of a multimap.
        ///
        /// This `struct` is created by the `into_keys` method on multimap.
        pub struct IntoKeys<$($generic_ids)*> {
            inner: $inner_iter,
        }

        impl<$($generic_ids)*> Iterator for IntoKeys<$($generic_ids)*> {
            type Item = K;

            fn next(&mut self) -> Option<Self::Item> {
                self.inner.next()
            }
        }

        impl<$($generic_ids)*> ExactSizeIterator for IntoKeys<$($generic_ids)*> {
            fn len(&self) -> usize {
                self.inner.len()
            }
        }

        impl<$($generic_ids)*> std::iter::FusedIterator for IntoKeys<$($generic_ids)*> {}

        impl<K, V, S> $type<K, V, S> {
            /// Return an owning iterator over the keys of the multimap.
            pub fn into_keys(self) -> IntoKeys<$($generic_ids)*> {
                IntoKeys {
                    inner: self.inner.into_keys(),
                }
            }
        }
    }
}
