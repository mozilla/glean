use std::collections::HashMap;

use serde_json::{json, Value as JsonValue};

use super::StorageDump;
use crate::CommonMetricData;

pub struct StringStorageImpl {
    store: HashMap<String, HashMap<String, String>>,
}

impl StringStorageImpl {
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
        }
    }

    pub fn record(&mut self, data: &CommonMetricData, value: String) {
        let name = data.fullname();
        for ping_name in data.storage_names() {
            let data_store = self
                .store
                .entry(ping_name.clone())
                .or_insert_with(HashMap::new);
            data_store.insert(name.clone(), value.clone());
        }
    }
}

impl StorageDump for StringStorageImpl {
    fn snapshot(&mut self, store_name: &str, clear_store: bool) -> Option<JsonValue> {
        let result = self.store.get(store_name).map(|store| json!(store));
        if clear_store {
            self.store.remove(store_name);
        }
        result
    }
}
