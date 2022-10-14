use std::collections::HashMap;

pub struct KvStore {
    map: HashMap<String, String>,
}

impl Default for KvStore {
    fn default() -> Self {
        Self::new()
    }
}

impl KvStore {
    pub fn new() -> Self {
        KvStore {
            map: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: String, val: String) {
        self.map.insert(key, val);
    }

    pub fn get(&self, key: String) -> Option<String> {
        self.map.get(&key).map(|val| val.to_owned())
    }

    pub fn remove(&mut self, key: String) {
        self.map.remove(&key);
    }
}
