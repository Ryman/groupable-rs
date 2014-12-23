#![feature(macro_rules)]
use std::collections::{HashMap, VecMap};
use std::hash::Hash;

/// Conversion from an `Iterator` of pairs.
pub trait Groupable<K, V> {
    /// Loops through the entire iterator, grouping all keys into a container
    /// implementing `FromKeyedIterator` with a container of values per key.
    /// The values will be aggregated per key into a container implementing
    /// `Extendable` for the value type.
    ///
    /// # Example
    ///
    /// ```rust
    /// use std::collections::HashMap;
    /// use groupable::Groupable;
    ///
    /// let evens = range(0u, 10).map(|i| (i % 2 == 0, i))
    ///                         .group::<HashMap<bool, Vec<uint>>>();
    ///
    /// assert_eq!(evens[true].as_slice(), [0, 2, 4, 6, 8].as_slice());
    /// assert_eq!(evens[false].as_slice(), [1, 3, 5, 7, 9].as_slice());
    /// ```
    fn group<B: FromKeyedIterator<K, V>>(&mut self) -> B;
}

impl<K, V, I: Iterator<(K, V)>> Groupable<K, V> for I {
    fn group<B: FromKeyedIterator<K, V>>(&mut self) -> B {
        FromKeyedIterator::from_keyed_iter(self.by_ref())
    }
}

/// Conversion from an `Iterator` of key-value pairs.
pub trait FromKeyedIterator<K, V> {
    /// Build a container with groups of elements from an external iterator.
    fn from_keyed_iter<I: Iterator<(K, V)>>(I) -> Self;
}

macro_rules! group_into(
    ($iter:ident, $map:ident) => (
        for (key, val) in $iter {
            let val_iter = Some(val).into_iter();
            match $map.find_mut(&key) {
                Some(collection) => {
                    collection.extend(val_iter);
                    continue
                },
                None => {} // insert below
            }

            $map.insert(key, FromIterator::from_iter(val_iter));
        }
    )
);

macro_rules! impl_keyed_iter (
    ($name:ident: $($bounds:ident),+) => (
        impl <K: $($bounds+)+, V, U: Extend<V> + FromIterator<V>> FromKeyedIterator<K, V> for $name<K, U> {
            fn from_keyed_iter<T: Iterator<(K, V)>>(mut iter: T) -> $name<K, U> {
                let mut map = $name::<K, U>::new();
                group_into!(iter, map);
                map
            }
        }
    )
);

macro_rules! impl_uint_keyed_iter (
    ($name:ident) => (
        impl <V, U: Extend<V> + FromIterator<V>> FromKeyedIterator<uint, V> for $name<U> {
            fn from_keyed_iter<T: Iterator<(uint, V)>>(mut iter: T) -> $name<U> {
                let mut map = $name::<U>::new();
                group_into!(iter, map);
                map
            }
        }
    )
);

impl_keyed_iter!(HashMap: Ord, Hash);
impl_uint_keyed_iter!(VecMap);
