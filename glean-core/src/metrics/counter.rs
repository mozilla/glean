use crate::storage::GenericStorage;
use crate::CommonMetricData;

pub struct CounterMetric {
    meta: CommonMetricData,
}

impl CounterMetric {
    pub fn new(meta: CommonMetricData) -> Self {
        Self { meta }
    }

    pub fn add(&self, amount: u32) {
        if !self.meta.should_record() {
            return;
        }

        let amount = amount as u64;
        GenericStorage.record_with("counter", &self.meta, |old_value| {
            match old_value {
                Some(rkv::Value::U64(old_value)) => rkv::OwnedValue::U64(old_value + amount),
                _ => rkv::OwnedValue::U64(amount),
            }
        })
    }
}
