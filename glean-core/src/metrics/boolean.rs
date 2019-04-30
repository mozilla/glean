use crate::metrics::Metric;
use crate::storage::GenericStorage;
use crate::CommonMetricData;

pub struct BooleanMetric {
    meta: CommonMetricData,
}

impl BooleanMetric {
    pub fn new(meta: CommonMetricData) -> Self {
        Self { meta }
    }

    pub fn set(&self, value: bool) {
        if !self.meta.should_record() {
            return;
        }

        let value = Metric::Boolean(value);
        GenericStorage.record(&self.meta, &value)
    }
}
