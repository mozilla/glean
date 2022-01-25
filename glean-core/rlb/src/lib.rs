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
//!
//! ## Example
//!
//! Initialize Glean, register a ping and then send it.
//!
//! ```rust,no_run
//! # use glean::{Configuration, ClientInfoMetrics, Error, private::*};
//! let cfg = Configuration {
//!     data_path: "/tmp/data".into(),
//!     application_id: "org.mozilla.glean_core.example".into(),
//!     upload_enabled: true,
//!     max_events: None,
//!     delay_ping_lifetime_io: false,
//!     server_endpoint: None,
//!     uploader: None,
//!     use_core_mps: false,
//! };
//! glean::initialize(cfg, ClientInfoMetrics::unknown());
//!
//! let prototype_ping = PingType::new("prototype", true, true, vec!());
//!
//! prototype_ping.submit(None);
//! ```

use std::collections::HashMap;

pub use configuration::Configuration;
use configuration::DEFAULT_GLEAN_ENDPOINT;
pub use core_metrics::ClientInfoMetrics;
pub use glean_core::{
    metrics::{Datetime, DistributionData, MemoryUnit, RecordedEvent, TimeUnit, TimerId},
    traits, CommonMetricData, Error, ErrorType, Glean, HistogramType, Lifetime, RecordedExperiment,
    Result,
};

mod configuration;
mod core_metrics;
pub mod net;
pub mod private;
mod system;

#[cfg(test)]
mod common_test;

const LANGUAGE_BINDING_NAME: &str = "Rust";

/// Creates and initializes a new Glean object.
///
/// See [`glean_core::Glean::new`] for more information.
///
/// # Arguments
///
/// * `cfg` - the [`Configuration`] options to initialize with.
/// * `client_info` - the [`ClientInfoMetrics`] values used to set Glean
///   core metrics.
pub fn initialize(cfg: Configuration, client_info: ClientInfoMetrics) {
    initialize_internal(cfg, client_info);
}

struct GleanEvents {
    /// An instance of the upload manager
    upload_manager: net::UploadManager,
}

impl glean_core::OnGleanEvents for GleanEvents {
    fn on_initialize_finished(&self) {
        // intentionally left empty
    }

    fn trigger_upload(&self) {
        self.upload_manager.trigger_upload();
    }

    fn start_metrics_ping_scheduler(&self) -> bool {
        // We rely on the glean-core MPS.
        // We always trigger an upload as it might have submitted a ping.
        true
    }

    fn cancel_uploads(&self) {
        // intentionally left empty
    }
}

fn initialize_internal(cfg: Configuration, client_info: ClientInfoMetrics) -> Option<()> {
    // Initialize the ping uploader.
    let upload_manager = net::UploadManager::new(
        cfg.server_endpoint
            .unwrap_or_else(|| DEFAULT_GLEAN_ENDPOINT.to_string()),
        cfg.uploader
            .unwrap_or_else(|| Box::new(net::HttpUploader) as Box<dyn net::PingUploader>),
    );

    // Now make this the global object available to others.
    let callbacks = Box::new(GleanEvents { upload_manager });

    let core_cfg = glean_core::InternalConfiguration {
        upload_enabled: cfg.upload_enabled,
        data_path: cfg.data_path.display().to_string(),
        application_id: cfg.application_id.clone(),
        language_binding_name: LANGUAGE_BINDING_NAME.into(),
        max_events: cfg.max_events.map(|m| m as u32),
        delay_ping_lifetime_io: cfg.delay_ping_lifetime_io,
        app_build: client_info.app_build.clone(),
        use_core_mps: cfg.use_core_mps,
    };

    let success = glean_core::glean_initialize(core_cfg, client_info.into(), callbacks);

    if !success {
        return None;
    }

    Some(())
}

/// Shuts down Glean in an orderly fashion.
pub fn shutdown() {
    if global_glean().is_none() {
        log::warn!("Shutdown called before Glean is initialized");
        if let Err(e) = dispatcher::kill() {
            log::error!("Can't kill dispatcher thread: {:?}", e);
        }

        return;
    }

    crate::launch_with_glean_mut(|glean| {
        glean.cancel_metrics_ping_scheduler();
        glean.set_dirty_flag(false);
    });

    if let Err(e) = dispatcher::shutdown() {
        log::error!("Can't shutdown dispatcher thread: {:?}", e);
    }

    // Be sure to call this _after_ draining the dispatcher
    crate::with_glean(|glean| {
        if let Err(e) = glean.persist_ping_lifetime_data() {
            log::error!("Can't persist ping lifetime data: {:?}", e);
        }
    });
}

