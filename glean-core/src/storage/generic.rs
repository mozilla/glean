use bincode::serialize;

use crate::Glean;
use crate::CommonMetricData;
use crate::metrics::Metric;

pub struct GenericStorage;

impl GenericStorage {
    pub fn record(&self, data: &CommonMetricData, value: &Metric) {
        let name = data.fullname();
        let encoded = serialize(value).unwrap();
        let value = rkv::Value::Blob(&encoded);

        for ping_name in data.storage_names() {
            Glean::singleton().record(data.lifetime, ping_name, &name, &value);
        }
    }

    pub fn record_with<F>(&self, data: &CommonMetricData, transform: F) where F: Fn(Option<Metric>) -> Metric {
        let name = data.fullname();
        for ping_name in data.storage_names() {
            Glean::singleton().record_with(data.lifetime, ping_name, &name, &transform);
        }
    }
}
