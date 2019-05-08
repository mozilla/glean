use crate::metrics::Metric;
use crate::database::Database;
use crate::CommonMetricData;

#[derive(Debug)]
pub struct UuidMetric {
    meta: CommonMetricData,
}

impl UuidMetric {
    pub fn new(meta: CommonMetricData) -> Self {
        Self { meta }
    }

    pub fn set(&self, storage: &Database, value: uuid::Uuid) {
        if !self.meta.should_record() {
            return;
        }

        let s = value.to_string();
        let value = Metric::Uuid(s);
        storage.record(&self.meta, &value)
    }

    pub fn generate(&self, storage: &Database) -> uuid::Uuid {
        let uuid = uuid::Uuid::new_v4();
        self.set(storage, uuid);
        uuid
    }

    pub fn generate_if_missing(&self, storage: &Database) {
        storage.record_with(&self.meta, |old_value| match old_value {
            Some(Metric::Uuid(old_value)) => Metric::Uuid(old_value),
            _ => {
                let uuid = uuid::Uuid::new_v4();
                let new_value = uuid.to_string();
                Metric::Uuid(new_value)
            }
        })
    }
}
