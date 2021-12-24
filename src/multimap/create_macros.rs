#[macro_export]
macro_rules! indexsetmultimap {
    (@single $($x:tt)*) => (());
    (@count $($rest:expr),*) => (<[()]>::len(&[$(indexsetmultimap!(@single $rest)),*]));

    ($($key:expr => {$($value:expr),* },)+) => { indexsetmultimap!($($key => $($value,)* ),+) };
    ($($key:expr => {$($value:expr),* }),*) => {
        {
            let _cap = indexsetmultimap!(@count $($key),*);
            let mut _map = indexmap::IndexMap::with_capacity(_cap);
            $(
                let _ = _map.insert($key, indexmap::indexset!{$( $value, )*});
            )*
            IndexSetMultimap::from(_map)
        }
    };
}

#[macro_export]
macro_rules! indexvecmultimap {
    (@single $($x:tt)*) => (());
    (@count $($rest:expr),*) => (<[()]>::len(&[$(indexvecmultimap!(@single $rest)),*]));

    ($($key:expr => {$($value:expr),* },)+) => { indexvecmultimap!($($key => $($value,)* ),+) };
    ($($key:expr => {$($value:expr),* }),*) => {
        {
            let _cap = indexvecmultimap!(@count $($key),*);
            let mut _map = indexmap::IndexMap::with_capacity(_cap);
            $(
                let _ = _map.insert($key, vec!{$( $value, )*});
            )*
            IndexVecMultimap::from(_map)
        }
    };
}

#[macro_export]
macro_rules! hashvecmultimap {
    (@single $($x:tt)*) => (());
    (@count $($rest:expr),*) => (<[()]>::len(&[$(hashvecmultimap!(@single $rest)),*]));

    ($($key:expr => {$($value:expr),* },)+) => { hashvecmultimap!($($key => $($value,)* ),+) };
    ($($key:expr => {$($value:expr),* }),*) => {
        {
            let _cap = hashvecmultimap!(@count $($key),*);
            let mut _map = std::collections::HashMap::with_capacity(_cap);
            $(
                let _ = _map.insert($key, vec!{$( $value, )*});
            )*
            HashVecMultimap::from(_map)
        }
    };
}

#[macro_export]
macro_rules! hashsetmultimap {
    (@single $($x:tt)*) => (());
    (@count $($rest:expr),*) => (<[()]>::len(&[$(hashsetmultimap!(@single $rest)),*]));

    ($($key:expr => {$($value:expr),* },)+) => { hashsetmultimap!($($key => $($value,)* ),+) };
    ($($key:expr => {$($value:expr),* }),*) => {
        {
            let _cap = hashsetmultimap!(@count $($key),*);
            let mut _map = std::collections::HashMap::with_capacity(_cap);
            $(
                let _ = _map.insert($key, maplit::hashset!{$( $value, )*});
            )*
            HashSetMultimap::from(_map)
        }
    };
}
