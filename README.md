# More collections &emsp; [![Latest Version]][crates.io] [![Docs badge]][docs.rs] [![License: Apache-2.0][License badge]](/LICENSE)

[Latest Version]: https://img.shields.io/crates/v/more_collections.svg
[crates.io]: https://crates.io/crates/more_collections

[License badge]: https://img.shields.io/crates/l/more_collections.svg

[Docs badge]: https://img.shields.io/badge/docs.rs-rustdoc-yellow
[docs.rs]: https://docs.rs/more_collections/

Additional Rust collections not found in [std::collections](https://doc.rust-lang.org/std/collections/).

## Small* collections

Built on top of the excellent [smallvec](https://github.com/servo/rust-smallvec) crate, `SmallMap` and `SmallSet` are a `Map` and `Set` respectively that are inlined if they contain fewer values than a (statically chosen) capacity `C`, otherwise they are heap allocated and backed by an `IndexMap`. 

## Multimaps 

| Completion | Name                 | Behaves as                 |
| ---------- | -------------------- | -------------------------- |
| 🟩🟩🟩🟩🟨      | HashSetMultimap      | `HashMap<K, HashSet<V>>`   |
| 🟩🟩🟩🟩🟨      | HashVecMultimap      | `HashMap<K, Vec<V>>`       |
| 🟩🟩🟩🟩🟨      | IndexSetMultimap     | `IndexMap<K, IndexSet<V>>` |
| 🟩🟩🟩🟩🟨      | IndexVecMultimap     | `IndexMap<K, Vec<V>>`      |
| ⬜️⬜️⬜️⬜️⬜️      | BTreeSetMultimap     | `BTreeMap<K, BTreeSet<V>>` |
| ⬜️⬜️⬜️⬜️⬜️      | BTreeVecMultimap     | `BTreeMap<K, Vec<V>>`      |
| ⬜️⬜️⬜️⬜️⬜️      | EnumHashSetMultimap  | `EnumMap<K, HashSet<V>>`   |
| ⬜️⬜️⬜️⬜️⬜️      | EnumIndexSetMultimap | `EnumMap<K, IndexSet<V>>`  |
| ⬜️⬜️⬜️⬜️⬜️      | EnumVecMultimap      | `EnumMap<K, Vec<V>>     `  |
| ⬜️⬜️⬜️⬜️⬜️      | EnumEnumMultimap     | `EnumMap<K, EnumSet<V>>`   |

## Multisets

| Completion | Name          | Behaves as          |
| ---------- | ------------- | ------------------- |
| ⬜️⬜️⬜️⬜️⬜️      | IndexMultiset | `IndexMap<K,usize>` |
| ⬜️⬜️⬜️⬜️⬜️      | HashMultiset  | `HashMap<K,usize>`  |
| ⬜️⬜️⬜️⬜️⬜️      | BTreeMultiset | `BTreeMap<K,usize>` |
| ⬜️⬜️⬜️⬜️⬜️      | EnumMultiset  | `EnumMap<K,usize>`  |

## Multimaps overview

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
| TODO consider adding more mutators    |
| __Traits__                            |
| Extend                                | ✅               | ✅               | ✅                | ✅                |
| FromIterator                          | ✅               | ✅               | ✅                | ✅                |
| From wrapped type                     | ✅               | ✅               | ✅                | ✅                |
| IntoIterator                          | ✅               | ✅               | ✅                | ✅                |
| Default                               | ✅               | ✅               | ✅                | ✅                |
| Index                                 | ✅               | ✅               | ✅                | ✅                |
| Eq                                    | ✅               | ✅               | ✅                | ✅                |
| PartialEq                             | ✅               | ✅               | ✅                | ✅                |
| Debug                                 | ✅               | ✅               | ✅                | ✅                |
| Clone                                 | ✅               | ✅               | ✅                | ✅                |
| Serializable                          | planned         | planned         | planned          | planned          |
| Deserializable                        | planned         | planned         | planned          | planned          |


# Todo

 - [ ] Add benches
 - [ ] Look into `no_std` compatibility
 - [ ] Look into `WASM` compatibility
 - [ ] Look into no-unsafe clippy rule
 - [ ] Add features to disable parts of the crate (e.g. dependencies should be optional)
 - [ ] Implement custom iterators following this advice: https://stackoverflow.com/questions/52079983/what-is-the-advantage-of-publishing-a-concrete-type-in-a-crates-api-instead-of
