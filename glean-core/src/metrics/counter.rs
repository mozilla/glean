use crate::metrics::Metric;
use crate::storage::GenericStorage;
use crate::CommonMetricData;

pub struct CounterMetric {
    meta: CommonMetricData,
}

impl CounterMetric {
    pub fn new(meta: CommonMetricData) -> Self {
        Self { meta }
    }

    pub fn add(&self, amount: u64) {
        if !self.meta.should_record() {
            return;
        }

        GenericStorage.record_with(&self.meta, |old_value| match old_value {
            Some(Metric::Counter(old_value)) => Metric::Counter(old_value + amount),
            _ => Metric::Counter(amount),
        })
    }
}
