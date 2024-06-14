#[macro_use]
mod gen_macros;

pub mod hash_multiset;
pub mod index_multiset;

pub use hash_multiset::HashMultiset;
pub use index_multiset::IndexMultiset;
