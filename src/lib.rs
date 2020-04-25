#![deny(missing_docs)]
//! In-memory key-value store.
//!
//! kvs stores `String` type as its key and value.

pub use kvstore::KvStore;

mod kvstore;
