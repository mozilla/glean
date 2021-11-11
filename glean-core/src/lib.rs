// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#![deny(rustdoc::broken_intra_doc_links)]
#![deny(missing_docs)]

//! Glean is a modern approach for recording and sending Telemetry data.
//!
//! It's in use at Mozilla.
//!
//! All documentation can be found online:
//!
//! ## [The Glean SDK Book](https://mozilla.github.io/glean)

use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;

use once_cell::sync::{Lazy, OnceCell};
use uuid::Uuid;

mod common_metric_data;
mod core;
mod core_metrics;
mod coverage;
mod database;
mod debug;
mod dispatcher;
mod error;
mod error_recording;
mod event_database;
mod glean_metrics;
mod histogram;
mod internal_metrics;
mod internal_pings;
pub mod metrics;
pub mod ping;
mod scheduler;
pub mod storage;
mod system;
pub mod traits;
pub mod upload;
mod util;

pub use crate::common_metric_data::{CommonMetricData, Lifetime};
pub use crate::core::Glean;
use crate::core_metrics::ClientInfoMetrics;
pub use crate::error::{Error, ErrorKind, Result};
pub use crate::error_recording::{test_get_num_recorded_errors, ErrorType};
pub use crate::metrics::{CounterMetric, PingType, RecordedExperiment};

const GLEAN_VERSION: &str = env!("CARGO_PKG_VERSION");
const GLEAN_SCHEMA_VERSION: u32 = 1;
const DEFAULT_MAX_EVENTS: u32 = 500;
static KNOWN_CLIENT_ID: Lazy<Uuid> =
    Lazy::new(|| Uuid::parse_str("c0ffeec0-ffee-c0ff-eec0-ffeec0ffeec0").unwrap());

// The names of the pings directories.
pub(crate) const PENDING_PINGS_DIRECTORY: &str = "pending_pings";
pub(crate) const DELETION_REQUEST_PINGS_DIRECTORY: &str = "deletion_request";

/// Set when `glean::initialize()` returns.
/// This allows to detect calls that happen before `glean::initialize()` was called.
/// Note: The initialization might still be in progress, as it runs in a separate thread.
static INITIALIZE_CALLED: AtomicBool = AtomicBool::new(false);

/// Keep track of the debug features before Glean is initialized.
static PRE_INIT_DEBUG_VIEW_TAG: OnceCell<Mutex<String>> = OnceCell::new();
static PRE_INIT_LOG_PINGS: AtomicBool = AtomicBool::new(false);
static PRE_INIT_SOURCE_TAGS: OnceCell<Mutex<Vec<String>>> = OnceCell::new();

/// Keep track of pings registered before Glean is initialized.
static PRE_INIT_PING_REGISTRATION: OnceCell<Mutex<Vec<metrics::PingType>>> = OnceCell::new();

/// Configuration for Glean
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

/// Returns a timestamp corresponding to "now" with millisecond precision.
pub fn get_timestamp_ms() -> u64 {
    const NANOS_PER_MILLI: u64 = 1_000_000;
    zeitstempel::now() / NANOS_PER_MILLI
}

/// State to keep track for the Rust Language bindings.
///
/// This is useful for setting Glean SDK-owned metrics when
/// the state of the upload is toggled.
struct State {
    /// Client info metrics set by the application.
    client_info: ClientInfoMetrics,

    callbacks: Box<dyn OnGleanEvents>,
}

/// A global singleton storing additional state for Glean.
///
/// Requires a Mutex, because in tests we can actual reset this.
static STATE: OnceCell<Mutex<State>> = OnceCell::new();

/// Get a reference to the global state object.
///
/// Panics if no global state object was set.
fn global_state() -> &'static Mutex<State> {
    STATE.get().unwrap()
}

