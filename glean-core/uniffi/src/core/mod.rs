use std::sync::Mutex;
use std::path::PathBuf;

use once_cell::sync::OnceCell;

use crate::Configuration;

static GLEAN: OnceCell<Mutex<Glean>> = OnceCell::new();

pub fn global_glean() -> Option<&'static Mutex<Glean>> {
    GLEAN.get()
}

/// Sets or replaces the global Glean object.
pub fn setup_glean(glean: Glean) -> Result<(), ()> {
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
    data_path: PathBuf,
    application_id: String,
    max_events: u32,
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
    pub fn new(cfg: Configuration) -> Result<Self, ()> {
        log::info!("Creating new Glean Uniffi");

        let application_id = sanitize_application_id(&cfg.application_id);
        if application_id.is_empty() {
            return Err(());
        }

        let this = Self {
            upload_enabled: cfg.upload_enabled,
            // In the subprocess, we want to avoid accessing the database entirely.
            // The easiest way to ensure that is to just not initialize it.
            data_path: PathBuf::from(&cfg.data_dir),
            application_id,
            max_events: cfg.max_events.unwrap_or(500),
            schedule_metrics_pings: cfg.use_core_mps,
        };

        Ok(this)
    }

    pub fn set_upload_enabled(&mut self, enabled: bool) {
        self.upload_enabled = enabled;
    }
}
