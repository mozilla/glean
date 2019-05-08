use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

use lazy_static::lazy_static;
use rkv::SingleStore;
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
use metrics::Metric;
use ping::PingMaker;

lazy_static! {
    static ref GLEAN_SINGLETON: Glean = Glean::new();
}

const GLEAN_SCHEMA_VERSION: u32 = 1;

#[derive(Debug)]
pub struct Glean {
    inner: RwLock<Inner>,
}

impl Glean {
    fn new() -> Self {
        Self {
            inner: RwLock::new(Inner::new()),
        }
    }

    /// Get the global singleton instance of Glean.
    ///
    /// This is internally used by metrics and for coordinating storage.
    ///
    /// Use `initialize()` to properly initialize this object.
    pub fn singleton() -> &'static Glean {
        &*GLEAN_SINGLETON
    }

    /// Initialize the global Glean object.
    ///
    /// This will create the necessary directories and files in `data_path`.
    /// This will also initialize the core metrics.
    pub fn initialize(&self, data_path: &str, application_id: &str) {
        {
            let mut inner = self.write();
            inner.initialize(data_path, application_id);

            // drop lock before we call any metric setters
        }

        self.initialize_core_metrics(data_path);
    }

    fn initialize_core_metrics(&self, data_path: &str) {
        internal_metrics::first_run.set(first_run::is_first_run(data_path));
        internal_metrics::client_id.generate_if_missing();
    }

    fn read(&self) -> RwLockReadGuard<Inner> {
        self.inner.read().unwrap()
    }

    fn write(&self) -> RwLockWriteGuard<Inner> {
        self.inner.write().unwrap()
    }

    /// Determine whether the global Glean object is fully initialized yet.
    pub fn is_initialized(&self) -> bool {
        self.read().is_initialized()
    }

    /// Set whether upload is enabled or not.
    ///
    /// When upload is disabled, no data will be recorded.
    pub fn set_upload_enabled(&self, flag: bool) {
        self.write().set_upload_enabled(flag)
    }

    /// Determine whether upload is enabled.
    ///
    /// When upload is disabled, no data will be recorded.
    pub fn is_upload_enabled(&self) -> bool {
        self.read().is_upload_enabled()
    }

    pub(crate) fn iter_store_from<F>(&self, lifetime: Lifetime, iter_start: &str, transaction_fn: F)
    where
        F: FnMut(&[u8], &Metric),
    {
        self.read()
            .data_store
            .iter_store_from(lifetime, iter_start, transaction_fn)
    }

    pub(crate) fn write_with_store<F>(&self, store_name: Lifetime, transaction_fn: F)
    where
        F: FnMut(rkv::Writer, SingleStore),
    {
        self.write()
            .data_store
            .write_with_store(store_name, transaction_fn)
    }

    pub(crate) fn record(&self, lifetime: Lifetime, ping_name: &str, key: &str, metric: &Metric) {
        self.write()
            .data_store
            .record(lifetime, ping_name, key, metric)
    }

    pub(crate) fn record_with<F>(
        &self,
        lifetime: Lifetime,
        ping_name: &str,
        key: &str,
        transform: F,
    ) where
        F: Fn(Option<Metric>) -> Metric,
    {
        self.write()
            .data_store
            .record_with(lifetime, ping_name, key, transform)
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
        let ping_content = ::serde_json::to_string_pretty(&ping_maker.collect(ping_name)).unwrap();
        ping_maker.store_ping(
            &doc_id,
            &self.read().get_data_path(),
            &url_path,
            &ping_content,
        )
    }
}
