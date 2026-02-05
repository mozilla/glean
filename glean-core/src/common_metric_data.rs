// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::sync::atomic::{AtomicU8, Ordering};

use malloc_size_of_derive::MallocSizeOf;
use rusqlite::Transaction;

use crate::error::{Error, ErrorKind};
use crate::error_recording::record_error_sqlite;
use crate::metrics::dual_labeled_counter::validate_dual_label_sqlite;
use crate::metrics::labeled::validate_dynamic_label_sqlite;
use crate::ErrorType;
use serde::{Deserialize, Serialize};

/// The supported metrics' lifetimes.
///
/// A metric's lifetime determines when its stored data gets reset.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Deserialize, Serialize, Default, MallocSizeOf)]
#[repr(i32)] // Use i32 to be compatible with our JNA definition
#[serde(rename_all = "lowercase")]
pub enum Lifetime {
    /// The metric is reset with each sent ping
    #[default]
    Ping,
    /// The metric is reset on application restart
    Application,
    /// The metric is reset with each user profile
    User,
}

impl Lifetime {
    /// String representation of the lifetime.
    pub fn as_str(self) -> &'static str {
        match self {
            Lifetime::Ping => "ping",
            Lifetime::Application => "app",
            Lifetime::User => "user",
        }
    }
}

impl TryFrom<i32> for Lifetime {
    type Error = Error;

    fn try_from(value: i32) -> Result<Lifetime, Self::Error> {
        match value {
            0 => Ok(Lifetime::Ping),
            1 => Ok(Lifetime::Application),
            2 => Ok(Lifetime::User),
            e => Err(ErrorKind::Lifetime(e).into()),
        }
    }
}

/// The common set of data shared across all different metric types.
#[derive(Default, Debug, Clone, Deserialize, Serialize, MallocSizeOf)]
pub struct CommonMetricData {
    /// The metric's name.
    pub name: String,
    /// The metric's category.
    pub category: String,
    /// List of ping names to include this metric in.
    pub send_in_pings: Vec<String>,
    /// The metric's lifetime.
    pub lifetime: Lifetime,
    /// Whether or not the metric is disabled.
    ///
    /// Disabled metrics are never recorded.
    pub disabled: bool,
    /// Dynamic label.
    ///
    /// When a [`LabeledMetric<T>`](crate::metrics::LabeledMetric) factory creates the specific
    /// metric to be recorded to, dynamic labels are stored in the specific
    /// label so that we can validate them when the Glean singleton is
    /// available.
    pub dynamic_label: Option<DynamicLabelType>,
}

/// The type of dynamic label applied to a base metric. Used to help identify
/// the necessary validation to be performed.
#[derive(Debug, Clone, Deserialize, Serialize, MallocSizeOf, uniffi::Enum)]
pub enum DynamicLabelType {
    /// Static Label -- no validation required
    Static(String),
    /// A dynamic label applied from a `LabeledMetric`
    Label(String),
    /// A label applied by a `DualLabeledCounter` that contains a dynamic key
    KeyOnly(String, String),
    /// A label applied by a `DualLabeledCounter` that contains a dynamic category
    CategoryOnly(String, String),
    /// A label applied by a `DualLabeledCounter` that contains a dynamic key and category
    KeyAndCategory(String, String),
}

impl Default for DynamicLabelType {
    fn default() -> Self {
        Self::Label(String::new())
    }
}

#[derive(Default, Debug, MallocSizeOf)]
pub struct CommonMetricDataInternal {
    pub inner: CommonMetricData,
    pub disabled: AtomicU8,
}

impl Clone for CommonMetricDataInternal {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            disabled: AtomicU8::new(self.disabled.load(Ordering::Relaxed)),
        }
    }
}

impl From<CommonMetricData> for CommonMetricDataInternal {
    fn from(input_data: CommonMetricData) -> Self {
        let disabled = input_data.disabled;
        Self {
            inner: input_data,
            disabled: AtomicU8::new(u8::from(disabled)),
        }
    }
}

pub enum LabelCheck {
    NoLabel,
    Label(String),
    Error(String, i32),
}

impl LabelCheck {
    pub fn label(&self) -> &str {
        use LabelCheck::*;
        match self {
            NoLabel => "",
            Label(label) | Error(label, _) => label,
        }
    }

    pub fn record_error(&self, tx: &mut Transaction, metric_name: &str, send_in_pings: &[String]) {
        let LabelCheck::Error(_, count) = self else {
            return;
        };

        record_error_sqlite(
            tx,
            metric_name,
            send_in_pings,
            ErrorType::InvalidLabel,
            *count,
        );
    }

    fn map(self, mut f: impl FnMut(String) -> String) -> Self {
        use LabelCheck::*;

        match self {
            NoLabel => NoLabel,
            Label(s) => Label(f(s)),
            // shoud use `MAX_LABEL_LENGTH`, but we can't const-format this
            Error(s, cnt) => Error(f(s), cnt),
        }
    }
}

impl CommonMetricDataInternal {
    /// Creates a new metadata object.
    pub fn new<A: Into<String>, B: Into<String>, C: Into<String>>(
        category: A,
        name: B,
        ping_name: C,
    ) -> CommonMetricDataInternal {
        CommonMetricDataInternal {
            inner: CommonMetricData {
                name: name.into(),
                category: category.into(),
                send_in_pings: vec![ping_name.into()],
                ..Default::default()
            },
            disabled: AtomicU8::new(0),
        }
    }

    /// The metric's base identifier, including the category and name, but not the label.
    ///
    /// If `category` is empty, it's ommitted.
    /// Otherwise, it's the combination of the metric's `category` and `name`.
    pub(crate) fn base_identifier(&self) -> String {
        if self.inner.category.is_empty() {
            self.inner.name.clone()
        } else {
            format!("{}.{}", self.inner.category, self.inner.name)
        }
    }

    /// TODO
    ///
    /// If `category` is empty, it's ommitted.
    /// Otherwise, it's the combination of the metric's `category`, `name` and `label`.
    pub(crate) fn check_labels(&self, tx: &Transaction<'_>) -> LabelCheck {
        let base_identifier = self.base_identifier();

        if let Some(label) = &self.inner.dynamic_label {
            match label {
                DynamicLabelType::Static(label) => LabelCheck::Label(label.to_string()),
                DynamicLabelType::Label(label) => {
                    validate_dynamic_label_sqlite(tx, &base_identifier, label)
                }
                DynamicLabelType::KeyOnly(key, static_category) => {
                    validate_dual_label_sqlite(tx, &base_identifier, key, "")
                        .map(|key| format!("{key}{static_category}"))
                }
                DynamicLabelType::CategoryOnly(static_key, category) => {
                    validate_dual_label_sqlite(tx, &base_identifier, "", category)
                        .map(|category| format!("{static_key}{category}"))
                }
                DynamicLabelType::KeyAndCategory(key, category) => {
                    validate_dual_label_sqlite(tx, &base_identifier, key, category)
                }
            }
        } else {
            LabelCheck::NoLabel
        }
    }

    /// The list of storages this metric should be recorded into.
    pub fn storage_names(&self) -> &[String] {
        &self.inner.send_in_pings
    }
}