/// Set or replace the global bindings State object.
fn setup_state(state: State) {
    // The `OnceCell` type wrapping our state is thread-safe and can only be set once.
    // Therefore even if our check for it being empty succeeds, setting it could fail if a
    // concurrent thread is quicker in setting it.
    // However this will not cause a bigger problem, as the second `set` operation will just fail.
    // We can log it and move on.
    //
    // For all wrappers this is not a problem, as the State object is intialized exactly once on
    // calling `initialize` on the global singleton and further operations check that it has been
    // initialized.
    if STATE.get().is_none() {
        if STATE.set(Mutex::new(state)).is_err() {
            log::error!(
                "Global Glean state object is initialized already. This probably happened concurrently."
            );
        }
    } else {
        // We allow overriding the global State object to support test mode.
        // In test mode the State object is fully destroyed and recreated.
        // This all happens behind a mutex and is therefore also thread-safe.
        let mut lock = STATE.get().unwrap().lock().unwrap();
        *lock = state;
    }
}

/// A callback object used to trigger actions on the foreign-language side.
///
/// A callback object is stored in glean-core for the entire lifetime of the application.
pub trait OnGleanEvents: Send {
    /// Initialization finished.
    ///
    /// The language SDK can do additional things from within the same initializer thread,
    /// e.g. starting to observe application events for foreground/background behavior.
    /// The observer then needs to call the respective client activity API.
    fn on_initialize_finished(&self);

    /// Trigger the uploader whenever a ping was submitted.
    ///
    /// This should not block.
    /// The uploader needs to asynchronously poll Glean for new pings to upload.
    fn trigger_upload(&self);

    /// Start the Metrics Ping Scheduler.
    fn start_metrics_ping_scheduler(&self);
}

/// Initializes Glean.
///
/// # Arguments
///
/// * `cfg` - the [`Configuration`] options to initialize with.
/// * `client_info` - the [`ClientInfoMetrics`] values used to set Glean
///   core metrics.
/// * `callbacks` - A callback object, stored for the entire application lifetime.
pub fn glean_initialize(
    cfg: InternalConfiguration,
    client_info: ClientInfoMetrics,
    callbacks: Box<dyn OnGleanEvents>,
) -> bool {
    initialize_inner(cfg, client_info, callbacks).is_ok()
}

