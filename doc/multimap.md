# Multimaps

Below is an overview of the all methods and traits that are implemented or planned for each multimap implementation. 

## Methods

| Method                                | HashSetMultimap | HashVecMultimap | IndexSetMultimap | IndexVecMultimap |
| ------------------------------------- | --------------- | --------------- | ---------------- | ---------------- |
| new()                                 | ✅               | ✅               | ✅                | ✅                |
| with_key_capacity()                   | ✅               | ✅               | ✅                | ✅                |
| with_hasher()                         | ✅               | ✅               | ✅                | ✅                |
| with_key_capacity_and_hasher()        | ✅               | ✅               | ✅                | ✅                |
| key_capacity()                        | ✅               | ✅               | ✅                | ✅                |
| keys()                                | ✅               | ✅               | ✅                | ✅                |
| values()                              | ✅               | ✅               | ✅                | ✅                |
| values_mut()                          | maybe           | maybe           | maybe            | maybe            |
| iter()                                | ✅               | ✅               | ✅                | ✅                |
| len()                                 | ✅               | ✅               | ✅                | ✅                |
| is_empty()                            | ✅               | ✅               | ✅                | ✅                |
| keys_len()                            | ✅               | ✅               | ✅                | ✅                |
| reserve()                             | ✅               | ✅               | ✅                | ✅                |
| shrink_keys_to_fit()                  | ✅               | ✅               | ✅                | ✅                |
| shrink_values_to_fit()                | ✅               | ✅               | ✅                | ✅                |
| shrink_keys_to()                      | planned         | planned         | -                | -                |
| shrink_values_to()                    | planned         | planned         | -                | planned          |
| entry()                               | planned         | planned         | planned          | planned          |
| get()                                 | ✅               | ✅               | ✅                | ✅                |
| get_key_values()                      | ✅               | ✅               | ✅                | ✅                |
| contains_key()                        | ✅               | ✅               | ✅                | ✅                |
| get_mut()                             | maybe           | maybe           | maybe            | maybe            |
| insert()                              | ✅               | ✅               | ✅                | ✅                |
| remove_key()                          | ✅               | ✅               | ✅                | ✅                |
| remove_key_entry()                    | ✅               | ✅               | ✅                | ✅                |
| retain()                              | ✅               | ✅               | ✅                | ✅                |
| into_keys()                           | ✅               | ✅               | ✅                | ✅                |
| into_values()                         | ✅               | ✅               | ✅                | ✅                |
| remove()                              | ✅               | ✅               | ✅                | ✅                |
| contains()                            | ✅               | ✅               | ✅                | ✅                |
| as_map()                              | ✅               | ✅               | ✅                | ✅                |
| into_map()                            | ✅               | ✅               | ✅                | ✅                |
| __IndexMap keys methods__             |
| insert_full()                         | -               | -               | ✅                | ✅                |
| get_full()                            | -               | -               | ✅                | ✅                |
| get_key_index()                       | -               | -               | ✅                | ✅                |
| get_full_mut()                        | -               | -               | maybe            | maybe            |
| swap_remove()                         | -               | -               | planned          | planned          |
| swap_remove_entry()                   | -               | -               | planned          | planned          |
| swap_remove_full()                    | -               | -               | planned          | planned          |
| shift_remove()                        | -               | -               | planned          | planned          |
| shift_remove_entry()                  | -               | -               | planned          | planned          |
| shift_remove_full()                   | -               | -               | planned          | planned          |
| pop()                                 | -               | -               | planned          | planned          |
| sort_keys()                           | -               | -               | planned          | planned          |
| sort_by()                             | -               | -               | planned          | planned          |
| sorted_by()                           | -               | -               | planned          | planned          |
| reverse()                             | -               | -               | planned          | planned          |
| get_index()                           | -               | -               | planned          | planned          |
| get_index_mut()                       | -               | -               | maybe            | maybe            |
| first()                               | -               | -               | planned          | planned          |
| first_mut()                           | -               | -               | maybe            | maybe            |
| last()                                | -               | -               | planned          | planned          |
| last_mut()                            | -               | -               | maybe            | maybe            |
| swap_remove_index()                   | -               | -               | planned          | planned          |
| shift_remove_index()                  | -               | -               | planned          | planned          |
| swap_indices()                        | -               | -               | planned          | planned          |
| __Set values methods__                |
| difference()                          | planned         | -               | planned          | -                |
| symmetric_difference()                | planned         | -               | planned          | -                |
| intersection()                        | planned         | -               | planned          | -                |
| union()                               | planned         | -               | planned          | -                |
| is_disjoint()                         | planned         | -               | planned          | -                |
| is_subset()                           | planned         | -               | planned          | -                |
| is_superset()                         | planned         | -               | planned          | -                |
| __Consistent ordered values methods__ |
| sort_values()                         | -               | planned         | planned          | planned          |
| sort_values_by()                      | -               | planned         | planned          | planned          |

