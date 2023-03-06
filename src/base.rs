pub trait IndexedVector<K, V> {
    fn insert(&mut self, item: V);
    fn search(&self, key: &K) -> Vec<&V>;
}
