use crate::storage::GenericStorage;
use crate::CommonMetricData;
use crate::metrics::Metric;

pub struct StringMetric {
    meta: CommonMetricData,
}

impl StringMetric {
    pub fn new(meta: CommonMetricData) -> Self {
        Self { meta }
    }

    pub fn set<S: Into<String>>(&self, value: S) {
        if !self.meta.should_record() {
            return;
        }

        let s = value.into();
        let value = Metric::String(s);
        GenericStorage.record(&self.meta, &value)
    }
}