TODO consider adding more mutators

## Traits

| Method            | HashSetMultimap | HashVecMultimap | IndexSetMultimap | IndexVecMultimap |
| ----------------- | --------------- | --------------- | ---------------- | ---------------- |
| Extend            | ✅               | ✅               | ✅                | ✅                |
| FromIterator      | ✅               | ✅               | ✅                | ✅                |
| From wrapped type | ✅               | ✅               | ✅                | ✅                |
| IntoIterator      | ✅               | ✅               | ✅                | ✅                |
| Default           | ✅               | ✅               | ✅                | ✅                |
| Index             | ✅               | ✅               | ✅                | ✅                |
| Eq                | ✅               | ✅               | ✅                | ✅                |
| PartialEq         | ✅               | ✅               | ✅                | ✅                |
| Debug             | ✅               | ✅               | ✅                | ✅                |
| Clone             | ✅               | ✅               | ✅                | ✅                |

## Iterators
TODO fill this section in

### HashSetMultimap

| Method / trait        | `Iter` | `IterMut` | `IntoIter` | `Drain` | `Keys` | `IntoKeys` | `Values` | `ValuesMut` | `IntoValues` |
| --------------------- | ------ | --------- | ---------- | ------- | ------ | ---------- | -------- | ----------- | ------------ |
| `as_slice()`          |        |           |            |         |        |            |          |             |              |
| `as_mut_slice()`      |        |           |            |         |        |            |          |             |              |
| `Iterator`            |        |           |            |         |        |            |          |             |              |
| `DoubleEndedIterator` |        |           |            |         |        |            |          |             |              |
| `ExactSizeIterator`   |        |           |            |         |        |            |          |             |              |
| `FusedIterator`       |        |           |            |         |        |            |          |             |              |
| `Clone`               |        |           |            |         |        |            |          |             |              |
| `Debug`               |        |           |            |         |        |            |          |             |              |

### HashVecMultimap

| Method / trait        | `Iter` | `IterMut` | `IntoIter` | `Drain` | `Keys` | `IntoKeys` | `Values` | `ValuesMut` | `IntoValues` |
| --------------------- | ------ | --------- | ---------- | ------- | ------ | ---------- | -------- | ----------- | ------------ |
| `as_slice()`          |        |           |            |         |        |            |          |             |              |
| `as_mut_slice()`      |        |           |            |         |        |            |          |             |              |
| `Iterator`            |        |           |            |         |        |            |          |             |              |
| `DoubleEndedIterator` |        |           |            |         |        |            |          |             |              |
| `ExactSizeIterator`   |        |           |            |         |        |            |          |             |              |
| `FusedIterator`       |        |           |            |         |        |            |          |             |              |
| `Clone`               |        |           |            |         |        |            |          |             |              |
| `Debug`               |        |           |            |         |        |            |          |             |              |

### IndexSetMultimap

