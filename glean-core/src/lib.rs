use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};
use lazy_static::lazy_static;

mod common_metric_data;
mod internal_metrics;
pub mod metrics;
pub mod storage;

pub use common_metric_data::{CommonMetricData, Lifetime};

lazy_static! {
    static ref GLEAN_SINGLETON: Glean = Glean::new();
}

#[derive(Debug)]
pub struct Glean {
    inner: RwLock<Inner>,
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

    pub fn initialize(&self) {
        {
            let mut inner = self.inner.write().unwrap();
            inner.initialized = true;

            // drop lock before we call any metric setters
        }

        internal_metrics::clientId.set("glean.rs-sample")
    }

    fn read(&self) -> RwLockReadGuard<Inner> {
        self.inner.read().unwrap()
    }

    fn write(&self) -> RwLockWriteGuard<Inner> {
        self.inner.write().unwrap()
    }

    pub fn is_initialized(&self) -> bool {
        self.read().initialized
    }

    pub fn set_upload_enabled(&self, flag: bool) {
        self.write().upload_enabled = flag;
    }

    pub fn is_upload_enabled(&self) -> bool {
        self.read().upload_enabled
    }
}

#[derive(Debug)]
struct Inner {
    initialized: bool,
    upload_enabled: bool,
}

impl Inner {
    fn new() -> Self {
        Self {
            initialized: true,
            upload_enabled: true,
        }
    }
}
