use crate::metrics::Metric;
use crate::CommonMetricData;
use crate::Glean;

#[derive(Debug)]
pub struct BooleanMetric {
    meta: CommonMetricData,
}

impl BooleanMetric {
    pub fn new(meta: CommonMetricData) -> Self {
        Self { meta }
    }

    pub fn set(&self, glean: &Glean, value: bool) {
        if !self.meta.should_record() || !glean.is_upload_enabled() {
            return;
        }

        let value = Metric::Boolean(value);
        glean.storage().record(&self.meta, &value)
    }
}
