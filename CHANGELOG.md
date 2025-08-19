# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.14.4](https://github.com/rinde/more_collections/compare/v0.14.3...v0.14.4) - 2025-08-15

### Added

- specify msrv ([#100](https://github.com/rinde/more_collections/pull/100))

### Other

- bump the cargo group with 2 updates ([#118](https://github.com/rinde/more_collections/pull/118))
- bump the cargo group with 2 updates ([#117](https://github.com/rinde/more_collections/pull/117))
- bump the cargo group across 1 directory with 4 updates ([#114](https://github.com/rinde/more_collections/pull/114))
- update Rust to 1.88.0 ([#116](https://github.com/rinde/more_collections/pull/116))
- bump the cargo group across 1 directory with 4 updates ([#110](https://github.com/rinde/more_collections/pull/110))
- bump the cargo group with 3 updates ([#107](https://github.com/rinde/more_collections/pull/107))
- bump derive_more from 1.0.0 to 2.0.1 in the cargo group ([#106](https://github.com/rinde/more_collections/pull/106))
- bump serde_json from 1.0.137 to 1.0.138 in the cargo group across 1 directory ([#105](https://github.com/rinde/more_collections/pull/105))
- update `rand` to 0.9.0 ([#104](https://github.com/rinde/more_collections/pull/104))
- update Rust to 1.84.0 ([#103](https://github.com/rinde/more_collections/pull/103))
- bump the cargo group with 2 updates ([#101](https://github.com/rinde/more_collections/pull/101))
- bump the cargo group with 2 updates ([#98](https://github.com/rinde/more_collections/pull/98))
- bump serde from 1.0.215 to 1.0.216 in the cargo group ([#96](https://github.com/rinde/more_collections/pull/96))
- bump indexmap from 2.6.0 to 2.7.0 in the cargo group ([#94](https://github.com/rinde/more_collections/pull/94))
# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.14.3](https://github.com/rinde/more_collections/compare/v0.14.2...v0.14.3) - 2024-11-22

### Other

- bump serde from 1.0.214 to 1.0.215 in the cargo group ([#91](https://github.com/rinde/more_collections/pull/91))
- bump serde from 1.0.213 to 1.0.214 in the cargo group ([#90](https://github.com/rinde/more_collections/pull/90))
- bump the cargo group with 2 updates ([#89](https://github.com/rinde/more_collections/pull/89))
- bump derive_more from 0.99.18 to 1.0.0 in the cargo group ([#86](https://github.com/rinde/more_collections/pull/86))

## [0.14.2](https://github.com/rinde/more_collections/compare/v0.14.1...v0.14.2) - 2024-10-09

### Other

- Bump the cargo-dependencies group with 5 updates ([#80](https://github.com/rinde/more_collections/pull/80))

## [0.14.1](https://github.com/rinde/more_collections/compare/v0.14.0...v0.14.1) - 2024-09-16

### Added

- add serde support for VecMap ([#65](https://github.com/rinde/more_collections/pull/65))

### Other

- Remove unnecessary `Clone` bounds ([#67](https://github.com/rinde/more_collections/pull/67))

## [0.14.0](https://github.com/rinde/more_collections/compare/v0.13.6...v0.14.0) - 2024-06-14

### Changed
- [**breaking**] add blanket impl for IndexKey instead of macro on all usigned ints ([#64](https://github.com/rinde/more_collections/pull/64))
- [**breaking**] `IndexSetMultimap.remove_*` and `IndexVecMultimap.remove_*` have been removed in favor of `*.shift_remove_*` and `*.swap_remove_*` ([#63](https://github.com/rinde/more_collections/pull/63))

### Other
- Update rust to 1.79.0 ([#61](https://github.com/rinde/more_collections/pull/61))

## [0.13.6](https://github.com/rinde/more_collections/compare/v0.13.5...v0.13.6) - 2024-04-15

### Other
- bump deps ([#59](https://github.com/rinde/more_collections/pull/59))

## 0.13.5
 - [Relax `PartialEq` bound](https://github.com/rinde/more_collections/pull/53) by [Felerius](https://github.com/Felerius)

## 0.13.4
 - [Make functions `const` where possible](https://github.com/rinde/more_collections/pull/50)
 - [Enable pedantic lints](https://github.com/rinde/more_collections/pull/52/files) which adds `Debug` implementations to all types, improves docs, and more.

## 0.13.3
 - Bug fix: [Fix docs visibility](https://github.com/rinde/more_collections/pull/47) by [Felerius](https://github.com/Felerius)

## 0.13.2
 - Bug fix: [Correct `Eq` for `VecMap`](https://github.com/rinde/more_collections/pull/46) by [Felerius](https://github.com/Felerius)

## 0.13.0
 - Add `Extend` impl to `VecMap`

## 0.12.0
 - Extend `vecmap![]` macro to also support `vecmap!["".to_string(); 7]` syntax.

## 0.11.0
 - Introduce `VecMap` a `IndexMap`-like collection backed by a `Vec`.

## 0.10.0
 - `SmallMap` add `contains_key()`, `DoubleEndedIterator` for `Iter`.
 - `SmallSet` add `IntoIterator`, `difference()`, `symmetric_difference()`, `intersection()`, `union()`,  `contains()`, and `DoubleEndedIterator` for `Iter`.
 - bump `indexmap` to 2.1.0
 - bump `smallvec` to 1.11.2

## 0.9.0
 - Add `Clone` to `iter()`, `keys()`, and `values()` for all multimaps.

## 0.8.1
 - Fix bug such that `SmallMap::from_iter()` removes duplicate keys, also when inline

## 0.8.0
 - Add `or_insert()` to `Entry` of `SmallMap`
 - Bump deps

## 0.7.0
 - Add `get_index()` to `Index*Multimap`s by [@jankeu](https://github.com/jankeu)
 - Update Rust version and dependencies by [@jankeu](https://github.com/jankeu)

## 0.6.1
 - Bugfix: [Correct partial eq bounds](https://github.com/rinde/more_collections/pull/18) by [Fabian Braun](https://github.com/fabian-braun).
 - Bump `IndexMap` dependency to 1.9.3

## 0.6.0
 - `SmallSet`: add `insert_full()`
 - `SmallMap`: add `insert_full()` and return value for `entry().or_insert()`
 - `SmallMap`: relax type requirements for `Index` and `IndexMut`

## 0.5.1
 - improve docs

## 0.5.0
 - add `SmallMap` and `SmallSet` implementations
 - bump `IndexMap` dependency to 1.9.2

## 0.4.0
 - bump `IndexMap` dependency to 1.9.1

## 0.3.0
 - move macros into respective files

## 0.2.0
 - add `Index` implementation
 - add crate features to selectively enable implementations
 - add iterators

## 0.1.0
 - initial release
