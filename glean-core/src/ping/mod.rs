// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Ping collection, assembly & sending.

use std::fs::{create_dir_all, File};
use std::io::Write;
use std::path::{Path, PathBuf};

use log::info;
use serde_json::{json, Value as JsonValue};

use crate::common_metric_data::{CommonMetricData, Lifetime};
use crate::metrics::{CounterMetric, DatetimeMetric, Metric, MetricType, PingType, TimeUnit};
use crate::storage::StorageManager;
use crate::util::{get_iso_time_string, local_now_with_offset};
use crate::Glean;

// An internal ping name, not to be touched by anything else
const INTERNAL_STORAGE: &str = "glean_internal_info";

/// Collect a ping's data, assemble it into its full payload and store it on disk.
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
    /// Create a new PingMaker
    pub fn new() -> Self {
        Self
    }

    fn get_ping_seq(&self, glean: &Glean, storage_name: &str) -> usize {
        // Sequence numbers are stored as a counter under a name that includes the storage name
        let seq = CounterMetric::new(CommonMetricData {
            name: format!("{}#sequence", storage_name),
            // We don't need a category, the name is already unique
            category: "".into(),
            send_in_pings: vec![INTERNAL_STORAGE.into()],
            lifetime: Lifetime::User,
            ..Default::default()
        });

        let current_seq = match StorageManager.snapshot_metric(
            glean.storage(),
            INTERNAL_STORAGE,
            &seq.meta().identifier(),
        ) {
            Some(Metric::Counter(i)) => i,
            _ => 0,
        };

        // Increase to next sequence id
        seq.add(glean, 1);

        current_seq as usize
    }

    /// Get the formatted start and end times for this ping and update for the next ping.
    fn get_start_end_times(&self, glean: &Glean, storage_name: &str) -> (String, String) {
        let time_unit = TimeUnit::Minute;

        let start_time = DatetimeMetric::new(
            CommonMetricData {
                name: format!("{}#start", storage_name),
                category: "".into(),
                send_in_pings: vec![INTERNAL_STORAGE.into()],
                lifetime: Lifetime::User,
                ..Default::default()
            },
            time_unit,
        );

        // "start_time" is the time the ping was generated the last time.
        // If not available, we use the date the Glean object was initialized.
        let start_time_data = start_time
            .get_value(glean, INTERNAL_STORAGE)
            .unwrap_or_else(|| glean.start_time());
        let end_time_data = local_now_with_offset();

        // Update the start time with the current time.
        start_time.set(glean, Some(end_time_data));

        // Format the times.
        let start_time_data = get_iso_time_string(start_time_data, time_unit);
        let end_time_data = get_iso_time_string(end_time_data, time_unit);
        (start_time_data, end_time_data)
    }

    fn get_ping_info(&self, glean: &Glean, storage_name: &str) -> JsonValue {
        let (start_time, end_time) = self.get_start_end_times(glean, storage_name);
        json!({
            "ping_type": storage_name,
            "seq": self.get_ping_seq(glean, storage_name),
            "start_time": start_time,
            "end_time": end_time,
        })
    }

    fn get_client_info(&self, glean: &Glean, include_client_id: bool) -> JsonValue {
        // Add the "telemetry_sdk_build", which is the glean-core version.
        let version = env!("CARGO_PKG_VERSION");
        let mut map = json!({
            "telemetry_sdk_build": version,
        });

        // Flatten the whole thing.
        if let Some(client_info) =
            StorageManager.snapshot_as_json(glean.storage(), "glean_client_info", true)
        {
            let client_info_obj = client_info.as_object().unwrap(); // safe, snapshot always returns an object.
            for (_key, value) in client_info_obj {
                merge(&mut map, value);
            }
        };

        if !include_client_id {
            map.as_object_mut().unwrap().remove("client_id");
        }

        json!(map)
    }

    /// Collect a snapshot for the given ping from storage and attach required meta information.
    ///
    /// ## Arguments
    ///
    /// * `glean` - the Glean instance to collect data from.
    /// * `ping` - the ping to collect for.
    ///
    /// ## Return value
    ///
    /// Returns a fully assembled JSON representation of the ping payload.
    /// If there is no data stored for the ping, `None` is returned.
    pub fn collect(&self, glean: &Glean, ping: &PingType) -> Option<JsonValue> {
        info!("Collecting {}", ping.name);

        let metrics_data = match StorageManager.snapshot_as_json(glean.storage(), &ping.name, true)
        {
            None => {
                info!("Storage for {} empty. Bailing out.", ping.name);
                return None;
            }
            Some(data) => data,
        };

        let ping_info = self.get_ping_info(glean, &ping.name);
        let client_info = self.get_client_info(glean, ping.include_client_id);

        Some(json!({
            "ping_info": ping_info,
            "client_info": client_info,
            "metrics": metrics_data,
        }))
    }

    /// Collect a snapshot for the given ping from storage and attach required meta information as a JSON string.
    ///
    /// ## Arguments
    ///
    /// * `glean` - the Glean instance to collect data from.
    /// * `ping` - the ping to collect for.
    ///
    /// ## Return value
    ///
    /// Returns a fully assembled JSON string of the ping payload.
    /// If there is no data stored for the ping, `None` is returned.
    pub fn collect_string(&self, glean: &Glean, ping: &PingType) -> Option<String> {
        self.collect(glean, ping)
            .map(|ping| ::serde_json::to_string_pretty(&ping).unwrap())
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
        ping_content: &JsonValue,
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
            file.write_all(::serde_json::to_string(ping_content)?.as_bytes())?;
        }

        std::fs::rename(temp_ping_path, ping_path)?;

        Ok(())
    }
}
