use crate::storage::BooleanStorage;
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

        let mut lock = BooleanStorage.write().unwrap();
        lock.record(&self.meta, value)
    }
}
