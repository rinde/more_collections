#[macro_export]
macro_rules! multimap_base_impl {
    ($keys:ty, $values:ty ) => {
        pub fn new() -> Self {
            Self {
                inner: <$keys>::new(),
                len: 0,
            }
        }

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
    };
}

#[macro_export]
macro_rules! multimap_mutators_impl {
    ($keys:ty, $values:ty, $values_ctx:expr) => {
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

        // pub fn remove_key<Q: ?Sized>(&mut self, key: &Q) -> Option<$values>
        // where $keys_ref
        // {
        //     if let Some(inner_set) = self.inner.remove(key) {
        //         self.decrement_len(inner_set.len());
        //         Some(inner_set)
        //     } else {
        //         None
        //     }
        // }
    };
}
