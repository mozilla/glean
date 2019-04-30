use bincode::serialize;

use crate::metrics::Metric;
use crate::CommonMetricData;
use crate::Glean;

pub struct GenericStorage;

impl GenericStorage {
    pub fn record(&self, data: &CommonMetricData, value: &Metric) {
        let name = data.identifier();
        let encoded = serialize(value).unwrap();
        let value = rkv::Value::Blob(&encoded);

        for ping_name in data.storage_names() {
            Glean::singleton().record(data.lifetime, ping_name, &name, &value);
        }
    }

    pub fn record_with<F>(&self, data: &CommonMetricData, transform: F)
    where
        F: Fn(Option<Metric>) -> Metric,
    {
        let name = data.identifier();
        for ping_name in data.storage_names() {
            Glean::singleton().record_with(data.lifetime, ping_name, &name, &transform);
        }
    }
}
