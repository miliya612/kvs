use crate::Result;
use std::collections::HashMap;
use std::path::PathBuf;

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
    /// Return an error if the value is not written successfully.
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
    pub fn set(&mut self, _key: String, _value: String) -> Result<()> {
        //self.store.insert(key, value);
        panic!("unimplemented!");
    }
    /// Get the value associated with the given key.
    ///
    /// If the key does not exist, return `None`.
    /// Return an error if the value is not read successfully.
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
    pub fn get(&self, _key: String) -> Result<Option<String>> {
        //self.store.get(&key).cloned()
        panic!("unimplemented!");
    }
    /// Remove the value accosiated with the given key.
    ///
    /// Return an error if the key does not exist or is not removed successfully.
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
    pub fn remove(&mut self, _key: String) -> Result<()> {
        //self.store.remove(&key);
        panic!("unimplemented!");
    }

    /// Open the KvStore at a given path. Return the KvStore.
    pub fn open<T: Into<PathBuf>>(_path: T) -> Result<KvStore> {
        panic!("unimplemented!");
    }
}