fn initialize_inner(
    cfg: InternalConfiguration,
    client_info: ClientInfoMetrics,
    callbacks: Box<dyn OnGleanEvents>,
) -> Result<()> {
    if was_initialize_called() {
        log::error!("Glean should not be initialized multiple times");
        todo!();
    }

    let _init_handle = std::thread::Builder::new()
        .name("glean.init".into())
        .spawn(move || {
            let upload_enabled = cfg.upload_enabled;

            let glean = match Glean::new(cfg) {
                Ok(glean) => glean,
                Err(err) => {
                    log::error!("Failed to initialize Glean: {}", err);
                    return;
                }
            };
            if core::setup_glean(glean).is_err() {
                return;
            }

            log::info!("Glean initialized");

            setup_state(State {
                client_info,
                callbacks,
            });

            core::with_glean_mut(|glean| {
                let state = global_state().lock().unwrap();

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

                // Get the current value of the dirty flag so we know whether to
                // send a dirty startup baseline ping below.  Immediately set it to
                // `false` so that dirty startup pings won't be sent if Glean
                // initialization does not complete successfully.
                // TODO Bug 1672956 will decide where to set this flag again.
                let dirty_flag = glean.is_dirty_flag_set();
                glean.set_dirty_flag(false);

                // Register builtin pings.
                // Unfortunately we need to manually list them here to guarantee
                // they are registered synchronously before we need them.
                // We don't need to handle the deletion-request ping. It's never touched
                // from the language implementation.
                //
                // Note: this will actually double-register them.
                // On instantiation they will launch a task to register themselves.
                // That task could fail to run if the dispatcher queue is full.
                // We still register them here synchronously.
                glean.register_ping_type(&glean_metrics::pings::baseline);
                glean.register_ping_type(&glean_metrics::pings::metrics);
                glean.register_ping_type(&glean_metrics::pings::events);

                // Perform registration of pings that were attempted to be
                // registered before init.
                if let Some(tags) = PRE_INIT_PING_REGISTRATION.get() {
                    let lock = tags.try_lock();
                    if let Ok(pings) = lock {
                        for ping in &*pings {
                            glean.register_ping_type(ping);
                        }
                    }
                }

                // If this is the first time ever the Glean SDK runs, make sure to set
                // some initial core metrics in case we need to generate early pings.
                // The next times we start, we would have them around already.
                let is_first_run = glean.is_first_run();
                if is_first_run {
                    initialize_core_metrics(glean, &state.client_info);
                }

                // Deal with any pending events so we can start recording new ones
                let pings_submitted = glean.on_ready_to_submit_pings();

                // We need to kick off upload in these cases:
                // 1. Pings were submitted through Glean and it is ready to upload those pings;
                // 2. Upload is disabled, to upload a possible deletion-request ping.
                if pings_submitted || !upload_enabled {
                    state.callbacks.trigger_upload();
                }

                // Set up information and scheduling for Glean owned pings. Ideally, the "metrics"
                // ping startup check should be performed before any other ping, since it relies
                // on being dispatched to the API context before any other metric.
                state.callbacks.start_metrics_ping_scheduler();

                // Check if the "dirty flag" is set. That means the product was probably
                // force-closed. If that's the case, submit a 'baseline' ping with the
                // reason "dirty_startup". We only do that from the second run.
                if !is_first_run && dirty_flag {
                    // The `submit_ping_by_name_sync` function cannot be used, otherwise
                    // startup will cause a dead-lock, since that function requests a
                    // write lock on the `glean` object.
                    // Note that unwrapping below is safe: the function will return an
                    // `Ok` value for a known ping.
                    if glean.submit_ping_by_name("baseline", Some("dirty_startup")) {
                        state.callbacks.trigger_upload();
                    }
                }

                // From the second time we run, after all startup pings are generated,
                // make sure to clear `lifetime: application` metrics and set them again.
                // Any new value will be sent in newly generated pings after startup.
                if !is_first_run {
                    glean.clear_application_lifetime_metrics();
                    initialize_core_metrics(glean, &state.client_info);
                }
            });

            // Signal Dispatcher that init is complete
            match dispatcher::flush_init() {
                Ok(task_count) if task_count > 0 => {
                    core::with_glean(|glean| {
                        glean_metrics::error::preinit_tasks_overflow
                            .add_sync(glean, task_count as i32);
                    });
                }
                Ok(_) => {}
                Err(err) => log::error!("Unable to flush the preinit queue: {}", err),
            }

            let state = global_state().lock().unwrap();
            state.callbacks.on_initialize_finished();
        })
        .expect("Failed to spawn Glean's init thread");

    // Mark the initialization as called: this needs to happen outside of the
    // dispatched block!
    INITIALIZE_CALLED.store(true, Ordering::SeqCst);
    Ok(())
}

fn initialize_core_metrics(glean: &Glean, client_info: &ClientInfoMetrics) {
    core_metrics::internal_metrics::app_build.set_sync(glean, &client_info.app_build[..]);
    core_metrics::internal_metrics::app_display_version
        .set_sync(glean, &client_info.app_display_version[..]);
    if let Some(app_channel) = client_info.channel.as_ref() {
        core_metrics::internal_metrics::app_channel.set_sync(glean, app_channel);
    }
    core_metrics::internal_metrics::os_version.set_sync(glean, system::get_os_version());
    core_metrics::internal_metrics::architecture.set_sync(glean, system::ARCH.to_string());
}

/// Checks if [`initialize`] was ever called.
///
/// # Returns
///
/// `true` if it was, `false` otherwise.
fn was_initialize_called() -> bool {
    INITIALIZE_CALLED.load(Ordering::SeqCst)
}

/// Initialize the logging system based on the target platform. This ensures
/// that logging is shown when executing the Glean SDK unit tests.
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

