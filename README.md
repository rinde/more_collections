# More collections

Rust crate with additional collections not found in [std::collections](https://doc.rust-lang.org/std/collections/).

## Multimaps 

| Status | Name                 | Behaves like               |
| ------ | -------------------- | -------------------------- |
| 🔷      | HashSetMultimap      | `HashMap<K, HashSet<V>>`   |
| 🔷      | HashVecMultimap      | `HashMap<K, Vec<V>>`       |
| 🔷      | IndexSetMultimap     | `IndexMap<K, IndexSet<V>>` |
| 🔷      | IndexVecMultimap     | `IndexMap<K, Vec<V>>`      |
| 💡      | SortedSetMultimap    | `BTreeMap<K, BTreeSet<V>>` |
| 💡      | SortedVecMultimap    | `BTreeMap<K, Vec<V>>`      |
| 💡      | EnumHashSetMultimap  | `EnumMap<K, HashSet<V>>`   |
| 💡      | EnumIndexSetMultimap | `EnumMap<K, IndexSet<V>>`  |
| 💡      | EnumVecMultimap      | `EnumMap<K, Vec<V>>     `  |
| 💡      | EnumEnumMultimap     | `EnumMap<K, EnumSet<V>>`   |

## Multisets

| Status | Name          | Keys     |
| ------ | ------------- | -------- |
| 💡      | IndexMultiset | IndexMap |
| 💡      | HashMultiset  | HashMap  |
| 💡      | EnumMultiset  | EnumMap  |
| 💡      | UsizeMap      | Vec      |

## Legend
| Symbol | Meaning        |
| ------ | -------------- |
| ✅      | completed      |
| 🔷      | testing        |
| 🟡      | in development |
| 💡      | ideation phase |
