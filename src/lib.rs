#![deny(missing_docs)]
//! Networked key-value database with typical methods to insert, update and delete data.

pub use kvs::KvStore;
pub use kvs::Result;

mod kvs;
