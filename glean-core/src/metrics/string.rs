use crate::storage::GenericStorage;
use crate::CommonMetricData;

pub struct StringMetric {
    meta: CommonMetricData,
}

impl StringMetric {
    pub fn new(meta: CommonMetricData) -> Self {
        Self { meta }
    }

    pub fn set<S: AsRef<str>>(&self, value: S) {
        if !self.meta.should_record() {
            return;
        }

        let s = value.as_ref();
        let value = rkv::Value::Str(s);
        GenericStorage.record("string", &self.meta, &value)
    }
}
