use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::SystemTime;

/// Discriminates different categories of memories managed by the engine.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum MemoryType {
    Episodic,
    Semantic,
    Analytical,
}

/// Primary record stored by the memory engine.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryRecord {
    pub id: String,
    pub r#type: MemoryType,
    pub content: String,
    pub metadata: HashMap<String, String>,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
}

impl MemoryRecord {
    pub fn new(id: impl Into<String>, r#type: MemoryType, content: impl Into<String>) -> Self {
        let now = SystemTime::now();
        Self {
            id: id.into(),
            r#type,
            content: content.into(),
            metadata: HashMap::new(),
            created_at: now,
            updated_at: now,
        }
    }

    pub fn with_metadata(mut self, metadata: HashMap<String, String>) -> Self {
        self.metadata = metadata;
        self
    }
}
