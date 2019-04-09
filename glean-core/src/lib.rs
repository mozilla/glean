use lazy_static::lazy_static;

mod common_metric_data;
mod internal_metrics;
pub mod metrics;
pub mod storage;

pub use common_metric_data::CommonMetricData;

lazy_static! {
    static ref GLEAN_SINGLETON: Glean = Glean::new();
}

#[derive(Debug)]
pub struct Glean;

impl Glean {
    fn new() -> Self {
        internal_metrics::clientId.set("glean.rs-sample");

        Self
    }

    pub fn initialize() {
        Glean::singleton();
    }

    pub fn singleton() -> &'static Glean {
        &*GLEAN_SINGLETON
    }
}
