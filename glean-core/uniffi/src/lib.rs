// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

uniffi_macros::include_scaffolding!("glean_core");

mod core;
mod common_metric_data;
mod private;

pub use crate::core::Glean;
pub use common_metric_data::{CommonMetricData, Lifetime};
pub use private::CounterMetric;

#[derive(Debug, Clone)]
pub struct Configuration {
    /// Whether upload should be enabled.
    pub upload_enabled: bool,
    /// Path to a directory to store all data in.
    pub data_dir: String,
    /// The application ID (will be sanitized during initialization).
    pub application_id: String,
    /// The name of the programming language used by the binding creating this instance of Glean.
    pub language_binding_name: String,
    /// The maximum number of events to store before sending a ping containing events.
    pub max_events: Option<u32>,
    /// Whether Glean should delay persistence of data from metrics with ping lifetime.
    pub delay_ping_lifetime_io: bool,
    /// The application's build identifier. If this is different from the one provided for a previous init,
    /// and use_core_mps is `true`, we will trigger a "metrics" ping.
    pub app_build: String,
    /// Whether Glean should schedule "metrics" pings.
    pub use_core_mps: bool,
}

pub fn initialize(cfg: Configuration) -> bool {
    Glean::new(cfg).and_then(|glean| {
        core::setup_glean(glean)
    }).is_ok()
}

pub fn glean_enable_logging() {
    #[cfg(target_os = "android")]
    {
        let _ = std::panic::catch_unwind(|| {
            android_logger::init_once(
                android_logger::Config::default()
                    .with_min_level(log::Level::Debug)
                    .with_tag("libglean_ffi"),
            );
            log::trace!("Android logging should be hooked up!")
        });
    }

    #[cfg(all(not(target_os = "android"), not(target_os = "ios")))]
    {
        match env_logger::try_init() {
            Ok(_) => log::trace!("stdout logging should be hooked up!"),
            // Please note that this is only expected to fail during unit tests,
            // where the logger might have already been initialized by a previous
            // test. So it's fine to print with the "logger".
            Err(_) => log::warn!("stdout logging was already initialized"),
        };
    }
}
