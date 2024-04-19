use std::collections::HashMap;
use super::Cache;

#[allow(dead_code)]
pub struct MemoryCache {
    data: HashMap<String, Vec<u8>>
}

#[allow(dead_code)]
impl MemoryCache {
    pub fn new() -> MemoryCache {
        MemoryCache{
            data: HashMap::new()
        }
    }
}

#[allow(dead_code)]
impl Cache for MemoryCache {
    fn has(&self, id: &str) -> bool {
        self.data.contains_key(id)
    }

    fn save(&mut self, id: &str, data: Vec<u8>) {
        self.data.insert(String::from(id), data);
    }

    fn get(&self, id: &str) -> Option<&Vec<u8>> {
        self.data.get(id)
    }

    fn delete(&mut self, id: &str) {
        self.data.remove(id);
    }
}