/// Unblock the global dispatcher to start processing queued tasks.
///
/// This should _only_ be called if it is guaranteed that `initialize` will never be called.
///
/// **Note**: Exported as a FFI function to be used by other language bindings (e.g. Kotlin/Swift)
/// to unblock the RLB-internal dispatcher.
/// This allows the usage of both the RLB and other language bindings (e.g. Kotlin/Swift)
/// within the same application.
#[no_mangle]
#[inline(never)]
pub extern "C" fn rlb_flush_dispatcher() {
    log::trace!("FLushing RLB dispatcher through the FFI");

    let was_initialized = was_initialize_called();

    // Panic in debug mode
    debug_assert!(!was_initialized);

    // In release do a check and bail out
    if was_initialized {
        log::error!(
            "Tried to flush the dispatcher from outside, but Glean was initialized in the RLB."
        );
        return;
    }

    if let Err(err) = dispatcher::flush_init() {
        log::error!("Unable to flush the preinit queue: {}", err);
    }
}

/// Block on the dispatcher emptying.
///
/// This will panic if called before Glean is initialized.
fn block_on_dispatcher() {
    assert!(
        was_initialize_called(),
        "initialize was never called. Can't block on the dispatcher queue."
    );
    dispatcher::block_on_queue().unwrap();
}

/// Checks if [`initialize`] was ever called.
///
/// # Returns
///
/// `true` if it was, `false` otherwise.
fn was_initialize_called() -> bool {
    INITIALIZE_CALLED.load(Ordering::SeqCst)
}

fn initialize_core_metrics(
    glean: &Glean,
    client_info: &ClientInfoMetrics,
    channel: Option<String>,
) {
    core_metrics::internal_metrics::app_build.set_sync(glean, &client_info.app_build[..]);
    core_metrics::internal_metrics::app_display_version
        .set_sync(glean, &client_info.app_display_version[..]);
    if let Some(app_channel) = channel {
        core_metrics::internal_metrics::app_channel.set_sync(glean, app_channel);
    }
    core_metrics::internal_metrics::os_version.set_sync(glean, system::get_os_version());
    core_metrics::internal_metrics::architecture.set_sync(glean, system::ARCH.to_string());
}

/// Sets whether upload is enabled or not.
///
/// See [`glean_core::Glean::set_upload_enabled`].
pub fn set_upload_enabled(enabled: bool) {
    if !was_initialize_called() {
        let msg =
            "Changing upload enabled before Glean is initialized is not supported.\n \
            Pass the correct state into `Glean.initialize()`.\n \
            See documentation at https://mozilla.github.io/glean/book/user/general-api.html#initializing-the-glean-sdk";
        log::error!("{}", msg);
        return;
    }

    // Changing upload enabled always happens asynchronous.
    // That way it follows what a user expect when calling it inbetween other calls:
    // it executes in the right order.
    //
    // Because the dispatch queue is halted until Glean is fully initialized
    // we can safely enqueue here and it will execute after initialization.
    crate::launch_with_glean_mut(move |glean| {
        let state = global_state().lock().unwrap();
        let old_enabled = glean.is_upload_enabled();
        glean.set_upload_enabled(enabled);

        if !old_enabled && enabled {
            glean.start_metrics_ping_scheduler();
            // If uploading is being re-enabled, we have to restore the
            // application-lifetime metrics.
            initialize_core_metrics(glean, &state.client_info, state.channel.clone());
        }

        if old_enabled && !enabled {
            glean.cancel_metrics_ping_scheduler();
            // If uploading is disabled, we need to send the deletion-request ping:
            // note that glean-core takes care of generating it.
            state.upload_manager.trigger_upload();
        }
    });
}

/// Register a new [`PingType`](private::PingType).
pub fn register_ping_type(ping: &private::PingType) {
    // If this happens after Glean.initialize is called (and returns),
    // we dispatch ping registration on the thread pool.
    // Registering a ping should not block the application.
    // Submission itself is also dispatched, so it will always come after the registration.
    if was_initialize_called() {
        let ping = ping.clone();
        crate::launch_with_glean_mut(move |glean| {
            glean.register_ping_type(&ping.ping_type);
        })
    } else {
        // We need to keep track of pings, so they get re-registered after a reset or
        // if ping registration is attempted before Glean initializes.
        // This state is kept across Glean resets, which should only ever happen in test mode.
        // It's a set and keeping them around forever should not have much of an impact.
        let m = PRE_INIT_PING_REGISTRATION.get_or_init(Default::default);
        let mut lock = m.lock().unwrap();
        lock.push(ping.clone());
    }
}

