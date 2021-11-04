use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Mutex;

use once_cell::sync::OnceCell;

use crate::debug::DebugOptions;
use crate::database::Database;
use crate::private::{ExperimentMetric, RecordedExperiment};
use crate::InternalConfiguration;
use crate::{ErrorKind, Result};

static GLEAN: OnceCell<Mutex<Glean>> = OnceCell::new();

pub fn global_glean() -> Option<&'static Mutex<Glean>> {
    GLEAN.get()
}

/// Sets or replaces the global Glean object.
pub fn setup_glean(glean: Glean) -> Result<()> {
    // The `OnceCell` type wrapping our Glean is thread-safe and can only be set once.
    // Therefore even if our check for it being empty succeeds, setting it could fail if a
    // concurrent thread is quicker in setting it.
    // However this will not cause a bigger problem, as the second `set` operation will just fail.
    // We can log it and move on.
    //
    // For all wrappers this is not a problem, as the Glean object is intialized exactly once on
    // calling `initialize` on the global singleton and further operations check that it has been
    // initialized.
    if GLEAN.get().is_none() {
        if GLEAN.set(Mutex::new(glean)).is_err() {
            log::warn!(
                "Global Glean object is initialized already. This probably happened concurrently."
            )
        }
    } else {
        // We allow overriding the global Glean object to support test mode.
        // In test mode the Glean object is fully destroyed and recreated.
        // This all happens behind a mutex and is therefore also thread-safe..
        let mut lock = GLEAN.get().unwrap().lock().unwrap();
        *lock = glean;
    }
    Ok(())
}

pub fn with_glean<F, R>(f: F) -> R
where
    F: FnOnce(&Glean) -> R,
{
    let glean = global_glean().expect("Global Glean object not initialized");
    let lock = glean.lock().unwrap();
    f(&lock)
}

pub fn with_glean_mut<F, R>(f: F) -> R
where
    F: FnOnce(&mut Glean) -> R,
{
    let glean = global_glean().expect("Global Glean object not initialized");
    let mut lock = glean.lock().unwrap();
    f(&mut lock)
}

#[derive(Debug)]
pub struct Glean {
    upload_enabled: bool,
    data_store: Option<Database>,
    data_path: PathBuf,
    application_id: String,
    max_events: u32,
    debug: DebugOptions,
    schedule_metrics_pings: bool,
}

fn sanitize_application_id(application_id: &str) -> String {
    let mut last_dash = false;
    application_id
        .chars()
        .filter_map(|x| match x {
            'A'..='Z' | 'a'..='z' | '0'..='9' => {
                last_dash = false;
                Some(x.to_ascii_lowercase())
            }
            _ => {
                let result = if last_dash { None } else { Some('-') };
                last_dash = true;
                result
            }
        })
        .collect()
}

impl Glean {
    pub fn new(cfg: InternalConfiguration) -> Result<Self> {
        log::info!("Creating new Glean Uniffi");

        let application_id = sanitize_application_id(&cfg.application_id);
        if application_id.is_empty() {
            return Err(ErrorKind::InvalidConfig.into());
        }

        let data_path = PathBuf::from(&cfg.data_path);
        let data_store = Some(Database::new(&data_path, cfg.delay_ping_lifetime_io)?);

        let this = Self {
            upload_enabled: cfg.upload_enabled,
            data_store,
            data_path,
            application_id,
            max_events: cfg.max_events.unwrap_or(500),
            debug: DebugOptions::new(),
            schedule_metrics_pings: cfg.use_core_mps,
        };

        Ok(this)
    }

    /// For tests make it easy to create a Glean object using only the required configuration.
    #[cfg(test)]
    pub(crate) fn with_options(
        data_path: &str,
        application_id: &str,
        upload_enabled: bool,
    ) -> Self {
        let cfg = InternalConfiguration {
            data_path: data_path.into(),
            application_id: application_id.into(),
            language_binding_name: "Rust".into(),
            upload_enabled,
            max_events: None,
            delay_ping_lifetime_io: false,
            app_build: "Unknown".into(),
            use_core_mps: false,
        };

        let glean = Self::new(cfg).unwrap();
        glean
    }

