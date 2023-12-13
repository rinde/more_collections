# VecMap

Below is an overview of the all methods and traits that are implemented for `VecMap`. The list uses `IndexMap` as a reference.

## Methods and traits

| Method                 | VecMap                       |
| ---------------------- | ---------------------------- |
| new()                  | ✅                            |
| into_entries()         | not planned                  |
| as_entries()           | not planned                  |
| as_entries_mut()       | not planned                  |
| with_entries()         | not planned                  |
| with_capacity()        | ✅                            |
| capacity()             |                              |
| len()                  | ✅                            |
| is_empty()             | ✅                            |
| iter()                 | ✅                            |
| iter_mut()             |                              |
| keys()                 | ✅                            |
| into_keys()            |                              |
| values()               |                              |
| values_mut()           |                              |
| into_values()          |                              |
| clear()                | ✅                            |
| truncate()             |                              |
| drain()                |                              |
| split_off()            |                              |
| reserve()              |                              |
| reserve_exact()        |                              |
| try_reserve()          |                              |
| try_reserve_exact()    |                              |
| shrink_to_fit()        |                              |
| shrink_to()            |                              |
| insert()               | ✅                            |
| insert_full()          | not planned (redundant)      |
| entry()                | ✅                            |
| contains_key()         | ✅                            |
| get()                  | ✅                            |
| get_key_value()        | not planned (redundant)      |
| get_full()             | not planned (redundant)      |
| get_index_of()         | not planned (redundant)      |
| get_mut()              | ✅                            |
| get_full_mut()         | not planned (redundant)      |
| remove()               | ✅                            |
| remove_entry()         |                              |
| swap_remove()          | not planned (not applicable) |
| swap_remove_entry()    | not planned (not applicable) |
| swap_remove_full()     | not planned (not applicable) |
| shift_remove()         | not planned (not applicable) |
| shift_remove_entry()   | not planned (not applicable) |
| shift_remove_full()    | not planned (not applicable) |
| pop()                  |                              |
| retain()               |                              |
| retain_mut()           |                              |
| sort_keys()            | not planned (not applicable) |
| sort_by()              | not planned (not applicable) |
| sorted_by()            | not planned (not applicable) |
| sort_unstable_keys()   | not planned (not applicable) |
| sort_unstable_by()     | not planned (not applicable) |
| sort_by_cached_key()   | not planned (not applicable) |
| binary_search_keys()   | not planned (not applicable) |
| binary_search_by()     | not planned (not applicable) |
| binary_search_by_key() | not planned (not applicable) |
| partition_point()      | not planned                  |
| reverse()              | not planned                  |
| as_slice()             | not planned                  |
| as_mut_slice()         | not planned                  |
| into_boxed_slice()     | not planned                  |
| get_index()            | not planned (redundant)      |
| get_index_mut()        | not planned (redundant)      |
| get_range()            |                              |
| get_range_mut()        |                              |
| first()                |                              |
| first_mut()            |                              |
| last()                 |                              |
| last_mut()             |                              |
| swap_remove_index()    | not planned (not applicable) |
| shift_remove_index()   | not planned (not applicable) |
| move_index()           | not planned (not applicable) |
| swap_indices()         | not planned (not applicable) |

## Traits

| Trait               | VecMap |
| ------------------- | ------ |
| Clone               | ✅      |
| Debug               | ✅      |
| Index<K>            | ✅      |
| IndexMut<K>         |        |
| Index<usize>        |        |
| IndexMut<usize>     |        |
| FromIterator<(K,V)> | ✅      |
| From<[(K,V); N]>    |        |
| Extend<(K,V)>       |        |
| Default             | ✅      |
| PartialEq           | ✅      |
| Eq                  | ✅      |
| IntoIterator        | ✅      |

## Entry

| Method / trait       | VecMap Entry |
| -------------------- | ------------ |
| or_insert()          | ✅            |
| or_insert_with()     | ✅            |
| or_insert_with_key() |              |
| key()                |              |
| index()              |              |
| and_modify()         | ✅            |
| or_default()         | ✅            |
| Debug                |              |

# Iterators

| Method / trait      | Iter        | IterMut     | IntoIter    | Drain       | Keys        | IntoKeys    | Values      | ValuesMut   | IntoValues  |
| ------------------- | ----------- | ----------- | ----------- | ----------- | ----------- | ----------- | ----------- | ----------- | ----------- |
| as_slice()          | not planned | not planned | not planned | not planned | not planned | not planned | not planned | not planned | not planned |
| as_mut_slice()      | not planned | not planned | not planned | not planned | not planned | not planned | not planned | not planned | not planned |
| Iterator            | ✅           |             | ✅           |             | ✅           |             |             |             |             |
| DoubleEndedIterator | ✅           |             |             |             |             |             |             |             |             |
| ExactSizeIterator   | ✅           |             | ✅           |             |             |             |             |             |             |
| FusedIterator       | ✅           |             |             |             |             |             |             |             |             |
| Clone               | ✅           |             |             |             |             |             |             |             |             |
| Copy                | ✅           |             |             |             |             |             |             |             |             |
| Debug               | ✅           |             |             |             |             |             |             |             |             |
| Default             |             |             |             |             |             |             |             |             |             |