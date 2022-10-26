#![warn(missing_docs)]
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
use std::io;
use std::path::PathBuf;
use std::result;

/// Represents the database instance and provides interface for data manipulation.
pub struct KvStore {
    map: HashMap<String, String>,
}

impl Default for KvStore {
    fn default() -> Self {
        Self::new()
    }
}

/// Unified error type that is used across the library.
/// Exposes underlying error type as an associated value.
#[derive(Debug)]
pub enum KvStoreError {
    Io(io::Error),
}

/// Shorthand type alias for generic result with error.
pub type Result<T> = result::Result<T, KvStoreError>;

impl KvStore {
    /// Creates KvStore instance with default configuration and returns it.
    pub fn new() -> Self {
        KvStore {
            map: HashMap::new(),
        }
    }

    /// Inserts new value into the database (or updates if key already exists).
    ///
    /// See top-level module description for examples
    pub fn set(&mut self, key: String, val: String) -> Result<()> {
        self.map.insert(key, val);
        Ok(())
    }

    /// Retrieves value by key. No panic if key doesn't exist.
    ///
    /// See top-level module description for examples
    pub fn get(&self, key: String) -> Result<Option<String>> {
        self.map.get(&key).map(|val| val.to_owned());
        Ok(None)
    }

    /// Removes value from the database. No panic if key doesn't exist.
    ///
    /// See top-level module description for examples
    pub fn remove(&mut self, key: String) -> Result<()> {
        self.map.remove(&key);
        Ok(())
    }

    /// Opens database file and loads it into memory.
    pub fn open(path: impl Into<PathBuf>) -> Result<KvStore> {
        path;
        Ok(KvStore::new())
    }
}
