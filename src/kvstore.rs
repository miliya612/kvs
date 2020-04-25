use std::collections::HashMap;

/// KvStore includes std::collections::HashMap as its storage.
#[derive(Default)]
pub struct KvStore {
    store: HashMap<String, String>,
}

impl KvStore {
    /// Creates an empty `KvStore`.
    ///
    /// # Examples
    ///
    /// ```
    /// use kvs::KvStore;
    /// let mut store: KvStore = KvStore::new();
    /// ```
    pub fn new() -> KvStore {
        KvStore {
            store: HashMap::new(),
        }
    }
    /// Sets a key-value pair into the `KvStore`.
    ///
    /// If given key was already set to the `KvStore` instance, its value would be
    /// overwritten with given value.
    ///
    /// # Examples
    ///
    /// ```
    /// use kvs::KvStore;
    ///
    /// let mut store = KvStore::new();
    ///
    /// store.set("lang".to_owned(), "English".to_owned());
    /// assert_eq!(store.get("lang".to_owned()), Some("English".to_owned()));
    /// store.set("lang".to_owned(), "Japanese".to_owned());
    /// assert_eq!(store.get("lang".to_owned()), Some("Japanese".to_owned()));
    /// ```
    pub fn set(&mut self, key: String, value: String) {
        self.store.insert(key, value);
    }
    /// Get the value associated with the given key.
    ///
    /// # Examples
    ///
    /// ```
    /// use kvs::KvStore;
    ///
    /// let mut store = KvStore::new();
    ///
    /// assert_eq!(store.get("lang".to_owned()), None);
    /// store.set("lang".to_owned(), "English".to_owned());
    /// assert_eq!(store.get("lang".to_owned()), Some("English".to_owned()));
    /// ```
    pub fn get(&self, key: String) -> Option<String> {
        self.store.get(&key).cloned()
    }
    /// Remove the value accosiated with the given key.
    ///
    /// # Examples
    ///
    /// ```
    /// use kvs::KvStore;
    ///
    /// let mut store = KvStore::new();
    ///
    /// store.set("lang".to_owned(), "English".to_owned());
    /// assert_eq!(store.get("lang".to_owned()), Some("English".to_owned()));
    /// store.remove("lang".to_owned());
    /// assert_eq!(store.get("lang".to_owned()), None);
    /// ```
    pub fn remove(&mut self, key: String) {
        self.store.remove(&key);
    }
}
