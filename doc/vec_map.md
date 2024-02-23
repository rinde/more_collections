# VecMap

`VecMap` is a `Vec`-backed map, for faster random access. Keys need to implement the `IndexKey` trait instead of `Hash`. A typical use case is in conjunction with newtypes that wrap an integer.

Below is an overview of all methods and traits that are implemented for `VecMap`. The list uses the methods and traits implemented by `IndexMap` as a reference. Pull requests are welcome for any not yet implemented methods and traits (unless marked as not planned).

TODO also compare with Vec methods

## Methods and traits

| Method                   | `VecMap`                     | `IndexVec`                   |
| ------------------------ | ---------------------------- | ---------------------------- |
| `new()`                  | ✅                            | ✅                            |
| `into_entries()`         | not planned                  | not planned                  |
| `as_entries()`           | not planned                  | not planned                  |
| `as_entries_mut()`       | not planned                  | not planned                  |
| `with_entries()`         | not planned                  | not planned                  |
| `with_capacity()`        | ✅                            | 🎯                            |
| `capacity()`             | ✅                            | 🎯                            |
| `len()`                  | ✅                            | 🎯                            |
| `is_empty()`             | ✅                            | 🎯                            |
| `iter()`                 | ✅                            | 🎯                            |
| `iter_mut()`             | ✅                            | 🎯                            |
| `keys()`                 | ✅                            | 🎯                            |
| `into_keys()`            |                              | 🎯                            |
| `values()`               | ✅                            | 🎯                            |
| `values_mut()`           |                              | 🎯                            |
| `into_values()`          |                              | 🎯                            |
| `clear()`                | ✅                            | 🎯                            |
| `truncate()`             |                              | 🎯                            |
| `drain()`                |                              | 🎯                            |
| `split_off()`            | not planned                  | not planned                  |
| `reserve()`              | ✅                            | 🎯                            |
| `reserve_exact()`        |                              | 🎯                            |
| `try_reserve()`          |                              | 🎯                            |
| `try_reserve_exact()`    |                              | 🎯                            |
| `shrink_to_fit()`        |                              | 🎯                            |
| `shrink_to()`            |                              | 🎯                            |
| `insert()`               | ✅                            | not planned                  |
| `insert_full()`          | not planned (redundant)      | not planned (redundant)      |
| `entry()`                | ✅                            | 🧐                            |
| `contains_key()`         | ✅                            | 🎯                            |
| `get()`                  | ✅                            | 🎯                            |
| `get_key_value()`        | not planned (redundant)      | not planned (redundant)      |
| `get_full()`             | not planned (redundant)      | not planned (redundant)      |
| `get_index_of()`         | not planned (redundant)      | not planned (redundant)      |
| `get_mut()`              | ✅                            | 🎯                            |
| `get_full_mut()`         | not planned (redundant)      | not planned (redundant)      |
| `remove()`               | ✅                            | not planned                  |
| `remove_entry()`         | not planned (redundant)      | not planned                  |
| `swap_remove()`          | not planned (not applicable) | not planned (not applicable) |
| `swap_remove_entry()`    | not planned (not applicable) | not planned (not applicable) |
| `swap_remove_full()`     | not planned (not applicable) | not planned (not applicable) |
| `shift_remove()`         | not planned (not applicable) | not planned (not applicable) |
| `shift_remove_entry()`   | not planned (not applicable) | not planned (not applicable) |
| `shift_remove_full()`    | not planned (not applicable) | not planned (not applicable) |
| `pop()`                  | ✅                            | 🎯                            |
| `retain()`               | ✅                            | not planned (not applicable) |
| `sort_keys()`            | not planned (not applicable) | not planned (not applicable) |
| `sort_by()`              | not planned (not applicable) | not planned (not applicable) |
| `sorted_by()`            | not planned (not applicable) | not planned (not applicable) |
| `sort_unstable_keys()`   | not planned (not applicable) | not planned (not applicable) |
| `sort_unstable_by()`     | not planned (not applicable) | not planned (not applicable) |
| `sort_by_cached_key()`   | not planned (not applicable) | not planned (not applicable) |
| `binary_search_keys()`   | not planned (not applicable) | not planned (not applicable) |
| `binary_search_by()`     | not planned (not applicable) | not planned (not applicable) |
| `binary_search_by_key()` | not planned (not applicable) | not planned (not applicable) |
| `partition_point()`      | not planned                  | 🧐                            |
| `reverse()`              | not planned                  | not planned (not applicable) |
| `as_slice()`             | not planned                  | 🧐                            |
| `as_mut_slice()`         | not planned                  | 🧐                            |
| `into_boxed_slice()`     | not planned                  | 🧐                            |
| `get_index()`            | not planned (redundant)      | not planned (redundant)      |
| `get_index_mut()`        | not planned (redundant)      | not planned (redundant)      |
| `get_range()`            |                              | 🎯                            |
| `get_range_mut()`        |                              | 🎯                            |
| `first()`                |                              | 🎯                            |
| `first_mut()`            |                              | 🎯                            |
| `last()`                 |                              | 🎯                            |
| `last_mut()`             |                              | 🎯                            |
| `swap_remove_index()`    | not planned (not applicable) | not planned (not applicable) |
| `shift_remove_index()`   | not planned (not applicable) | not planned (not applicable) |
| `move_index()`           | not planned (not applicable) | not planned (not applicable) |
| `swap_indices()`         | not planned (not applicable) | not planned (not applicable) |

