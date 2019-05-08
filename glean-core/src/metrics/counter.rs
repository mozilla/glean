use crate::metrics::Metric;
use crate::database::Database;
use crate::CommonMetricData;

#[derive(Debug)]
pub struct CounterMetric {
    meta: CommonMetricData,
}

impl CounterMetric {
    pub fn new(meta: CommonMetricData) -> Self {
        Self { meta }
    }

    pub fn add(&self, storage: &Database, amount: u64) {
        if !self.meta.should_record() {
            return;
        }

        storage.record_with(&self.meta, |old_value| match old_value {
            Some(Metric::Counter(old_value)) => Metric::Counter(old_value + amount),
            _ => Metric::Counter(amount),
        })
    }
}
