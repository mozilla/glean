use serde_json::{json, Value as JsonValue};
use log::info;

use crate::Glean;
use crate::Lifetime;
use crate::metrics::Metric;
use crate::storage::StorageManager;

pub struct PingMaker;

impl PingMaker {
    pub fn new() -> Self {
        Self
    }

    fn get_ping_seq(&self, storage: &str) -> usize {
        1
    }

    fn get_ping_info(&self, storage: &str) -> JsonValue {
        json!({
            "ping_type": storage,
            "seq": self.get_ping_seq(storage),
        })
    }

    fn get_client_info(&self) -> JsonValue {
        json!({

        })
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
