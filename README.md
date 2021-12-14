# Collections

| Symbol | Meaning        |
| ------ | -------------- |
| ✅      | completed      |
| 🟡      | in development |
| 💡      | ideation phase |
## Multimaps 

| Status | Name                 | Keys     | Values   |
| ------ | -------------------- | -------- | -------- |
| 🟡      | HashSetMultimap      | HashMap  | HashSet  |
| 🟡      | HashVecMultimap      | HashMap  | Vec      |
| 🟡      | IndexSetMultimap     | IndexMap | IndexSet |
| 🟡      | IndexVecMultimap     | IndexMap | Vec      |
| 💡      | SortedSetMultimap    | BTreeMap | BTreeSet |
| 💡      | SortedVecMultimap    | BTreeMap | Vec      |
| 💡      | EnumHashSetMultimap  | EnumMap  | HashSet  |
| 💡      | EnumIndexSetMultimap | EnumMap  | IndexSet |
| 💡      | EnumVecMultimap      | EnumMap  | Vec      |
| 💡      | EnumEnumMultimap     | EnumMap  | EnumSet  |

## Multisets

| Status | Name          | Keys     |
| ------ | ------------- | -------- |
| 💡      | IndexMultiset | IndexMap |
| 💡      | HashMultiset  | HashMap  |
| 💡      | EnumMultiset  | EnumMap  |
| 💡      | UsizeMap      | Vec      |


| Collection        | remove signature                                                                                                                                                |
| ----------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| HashSetMultimap   | `fn remove<Q: ?Sized, R: ?Sized>(&mut self, key: &Q, value: &R) -> bool`<br>`where`<br>`K: Borrow<Q>,`<br>`Q: Hash + Eq,`<br>`V: Borrow<R>,`<br>`R: Hash + Eq,` |
| HashVecMultimap   | `fn remove<Q: ?Sized, R: ?Sized>(&mut self, key: &Q, value: &R) -> bool`<br>`where`<br>`K: Borrow<Q>,`<br>`Q: Hash + Eq,`<br>`V: Borrow<R>,`<br>`R: Eq,`        |
| IndexSetMultimap  | `fn remove<Q: ?Sized, R: ?Sized>(&mut self, key: &Q, value: &R) -> bool`<br>`where`<br>`Q: Hash + Equivalent<K>,`<br>`R: Hash + Equivalent<V>`                  |
| IndexVecMultimap  | `fn remove<Q: ?Sized, R: ?Sized>(&mut self, key: &Q, value: &R) -> bool`<br>`where`<br>`Q: Hash + Equivalent<K>,`<br>`R: Equivalent<V>`                         |
| SortedSetMultimap | `fn remove<Q: ?Sized, R: ?Sized>(&mut self, key: &Q, value: &R) -> bool`<br>`where`<br>`K: Borrow<Q> + Ord,`<br>`Q: Ord,`<br>`V: Borrow<R> + Ord,`<br>`R: Ord`  |
| EnumHashSetMultimap | `fn remove<Q: ?Sized, R: ?Sized>(&mut self, key: &K, value: &R) -> bool`<br>`where`<br>`V: Borrow<R> + Ord,`<br>`R: Ord`  |

