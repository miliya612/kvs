use std::collections::HashMap;

pub struct KvStore {
    store: HashMap<String, String>,
}

impl KvStore {
    pub fn new() -> KvStore {
        KvStore {
            store: HashMap::new(),
        }
    }
    pub fn set(&mut self, _key: String, _value: String) {
        panic!();
    }
    pub fn get(&self, _key: String) -> Option<String> {
        panic!();
    }
    pub fn remove(&mut self, _key: String) {
        panic!();
    }
}
