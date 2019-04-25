use crate::storage::GenericStorage;
use crate::CommonMetricData;
use crate::metrics::Metric;

pub struct UuidMetric {
    meta: CommonMetricData,
}

impl UuidMetric {
    pub fn new(meta: CommonMetricData) -> Self {
        Self { meta }
    }

    pub fn set(&self, value: uuid::Uuid) {
        if !self.meta.should_record() {
            return;
        }

        let s = value.to_string();
        let value = Metric::Uuid(s);
        GenericStorage.record(&self.meta, &value)
    }

    pub fn generate(&self) -> uuid::Uuid {
        let uuid = uuid::Uuid::new_v4();
        self.set(uuid);
        return uuid;
    }
}
