//! FinMemory memory engine core library.
//!
//! Provides abstractions for storing and retrieving market intelligence memories.

mod model;
mod store;

pub use crate::model::{MemoryRecord, MemoryType};
pub use crate::store::{InMemoryStore, MemoryEngine};
