use std::collections::HashMap;

use serde_json::{json, Value as JsonValue};

use super::StorageDump;
use crate::CommonMetricData;

pub struct BooleanStorageImpl {
    store: HashMap<String, bool>,
}

impl BooleanStorageImpl {
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
        }
    }

    pub fn record(&mut self, data: &CommonMetricData, value: bool) {
        let name = data.name.clone();
        self.store.insert(name, value);
    }
}

impl StorageDump for BooleanStorageImpl {
    fn dump(&self) -> JsonValue {
        json!(self.store)
    }
}
