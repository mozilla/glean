use std::path::PathBuf;

use uuid::Uuid;

mod common_metric_data;
mod database;
mod error_recording;
mod first_run;
mod internal_metrics;
pub mod metrics;
pub mod ping;
pub mod storage;
mod util;

pub use crate::common_metric_data::{CommonMetricData, Lifetime};
pub use crate::error_recording::ErrorType;
use crate::internal_metrics::CoreMetrics;
use crate::storage::StorageManager;
use crate::database::Database;
use crate::util::sanitize_application_id;
use crate::ping::PingMaker;

const GLEAN_SCHEMA_VERSION: u32 = 1;

#[derive(Debug)]
pub struct Glean {
    initialized: bool,
    upload_enabled: bool,
    data_store: Database,
    core_metrics: CoreMetrics,
    data_path: Option<PathBuf>,
    application_id: Option<String>,
}

impl Glean {
    pub fn new() -> Self {
        log::info!("Creating new glean");
        Self {
            initialized: false,
            upload_enabled: true,
            data_store: Database::new(),
            core_metrics: CoreMetrics::new(),
            data_path: None,
            application_id: None,
        }
    }

    /// Initialize the global Glean object.
    ///
    /// This will create the necessary directories and files in `data_path`.
    /// This will also initialize the core metrics.
    pub fn initialize(&mut self, data_path: &str, application_id: &str) {
        self.data_store.initialize(data_path);
        self.initialize_core_metrics(data_path);

        self.data_path = Some(PathBuf::from(data_path));
        self.application_id = Some(sanitize_application_id(application_id));
        self.initialized = true;
    }

    fn initialize_core_metrics(&mut self, data_path: &str) {
        self.core_metrics.first_run.set(self.storage(), first_run::is_first_run(data_path));
        self.core_metrics.client_id.generate_if_missing(self.storage());
    }

    /// Determine whether the global Glean object is fully initialized yet.
    pub fn is_initialized(&self) -> bool {
        self.initialized
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
        // TODO: Error handling?
        self.application_id.as_ref().unwrap()
    }

    pub fn get_data_path(&self) -> &PathBuf {
        // TODO: Error handling?
        self.data_path.as_ref().unwrap()
    }


    pub fn storage(&self) -> &Database {
        &self.data_store
    }

    pub fn snapshot(&mut self, store_name: &str, clear_store: bool) -> String {
        StorageManager.snapshot(&self.storage(), store_name, clear_store)
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

    /// Send a ping by name.
    ///
    /// The ping content is assembled as soon as possible, but upload is not
    /// guaranteed to happen immediately, as that depends on the upload
    /// policies.
    ///
    /// TODO: (Verify this is correct):
    /// If the ping currently contains no content, it will not be sent.
    pub fn send_ping(&self, ping_name: &str) -> std::io::Result<()> {
        let ping_maker = PingMaker::new();
        let doc_id = Uuid::new_v4().to_string();
        let url_path = self.make_path(ping_name, &doc_id);
        let ping_content = ::serde_json::to_string_pretty(&ping_maker.collect(self.storage(), ping_name)).unwrap();
        ping_maker.store_ping(
            &doc_id,
            &self.get_data_path(),
            &url_path,
            &ping_content,
        )
    }
}
