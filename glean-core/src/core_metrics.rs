// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::metrics::{
    Datetime, DatetimeMetric, QuantityMetric, StringMetric, TimeUnit, TimespanMetric,
};
use crate::{CommonMetricData, Lifetime};

use malloc_size_of_derive::MallocSizeOf;
use once_cell::sync::Lazy;

/// Metrics included in every ping as `client_info`.
#[derive(Debug, Default, MallocSizeOf)]
pub struct ClientInfoMetrics {
    /// The build identifier generated by the CI system (e.g. "1234/A").
    pub app_build: String,
    /// The user visible version string (e.g. "1.0.3").
    pub app_display_version: String,
    /// The app's build date
    pub app_build_date: Datetime,

    /// The architecture of the device (e.g. "arm", "x86").
    pub architecture: String,
    /// The name of the operating system (e.g. "Linux", "Android", "iOS").
    pub os_version: String,

    /// The product-provided release channel (e.g. "beta").
    pub channel: Option<String>,
    /// The Android specific SDK version of the software running on this hardware device (e.g. "23").
    pub android_sdk_version: Option<String>,
    /// The Windows specific OS build version (e.g. 19043)
    pub windows_build_number: Option<i64>,
    /// The manufacturer of the device the application is running on.
    /// Not set if the device manufacturer can't be determined (e.g. on Desktop).
    pub device_manufacturer: Option<String>,
    /// The model of the device the application is running on.
    /// On Android, this is Build.MODEL, the user-visible marketing name, like "Pixel 2 XL".
    /// Not set if the device model can't be determined (e.g. on Desktop).
    pub device_model: Option<String>,
    /// The locale of the application during initialization (e.g. "es-ES").
    /// If the locale can't be determined on the system, the value is "und", to indicate "undetermined".
    pub locale: Option<String>,
}

/// Optional product attribution metrics carried in `client_info.attribution`.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct AttributionMetrics {
    /// The attribution source (e.g. "google-play").
    pub source: Option<String>,
    /// The attribution medium (e.g. "organic" for a search engine).
    pub medium: Option<String>,
    /// The attribution campaign (e.g. "mozilla-org").
    pub campaign: Option<String>,
    /// The attribution term (e.g. "browser with developer tools for android").
    pub term: Option<String>,
    /// The attribution content (e.g. "firefoxview").
    pub content: Option<String>,
}

impl AttributionMetrics {
    /// Update self with any non-`None` fields from `other`.
    pub fn update(&mut self, other: AttributionMetrics) {
        if let Some(source) = other.source {
            self.source = Some(source);
        }
        if let Some(medium) = other.medium {
            self.medium = Some(medium);
        }
        if let Some(campaign) = other.campaign {
            self.campaign = Some(campaign);
        }
        if let Some(term) = other.term {
            self.term = Some(term);
        }
        if let Some(content) = other.content {
            self.content = Some(content);
        }
    }
}

/// Optional product distribution metrics carried in `client_info.distribution`.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DistributionMetrics {
    /// The distribution name (e.g. "MozillaOnline").
    pub name: Option<String>,
}

impl DistributionMetrics {
    /// Update self with any non-`None` fields from `other`.
    pub fn update(&mut self, other: DistributionMetrics) {
        if let Some(name) = other.name {
            self.name = Some(name);
        }
    }
}

/// Metrics included in every ping as `client_info`.
impl ClientInfoMetrics {
    /// Creates the client info with dummy values for all.
    pub fn unknown() -> Self {
        ClientInfoMetrics {
            app_build: "Unknown".to_string(),
            app_display_version: "Unknown".to_string(),
            app_build_date: Datetime::default(),
            architecture: "Unknown".to_string(),
            os_version: "Unknown".to_string(),
            channel: Some("Unknown".to_string()),
            android_sdk_version: None,
            windows_build_number: None,
            device_manufacturer: None,
            device_model: None,
            locale: None,
        }
    }
}

#[allow(non_upper_case_globals)]
pub mod internal_metrics {
    use super::*;

    pub static app_build: Lazy<StringMetric> = Lazy::new(|| {
        StringMetric::new(CommonMetricData {
            name: "app_build".into(),
            category: "".into(),
            send_in_pings: vec!["glean_client_info".into()],
            lifetime: Lifetime::Application,
            disabled: false,
            ..Default::default()
        })
    });

    pub static app_display_version: Lazy<StringMetric> = Lazy::new(|| {
        StringMetric::new(CommonMetricData {
            name: "app_display_version".into(),
            category: "".into(),
            send_in_pings: vec!["glean_client_info".into()],
            lifetime: Lifetime::Application,
            disabled: false,
            ..Default::default()
        })
    });

