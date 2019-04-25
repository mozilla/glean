use serde_json::{json, Value as JsonValue};
use log::info;

use crate::storage::StorageManager;

pub struct PingMaker;

fn merge(a: &mut JsonValue, b: &JsonValue) {
    match (a, b) {
        (&mut JsonValue::Object(ref mut a), &JsonValue::Object(ref b)) => {
            for (k, v) in b {
                merge(a.entry(k.clone()).or_insert(JsonValue::Null), v);
            }
        }
        (a, b) => {
            *a = b.clone();
        }
    }
}

impl PingMaker {
    pub fn new() -> Self {
        Self
    }

    fn get_ping_seq(&self, _storage: &str) -> usize {
        1
    }

    fn get_ping_info(&self, storage: &str) -> JsonValue {
        json!({
            "ping_type": storage,
            "seq": self.get_ping_seq(storage),
        })
    }

    fn get_client_info(&self) -> JsonValue {
        let client_info = StorageManager.snapshot_as_json("glean_client_info", true);
        let mut map = json!({});

        // Flatten the whole thing.
        let client_info_obj = client_info.as_object().unwrap(); // safe, snapshot always returns an object.
        for (_key, value) in client_info_obj {
            merge(&mut map, value);
        }

        json!(map)
    }

    pub fn collect(&self, storage: &str) -> JsonValue {
        info!("Collecting {}", storage);

        let metrics_data = StorageManager.snapshot_as_json(storage, true);

        let ping_info = self.get_ping_info(storage);
        let client_info = self.get_client_info();

        json!({
            "ping_info": ping_info,
            "client_info": client_info,
            "metrics": metrics_data
        })
    }
}
