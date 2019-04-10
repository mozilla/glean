use std::collections::HashMap;

use serde_json::{json, Value as JsonValue};

use super::StorageDump;
use crate::CommonMetricData;

pub struct StringStorageImpl {
    store: HashMap<String, String>,
}

impl StringStorageImpl {
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
        }
    }

    pub fn record(&mut self, data: &CommonMetricData, value: String) {
        let name = data.fullname();
        self.store.insert(name, value);
    }
}

impl StorageDump for StringStorageImpl {
    fn dump(&self) -> JsonValue {
        json!(self.store)
    }
}
