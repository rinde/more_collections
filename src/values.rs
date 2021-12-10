use indexmap::IndexSet;
use std::hash::BuildHasher;
use std::hash::Hash;

pub trait InnerValues<T> {
    // fn with_hasher(hash_builder: S) -> Self;

    fn insert(&mut self, value: T) -> bool;

    fn remove(&mut self, value: &T) -> bool;

    fn contains(&self, value: &T) -> bool;

    fn len(&self) -> usize;

    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<T, S> InnerValues<T> for IndexSet<T, S>
where
    T: Hash + Eq,
    S: BuildHasher + Default,
{
    // fn with_hasher(hash_builder: S) -> Self {
    //     IndexSet::with_hasher(hash_builder)
    // }

    fn insert(&mut self, value: T) -> bool {
        IndexSet::insert(self, value)
    }

    fn remove(&mut self, value: &T) -> bool {
        IndexSet::remove(self, value)
    }

    fn is_empty(&self) -> bool {
        IndexSet::is_empty(self)
    }

    fn contains(&self, value: &T) -> bool {
        IndexSet::contains(&self, value)
    }

    fn len(&self) -> usize {
        IndexSet::len(&self)
    }
}

impl<T> InnerValues<T> for Vec<T>
where
    T: PartialEq,
{
    // fn with_hasher(_hash_builder: S) -> Self {
    //     vec![]
    // }

    fn insert(&mut self, value: T) -> bool {
        self.push(value);
        true
    }

    fn remove(&mut self, value: &T) -> bool {
        if let Some(index) = self.iter().position(|x| x == value) {
            self.remove(index);
            true
        } else {
            false
        }
    }

    fn is_empty(&self) -> bool {
        Vec::is_empty(&self)
    }

    fn contains(&self, value: &T) -> bool {
        self.iter().any(|x| x == value)
    }

    fn len(&self) -> usize {
        Vec::len(&self)
    }
}
