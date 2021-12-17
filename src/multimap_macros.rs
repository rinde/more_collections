#[macro_export]
macro_rules! multimap_base_impl {
    ($keys:ty, $values:ty ) => {
        pub fn new() -> Self {
            Self {
                inner: <$keys>::new(),
                len: 0,
            }
        }

        pub fn with_key_capacity(capacity: usize) -> Self {
            Self {
                inner: <$keys>::with_capacity(capacity),
                len: 0,
            }
        }

        pub fn key_capacity(&self) -> usize {
            self.inner.capacity()
        }
    };
}

#[macro_export]
macro_rules! multimap_mutators_impl {
    ($keys:ty, $values:ty, $values_ctx:expr, ($($keys_ref:tt)*), ($($values_ref:tt)*)) => {

        pub fn len(&self) -> usize {
            self.len
        }

        pub fn is_empty(&self) -> bool {
            self.len == 0
        }

        pub fn keys_len(&self) -> usize {
            self.inner.len()
        }

        pub fn iter(&self) -> impl Iterator<Item = (&K, &V)> {
            self.inner.iter().flat_map(|(k, v)| repeat(k).zip(v.iter()))
        }

        pub fn with_capacity_and_hasher(n: usize, hash_builder: S) -> Self {
            Self {
                inner: <$keys>::with_capacity_and_hasher(n, hash_builder),
                len: 0,
            }
        }

        pub fn with_hasher(hash_builder: S) -> Self {
            Self::with_capacity_and_hasher(0, hash_builder)
        }

        /// Insert the value into the multimap.
        ///
        /// If an equivalent entry already exists in the multimap, it returns
        /// `false` leaving the original value in the set and without altering its
        /// insertion order. Otherwise, it inserts the new entry and returns `true`.
        pub fn insert(&mut self, key: K, value: V) -> bool {
            // TODO write procedural macro to handle different case of Vec / HashSet
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

        /// Remove the key and all associated values from the multimap.
        ///
        /// Returns the set of values if at least one value is associated to `key`,
        /// returns `None` otherwise.
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

        /// Remove the entry from the multimap, and return `true` if it was present.
        pub fn remove<Q: ?Sized, R: ?Sized>(&mut self, key: &Q, value: &R) -> bool
        where
            $($keys_ref)*,
            $($values_ref)*,
        {
            if let Some(values) = self.inner.get_mut(key) {
                if values.remove(value) {
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

        /// Return a reference to the set stored for `key`, if it is present, else
        /// `None`.
        pub fn get<Q: ?Sized>(&self, key: &Q) -> Option<&$values>
        where
            $($keys_ref)*,
        {
            self.inner.get(key)
        }

         /// Return `true` if an equivalent to `key` exists in the map.
        pub fn contains_key<Q: ?Sized>(&self, key: &Q) -> bool
        where
            $($keys_ref)*,
        {
            self.get(key).is_some()
        }

        /// Return `true` if an equivalent `key` and `value` combination exists in
        /// the multimap.
        pub fn contains<Q: ?Sized, R:?Sized>(&self, key: &Q, value: &R) -> bool
        where
            $($keys_ref)*,
            $($values_ref)*,
        {
            if let Some(values) = self.inner.get(key) {
                values.contains(value)
            } else {
                false
            }
        }

        // TODO this method will have to be split out as it won't be needed in all implementations
        /// Reserve capacity for `additional` more keys.
        pub fn reserve(&mut self, additional: usize) {
            self.inner.reserve(additional);
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
