use crate::storage::GenericStorage;
use crate::CommonMetricData;
use crate::metrics::Metric;

pub struct StringListMetric {
    meta: CommonMetricData,
}

impl StringListMetric {
    pub fn new(meta: CommonMetricData) -> Self {
        Self { meta }
    }

    pub fn add<S: Into<String>>(&self, value: S) {
        if !self.meta.should_record() {
            return;
        }

        let value = value.into();
        GenericStorage.record_with(&self.meta, |old_value| {
            match old_value {
                Some(Metric::StringList(mut old_value)) => {
                    old_value.push(value.clone());
                    Metric::StringList(old_value)
                }
                _ => Metric::StringList(vec![value.clone()]),
            }
        })
    }
}
