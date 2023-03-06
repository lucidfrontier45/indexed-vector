use std::{collections::HashMap, hash::Hash};

use crate::IndexedVector;

pub struct HashIndexedVector<K, V> {
    map: HashMap<K, Vec<V>>,
    key_func: Box<dyn Fn(&V) -> K>,
}

impl<K: Eq + Hash, V> HashIndexedVector<K, V> {
    pub fn new(data: Vec<V>, key_func: Box<dyn Fn(&V) -> K>) -> Self {
        let mut map = HashMap::new();
        for item in data {
            let key = key_func(&item);
            map.entry(key).or_insert_with(Vec::new).push(item);
        }
        Self { map, key_func }
    }
}

impl<K: Eq + Hash, V> IndexedVector<K, V> for HashIndexedVector<K, V> {
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
    use crate::{HashIndexedVector, IndexedVector};

    #[test]
    fn test_hash_indexed_map() {
        let mut map =
            HashIndexedVector::new(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], Box::new(|x| x % 3));
        assert_eq!(map.search(&0), vec![&3, &6, &9]);
        assert_eq!(map.search(&1), vec![&1, &4, &7, &10]);
        assert_eq!(map.search(&2), vec![&2, &5, &8]);
        map.insert(11);
        assert_eq!(map.search(&0), vec![&3, &6, &9]);
        assert_eq!(map.search(&1), vec![&1, &4, &7, &10]);
        assert_eq!(map.search(&2), vec![&2, &5, &8, &11]);
    }
}
