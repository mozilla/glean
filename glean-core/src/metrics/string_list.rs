use crate::database::Database;
use crate::error_recording::{record_error, ErrorType};
use crate::metrics::Metric;
use crate::CommonMetricData;

// Maximum length of any list
const MAX_LIST_LENGTH: usize = 20;
// Maximum length of any string in the list
const MAX_STRING_LENGTH: usize = 50;

#[derive(Debug)]
pub struct StringListMetric {
    meta: CommonMetricData,
}

impl StringListMetric {
    pub fn new(meta: CommonMetricData) -> Self {
        Self { meta }
    }

    pub fn add<S: Into<String>>(&self, storage: &Database, value: S) {
        if !self.meta.should_record() {
            return;
        }

        let value = value.into();
        let value = if value.len() > MAX_STRING_LENGTH {
            record_error(storage, &self.meta, ErrorType::InvalidValue);
            value[0..MAX_STRING_LENGTH].to_string()
        } else {
            value
        };

        storage.record_with(&self.meta, |old_value| match old_value {
            Some(Metric::StringList(mut old_value)) => {
                if old_value.len() == MAX_LIST_LENGTH {
                    record_error(storage, &self.meta, ErrorType::InvalidValue);
                    return Metric::StringList(old_value);
                }
                old_value.push(value.clone());
                Metric::StringList(old_value)
            }
            _ => Metric::StringList(vec![value.clone()]),
        })
    }

    pub fn set(&self, storage: &Database, value: Vec<String>) {
        if !self.meta.should_record() {
            return;
        }

        let value = if value.len() > MAX_LIST_LENGTH {
            record_error(storage, &self.meta, ErrorType::InvalidValue);
            value[0..MAX_LIST_LENGTH].to_vec()
        } else {
            value
        };

        let value = value
            .into_iter()
            .map(|elem| {
                if elem.len() > MAX_STRING_LENGTH {
                    record_error(storage, &self.meta, ErrorType::InvalidValue);
                    elem[0..MAX_STRING_LENGTH].to_string()
                } else {
                    elem
                }
            })
            .collect();

        let value = Metric::StringList(value);
        storage.record(&self.meta, &value);
    }
}
