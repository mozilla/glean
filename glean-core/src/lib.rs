use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

use lazy_static::lazy_static;
use rkv::SingleStore;

mod common_metric_data;
mod error_recording;
mod first_run;
mod internal_metrics;
mod inner;
pub mod metrics;
pub mod ping;
pub mod storage;

pub use common_metric_data::{CommonMetricData, Lifetime};
pub use error_recording::ErrorType;
use metrics::Metric;
use inner::Inner;

lazy_static! {
    static ref GLEAN_SINGLETON: Glean = Glean::new();
}

#[derive(Debug)]
pub struct Glean {
    inner: RwLock<Inner>,
}

impl Default for Glean {
    fn default() -> Self {
        Self::new()
    }
}

impl Glean {
    pub fn new() -> Self {
        Self {
            inner: RwLock::new(Inner::new()),
        }
    }

    pub fn singleton() -> &'static Self {
        &*GLEAN_SINGLETON
    }

    pub fn initialize(&self, data_path: &str) {
        {
            let mut inner = self.write();
            inner.initialize(data_path);

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

    pub fn is_initialized(&self) -> bool {
        self.read().is_initialized()
    }

    pub fn set_upload_enabled(&self, flag: bool) {
        self.write().set_upload_enabled(flag)
    }

    pub fn is_upload_enabled(&self) -> bool {
        self.read().is_upload_enabled()
    }

    pub(crate) fn iter_store_from<F>(&self, lifetime: Lifetime, iter_start: &str, transaction_fn: F)
    where
        F: FnMut(&[u8], &Metric)
    {
        self.read().iter_store_from(lifetime, iter_start, transaction_fn)
    }

    pub(crate) fn write_with_store<F>(&self, store_name: Lifetime, transaction_fn: F)
    where
        F: FnMut(rkv::Writer, SingleStore),
    {
        self.write().write_with_store(store_name, transaction_fn)
    }

    pub(crate) fn record(&self, lifetime: Lifetime, ping_name: &str, key: &str, metric: &Metric) {
        self.write().record(lifetime, ping_name, key, metric)
    }

    pub(crate) fn record_with<F>(&self, lifetime: Lifetime, ping_name: &str, key: &str, transform: F)
    where
        F: Fn(Option<Metric>) -> Metric,
    {
        self.write().record_with(lifetime, ping_name, key, transform)
    }
}
