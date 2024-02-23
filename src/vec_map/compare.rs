#![cfg(test)]

use indexed_vec::Idx;
use indexed_vec::IndexVec;

#[test]
fn t() {
    MyIndex::new(7);

    let mut m = MyMap::new();
    m.push(9);
    m[MyIndex::new(2)] = 8;
}

safe_index::new! {
    /// My index
    MyIndex,

    /// My amazing map
    map: MyMap
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
struct MyIndexType(u16);

impl Idx for MyIndexType {
    fn new(v: usize) -> Self {
        MyIndexType(v as u16)
    }

    fn index(self) -> usize {
        self.0 as usize
    }
}

#[test]
fn t2() {
    let mut v = IndexVec::new();

    v.push(8);

    let res = v[MyIndexType::new(0)];

    v.push_with_idx(|i| 8);

    // v.indices()

    v.iter_enumerated().for_each(|(k, v)| {});
}
