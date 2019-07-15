// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::error_recording::{record_error, ErrorType};
use crate::metrics::Metric;
use crate::metrics::MetricType;
use crate::storage::StorageManager;
use crate::CommonMetricData;
use crate::Glean;
use crate::Lifetime;

// FIXME: this should be shared?
// An internal ping name, not to be touched by anything else
const INTERNAL_STORAGE: &str = "glean_internal_info";

/// The data for a single experiment.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RecordedExperimentData {
    pub branch: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra: Option<HashMap<String, String>>
}

/// An experiment metric.
///
/// Used to store active experiments.
/// This is used through the set_experiment_active/set_experiment_inactive Glean SDK API.
#[derive(Clone, Debug)]
pub struct ExperimentMetric {
    meta: CommonMetricData,
}

impl MetricType for ExperimentMetric {
    fn meta(&self) -> &CommonMetricData {
        &self.meta
    }

    fn meta_mut(&mut self) -> &mut CommonMetricData {
        &mut self.meta
    }
}

impl ExperimentMetric {
    /// Create a new experiment metric.
    ///
    /// ## Arguments
    ///
    /// * `id` - the id of the experiment.
    pub fn new(id: String) -> Self {
        Self {
            meta: CommonMetricData {
                name: format!("{}#experiment", id),
                // We don't need a category, the name is already unique
                category: "".into(),
                send_in_pings: vec![INTERNAL_STORAGE.into()],
                lifetime: Lifetime::Application,
                ..Default::default()
            }
        }
    }

    /// Record an experiment as active.
    ///
    /// ## Arguments
    ///
    /// * `glean` - The Glean instance this metric belongs to.
    /// * `branch` -  the active branch of the experiment.
    /// * `extra` - an optional, user defined String to String map used to provide richer
    ///             experiment context if needed.
    ///
    /// ## Notes
    ///
    /// Logs an error if the `amount` is 0 or negative.
    pub fn set_active(&self, glean: &Glean, branch: String, extra: Option<HashMap<String, String>>) {
        if !self.should_record(glean) {
            return;
        }

        let value = Metric::Experiment(RecordedExperimentData{ branch, extra });
        glean.storage().record(&self.meta, &value)
    }

    /// Record an experiment as inactive.
    ///
    /// ## Arguments
    ///
    /// * `glean` - The Glean instance this metric belongs to.
    pub fn set_inactive(&self, glean: &Glean) {
        if !self.should_record(glean) {
            return;
        }

        glean.storage().remove_single_metric(Lifetime::Application, INTERNAL_STORAGE, &self.meta.name)
    }

    /*
    /// **Test-only API (exported for FFI purposes).**
    ///
    /// Get the currently stored value as an integer.
    ///
    /// This doesn't clear the stored value.
    pub fn test_get_value(&self, glean: &Glean, storage_name: &str) -> Option<i32> {
        match StorageManager.snapshot_metric(glean.storage(), storage_name, &self.meta.identifier())
        {
            Some(Metric::Counter(i)) => Some(i),
            _ => None,
        }
    }*/
}
