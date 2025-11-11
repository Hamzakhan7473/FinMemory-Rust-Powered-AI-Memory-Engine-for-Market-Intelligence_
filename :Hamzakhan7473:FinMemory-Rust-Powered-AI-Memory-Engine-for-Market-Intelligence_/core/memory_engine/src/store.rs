use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use anyhow::Result;

use crate::model::{MemoryRecord, MemoryType};

/// Trait capturing pluggable storage backends for the engine.
pub trait MemoryStore: Send + Sync {
    fn upsert(&self, record: MemoryRecord) -> Result<()>;
    fn get(&self, id: &str) -> Option<MemoryRecord>;
    fn find_by_type(&self, memory_type: MemoryType) -> Vec<MemoryRecord>;
}

/// Simple in-memory store used for bootstrapping and tests.
#[derive(Default, Clone)]
pub struct InMemoryStore {
    state: Arc<RwLock<HashMap<String, MemoryRecord>>>,
}

impl MemoryStore for InMemoryStore {
    fn upsert(&self, record: MemoryRecord) -> Result<()> {
        let mut guard = self.state.write().expect("lock poisoned");
        guard.insert(record.id.clone(), record);
        Ok(())
    }

    fn get(&self, id: &str) -> Option<MemoryRecord> {
        let guard = self.state.read().expect("lock poisoned");
        guard.get(id).cloned()
    }

    fn find_by_type(&self, memory_type: MemoryType) -> Vec<MemoryRecord> {
        let guard = self.state.read().expect("lock poisoned");
        guard
            .values()
            .filter(|record| record.r#type == memory_type)
            .cloned()
            .collect()
    }
}

/// Core engine wrapping a store implementation and providing higher-level APIs.
pub struct MemoryEngine<S: MemoryStore> {
    store: S,
}

impl<S: MemoryStore> MemoryEngine<S> {
    pub fn new(store: S) -> Self {
        Self { store }
    }

    pub fn upsert(&self, record: MemoryRecord) -> Result<()> {
        self.store.upsert(record)
    }

    pub fn get(&self, id: &str) -> Option<MemoryRecord> {
        self.store.get(id)
    }

    pub fn list_by_type(&self, memory_type: MemoryType) -> Vec<MemoryRecord> {
        self.store.find_by_type(memory_type)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn upsert_and_get_round_trip() {
        let store = InMemoryStore::default();
        let engine = MemoryEngine::new(store.clone());

        let mut metadata = HashMap::new();
        metadata.insert("symbol".into(), "AAPL".into());

        let record = MemoryRecord::new("1", MemoryType::Analytical, "Apple earnings summary")
            .with_metadata(metadata.clone());

        engine.upsert(record.clone()).expect("upsert must succeed");

        let retrieved = engine.get("1").expect("record must exist");
        assert_eq!(retrieved.id, record.id);
        assert_eq!(retrieved.metadata, metadata);
    }

    #[test]
    fn list_by_type_filters_records() {
        let store = InMemoryStore::default();
        let engine = MemoryEngine::new(store);

        engine
            .upsert(MemoryRecord::new("1", MemoryType::Episodic, "Market opening bell"))
            .unwrap();
        engine
            .upsert(MemoryRecord::new("2", MemoryType::Analytical, "Weekly sector rotation"))
            .unwrap();

        let episodic = engine.list_by_type(MemoryType::Episodic);
        assert_eq!(episodic.len(), 1);
        assert_eq!(episodic[0].id, "1");
    }
}