    pub fn set_upload_enabled(&mut self, enabled: bool) {
        self.upload_enabled = enabled;
    }

    /// Determines whether upload is enabled.
    ///
    /// When upload is disabled, no data will be recorded.
    pub fn is_upload_enabled(&self) -> bool {
        self.upload_enabled
    }

    /// Gets a handle to the database.
    pub fn storage(&self) -> &Database {
        &self.data_store.as_ref().expect("No database found")
    }

    /// Whether or not this is the first run on this profile.
    pub fn is_first_run(&self) -> bool {
        false
    }

    /// **This is not meant to be used directly.**
    ///
    /// Clears all the metrics that have [`Lifetime::Application`].
    pub fn clear_application_lifetime_metrics(&self) {
        //log::trace!("Clearing Lifetime::Application metrics");
        //if let Some(data) = self.data_store.as_ref() {
        //    data.clear_lifetime(Lifetime::Application);
        //}

        // Set internally handled app lifetime metrics again.
        self.set_application_lifetime_core_metrics();
    }

    /// Sets internally-handled application lifetime metrics.
    fn set_application_lifetime_core_metrics(&self) {
        //self.core_metrics.os.set(self, system::OS);
    }

    /// **This is not meant to be used directly.**
    ///
    /// Sets the value of a "dirty flag" in the permanent storage.
    ///
    /// The "dirty flag" is meant to have the following behaviour, implemented
    /// by the consumers of the FFI layer:
    ///
    /// - on mobile: set to `false` when going to background or shutting down,
    ///   set to `true` at startup and when going to foreground.
    /// - on non-mobile platforms: set to `true` at startup and `false` at
    ///   shutdown.
    ///
    /// At startup, before setting its new value, if the "dirty flag" value is
    /// `true`, then Glean knows it did not exit cleanly and can implement
    /// coping mechanisms (e.g. sending a `baseline` ping).
    pub fn set_dirty_flag(&self, new_value: bool) {
        let _ = new_value;
    }

    /// **This is not meant to be used directly.**
    ///
    /// Checks the stored value of the "dirty flag".
    pub fn is_dirty_flag_set(&self) -> bool {
        false
    }

    /// Indicates that an experiment is running.
    ///
    /// Glean will then add an experiment annotation to the environment
    /// which is sent with pings. This information is not persisted between runs.
    ///
    /// # Arguments
    ///
    /// * `experiment_id` - The id of the active experiment (maximum 30 bytes).
    /// * `branch` - The experiment branch (maximum 30 bytes).
    /// * `extra` - Optional metadata to output with the ping.
    pub fn set_experiment_active(
        &self,
        experiment_id: String,
        branch: String,
        extra: HashMap<String, String>,
    ) {
        let metric = ExperimentMetric::new(self, experiment_id);
        metric.set_active(self, branch, extra);
    }

    /// Indicates that an experiment is no longer running.
    ///
    /// # Arguments
    ///
    /// * `experiment_id` - The id of the active experiment to deactivate (maximum 30 bytes).
    pub fn set_experiment_inactive(&self, experiment_id: String) {
        let metric = ExperimentMetric::new(self, experiment_id);
        metric.set_inactive(self);
    }

    /// **Test-only API (exported for FFI purposes).**
    ///
    /// Gets stored data for the requested experiment.
    ///
    /// # Arguments
    ///
    /// * `experiment_id` - The id of the active experiment (maximum 30 bytes).
    pub fn test_get_experiment_data(&self, experiment_id: String) -> Option<RecordedExperiment> {
        let metric = ExperimentMetric::new(self, experiment_id);
        metric.test_get_value(self)
    }

