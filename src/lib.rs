pub struct KvStore {}

impl Default for KvStore {
    fn default() -> Self {
        Self::new()
    }
}

impl KvStore {
    pub fn new() -> Self {
        KvStore {}
    }

    pub fn set(&mut self, key: String, val: String) {
        unimplemented!();
    }

    pub fn get(&self, key: String) -> Option<String> {
        None
    }

    pub fn remove(&mut self, key: String) {
        unimplemented!();
    }
}
