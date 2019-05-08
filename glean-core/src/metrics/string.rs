use crate::database::Database;
use crate::error_recording::{record_error, ErrorType};
use crate::metrics::Metric;
use crate::CommonMetricData;

const MAX_LENGTH_VALUE: usize = 50;

#[derive(Debug)]
pub struct StringMetric {
    meta: CommonMetricData,
}

impl StringMetric {
    pub fn new(meta: CommonMetricData) -> Self {
        Self { meta }
    }

    pub fn set<S: Into<String>>(&self, storage: &Database, value: S) {
        if !self.meta.should_record() {
            return;
        }

        let s = value.into();
        let s = if s.len() > MAX_LENGTH_VALUE {
            record_error(storage, &self.meta, ErrorType::InvalidValue);
            s[0..MAX_LENGTH_VALUE].to_string()
        } else {
            s
        };

        let value = Metric::String(s);
        storage.record(&self.meta, &value)
    }
}