    /// Sets a debug view tag.
    ///
    /// This will return `false` in case `value` is not a valid tag.
    ///
    /// When the debug view tag is set, pings are sent with a `X-Debug-ID` header with the value of the tag
    /// and are sent to the ["Ping Debug Viewer"](https://mozilla.github.io/glean/book/dev/core/internal/debug-pings.html).
    ///
    /// # Arguments
    ///
    /// * `value` - A valid HTTP header value. Must match the regex: "[a-zA-Z0-9-]{1,20}".
    pub fn set_debug_view_tag(&mut self, value: &str) -> bool {
        self.debug.debug_view_tag.set(value.into())
    }

    /// Return the value for the debug view tag or [`None`] if it hasn't been set.
    ///
    /// The `debug_view_tag` may be set from an environment variable
    /// (`GLEAN_DEBUG_VIEW_TAG`) or through the [`set_debug_view_tag`] function.
    pub(crate) fn debug_view_tag(&self) -> Option<&String> {
        self.debug.debug_view_tag.get()
    }

    /// Sets source tags.
    ///
    /// This will return `false` in case `value` contains invalid tags.
    ///
    /// Ping tags will show in the destination datasets, after ingestion.
    ///
    /// **Note** If one or more tags are invalid, all tags are ignored.
    ///
    /// # Arguments
    ///
    /// * `value` - A vector of at most 5 valid HTTP header values. Individual tags must match the regex: "[a-zA-Z0-9-]{1,20}".
    pub fn set_source_tags(&mut self, value: Vec<String>) -> bool {
        self.debug.source_tags.set(value)
    }

    /// Return the value for the source tags or [`None`] if it hasn't been set.
    ///
    /// The `source_tags` may be set from an environment variable (`GLEAN_SOURCE_TAGS`)
    /// or through the [`set_source_tags`] function.
    pub(crate) fn source_tags(&self) -> Option<&Vec<String>> {
        self.debug.source_tags.get()
    }

    /// Sets the log pings debug option.
    ///
    /// This will return `false` in case we are unable to set the option.
    ///
    /// When the log pings debug option is `true`,
    /// we log the payload of all succesfully assembled pings.
    ///
    /// # Arguments
    ///
    /// * `value` - The value of the log pings option
    pub fn set_log_pings(&mut self, value: bool) -> bool {
        self.debug.log_pings.set(value)
    }

    /// Return the value for the log pings debug option or [`None`] if it hasn't been set.
    ///
    /// The `log_pings` option may be set from an environment variable (`GLEAN_LOG_PINGS`)
    /// or through the [`set_log_pings`] function.
    pub(crate) fn log_pings(&self) -> bool {
        self.debug.log_pings.get().copied().unwrap_or(false)
    }

    /// Performs the collection/cleanup operations required by becoming active.
    ///
    /// This functions generates a baseline ping with reason `active`
    /// and then sets the dirty bit.
    pub fn handle_client_active(&mut self) {
        //if !self.internal_pings.baseline.submit(self, Some("active")) {
        //    log::info!("baseline ping not submitted on active");
        //}

        self.set_dirty_flag(true);
    }

    /// Performs the collection/cleanup operations required by becoming inactive.
    ///
    /// This functions generates a baseline and an events ping with reason
    /// `inactive` and then clears the dirty bit.
    pub fn handle_client_inactive(&mut self) {
        //if !self.internal_pings.baseline.submit(self, Some("inactive")) {
        //    log::info!("baseline ping not submitted on inactive");
        //}

        //if !self.internal_pings.events.submit(self, Some("inactive")) {
        //    log::info!("events ping not submitted on inactive");
        //}

        self.set_dirty_flag(false);
    }

    /// Signals that the environment is ready to submit pings.
    ///
    /// Should be called when Glean is initialized to the point where it can correctly assemble pings.
    /// Usually called from the language binding after all of the core metrics have been set
    /// and the ping types have been registered.
    ///
    /// # Returns
    ///
    /// Whether at least one ping was generated.
    pub fn on_ready_to_submit_pings(&self) -> bool {
        //self.event_data_store.flush_pending_events_on_startup(self)
        true
    }
}
