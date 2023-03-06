use std::{collections::BTreeMap, ops::RangeBounds};

use crate::IndexedVector;

/// A simple implementation of `IndexedVector` using `BTreeMap`.
/// It also supports range search.
pub struct BTreeIndexedVector<K, V> {
    map: BTreeMap<K, Vec<V>>,
    key_func: Box<dyn Fn(&V) -> K>,
}

impl<K: Ord, V> BTreeIndexedVector<K, V> {
    /// Create a new `BTreeIndexedVector` from a vector of items.
    /// The `key_func` is used to extract the key from an item.
    pub fn new<F: Fn(&V) -> K + 'static, C: IntoIterator<Item = V>>(data: C, key_func: F) -> Self {
        let mut map = BTreeMap::new();
        for item in data {
            let key = key_func(&item);
            map.entry(key).or_insert_with(Vec::new).push(item);
        }
        Self {
            map,
            key_func: Box::new(key_func),
        }
    }

    /// Search for items in the given range.
    pub fn search_range<R: RangeBounds<K>>(&self, range: R) -> Vec<&V> {
        self.map.range(range).flat_map(|(_, v)| v.iter()).collect()
    }
}

impl<K: Ord, V> IndexedVector<K, V> for BTreeIndexedVector<K, V> {
    fn search(&self, key: &K) -> Vec<&V> {
        self.map
            .get(key)
            .map(|v| v.iter().collect())
            .unwrap_or_default()
    }

    fn insert(&mut self, item: V) {
        let key = (self.key_func)(&item);
        self.map.entry(key).or_insert_with(Vec::new).push(item);
    }
}

#[cfg(test)]
mod test {
    use super::{BTreeIndexedVector, IndexedVector};

    #[test]
    fn test_btree_indexed_vector() {
        let mut map = BTreeIndexedVector::new(
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
            Box::new(|x: &i32| x % 3),
        );
        assert_eq!(map.search(&0), vec![&3, &6, &9]);
        assert_eq!(map.search(&1), vec![&1, &4, &7, &10]);
        assert_eq!(map.search(&2), vec![&2, &5, &8]);

        map.insert(11);
        assert_eq!(map.search(&0), vec![&3, &6, &9]);
        assert_eq!(map.search(&1), vec![&1, &4, &7, &10]);
        assert_eq!(map.search(&2), vec![&2, &5, &8, &11]);
    }

    #[test]
    fn test_search_range() {
        let map = BTreeIndexedVector::new(
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
            Box::new(|x: &i32| x % 3),
        );
        let mut res = map.search_range(0..2);
        res.sort();
        assert_eq!(res, vec![&1, &3, &4, &6, &7, &9, &10]);
    }
}
