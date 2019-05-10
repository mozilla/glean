use std::fs::{create_dir_all, File};
use std::io::Write;
use std::path::{Path, PathBuf};

use log::info;
use serde_json::{json, Value as JsonValue};

use crate::common_metric_data::CommonMetricData;
use crate::metrics::CounterMetric;
use crate::storage::StorageManager;
use crate::Glean;

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

impl Default for PingMaker {
    fn default() -> Self {
        Self::new()
    }
}

impl PingMaker {
    pub fn new() -> Self {
        Self
    }

    fn get_ping_seq(&self, glean: &Glean, storage_name: &str) -> usize {
        let seq = CounterMetric::new(CommonMetricData {
            name: format!("{}#sequence", storage_name),
            category: "".into(),
            send_in_pings: vec!["internal".into()],
            ..Default::default()
        });
        seq.add(glean, 1);
        seq.test_get_value(glean, "internal").unwrap() as usize
    }

    fn get_ping_info(&self, glean: &Glean, storage_name: &str) -> JsonValue {
        json!({
            "ping_type": storage_name,
            "seq": self.get_ping_seq(glean, storage_name),
            "start_time": "2019-03-29T09:50-04:00",
            "end_time": "2019-03-29T09:53-04:00"
        })
    }

    fn get_client_info(&self, glean: &Glean) -> JsonValue {
        let client_info =
            StorageManager.snapshot_as_json(glean.storage(), "glean_client_info", true);
        // Add the "telemetry_sdk_build", which is the glean-core version.
        let version = env!("CARGO_PKG_VERSION");
        let mut map = json!({
            "telemetry_sdk_build": version,
        });

        // Flatten the whole thing.
        let client_info_obj = client_info.as_object().unwrap(); // safe, snapshot always returns an object.
        for (_key, value) in client_info_obj {
            merge(&mut map, value);
        }

        json!(map)
    }

    pub fn collect(&self, glean: &Glean, storage_name: &str) -> JsonValue {
        info!("Collecting {}", storage_name);

        let metrics_data = StorageManager.snapshot_as_json(glean.storage(), storage_name, true);

        let ping_info = self.get_ping_info(glean, storage_name);
        let client_info = self.get_client_info(glean);

        json!({
            "ping_info": ping_info,
            "client_info": client_info,
            "metrics": metrics_data,
        })
    }

    pub fn collect_string(&self, glean: &Glean, storage_name: &str) -> String {
        let ping = self.collect(glean, storage_name);
        ::serde_json::to_string_pretty(&ping).unwrap()
    }

    fn get_pings_dir(&self, data_path: &Path) -> std::io::Result<PathBuf> {
        let pings_dir = data_path.join("pings");
        create_dir_all(&pings_dir)?;
        Ok(pings_dir)
    }

    /// Store a ping to disk in the pings directory.
    pub fn store_ping(
        &self,
        doc_id: &str,
        data_path: &Path,
        url_path: &str,
        ping_content: &str,
    ) -> std::io::Result<()> {
        let pings_dir = self.get_pings_dir(data_path)?;

        // Write to a temporary location and then move when done,
        // for transactional writes.
        let temp_ping_path = std::env::temp_dir().join(doc_id);
        let ping_path = pings_dir.join(doc_id);

        {
            let mut file = File::create(&temp_ping_path)?;
            file.write_all(url_path.as_bytes())?;
            file.write_all(b"\n")?;
            file.write_all(ping_content.as_bytes())?;
        }

        std::fs::rename(temp_ping_path, ping_path)?;

        Ok(())
    }
}
