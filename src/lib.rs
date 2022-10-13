pub struct KvStore {
}

impl KvStore {
  pub fn new() -> Self {
    KvStore {}
  }

  pub fn set(&mut self, key: String, val: String) {
    
  }

  pub fn get(&self, key: String) -> Option<String> {
    None
  }

  pub fn remove(&mut self, key: String) {
  }
}
