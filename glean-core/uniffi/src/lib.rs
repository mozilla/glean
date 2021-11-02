// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

uniffi_macros::include_scaffolding!("glean");

use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;

use once_cell::sync::OnceCell;

mod common_metric_data;
mod core;
mod database;
mod debug;
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

/// Set when `glean::finish_initialize()` returns.
/// This allows to detect calls that happen before `glean::initialize()` was called.
/// Note: The initialization might still be in progress, as it runs in a separate thread.
static INITIALIZE_CALLED: AtomicBool = AtomicBool::new(false);

/// Keep track of the debug features before Glean is initialized.
static PRE_INIT_DEBUG_VIEW_TAG: OnceCell<Mutex<String>> = OnceCell::new();
static PRE_INIT_LOG_PINGS: AtomicBool = AtomicBool::new(false);
static PRE_INIT_SOURCE_TAGS: OnceCell<Mutex<Vec<String>>> = OnceCell::new();

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

    core::with_glean_mut(|glean| {
        // The debug view tag might have been set before initialize,
        // get the cached value and set it.
        if let Some(tag) = PRE_INIT_DEBUG_VIEW_TAG.get() {
            let lock = tag.try_lock();
            if let Ok(ref debug_tag) = lock {
                glean.set_debug_view_tag(debug_tag);
            }
        }

        // The log pings debug option might have been set before initialize,
        // get the cached value and set it.
        let log_pigs = PRE_INIT_LOG_PINGS.load(Ordering::SeqCst);
        if log_pigs {
            glean.set_log_pings(log_pigs);
        }

        // The source tags might have been set before initialize,
        // get the cached value and set them.
        if let Some(tags) = PRE_INIT_SOURCE_TAGS.get() {
            let lock = tags.try_lock();
            if let Ok(ref source_tags) = lock {
                glean.set_source_tags(source_tags.to_vec());
            }
        }
    });
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

    INITIALIZE_CALLED.store(true, Ordering::SeqCst);
    true
}

/// Checks if [`initialize`] was ever called.
///
/// # Returns
///
/// `true` if it was, `false` otherwise.
fn was_initialize_called() -> bool {
    INITIALIZE_CALLED.load(Ordering::SeqCst)
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

/// Sets a debug view tag.
///
/// When the debug view tag is set, pings are sent with a `X-Debug-ID` header with the
/// value of the tag and are sent to the ["Ping Debug Viewer"](https://mozilla.github.io/glean/book/dev/core/internal/debug-pings.html).
///
/// # Arguments
///
/// * `tag` - A valid HTTP header value. Must match the regex: "[a-zA-Z0-9-]{1,20}".
///
/// # Returns
///
/// This will return `false` in case `tag` is not a valid tag and `true` otherwise.
/// If called before Glean is initialized it will always return `true`.
pub fn glean_set_debug_view_tag(tag: String) -> bool {
    if was_initialize_called() {
        core::with_glean_mut(|glean| glean.set_debug_view_tag(&tag))
    } else {
        // Glean has not been initialized yet. Cache the provided tag value.
        let m = PRE_INIT_DEBUG_VIEW_TAG.get_or_init(Default::default);
        let mut lock = m.lock().unwrap();
        *lock = tag.to_string();
        // When setting the debug view tag before initialization,
        // we don't validate the tag, thus this function always returns true.
        true
    }
}

/// Sets source tags.
///
/// Overrides any existing source tags.
/// Source tags will show in the destination datasets, after ingestion.
///
/// **Note** If one or more tags are invalid, all tags are ignored.
///
/// # Arguments
///
/// * `tags` - A vector of at most 5 valid HTTP header values. Individual
///   tags must match the regex: "[a-zA-Z0-9-]{1,20}".
pub fn glean_set_source_tags(tags: Vec<String>) -> bool {
    if was_initialize_called() {
        core::with_glean_mut(|glean| glean.set_source_tags(tags))
    } else {
        // Glean has not been initialized yet. Cache the provided source tags.
        let m = PRE_INIT_SOURCE_TAGS.get_or_init(Default::default);
        let mut lock = m.lock().unwrap();
        *lock = tags;
        // When setting the source tags before initialization,
        // we don't validate the tags, thus this function always returns true.
        true
    }
}

/// Sets the log pings debug option.
///
/// When the log pings debug option is `true`,
/// we log the payload of all succesfully assembled pings.
///
/// # Arguments
///
/// * `value` - The value of the log pings option
pub fn glean_set_log_pings(value: bool) {
    if was_initialize_called() {
        core::with_glean_mut(|glean| glean.set_log_pings(value));
    } else {
        PRE_INIT_LOG_PINGS.store(value, Ordering::SeqCst);
    }
}

// Split unit tests to a separate file, to reduce the length of this one.
#[cfg(test)]
#[path = "lib_unit_tests.rs"]
mod tests;
