// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use inherent::inherent;
use std::ffi::CString;

use glean_core::ErrorType;

use crate::dispatcher;

/// UUID metric wrapper around the FFI implementation
#[derive(Clone)]
pub struct UuidMetric(pub(crate) u64);

impl UuidMetric {
    /// The public constructor used by automatically generated metrics.
    pub fn new(meta: glean_core::CommonMetricData) -> Self {
        let metric = new_metric!(glean_new_uuid_metric, meta);
        Self(metric)
    }
}

#[inherent(pub)]
impl glean_core::traits::Uuid for UuidMetric {
    fn set(&self, value: uuid::Uuid) {
        let id = self.0;
        dispatcher::launch(move || {
            let value = value.to_hyphenated().to_string();
            let value = CString::new(value).unwrap();
            crate::sys::with_glean(|glean| unsafe { glean.glean_uuid_set(id, value.as_ptr()) });
        });
    }

    fn generate_and_set(&self) -> uuid::Uuid {
        // TODO: We can use glean-core's generate_and_set after bug 1673017.
        let uuid = uuid::Uuid::new_v4();
        self.set(uuid);
        uuid
    }

    fn test_get_value<'a, S: Into<Option<&'a str>>>(&self, _ping_name: S) -> Option<uuid::Uuid> {
        dispatcher::block_on_queue();
        None
    }

    fn test_get_num_recorded_errors<'a, S: Into<Option<&'a str>>>(
        &self,
        _error: ErrorType,
        _ping_name: S,
    ) -> i32 {
        dispatcher::block_on_queue();
        0
    }
}
