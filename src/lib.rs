use std::collections::HashMap;
use std::hash::Hash;

trait Groupable<K, V> {
    fn group<B: FromKeyedIterator<K, V>>(&mut self) -> B;
}

impl<K, V, I: Iterator<(K, V)>> Groupable<K, V> for I {
    fn group<B: FromKeyedIterator<K, V>>(&mut self) -> B {
        FromKeyedIterator::from_keyed_iter(self.by_ref())
    }
}

trait FromKeyedIterator<K, V> {
    fn from_keyed_iter<I: Iterator<(K, V)>>(I) -> Self;
}

impl <K: Ord + Hash, V, U: Extendable<V>> FromKeyedIterator<K, V> for HashMap<K, U> {
    fn from_keyed_iter<T: Iterator<(K, V)>>(mut iter: T) -> HashMap<K, U> {
        let mut map = HashMap::<K, U>::new();
            for (key, val) in iter {
            let val_iter = Some(val).move_iter();
            match map.find_mut(&key) {
                Some(collection) => {
                    collection.extend(val_iter);
                    continue
                },
                None => {} // insert below
            }

            map.insert(key, FromIterator::from_iter(val_iter));
        }
        map
    }
}