## Traits

| Trait                        | `VecMap`     | `IndexVec`   |
| ---------------------------- | ------------ | ------------ |
| `Clone`                      | ✅            | 🎯            |
| `Debug`                      | ✅            | 🎯            |
| `Index<K>`                   | ✅            | 🎯            |
| `IndexMut<K>`                | ✅            | 🎯            |
| `Index<usize>`               | not possible | not possible |
| `IndexMut<usize>`            | not possible | not possible |
| `FromIterator<(K,V)>`        | ✅            | 🎯            |
| `From<[(K,V); N]>`           |              | 🎯            |
| `Extend<(K,V)>`              | ✅            | 🎯            |
| `Default`                    | ✅            | 🎯            |
| `PartialEq`                  | ✅            | 🎯            |
| `Eq`                         | ✅            | 🎯            |
| `IntoIterator for &Self`     | ✅            | 🎯            |
| `IntoIterator for &mut Self` | ✅            | 🎯            |
| `IntoIterator for Self`      | ✅            | 🎯            |

## Entry

| Method / trait         | `VecMap` `Entry`        |
| ---------------------- | ----------------------- |
| `or_insert()`          | ✅                       |
| `or_insert_with()`     | ✅                       |
| `or_insert_with_key()` | not planned (redundant) |
| `key()`                | not planned (redundant) |
| `index()`              | not planned (redundant) |
| `and_modify()`         | ✅                       |
| `or_default()`         | ✅                       |
| `Debug`                | ✅                       |

# Iterators

| Method / trait        | `Iter`      | `IterMut`   | `IntoIter`  | `Drain`     | `Keys`      | `IntoKeys`  | `Values`    | `ValuesMut` | `IntoValues` |
| --------------------- | ----------- | ----------- | ----------- | ----------- | ----------- | ----------- | ----------- | ----------- | ------------ |
| `as_slice()`          | not planned | not planned | not planned | not planned | not planned | not planned | not planned | not planned | not planned  |
| `as_mut_slice()`      | not planned | not planned | not planned | not planned | not planned | not planned | not planned | not planned | not planned  |
| `Iterator`            | ✅           | ✅           | ✅           |             | ✅           |             | ✅           |             |              |
| `DoubleEndedIterator` | ✅           | ✅           | ✅           |             | ✅           |             | ✅           |             |              |
| `ExactSizeIterator`   | ✅           | ✅           | ✅           |             | ✅           |             | ✅           |             |              |
| `FusedIterator`       | ✅           | ✅           | ✅           |             | ✅           |             | ✅           |             |              |
| `Clone`               | ✅           |             | ✅           |             | ✅           |             | ✅           |             |              |
| `Debug`               | ✅           | ✅           | ✅           |             | ✅           |             | ✅           |             |              |

## Serde support

| Trait         | `VecMap` |
| ------------- | -------- |
| `Serialize`   |          |
| `Deserialize` |          |

## Rayon support

| Method / trait                            | VecMap                       |
| ----------------------------------------- | ---------------------------- |
| `IntoParallelIterator for Self` Self      |                              |
| `IntoParallelIterator for &mut Self`      |                              |
| `ParallelDrainRange<usize> for &mut Self` |                              |
| `par_keys()`                              |                              |
| `par_values()`                            |                              |
| `par_eq()`                                |                              |
| `par_values_mut()`                        |                              |
| `par_sort_keys()`                         | not planned (not applicable) |
| `par_sort_by()`                           | not planned (not applicable) |
| `par_sorted_by()`                         | not planned (not applicable) |
| `par_sort_unstable_keys()`                | not planned (not applicable) |
| `par_sort_unstable_by()`                  | not planned (not applicable) |
| `par_sorted_unstable_by()`                | not planned (not applicable) |
| `par_sort_by_cached_key()`                | not planned (not applicable) |
| `FromParallelIterator<(K, V)>`            |                              |
| `ParallelExtend<(K, V)>`                  |                              |
| `ParallelExtend<(&'a K, &'a V)>`          |                              |

### Iterators 
| Method / trait            | `IntoParIter` | `ParIter` | `ParIterMut` | `ParDrain` | `ParKeys` | `ParValues` | `ParValuesMut` |
| ------------------------- | ------------- | --------- | ------------ | ---------- | --------- | ----------- | -------------- |
| `Debug`                   |               |           |              |            |           |             |                |
| `Clone`                   |               |           |              |            |           |             |                |
| `ParallelIterator`        |               |           |              |            |           |             |                |
| `IndexedParallelIterator` |               |           |              |            |           |             |                |