/// Sets whether upload is enabled or not.
pub fn glean_set_upload_enabled(enabled: bool) {
    core::with_glean_mut(|glean| {
        let original_enabled = glean.is_upload_enabled();
        if !enabled {
            //changes_callback.will_be_disabled();
        }

        glean.set_upload_enabled(enabled);

        if !original_enabled && enabled {
            //changes_callback.on_enabled();
        }

        if original_enabled && !enabled {
            //changes_callback.on_disabled();
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
        *lock = tag;
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

/// Performs the collection/cleanup operations required by becoming active.
///
/// This functions generates a baseline ping with reason `active`
/// and then sets the dirty bit.
/// This should be called whenever the consuming product becomes active (e.g.
/// getting to foreground).
pub fn glean_handle_client_active() {
    crate::launch_with_glean_mut(move |glean| {
        glean.handle_client_active();

        // The above call may generate pings, so we need to trigger
        // the uploader. It's fine to trigger it if no ping was generated:
        // it will bail out.
        let state = global_state().lock().unwrap();
        state.callbacks.trigger_upload();
    });

    // The previous block of code may send a ping containing the `duration` metric,
    // in `glean.handle_client_active`. We intentionally start recording a new
    // `duration` after that happens, so that the measurement gets reported when
    // calling `handle_client_inactive`.
    core_metrics::internal_metrics::baseline_duration.start();
}

/// Performs the collection/cleanup operations required by becoming inactive.
///
/// This functions generates a baseline and an events ping with reason
/// `inactive` and then clears the dirty bit.
/// This should be called whenever the consuming product becomes inactive (e.g.
/// getting to background).
pub fn glean_handle_client_inactive() {
    // This needs to be called before the `handle_client_inactive` api: it stops
    // measuring the duration of the previous activity time, before any ping is sent
    // by the next call.
    core_metrics::internal_metrics::baseline_duration.stop();

    crate::launch_with_glean_mut(move |glean| {
        glean.handle_client_inactive();

        // The above call may generate pings, so we need to trigger
        // the uploader. It's fine to trigger it if no ping was generated:
        // it will bail out.
        let state = global_state().lock().unwrap();
        state.callbacks.trigger_upload();
    })
}

/// Collect and submit a ping for eventual upload by name.
pub fn glean_submit_ping_by_name(ping_name: String, reason: Option<String>) {
    dispatcher::launch(|| glean_submit_ping_by_name_sync(ping_name, reason))
}

/// Collect and submit a ping (by its name) for eventual upload, synchronously.
pub fn glean_submit_ping_by_name_sync(ping_name: String, reason: Option<String>) {
    core::with_glean(|glean| {
        if !glean.is_upload_enabled() {
            return;
        }

        let sent = glean.submit_ping_by_name(&ping_name, reason.as_deref());
        if sent {
            let state = global_state().lock().unwrap();
            state.callbacks.trigger_upload();
        }
    })
}

/// **TEST-ONLY Method**
///
/// Enable test mode
pub fn glean_set_test_mode(enabled: bool) {
    dispatcher::global::TESTING_MODE.store(enabled, Ordering::SeqCst);
}

/// **TEST-ONLY Method**
///
/// Destroy the underlying database.
pub fn glean_test_destroy_glean(clear_stores: bool) {
    if !was_initialize_called() {
        return;
    }

    dispatcher::reset_dispatcher();

    if core::global_glean().is_some() {
        core::with_glean_mut(|glean| {
            if clear_stores {
                glean.test_clear_all_stores()
            }
            glean.destroy_db()
        });
    }

    // Allow us to go through initialization again.
    INITIALIZE_CALLED.store(false, Ordering::SeqCst);
}

#[allow(missing_docs)]
mod ffi {
    use super::*;
    uniffi_macros::include_scaffolding!("glean");
}
pub use ffi::*;

// Split unit tests to a separate file, to reduce the file of this one.
#[cfg(test)]
#[path = "lib_unit_tests.rs"]
mod tests;
