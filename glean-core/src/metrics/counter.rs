use crate::storage::GenericStorage;
use crate::CommonMetricData;

pub struct CounterMetric {
    meta: CommonMetricData,
}

impl CounterMetric {
    pub fn new(meta: CommonMetricData) -> Self {
        Self { meta }
    }

    pub fn add(&self, value: u32) {
        if !self.meta.should_record() {
            return;
        }

        let value = rkv::Value::U64(value as u64);
        GenericStorage.record("counter", &self.meta, &value)
    }
}
