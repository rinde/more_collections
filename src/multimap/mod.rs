#[macro_use]
mod create_macros;

#[macro_use]
mod gen_macros;

#[cfg(feature = "hashsetmultimap")]
pub mod hash_set_multimap;
#[cfg(feature = "hashsetmultimap")]
pub use hash_set_multimap::HashSetMultimap;

#[cfg(feature = "hashvecmultimap")]
pub mod hash_vec_multimap;
#[cfg(feature = "hashvecmultimap")]
pub use hash_vec_multimap::HashVecMultimap;

#[cfg(all(feature = "indexmap", feature = "indexsetmultimap"))]
pub mod index_set_multimap;
#[cfg(all(feature = "indexmap", feature = "indexsetmultimap"))]
pub use index_set_multimap::IndexSetMultimap;

#[cfg(all(feature = "indexmap", feature = "indexvecmultimap"))]
pub mod index_vec_multimap;
#[cfg(all(feature = "indexmap", feature = "indexvecmultimap"))]
pub use index_vec_multimap::IndexVecMultimap;