| Method / trait        | `Iter` | `IterMut` | `IntoIter` | `Drain` | `Keys` | `IntoKeys` | `Values` | `ValuesMut` | `IntoValues` |
| --------------------- | ------ | --------- | ---------- | ------- | ------ | ---------- | -------- | ----------- | ------------ |
| `as_slice()`          |        |           |            |         |        |            |          |             |              |
| `as_mut_slice()`      |        |           |            |         |        |            |          |             |              |
| `Iterator`            |        |           |            |         |        |            |          |             |              |
| `DoubleEndedIterator` |        |           |            |         |        |            |          |             |              |
| `ExactSizeIterator`   |        |           |            |         |        |            |          |             |              |
| `FusedIterator`       |        |           |            |         |        |            |          |             |              |
| `Clone`               |        |           |            |         |        |            |          |             |              |
| `Debug`               |        |           |            |         |        |            |          |             |              |

### IndexVecMultimap

| Method / trait        | `Iter` | `IterMut` | `IntoIter` | `Drain` | `Keys` | `IntoKeys` | `Values` | `ValuesMut` | `IntoValues` |
| --------------------- | ------ | --------- | ---------- | ------- | ------ | ---------- | -------- | ----------- | ------------ |
| `as_slice()`          |        |           |            |         |        |            |          |             |              |
| `as_mut_slice()`      |        |           |            |         |        |            |          |             |              |
| `Iterator`            |        |           |            |         |        |            |          |             |              |
| `DoubleEndedIterator` |        |           |            |         |        |            |          |             |              |
| `ExactSizeIterator`   |        |           |            |         |        |            |          |             |              |
| `FusedIterator`       |        |           |            |         |        |            |          |             |              |
| `Clone`               |        |           |            |         |        |            |          |             |              |
| `Debug`               |        |           |            |         |        |            |          |             |              |

## Serde support

| Trait         | `HashSetMultimap` | `HashVecMultimap` | `IndexSetMultimap` | `IndexVecMultimap` |
| ------------- | ----------------- | ----------------- | ------------------ | ------------------ |
| `Serialize`   | planned           | planned           | planned            | planned            |
| `Deserialize` | planned           | planned           | planned            | planned            |

## Rayon support

| Method / trait                            | `HashSetMultimap` | `HashVecMultimap` | `IndexSetMultimap` | `IndexVecMultimap` |
| ----------------------------------------- | ----------------- | ----------------- | ------------------ | ------------------ |
| `IntoParallelIterator for Self` Self      |                   |                   |                    |                    |
| `IntoParallelIterator for &mut Self`      |                   |                   |                    |                    |
| `ParallelDrainRange<usize> for &mut Self` |                   |                   |                    |                    |
| `par_keys()`                              |                   |                   |                    |                    |
| `par_values()`                            |                   |                   |                    |                    |
| `par_eq()`                                |                   |                   |                    |                    |
| `par_values_mut()`                        |                   |                   |                    |                    |
| `par_sort_keys()`                         |                   |                   |                    |                    |
| `par_sort_by()`                           |                   |                   |                    |                    |
| `par_sorted_by()`                         |                   |                   |                    |                    |
| `par_sort_unstable_keys()`                |                   |                   |                    |                    |
| `par_sort_unstable_by()`                  |                   |                   |                    |                    |
| `par_sorted_unstable_by()`                |                   |                   |                    |                    |
| `par_sort_by_cached_key()`                |                   |                   |                    |                    |
| `FromParallelIterator<(K, V)>`            |                   |                   |                    |                    |
| `ParallelExtend<(K, V)>`                  |                   |                   |                    |                    |
| `ParallelExtend<(&'a K, &'a V)>`          |                   |                   |                    |                    |

### Iterators 
| Method / trait            | `IntoParIter` | `ParIter` | `ParIterMut` | `ParDrain` | `ParKeys` | `ParValues` | `ParValuesMut` |
| ------------------------- | ------------- | --------- | ------------ | ---------- | --------- | ----------- | -------------- |
| `Debug`                   |               |           |              |            |           |             |                |
| `Clone`                   |               |           |              |            |           |             |                |
| `ParallelIterator`        |               |           |              |            |           |             |                |
| `IndexedParallelIterator` |               |           |              |            |           |             |                |