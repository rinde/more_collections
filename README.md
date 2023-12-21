# More collections &emsp; [![Latest Version]][crates.io] [![Docs badge]][docs.rs] [![License: Apache-2.0][License badge]](/LICENSE)

[Latest Version]: https://img.shields.io/crates/v/more_collections.svg
[crates.io]: https://crates.io/crates/more_collections

[License badge]: https://img.shields.io/crates/l/more_collections.svg

[Docs badge]: https://img.shields.io/badge/docs.rs-rustdoc-yellow
[docs.rs]: https://docs.rs/more_collections/

Additional Rust collections not found in [std::collections](https://doc.rust-lang.org/std/collections/).

## Small* collections

Built on top of the excellent [smallvec](https://github.com/servo/rust-smallvec) crate, `SmallMap` and `SmallSet` are a `Map` and `Set` respectively that are inlined if they contain fewer values than a (statically chosen) capacity `C`, otherwise they are heap allocated and backed by an `IndexMap`. 

| Completion | Name           | Behaves as       |
| ---------- | -------------- | ---------------- |
| 🟩🟩🟨⬜️⬜️      | SmallMap       | `IndexMap<K, V>` |
| 🟩🟩🟨⬜️⬜️      | SmallSet       | `IndexSet<T>`    |
| ⬜️⬜️⬜️⬜️⬜️      | SmallSortedMap | `BTreeMap<K, V>` |
| ⬜️⬜️⬜️⬜️⬜️      | SmallSortedSet | `BTreeSet<T>`    |

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

[A detailed overview](doc/multimap.md).

## VecMap
| Completion | Name   | Drop-in replacement for | Implemented as   | Distinguishing features |
| ---------- | ------ | ----------------------- | ---------------- | ----------------------- |
| 🟩🟩🟩🟨⬜️      | VecMap | `IndexMap<K,V>`         | `Vec<Option<T>>` | Fast random access      |

[Click here for a detailed description and overview of the API](doc/vec_map.md).

## Multisets

| Completion | Name          | Behaves as          |
| ---------- | ------------- | ------------------- |
| ⬜️⬜️⬜️⬜️⬜️      | IndexMultiset | `IndexMap<K,usize>` |
| ⬜️⬜️⬜️⬜️⬜️      | HashMultiset  | `HashMap<K,usize>`  |
| ⬜️⬜️⬜️⬜️⬜️      | BTreeMultiset | `BTreeMap<K,usize>` |
| ⬜️⬜️⬜️⬜️⬜️      | EnumMultiset  | `EnumMap<K,usize>`  |

Some work is already done [in this PR](https://github.com/rinde/more_collections/pull/8).