/// Collects and submits a ping for eventual uploading.
///
/// See [`glean_core::Glean.submit_ping`].
pub(crate) fn submit_ping(ping: &private::PingType, reason: Option<&str>) {
    submit_ping_by_name(&ping.name, reason)
}

/// Collects and submits a ping for eventual uploading by name.
///
/// Note that this needs to be public in order for RLB consumers to
/// use Glean debugging facilities.
///
/// See [`glean_core::Glean.submit_ping_by_name`].
pub fn submit_ping_by_name(ping: &str, reason: Option<&str>) {
    let ping = ping.to_string();
    let reason = reason.map(|s| s.to_string());
    dispatcher::launch(move || {
        submit_ping_by_name_sync(&ping, reason.as_deref());
    })
}

/// Collect and submit a ping (by its name) for eventual upload, synchronously.
///
/// The ping will be looked up in the known instances of [`private::PingType`]. If the
/// ping isn't known, an error is logged and the ping isn't queued for uploading.
///
/// The ping content is assembled as soon as possible, but upload is not
/// guaranteed to happen immediately, as that depends on the upload
/// policies.
///
/// If the ping currently contains no content, it will not be assembled and
/// queued for sending, unless explicitly specified otherwise in the registry
/// file.
///
/// # Arguments
///
/// * `ping_name` - the name of the ping to submit.
/// * `reason` - the reason the ping is being submitted.
pub(crate) fn submit_ping_by_name_sync(ping: &str, reason: Option<&str>) {
    if !was_initialize_called() {
        log::error!("Glean must be initialized before submitting pings.");
        return;
    }

    let submitted_ping = with_glean(|glean| {
        if !glean.is_upload_enabled() {
            log::info!("Glean disabled: not submitting any pings.");
            // This won't actually return from `submit_ping_by_name`, but
            // returning `false` here skips spinning up the uploader below,
            // which is basically the same.
            return false;
        }

        glean.submit_ping_by_name(ping, reason.as_deref())
    });

    if submitted_ping {
        let state = global_state().lock().unwrap();
        state.upload_manager.trigger_upload();
    }
}

/// Indicate that an experiment is running.  Glean will then add an
/// experiment annotation to the environment which is sent with pings. This
/// infomration is not persisted between runs.
///
/// See [`glean_core::Glean::set_experiment_active`].
pub fn set_experiment_active(
    experiment_id: String,
    branch: String,
    extra: Option<HashMap<String, String>>,
) {
    crate::launch_with_glean(move |glean| {
        glean.set_experiment_active(experiment_id.to_owned(), branch.to_owned(), extra)
    })
}

/// Indicate that an experiment is no longer running.
///
/// See [`glean_core::Glean::set_experiment_inactive`].
pub fn set_experiment_inactive(experiment_id: String) {
    crate::launch_with_glean(move |glean| glean.set_experiment_inactive(experiment_id))
}