    pub static app_build_date: Lazy<DatetimeMetric> = Lazy::new(|| {
        DatetimeMetric::new(
            CommonMetricData {
                name: "build_date".into(),
                category: "".into(),
                send_in_pings: vec!["glean_client_info".into()],
                lifetime: Lifetime::Application,
                disabled: false,
                ..Default::default()
            },
            TimeUnit::Second,
        )
    });

    pub static app_channel: Lazy<StringMetric> = Lazy::new(|| {
        StringMetric::new(CommonMetricData {
            name: "app_channel".into(),
            category: "".into(),
            send_in_pings: vec!["glean_client_info".into()],
            lifetime: Lifetime::Application,
            disabled: false,
            ..Default::default()
        })
    });

    pub static os_version: Lazy<StringMetric> = Lazy::new(|| {
        StringMetric::new(CommonMetricData {
            name: "os_version".into(),
            category: "".into(),
            send_in_pings: vec!["glean_client_info".into()],
            lifetime: Lifetime::Application,
            disabled: false,
            ..Default::default()
        })
    });

    pub static architecture: Lazy<StringMetric> = Lazy::new(|| {
        StringMetric::new(CommonMetricData {
            name: "architecture".into(),
            category: "".into(),
            send_in_pings: vec!["glean_client_info".into()],
            lifetime: Lifetime::Application,
            disabled: false,
            ..Default::default()
        })
    });

    pub static android_sdk_version: Lazy<StringMetric> = Lazy::new(|| {
        StringMetric::new(CommonMetricData {
            name: "android_sdk_version".into(),
            category: "".into(),
            send_in_pings: vec!["glean_client_info".into()],
            lifetime: Lifetime::Application,
            disabled: false,
            ..Default::default()
        })
    });

    pub static windows_build_number: Lazy<QuantityMetric> = Lazy::new(|| {
        QuantityMetric::new(CommonMetricData {
            name: "windows_build_number".into(),
            category: "".into(),
            send_in_pings: vec!["glean_client_info".into()],
            lifetime: Lifetime::Application,
            disabled: false,
            ..Default::default()
        })
    });

    pub static device_manufacturer: Lazy<StringMetric> = Lazy::new(|| {
        StringMetric::new(CommonMetricData {
            name: "device_manufacturer".into(),
            category: "".into(),
            send_in_pings: vec!["glean_client_info".into()],
            lifetime: Lifetime::Application,
            disabled: false,
            ..Default::default()
        })
    });

    pub static device_model: Lazy<StringMetric> = Lazy::new(|| {
        StringMetric::new(CommonMetricData {
            name: "device_model".into(),
            category: "".into(),
            send_in_pings: vec!["glean_client_info".into()],
            lifetime: Lifetime::Application,
            disabled: false,
            ..Default::default()
        })
    });

    pub static locale: Lazy<StringMetric> = Lazy::new(|| {
        StringMetric::new(CommonMetricData {
            name: "locale".into(),
            category: "".into(),
            send_in_pings: vec!["glean_client_info".into()],
            lifetime: Lifetime::Application,
            disabled: false,
            ..Default::default()
        })
    });

    pub static baseline_duration: Lazy<TimespanMetric> = Lazy::new(|| {
        TimespanMetric::new(
            CommonMetricData {
                name: "duration".into(),
                category: "glean.baseline".into(),
                send_in_pings: vec!["baseline".into()],
                lifetime: Lifetime::Ping,
                disabled: false,
                ..Default::default()
            },
            TimeUnit::Second,
        )
    });
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn update_attribution() {
        let mut attr: AttributionMetrics = Default::default();
        let empty: AttributionMetrics = Default::default();

        // Ensure the identity operation works.
        attr.update(empty.clone());
        assert_eq!(None, attr.source);

        // Ensure simple updates work.
        attr.update(AttributionMetrics {
            source: Some("a source".into()),
            ..Default::default()
        });
        assert_eq!(Some("a source".into()), attr.source);

        // Ensure None doesn't overwrite.
        attr.update(empty);
        assert_eq!(Some("a source".into()), attr.source);

        // Ensure updates of Some work.
        attr.update(AttributionMetrics {
            source: Some("another source".into()),
            ..Default::default()
        });
        assert_eq!(Some("another source".into()), attr.source);
    }

    #[test]
    fn update_distribution() {
        let mut dist: DistributionMetrics = Default::default();
        let empty: DistributionMetrics = Default::default();

        // Ensure the identity operation works.
        dist.update(empty.clone());
        assert_eq!(None, dist.name);

        // Ensure simple updates work.
        dist.update(DistributionMetrics {
            name: Some("a name".into()),
        });
        assert_eq!(Some("a name".into()), dist.name);

        // Ensure None doesn't overwrite.
        dist.update(empty);
        assert_eq!(Some("a name".into()), dist.name);

        // Ensure updates of Some work.
        dist.update(DistributionMetrics {
            name: Some("another name".into()),
        });
        assert_eq!(Some("another name".into()), dist.name);
    }
}
