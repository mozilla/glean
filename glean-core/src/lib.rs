// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#![allow(clippy::new_without_default)]
#![allow(clippy::redundant_closure)]

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use chrono::{DateTime, FixedOffset};
use uuid::Uuid;

mod common_metric_data;
mod database;
mod error;
mod error_recording;
mod first_run;
mod internal_metrics;
pub mod metrics;
pub mod ping;
pub mod storage;
mod util;

pub use crate::common_metric_data::{CommonMetricData, Lifetime};
use crate::database::Database;
pub use crate::error::{Error, Result};
pub use crate::error_recording::{test_get_num_recorded_errors, ErrorType};
use crate::internal_metrics::CoreMetrics;
use crate::metrics::PingType;
use crate::ping::PingMaker;
use crate::storage::StorageManager;
use crate::util::{local_now_with_offset, sanitize_application_id};

const GLEAN_SCHEMA_VERSION: u32 = 1;

#[derive(Debug)]
pub struct Glean {
    upload_enabled: bool,
    data_store: Database,
    core_metrics: CoreMetrics,
    data_path: PathBuf,
    application_id: String,
    ping_registry: HashMap<String, PingType>,
    start_time: DateTime<FixedOffset>,
}

impl Glean {
    /// Initialize the global Glean object.
    ///
    /// This will create the necessary directories and files in `data_path`.
    /// This will also initialize the core metrics.
    pub fn new(data_path: &str, application_id: &str, upload_enabled: bool) -> Result<Self> {
        log::info!("Creating new glean");

        let application_id = sanitize_application_id(application_id);
        let mut glean = Self {
            upload_enabled,
            data_store: Database::new(data_path)?,
            core_metrics: CoreMetrics::new(),
            data_path: PathBuf::from(data_path),
            application_id,
            ping_registry: HashMap::new(),
            start_time: local_now_with_offset(),
        };
        glean.initialize_core_metrics()?;
        Ok(glean)
    }

    fn initialize_core_metrics(&mut self) -> Result<()> {
        if first_run::is_first_run(&self.data_path)? {
            self.core_metrics
                .first_run_date
                .set(self, "2019-05-09-04:00");
        }
        self.core_metrics.client_id.generate_if_missing(self);
        Ok(())
    }

    /// Set whether upload is enabled or not.
    ///
    /// When upload is disabled, no data will be recorded.
    pub fn set_upload_enabled(&mut self, flag: bool) {
        self.upload_enabled = flag;
    }

    /// Determine whether upload is enabled.
    ///
    /// When upload is disabled, no data will be recorded.
    pub fn is_upload_enabled(&self) -> bool {
        self.upload_enabled
    }

    pub fn get_application_id(&self) -> &str {
        &self.application_id
    }

    pub fn get_data_path(&self) -> &Path {
        &self.data_path
    }

    pub fn storage(&self) -> &Database {
        &self.data_store
    }

    pub fn snapshot(&mut self, store_name: &str, clear_store: bool) -> String {
        StorageManager
            .snapshot(&self.storage(), store_name, clear_store)
            .unwrap_or_else(|| String::from(""))
    }

    fn make_path(&self, ping_name: &str, doc_id: &str) -> String {
        format!(
            "/submit/{}/{}/{}/{}",
            self.get_application_id(),
            ping_name,
            GLEAN_SCHEMA_VERSION,
            doc_id
        )
    }

    /// Send a ping.
    ///
    /// The ping content is assembled as soon as possible, but upload is not
    /// guaranteed to happen immediately, as that depends on the upload
    /// policies.
    ///
    /// If the ping currently contains no content, it will not be sent.
    ///
    /// Returns true if a ping was assembled and queued, false otherwise.
    /// Returns an error if collecting or writing the ping to disk failed.
    pub fn send_ping(&self, ping: &PingType, log_ping: bool) -> Result<bool> {
        let ping_maker = PingMaker::new();
        let doc_id = Uuid::new_v4().to_string();
        let url_path = self.make_path(&ping.name, &doc_id);
        match ping_maker.collect(self, &ping) {
            None => {
                log::info!(
                    "No content for ping '{}', therefore no ping queued.",
                    ping.name
                );
                Ok(false)
            }
            Some(content) => {
                if log_ping {
                    // Use pretty-printing for log
                    log::info!("{}", ::serde_json::to_string_pretty(&content)?);
                }

                ping_maker.store_ping(&doc_id, &self.get_data_path(), &url_path, &content)?;
                Ok(true)
            }
        }
    }

    /// Send a ping by name.
    ///
    /// The ping content is assembled as soon as possible, but upload is not
    /// guaranteed to happen immediately, as that depends on the upload
    /// policies.
    ///
    /// If the ping currently contains no content, it will not be sent.
    ///
    /// Returns true if a ping was assembled and queued, false otherwise.
    /// Returns an error if collecting or writing the ping to disk failed.
    pub fn send_ping_by_name(&self, ping_name: &str, log_ping: bool) -> Result<bool> {
        match self.get_ping_by_name(ping_name) {
            None => {
                log::error!("Unknown ping type {}", ping_name);
                Ok(false)
            }
            Some(ping) => self.send_ping(ping, log_ping),
        }
    }

    pub fn get_ping_by_name(&self, ping_name: &str) -> Option<&PingType> {
        self.ping_registry.get(ping_name)
    }

    pub fn register_ping_type(&mut self, ping: &PingType) {
        if self.ping_registry.contains_key(&ping.name) {
            log::error!("Duplicate ping named {}", ping.name)
        }

        self.ping_registry.insert(ping.name.clone(), ping.clone());
    }

    /// Get create time of the Glean object.
    pub(crate) fn start_time(&self) -> DateTime<FixedOffset> {
        self.start_time
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn path_is_constructed_from_data() {
        let t = tempfile::tempdir().unwrap();
        let name = t.path().display().to_string();
        let glean = Glean::new(&name, "org.mozilla.glean", true).unwrap();

        assert_eq!(
            "/submit/org-mozilla-glean/baseline/1/this-is-a-docid",
            glean.make_path("baseline", "this-is-a-docid")
        );
    }
}
