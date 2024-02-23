
| Criterion                                       | `VecMap` | `IndexVec` | `IndexVec` (`rustc`) | [`indexed_vec`][indexed_vec] | [`safe_index`][safe_index] |
| ----------------------------------------------- | -------- | ---------- | -------------------- | ---------------------------- | -------------------------- |
| Allows constructing/customizing key type        | ✅        | ✅          | ✅                    | ✅                            | ❌                          |
| Provides macro to generate key type             | 🎯 (TODO) | 🎯 (TODO)   | ✅                    | ✅                            | ✅                          |
| Can be used as library                          | ✅        | ✅          | ❌                    | ✅                            | ✅                          |
| Key stability (key always points to same value) | ✅        | 🎯          | ✅                    | ✅                            | ❌                          |
| Has a map-like API                              | ✅        | ✅          | ❌                    | ❌                            | ❌                          |

[safe_index]:(https://docs.rs/safe_index/latest/safe_index/)
[indexed_vec]:(https://crates.io/crates/indexed_vec)