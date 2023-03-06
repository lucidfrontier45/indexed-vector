pub trait IndexedVector<K, V> {
    /// Insert an item into the vector.
    fn insert(&mut self, item: V);
    /// Search for items with the given key.
    fn search(&self, key: &K) -> Vec<&V>;
}
