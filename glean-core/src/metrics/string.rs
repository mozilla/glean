use crate::storage::StringStorage;
use crate::CommonMetricData;

pub struct StringMetric {
    meta: CommonMetricData,
}

impl StringMetric {
    pub fn new(meta: CommonMetricData) -> Self {
        Self { meta }
    }

    pub fn set<S: Into<String>>(&self, value: S) {
        let mut lock = StringStorage.write().unwrap();
        lock.record(&self.meta, value.into())
    }
}
