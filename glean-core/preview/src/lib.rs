// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

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
//! # use glean_preview::{Configuration, ClientInfoMetrics, Error, metrics::*};
//! # fn main() -> Result<(), Error> {
//! let cfg = Configuration {
//!     data_path: "/tmp/data".into(),
//!     application_id: "org.mozilla.glean_core.example".into(),
//!     upload_enabled: true,
//!     max_events: None,
//!     delay_ping_lifetime_io: false,
//!     channel: None,
//! };
//! glean_preview::initialize(cfg, ClientInfoMetrics::unknown())?;
//!
//! let prototype_ping = PingType::new("prototype", true, true, vec!());
//!
//! glean_preview::register_ping_type(&prototype_ping);
//!
//! prototype_ping.submit(None);
//! # Ok(())
//! # }
//! ```

use once_cell::sync::OnceCell;
use std::sync::Mutex;

pub use configuration::Configuration;
pub use core_metrics::ClientInfoMetrics;
pub use glean_core::{global_glean, setup_glean, CommonMetricData, Error, Glean, Lifetime, Result};

mod configuration;
mod core_metrics;
pub mod metrics;
mod system;

const LANGUAGE_BINDING_NAME: &str = "Rust";

/// Application state to keep track of.
#[derive(Debug)]
struct AppState {
    /// The channel the application is being distributed on.
    channel: Option<String>,

    /// Client info metrics set by the application.
    client_info: ClientInfoMetrics,
}

/// A global singleton storing additional state for Glean.
///
/// Requires a Mutex, because in tests we can actual reset this.
static STATE: OnceCell<Mutex<AppState>> = OnceCell::new();

/// Get a reference to the global state object.
///
/// Panics if no global state object was set.
fn global_state() -> &'static Mutex<AppState> {
    STATE.get().unwrap()
}

/// Set or replace the global Glean object.
fn setup_state(state: AppState) {
    if STATE.get().is_none() {
        STATE.set(Mutex::new(state)).unwrap();
    } else {
        let mut lock = STATE.get().unwrap().lock().unwrap();
        *lock = state;
    }
}

fn with_glean<F, R>(f: F) -> R
where
    F: Fn(&Glean) -> R,
{
    let glean = global_glean().expect("Global Glean object not initialized");
    let lock = glean.lock().unwrap();
    f(&lock)
}

fn with_glean_mut<F, R>(f: F) -> R
where
    F: Fn(&mut Glean) -> R,
{
    let glean = global_glean().expect("Global Glean object not initialized");
    let mut lock = glean.lock().unwrap();
    f(&mut lock)
}

/// Creates and initializes a new Glean object.
///
/// See `glean_core::Glean::new`.
pub fn initialize(cfg: Configuration, client_info: ClientInfoMetrics) -> Result<()> {
    let core_cfg = glean_core::Configuration {
        upload_enabled: cfg.upload_enabled,
        data_path: cfg.data_path.clone(),
        application_id: cfg.application_id.clone(),
        language_binding_name: LANGUAGE_BINDING_NAME.into(),
        max_events: cfg.max_events,
        delay_ping_lifetime_io: cfg.delay_ping_lifetime_io,
    };
    let glean = Glean::new(core_cfg)?;

    // First initialize core metrics
    initialize_core_metrics(&glean, &client_info, cfg.channel.clone());

    // Now make this the global object available to others.
    setup_state(AppState {
        channel: cfg.channel,
        client_info,
    });
    glean_core::setup_glean(glean)?;

    Ok(())
}

fn initialize_core_metrics(
    glean: &Glean,
    client_info: &ClientInfoMetrics,
    channel: Option<String>,
) {
    let core_metrics = core_metrics::InternalMetrics::new();

    core_metrics
        .app_build
        .set(glean, &client_info.app_build[..]);
    core_metrics
        .app_display_version
        .set(glean, &client_info.app_display_version[..]);
    if let Some(app_channel) = channel {
        core_metrics.app_channel.set(glean, app_channel);
    }
    core_metrics.os_version.set(glean, "unknown".to_string());
    core_metrics
        .architecture
        .set(glean, system::ARCH.to_string());
    core_metrics
        .device_manufacturer
        .set(glean, "unknown".to_string());
    core_metrics.device_model.set(glean, "unknown".to_string());
}

/// Sets whether upload is enabled or not.
///
/// See `glean_core::Glean.set_upload_enabled`.
pub fn set_upload_enabled(enabled: bool) -> bool {
    with_glean_mut(|glean| {
        let state = global_state().lock().unwrap();
        let old_enabled = glean.is_upload_enabled();
        glean.set_upload_enabled(enabled);

        if !old_enabled && enabled {
            // If uploading is being re-enabled, we have to restore the
            // application-lifetime metrics.
            initialize_core_metrics(&glean, &state.client_info, state.channel.clone());
        }

        enabled
    })
}

/// Determines whether upload is enabled.
///
/// See `glean_core::Glean.is_upload_enabled`.
pub fn is_upload_enabled() -> bool {
    with_glean(|glean| glean.is_upload_enabled())
}

/// Register a new [`PingType`](metrics/struct.PingType.html).
pub fn register_ping_type(ping: &metrics::PingType) {
    with_glean_mut(|glean| {
        glean.register_ping_type(&ping.ping_type);
    })
}

/// Collects and submits a ping for eventual uploading.
///
/// See `glean_core::Glean.submit_ping`.
///
/// # Returns
///
/// Whether the ping was successfully assembled and queued.
pub fn submit_ping(ping: &metrics::PingType, reason: Option<&str>) -> bool {
    submit_ping_by_name(&ping.name, reason)
}

/// Collects and submits a ping for eventual uploading by name.
///
/// See `glean_core::Glean.submit_ping_by_name`.
///
/// # Returns
///
/// Whether the ping was succesfully assembled and queued.
pub fn submit_ping_by_name(ping: &str, reason: Option<&str>) -> bool {
    with_glean(|glean| glean.submit_ping_by_name(ping, reason).unwrap_or(false))
}

#[cfg(test)]
mod test;
