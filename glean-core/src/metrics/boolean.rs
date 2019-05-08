use crate::database::Database;
use crate::metrics::Metric;
use crate::CommonMetricData;

#[derive(Debug)]
pub struct BooleanMetric {
    meta: CommonMetricData,
}

impl BooleanMetric {
    pub fn new(meta: CommonMetricData) -> Self {
        Self { meta }
    }

    pub fn set(&self, storage: &Database, value: bool) {
        if !self.meta.should_record() {
            return;
        }

        let value = Metric::Boolean(value);
        storage.record(&self.meta, &value)
    }
}
