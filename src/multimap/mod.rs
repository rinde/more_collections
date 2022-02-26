#[macro_use]
mod create_macros;

#[macro_use]
mod gen_macros;

pub mod hash_set_multimap;
pub mod hash_vec_multimap;
pub mod index_set_multimap;
pub mod index_vec_multimap;

pub use hash_set_multimap::HashSetMultimap;
pub use hash_vec_multimap::HashVecMultimap;
pub use index_set_multimap::IndexSetMultimap;
pub use index_vec_multimap::IndexVecMultimap;
