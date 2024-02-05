use crate::vec_map::dense::DenseVecMap;

#[doc(hidden)]
#[macro_export]
macro_rules! vecmap_base_impl {
    ($name:ident, ($($value_bound:tt)*), $wrap_elem:expr) => {
        impl<K: IndexKey, $($value_bound)*> $name<K, V> {
            #[doc = concat!("Initializes [`", stringify!($name),"`] with ")]
            #[doc = "capacity to hold exactly `n` elements in the index range of `0..n`."]
            pub fn with_capacity(n: usize) -> Self {
                Self {
                    data: vec![Default::default(); n],
                    len: 0,
                    _marker: PhantomData,
                }
            }

            #[doc = concat!("Initializes [`", stringify!($name),"`] with `n` occurences of `elem`")]
            pub fn from_elem(elem: V, n: usize) -> Self {
                Self {
                    data: vec![$wrap_elem(elem); n],
                    len: n,
                    _marker: PhantomData,
                }
            }

            #[doc = concat!("Clears all data from the [`", stringify!($name),"`] without changing the capacity.")]
            pub fn clear(&mut self) {
                self.len = 0;
                self.data = vec![Default::default(); self.capacity()];
            }

            /// Reserve capacity for `additional` key-value pairs.
            pub fn reserve(&mut self, additional: usize) {
                self.data.extend(vec![Default::default(); additional]);
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! vecmap_base2_impl {
    ($name:ident, ($($value_bound:tt)*), $wrap_elem:expr) => {
        impl<K: IndexKey, $($value_bound)*> $name<K, V> {
            /// Initializes an empty map.
            ///
            /// For performance reasons it's almost always better to avoid dynamic
            /// resizing by using [`Self::with_capacity()`] instead.
            pub const fn new() -> Self {
                Self {
                    data: vec![],
                    len: 0,
                    _marker: PhantomData,
                }
            }

            /// Returns the number of elements the map can hold without reallocating.
            ///
            /// The index range of items that the map can hold without reallocating is
            /// `0..capacity`.
            pub fn capacity(&self) -> usize {
                self.data.len()
            }
        }
    };
}
