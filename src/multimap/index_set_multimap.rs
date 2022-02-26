use std::borrow::Borrow;
use std::collections::hash_map::RandomState;
use std::hash::BuildHasher;
use std::hash::Hash;

use indexmap::Equivalent;
use indexmap::IndexMap;
use indexmap::IndexSet;

/// Multimap implementation that behaves like `IndexMap<K, IndexSet<V>>`.
#[derive(Debug, Clone)]
pub struct IndexSetMultimap<K, V, S = RandomState> {
    inner: IndexMap<K, IndexSet<V, S>, S>,
    len: usize,
}

impl<K, V> IndexSetMultimap<K, V> {
    multimap_base_impl! {IndexMap<K, IndexSet<V>>}
}

impl<K, V, S> IndexSetMultimap<K, V, S> {
    multimap_base2_impl! {IndexMap}
}

impl<K, V, S> IndexSetMultimap<K, V, S>
where
    K: Hash + Eq,
    V: Hash + Eq,
    S: BuildHasher + Default,
{
    multimap_mutators_impl! {
        IndexMap<K, IndexSet<V,S>, S>,
        IndexSet<V,S>,
        IndexSet::with_hasher(S::default()),
        set,
        (Q: Hash + Equivalent<K>),
        (R: Hash + Equivalent<V>)
    }
    index_multimap_impl! {
        IndexMap<K, IndexSet<V,S>, S>,
        IndexSet<V,S>,
        IndexSet::with_hasher(S::default()),
        set,
        (Q: Hash + Equivalent<K>),
        (R: Hash + Equivalent<V>)
    }
}

multimap_extend! {
    IndexSetMultimap,
    (K, V, S),
    IndexMap,
    IndexSet<V,S>,
    (K: Hash + Eq),
    (V: Hash + Eq),
    (K: Hash + Eq, Q: Hash + Equivalent<K>)
}
multimap_eq! { IndexSetMultimap, (Hash + Eq)}

impl_into_iterator! {
    IndexSetMultimap,
    (K,V,S),
    indexmap::map::IntoIter<K, IndexSet<V, S>>,
    indexmap::set::IntoIter<V>
}

// /// An owning iterator over the entries of a multimap.
// pub struct IntoIter<K, V, S> {
//     outer: indexmap::map::IntoIter<K, IndexSet<V, S>>,
//     inner: Option<(K, indexmap::set::IntoIter<V>)>,
//     len: usize,
// }

// impl<K, V, S> Iterator for IntoIter<K, V, S>
// where
//     K: Clone,
// {
//     type Item = (K, V);

//     fn next(&mut self) -> Option<Self::Item> {
//         if let Some((current_key, inner_iter)) = &mut self.inner {
//             let next = inner_iter.next();

//             if let Some(next_value) = next {
//                 Some((current_key.clone(), next_value))
//             } else {
//                 if let Some((key, values)) = self.outer.next() {
//                     let mut new_inner_iter = values.into_iter();
//                     let v = new_inner_iter.next().unwrap();
//                     self.inner = Some((key.clone(), new_inner_iter));

//                     Some((key, v))
//                 } else {
//                     None
//                 }
//             }
//         } else {
//             None
//         }
//     }
// }

// impl<K: Clone, V, S> ExactSizeIterator for IntoIter<K, V, S> {
//     fn len(&self) -> usize {
//         self.len
//     }
// }

// impl<K: Clone, V, S> FusedIterator for IntoIter<K, V, S> {}

// impl<K: Clone, V, S> IntoIterator for IndexSetMultimap<K, V, S> {
//     type Item = (K, V);

//     type IntoIter = IntoIter<K, V, S>;

//     fn into_iter(self) -> Self::IntoIter {
//         let mut iter = self.inner.into_iter();
//         let inner = iter.next().map(|(k, v)| (k, v.into_iter()));
//         IntoIter {
//             outer: iter,
//             inner,
//             len: self.len,
//         }
//     }
// }