/// Performs the collection/cleanup operations required by becoming active.
///
/// This functions generates a baseline ping with reason `active`
/// and then sets the dirty bit.
/// This should be called whenever the consuming product becomes active (e.g.
/// getting to foreground).
pub fn handle_client_active() {
    crate::launch_with_glean_mut(|glean| {
        glean.handle_client_active();

        // The above call may generate pings, so we need to trigger
        // the uploader. It's fine to trigger it if no ping was generated:
        // it will bail out.
        let state = global_state().lock().unwrap();
        state.upload_manager.trigger_upload();
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
pub fn handle_client_inactive() {
    // This needs to be called before the `handle_client_inactive` api: it stops
    // measuring the duration of the previous activity time, before any ping is sent
    // by the next call.
    core_metrics::internal_metrics::baseline_duration.stop();

    crate::launch_with_glean_mut(|glean| {
        glean.handle_client_inactive();

        // The above call may generate pings, so we need to trigger
        // the uploader. It's fine to trigger it if no ping was generated:
        // it will bail out.
        let state = global_state().lock().unwrap();
        state.upload_manager.trigger_upload();
    })
}

/// TEST ONLY FUNCTION.
/// Checks if an experiment is currently active.
#[allow(dead_code)]
pub(crate) fn test_is_experiment_active(experiment_id: String) -> bool {
    block_on_dispatcher();
    with_glean(|glean| glean.test_is_experiment_active(experiment_id.to_owned()))
}

/// TEST ONLY FUNCTION.
/// Returns the [`RecordedExperimentData`] for the given `experiment_id` or panics if
/// the id isn't found.
#[allow(dead_code)]
pub(crate) fn test_get_experiment_data(experiment_id: String) -> RecordedExperimentData {
    block_on_dispatcher();
    with_glean(|glean| {
        let json_data = glean
            .test_get_experiment_data_as_json(experiment_id.to_owned())
            .unwrap_or_else(|| panic!("No experiment found for id: {}", experiment_id));
        serde_json::from_str::<RecordedExperimentData>(&json_data).unwrap()
    })
}

/// Destroy the global Glean state.
pub(crate) fn destroy_glean(clear_stores: bool) {
    // Destroy the existing glean instance from glean-core.
    if was_initialize_called() {
        // Reset the dispatcher first (it might still run tasks against the database)
        dispatcher::reset_dispatcher();

        // Wait for any background uploader thread to finish.
        // This needs to be done before the check below,
        // as the uploader will also try to acquire a lock on the global Glean.
        //
        // Note: requires the block here, so we drop the lock again.
        {
            let state = global_state().lock().unwrap();
            state.upload_manager.test_wait_for_upload();
        }

        // We need to check if the Glean object (from glean-core) is
        // initialized, otherwise this will crash on the first test
        // due to bug 1675215 (this check can be removed once that
        // bug is fixed).
        if global_glean().is_some() {
            with_glean_mut(|glean| {
                if clear_stores {
                    glean.test_clear_all_stores()
                }
                glean.destroy_db()
            });
        }
        // Allow us to go through initialization again.
        INITIALIZE_CALLED.store(false, Ordering::SeqCst);

        // If Glean initialization previously didn't finish,
        // then the global state might not have been reset
        // and thus needs to be cleared here.
        let state = global_state().lock().unwrap();
        state.upload_manager.test_clear_upload_thread();
    }
}

/// TEST ONLY FUNCTION.
/// Resets the Glean state and triggers init again.
pub fn test_reset_glean(cfg: Configuration, client_info: ClientInfoMetrics, clear_stores: bool) {
    destroy_glean(clear_stores);

    if let Some(handle) = initialize_internal(cfg, client_info) {
        handle.join().unwrap();
    }
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
pub fn set_debug_view_tag(tag: &str) -> bool {
    if was_initialize_called() {
        with_glean_mut(|glean| glean.set_debug_view_tag(tag))
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

/// Sets the log pings debug option.
///
/// When the log pings debug option is `true`,
/// we log the payload of all succesfully assembled pings.
///
/// # Arguments
///
/// * `value` - The value of the log pings option
pub fn set_log_pings(value: bool) {
    if was_initialize_called() {
        with_glean_mut(|glean| glean.set_log_pings(value));
    } else {
        PRE_INIT_LOG_PINGS.store(value, Ordering::SeqCst);
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
pub fn set_source_tags(tags: Vec<String>) {
    if was_initialize_called() {
        crate::launch_with_glean_mut(|glean| {
            glean.set_source_tags(tags);
        });
    } else {
        // Glean has not been initialized yet. Cache the provided source tags.
        let m = PRE_INIT_SOURCE_TAGS.get_or_init(Default::default);
        let mut lock = m.lock().unwrap();
        *lock = tags;
    }
}

/// Returns a timestamp corresponding to "now" with millisecond precision.
pub fn get_timestamp_ms() -> u64 {
    glean_core::get_timestamp_ms()
}

/// Asks the database to persist ping-lifetime data to disk. Probably expensive to call.
/// Only has effect when Glean is configured with `delay_ping_lifetime_io: true`.
/// If Glean hasn't been initialized this will dispatch and return Ok(()),
/// otherwise it will block until the persist is done and return its Result.
pub fn persist_ping_lifetime_data() -> Result<()> {
    if !was_initialize_called() {
        crate::launch_with_glean(|glean| {
            // This is async, we can't get the Error back to the caller.
            let _ = glean.persist_ping_lifetime_data();
        });
        Ok(())
    } else {
        // Calling the dispatcher directly to not panic on errors.
        // Blocking on the queue will fail when the queue is already shutdown,
        // which is equivalent to Glean not being initialized
        // (In production Glean can't be re-initialized).
        dispatcher::block_on_queue().map_err(|_| Error::not_initialized())?;
        with_glean(|glean| glean.persist_ping_lifetime_data())
    }
}

#[cfg(test)]
mod test;
