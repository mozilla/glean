use crate::storage::CounterStorage;
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

        let mut lock = CounterStorage.write().unwrap();
        lock.record(&self.meta, value)
    }
}
