use crate::Glean;
use crate::CommonMetricData;

pub struct GenericStorage;

impl GenericStorage {
    pub fn record(&self, typ: &str, data: &CommonMetricData, value: &rkv::Value) {
        let name = data.fullname();
        for ping_name in data.storage_names() {
            Glean::singleton().record(data.lifetime, typ, ping_name, &name, value);
        }
    }

    pub fn record_with<F>(&self, typ: &str, data: &CommonMetricData, transform: F) where F: Fn(Option<rkv::Value>) -> rkv::OwnedValue {
        let name = data.fullname();
        for ping_name in data.storage_names() {
            Glean::singleton().record_with(data.lifetime, typ, ping_name, &name, &transform);
        }
    }
}
