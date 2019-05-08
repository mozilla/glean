use uuid::Uuid;

mod common_metric_data;
mod database;
mod error_recording;
mod first_run;
mod inner;
mod internal_metrics;
pub mod metrics;
pub mod ping;
pub mod storage;
mod util;

pub use common_metric_data::{CommonMetricData, Lifetime};
pub use error_recording::ErrorType;
use inner::Inner;
use ping::PingMaker;
use database::Database;
use internal_metrics::CoreMetrics;

const GLEAN_SCHEMA_VERSION: u32 = 1;

#[derive(Debug)]
pub struct Glean {
    inner: Inner,
    core_metrics: CoreMetrics,
}

impl Glean {
    pub fn new() -> Self {
        Self {
            inner: Inner::new(),
            core_metrics: CoreMetrics::new(),
        }
    }

    /// Initialize the global Glean object.
    ///
    /// This will create the necessary directories and files in `data_path`.
    /// This will also initialize the core metrics.
    pub fn initialize(&mut self, data_path: &str, application_id: &str) {
        {
            self.write().initialize(data_path, application_id);

            // drop lock before we call any metric setters
        }

        self.initialize_core_metrics(data_path);
    }

    fn initialize_core_metrics(&mut self, data_path: &str) {
        self.core_metrics.first_run.set(self.storage(), first_run::is_first_run(data_path));
        self.core_metrics.client_id.generate_if_missing(self.storage());
    }

    fn read(&self) -> &Inner {
        &self.inner
    }

    fn write(&mut self) -> &mut Inner {
        &mut self.inner
    }

    /// Determine whether the global Glean object is fully initialized yet.
    pub fn is_initialized(&self) -> bool {
        self.read().is_initialized()
    }

    /// Set whether upload is enabled or not.
    ///
    /// When upload is disabled, no data will be recorded.
    pub fn set_upload_enabled(&mut self, flag: bool) {
        self.write().set_upload_enabled(flag)
    }

    /// Determine whether upload is enabled.
    ///
    /// When upload is disabled, no data will be recorded.
    pub fn is_upload_enabled(&self) -> bool {
        self.read().is_upload_enabled()
    }

    pub fn storage(&self) -> &Database {
        self.read().storage()
    }

    pub fn snapshot(&mut self, store_name: &str, clear_store: bool) -> String {
        self.write().snapshot(store_name, clear_store)
    }

    fn make_path(&self, ping_name: &str, doc_id: &str) -> String {
        format!(
            "/submit/{}/{}/{}/{}",
            self.read().get_application_id(),
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
            &self.read().get_data_path(),
            &url_path,
            &ping_content,
        )
    }
}
