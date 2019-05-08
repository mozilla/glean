use crate::metrics::Metric;
use crate::CommonMetricData;
use crate::Glean;

#[derive(Debug)]
pub struct CounterMetric {
    meta: CommonMetricData,
}

impl CounterMetric {
    pub fn new(meta: CommonMetricData) -> Self {
        Self { meta }
    }

    pub fn add(&self, glean: &Glean, amount: u64) {
        if !self.meta.should_record() || !glean.is_upload_enabled() {
            return;
        }

        glean
            .storage()
            .record_with(&self.meta, |old_value| match old_value {
                Some(Metric::Counter(old_value)) => Metric::Counter(old_value + amount),
                _ => Metric::Counter(amount),
            })
    }
}
