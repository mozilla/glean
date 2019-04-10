use std::collections::HashMap;

use serde_json::{json, Value as JsonValue};

use super::StorageDump;
use crate::CommonMetricData;

pub struct BooleanStorageImpl {
    store: HashMap<String, HashMap<String, bool>>,
}

impl BooleanStorageImpl {
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
        }
    }

    pub fn record(&mut self, data: &CommonMetricData, value: bool) {
        let name = data.fullname();
        for ping_name in data.storage_names() {
            let data_store = self.store.entry(ping_name.clone()).or_insert_with(HashMap::new);
            data_store.insert(name.clone(), value);
        }
    }
}

impl StorageDump for BooleanStorageImpl {
    fn snapshot(&self, store_name: &str, _clear_store: bool) -> Option<JsonValue> {
        self.store.get(store_name).map(|store| json!(store))
    }
}
