// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::metrics::Metric;
use crate::CommonMetricData;
use crate::Glean;

#[derive(Debug)]
pub struct BooleanMetric {
    meta: CommonMetricData,
}

impl BooleanMetric {
    pub fn new(meta: CommonMetricData) -> Self {
        Self { meta }
    }

    pub fn set(&self, glean: &Glean, value: bool) {
        if !self.meta.should_record() || !glean.is_upload_enabled() {
            return;
        }

        let value = Metric::Boolean(value);
        glean.storage().record(&self.meta, &value)
    }
}
