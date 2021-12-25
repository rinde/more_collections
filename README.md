# More collections

Rust crate with additional collections not found in [std::collections](https://doc.rust-lang.org/std/collections/).

## Multimaps 

| Completion | Name                 | Behaves as                 |
| ---------- | -------------------- | -------------------------- |
| 🟩🟩🟨⬜️⬜️      | HashSetMultimap      | `HashMap<K, HashSet<V>>`   |
| 🟩🟩🟨⬜️⬜️      | HashVecMultimap      | `HashMap<K, Vec<V>>`       |
| 🟩🟩🟨⬜️⬜️      | IndexSetMultimap     | `IndexMap<K, IndexSet<V>>` |
| 🟩🟩🟨⬜️⬜️      | IndexVecMultimap     | `IndexMap<K, Vec<V>>`      |
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

