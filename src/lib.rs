//! **Networked key-value database with typical methods to insert,
//! update and delete data.**
//!
//! ## Examples
//!
//! ```
//! # use kvs::KvStore;
//! #
//! # fn main() {
//! // needs to be mut as set and remove methods below change state
//! let mut store = KvStore::new();
//! // set will insert data if it doesn't exist (or update if exists)
//! store.set(String::from("Milan"), String::from("Italy"));
//! store.set(String::from("London"), String::from("UK"));
//! println!("Milan is in {:?}", store.get(String::from("Milan")));
//! println!("London is in {:?}", store.get(String::from("London")));
//!
//! // make sure the store doesn't contain data that wasn't put there
//! assert!(store.get(String::from("Rome")) == None);
//!
//! // remove value from the database based on "Milan" key
//! store.remove(String::from("Milan"));
//! // make sure key-value pair is not present after removal
//! assert!(store.get(String::from("Milan")) == None);
//! # }
//! ```
//!

use std::collections::HashMap;

/// Represents the database instance and provides interface for data manipulation.
pub struct KvStore {
    map: HashMap<String, String>,
}

impl Default for KvStore {
    fn default() -> Self {
        Self::new()
    }
}

impl KvStore {
    /// Creates KvStore instance with default configuration and returns it.
    pub fn new() -> Self {
        KvStore {
            map: HashMap::new(),
        }
    }

    /// Inserts new value into the database (or updates if key already exists).
    pub fn set(&mut self, key: String, val: String) {
        self.map.insert(key, val);
    }

    /// Retrieves value by key. No panic if key doesn't exist.
    pub fn get(&self, key: String) -> Option<String> {
        self.map.get(&key).map(|val| val.to_owned())
    }

    /// Removes value from the database. No panic if key doesn't exist.
    pub fn remove(&mut self, key: String) {
        self.map.remove(&key);
    }
}
