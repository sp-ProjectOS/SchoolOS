use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct MemoryStorage {
    storage: HashMap<String, String>,
}

impl MemoryStorage {
    pub fn new() -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(Self {
            storage: HashMap::new(),
        }))
    }
}

fn init_memory_storage() -> Arc<Mutex<MemoryStorage>> {
    MemoryStorage::new()
}
