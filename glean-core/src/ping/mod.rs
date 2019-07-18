// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Ping collection, assembly & submission.

use std::fs::{create_dir_all, File};
use std::io::Write;
use std::path::{Path, PathBuf};

use log::info;
use serde_json::{json, Value as JsonValue};

use crate::common_metric_data::{CommonMetricData, Lifetime};
use crate::metrics::{CounterMetric, DatetimeMetric, Metric, MetricType, PingType, TimeUnit};
use crate::storage::StorageManager;
use crate::util::{get_iso_time_string, local_now_with_offset};
use crate::{Glean, Result};

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
    /// Create a new PingMaker.
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
        let mut map = json!({
            "ping_type": storage_name,
            "seq": self.get_ping_seq(glean, storage_name),
            "start_time": start_time,
            "end_time": end_time,
        });

        // Get the experiment data, if available.
        if let Some(experiment_data) =
            StorageManager.snapshot_experiments_as_json(glean.storage(), INTERNAL_STORAGE)
        {
            map.as_object_mut()
                .unwrap()
                .insert("experiments".to_string(), experiment_data);
        };

        map
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

        let metrics_data = StorageManager.snapshot_as_json(glean.storage(), &ping.name, true);
        let events_data = glean.event_storage().snapshot_as_json(&ping.name, true);

        if metrics_data.is_none() && events_data.is_none() {
            info!("Storage for {} empty. Bailing out.", ping.name);
            return None;
        }

        let ping_info = self.get_ping_info(glean, &ping.name);
        let client_info = self.get_client_info(glean, ping.include_client_id);

        let mut json = json!({
            "ping_info": ping_info,
            "client_info": client_info
        });
        let json_obj = json.as_object_mut()?;
        if let Some(metrics_data) = metrics_data {
            json_obj.insert("metrics".to_string(), metrics_data);
        }
        if let Some(events_data) = events_data {
            json_obj.insert("events".to_string(), events_data);
        }

        Some(json)
    }

    /// Collect a snapshot for the given ping from storage and attach required meta information,
    /// returning it as a string containing JSON.
    ///
    /// ## Arguments
    ///
    /// * `glean` - the Glean instance to collect data from.
    /// * `ping` - the ping to collect for.
    ///
    /// ## Return value
    ///
    /// Returns a fully assembled ping payload in a string encoded as JSON.
    /// If there is no data stored for the ping, `None` is returned.
    pub fn collect_string(&self, glean: &Glean, ping: &PingType) -> Option<String> {
        self.collect(glean, ping)
            .map(|ping| ::serde_json::to_string_pretty(&ping).unwrap())
    }

    /// Get path to a directory for ping storage.
    ///
    /// The directory will be created inside the `data_path`.
    /// The `pings` directory (and its parents) is created if it does not exist.
    fn get_pings_dir(&self, data_path: &Path) -> std::io::Result<PathBuf> {
        let pings_dir = data_path.join("pings");
        create_dir_all(&pings_dir)?;
        Ok(pings_dir)
    }

    /// Get path to a directory for temporary storage.
    ///
    /// The directory will be created inside the `data_path`.
    /// The `tmp` directory (and its parents) is created if it does not exist.
    fn get_tmp_dir(&self, data_path: &Path) -> std::io::Result<PathBuf> {
        let pings_dir = data_path.join("tmp");
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
        let temp_dir = self.get_tmp_dir(data_path)?;

        // Write to a temporary location and then move when done,
        // for transactional writes.
        let temp_ping_path = temp_dir.join(doc_id);
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

    /// Clear any pending pings in the queue.
    pub fn clear_pending_pings(&self, data_path: &Path) -> Result<()> {
        let pings_dir = self.get_pings_dir(data_path)?;
        std::fs::remove_dir_all(&pings_dir)?;
        create_dir_all(&pings_dir)?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const GLOBAL_APPLICATION_ID: &str = "org.mozilla.glean.test.app";
    pub fn new_glean() -> (Glean, tempfile::TempDir) {
        let dir = tempfile::tempdir().unwrap();
        let tmpname = dir.path().display().to_string();
        let glean = Glean::new(&tmpname, GLOBAL_APPLICATION_ID, true).unwrap();
        (glean, dir)
    }

    #[test]
    fn sequence_numbers_should_be_reset_when_toggling_uploading() {
        let (mut glean, _) = new_glean();
        let ping_maker = PingMaker::new();

        assert_eq!(0, ping_maker.get_ping_seq(&glean, "custom"));
        assert_eq!(1, ping_maker.get_ping_seq(&glean, "custom"));

        glean.set_upload_enabled(false);
        assert_eq!(0, ping_maker.get_ping_seq(&glean, "custom"));
        assert_eq!(0, ping_maker.get_ping_seq(&glean, "custom"));

        glean.set_upload_enabled(true);
        assert_eq!(0, ping_maker.get_ping_seq(&glean, "custom"));
        assert_eq!(1, ping_maker.get_ping_seq(&glean, "custom"));
    }

}
