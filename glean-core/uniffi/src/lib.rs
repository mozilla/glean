// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

uniffi_macros::include_scaffolding!("glean");

use std::collections::HashMap;

mod common_metric_data;
mod core;
mod database;
mod dispatcher;
mod error;
mod error_recording;
mod private;
mod storage;
mod util;

pub use crate::core::Glean;
pub use crate::error::{Error, ErrorKind, Result};
pub use common_metric_data::{CommonMetricData, Lifetime};
pub use private::{CounterMetric, RecordedExperiment};

#[derive(Debug, Clone)]
pub struct InternalConfiguration {
    /// Whether upload should be enabled.
    pub upload_enabled: bool,
    /// Path to a directory to store all data in.
    pub data_path: String,
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

/// Launches a new task on the global dispatch queue with a reference to the Glean singleton.
fn launch_with_glean(callback: impl FnOnce(&Glean) + Send + 'static) {
    dispatcher::launch(|| core::with_glean(callback));
}

/// Launches a new task on the global dispatch queue with a mutable reference to the
/// Glean singleton.
fn launch_with_glean_mut(callback: impl FnOnce(&mut Glean) + Send + 'static) {
    dispatcher::launch(|| core::with_glean_mut(callback));
}

/// Block on the dispatcher emptying.
///
/// This will panic if called before Glean is initialized.
fn block_on_dispatcher() {
    dispatcher::block_on_queue()
}

pub fn glean_initialize(cfg: InternalConfiguration) -> bool {
    initialize_inner(cfg).is_ok()
}

pub fn initialize_inner(cfg: InternalConfiguration) -> Result<()> {
    let glean = Glean::new(cfg)?;
    core::setup_glean(glean)?;
    Ok(())
}

pub fn glean_finish_initialize() -> bool {
    // Signal Dispatcher that init is complete
    log::info!("Flushing dispatcher after initialization finished.");
    match dispatcher::flush_init() {
        Ok(task_count) if task_count > 0 => {
            log::info!("Dispatcher flushed with a total of {} tasks.", task_count);
            //with_glean(|glean| {
            //    glean_metrics::error::preinit_tasks_overflow
            //        .add_sync(&glean, task_count as i32);
            //    });
        }
        Ok(_) => {}
        Err(err) => log::error!("Unable to flush the preinit queue: {}", err),
    }

    true
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

pub trait OnUploadEnabledChanges {
    fn will_be_disabled(&self);
    fn on_enabled(&self);
    fn on_disabled(&self);
}

pub fn glean_set_upload_enabled(enabled: bool, changes_callback: Box<dyn OnUploadEnabledChanges>) {
    core::with_glean_mut(|glean| {
        let original_enabled = glean.is_upload_enabled();
        if !enabled {
            changes_callback.will_be_disabled();
        }

        glean.set_upload_enabled(enabled);

        if !original_enabled && enabled {
            changes_callback.on_enabled();
        }

        if original_enabled && !enabled {
            changes_callback.on_disabled();
        }
    })
}

/// Indicate that an experiment is running.  Glean will then add an
/// experiment annotation to the environment which is sent with pings. This
/// infomration is not persisted between runs.
///
/// See [`glean_core::Glean::set_experiment_active`].
pub fn glean_set_experiment_active(
    experiment_id: String,
    branch: String,
    extra: HashMap<String, String>,
) {
    launch_with_glean(|glean| glean.set_experiment_active(experiment_id, branch, extra))
}

/// Indicate that an experiment is no longer running.
///
/// See [`glean_core::Glean::set_experiment_inactive`].
pub fn glean_set_experiment_inactive(experiment_id: String) {
    launch_with_glean(|glean| glean.set_experiment_inactive(experiment_id))
}

/// TEST ONLY FUNCTION.
/// Returns the [`RecordedExperiment`] for the given `experiment_id`
/// or `None` if the id isn't found.
pub fn glean_test_get_experiment_data(experiment_id: String) -> Option<RecordedExperiment> {
    block_on_dispatcher();
    core::with_glean(|glean| glean.test_get_experiment_data(experiment_id.to_owned()))
}

// Split unit tests to a separate file, to reduce the length of this one.
#[cfg(test)]
#[path = "lib_unit_tests.rs"]
mod tests;
