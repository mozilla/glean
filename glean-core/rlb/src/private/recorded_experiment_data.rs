// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.


#[cfg(test)]
use serde::Deserialize;
#[cfg(test)]
use crate::HashMap;

/// Deserialized experiment data.
#[cfg(test)]
#[derive(Clone, Deserialize, Debug)]
pub struct RecordedExperimentData {
    /// The experiment's branch as set through `setExperimentActive`.
    pub branch: String,
    /// Any extra data associated with this experiment through `setExperimentActive`.
    pub extra: Option<HashMap<String, String>>,
}
