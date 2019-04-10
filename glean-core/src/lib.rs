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
    initialized: bool,
    upload_enabled: bool,
}

impl Glean {
    fn new() -> Self {
        internal_metrics::clientId.set("glean.rs-sample");

        Self {
            initialized: true,
            upload_enabled: true,
        }
    }

    pub fn initialize() {
        Glean::singleton();
    }

    pub fn singleton() -> &'static Glean {
        &*GLEAN_SINGLETON
    }

    pub fn is_initialized(&self) -> bool {
        self.initialized
    }

    pub fn is_upload_enabled(&self) -> bool {
        self.upload_enabled
    }
}
