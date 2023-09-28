# 0.7.0
 - Add `get_index()` to `Index*Multimap`s by [@jankeu](https://github.com/jankeu)
 - Update Rust version and dependencies by [@jankeu](https://github.com/jankeu)

# 0.6.1
 - Bugfix: [Correct partial eq bounds](https://github.com/rinde/more_collections/pull/18) by [Fabian Braun](https://github.com/fabian-braun).
 - Bump `IndexMap` dependency to 1.9.3

# 0.6.0
 - `SmallSet`: add `insert_full()`
 - `SmallMap`: add `insert_full()` and return value for `entry().or_insert()`
 - `SmallMap`: relax type requirements for `Index` and `IndexMut`

# 0.5.1
 - improve docs

# 0.5.0
 - add `SmallMap` and `SmallSet` implementations
 - bump `IndexMap` dependency to 1.9.2

# 0.4.0
 - bump `IndexMap` dependency to 1.9.1

# 0.3.0
 - move macros into respective files
# 0.2.0
 - add `Index` implementation
 - add crate features to selectively enable implementations
 - add iterators
 
# 0.1.0
 - initial release