// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use serde::{Deserialize, Serialize};
use serde_json::json;
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

/// The maximum length of the experiment id and the branch id. Identifiers
/// longer than this number of characters are truncated.
const MAX_EXPERIMENTS_IDS_LEN: usize = 30;

/// The data for a single experiment.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RecordedExperimentData {
    pub branch: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra: Option<HashMap<String, String>>,
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
    /// * `id` - the id of the experiment. Please note that this will be
    ///          truncated to MAX_EXPERIMENTS_IDS_LEN, if needed.
    pub fn new(id: String) -> Self {
        // Make sure that experiment id is within the expected limit.
        let truncated_id = if id.len() > MAX_EXPERIMENTS_IDS_LEN {
            log::warn!(
                "Value length {} for experiment id exceeds maximum of {}",
                id.len(),
                MAX_EXPERIMENTS_IDS_LEN
            );
            id[0..MAX_EXPERIMENTS_IDS_LEN].to_string()
        } else {
            id
        };

        Self {
            meta: CommonMetricData {
                name: format!("{}#experiment", truncated_id),
                // We don't need a category, the name is already unique
                category: "".into(),
                send_in_pings: vec![INTERNAL_STORAGE.into()],
                lifetime: Lifetime::Application,
                ..Default::default()
            },
        }
    }

    /// Record an experiment as active.
    ///
    /// ## Arguments
    ///
    /// * `glean` - The Glean instance this metric belongs to.
    /// * `branch` -  the active branch of the experiment. Please note that this will be
    ///               truncated to MAX_EXPERIMENTS_IDS_LEN, if needed.
    /// * `extra` - an optional, user defined String to String map used to provide richer
    ///             experiment context if needed.
    pub fn set_active(
        &self,
        glean: &Glean,
        branch: String,
        extra: Option<HashMap<String, String>>,
    ) {
        if !self.should_record(glean) {
            return;
        }

        // Make sure that branch id is within the expected limit.
        let truncated_branch = if branch.len() > MAX_EXPERIMENTS_IDS_LEN {
            log::warn!(
                "Value length {} for branch exceeds maximum of {}",
                branch.len(),
                MAX_EXPERIMENTS_IDS_LEN
            );
            branch[0..MAX_EXPERIMENTS_IDS_LEN].to_string()
        } else {
            branch
        };

        let value = Metric::Experiment(RecordedExperimentData {
            branch: truncated_branch,
            extra,
        });
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

        glean.storage().remove_single_metric(
            Lifetime::Application,
            INTERNAL_STORAGE,
            &self.meta.name,
        )
    }

    /// **Test-only API (exported for FFI purposes).**
    ///
    /// Get the currently stored experiment data as a JSON representation of
    /// the RecordedExperimentData.
    ///
    /// This doesn't clear the stored value.
    pub fn test_get_value_as_json_string(&self, glean: &Glean) -> Option<String> {
        match StorageManager.snapshot_metric(
            glean.storage(),
            INTERNAL_STORAGE,
            &self.meta.identifier(),
        ) {
            Some(Metric::Experiment(e)) => Some(json!(e).to_string()),
            _ => None,
        }
    }
}
