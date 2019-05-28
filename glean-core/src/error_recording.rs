// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! # Error Recording
//!
//! Glean keeps track of errors that occured due to invalid labels or invalid values when recording
//! other metrics.
//!
//! Error counts are stored in labeled counters in the `glean.error` category.
//! The labeled counter metrics that store the errors are defined in the `metrics.yaml` for documentation purposes,
//! but are not actually used directly, since the `send_in_pings` value needs to match the pings of the metric that is erroring (plus the "metrics" ping),
//! not some constant value that we could define in `metrics.yaml`.

use std::fmt::Display;

use crate::metrics::CounterMetric;
use crate::CommonMetricData;
use crate::Glean;
use crate::Lifetime;

/// The possible error types for metric recording.
#[derive(Debug)]
pub enum ErrorType {
    /// For when the value to be recorded does not match the metric-specific restrictions
    InvalidValue,
    /// For when the label of a labeled metric does not match the restrictions
    InvalidLabel,
}

impl ErrorType {
    /// The error type's metric name
    pub fn to_string(&self) -> &'static str {
        match self {
            ErrorType::InvalidValue => "invalid_value",
            ErrorType::InvalidLabel => "invalid_label",
        }
    }
}

/// Records an error into Glean.
///
/// Errors are recorded as labeled counters in the `glean.error` category.
///
/// *Note*: We do make assumptions here how labeled metrics are encoded, namely by having the name
/// `<name>/<label>`.
/// Errors do not adhere to the usual "maximum label" restriction.
///
/// ## Arguments
///
/// * glean - The Glean instance containing the database
/// * meta - The metric's meta data
/// * error -  The error type to record
/// * message - The message to log. This message is not sent with the ping.
///             It does not need to include the metric name, as that is automatically prepended to the message.
pub fn record_error(
    glean: &Glean,
    meta: &CommonMetricData,
    error: ErrorType,
    message: impl Display,
) {
    // Split off any label of the identifier
    let identifier = meta.identifier();
    let name = identifier.splitn(2, '/').next().unwrap(); // safe unwrap, first field of a split always valid

    // Record errors in the pings the metric is in, as well as the metrics ping.
    let mut send_in_pings = meta.send_in_pings.clone();
    if !send_in_pings.contains(&"metrics".to_string()) {
        send_in_pings.push("metrics".into());
    }

    let metric = CounterMetric::new(CommonMetricData {
        name: format!("{}/{}", error.to_string(), name),
        category: "glean.error".into(),
        lifetime: Lifetime::Ping,
        send_in_pings,
        ..Default::default()
    });

    log::warn!("{}: {}", identifier, message);
    metric.add(glean, 1);
}
