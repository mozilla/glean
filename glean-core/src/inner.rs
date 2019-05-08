use crate::database::Database;
use std::path::PathBuf;

use crate::util::sanitize_application_id;

#[derive(Debug)]
pub struct Inner {
    initialized: bool,
    upload_enabled: bool,
    pub data_store: Database,
    data_path: Option<PathBuf>,
    application_id: Option<String>,
}

impl Inner {
    pub fn new() -> Self {
        log::info!("Creating new Inner glean");

        Self {
            initialized: false,
            upload_enabled: true,
            data_store: Database::new(),
            data_path: None,
            application_id: None,
        }
    }

    pub fn initialize(&mut self, data_path: &str, application_id: &str) {
        self.data_store.initialize(data_path);
        self.data_path = Some(PathBuf::from(data_path));
        self.application_id = Some(sanitize_application_id(application_id));
        self.initialized = true;
    }

    pub fn is_initialized(&self) -> bool {
        self.initialized
    }

    pub fn set_upload_enabled(&mut self, flag: bool) {
        self.upload_enabled = flag;
    }

    pub fn is_upload_enabled(&self) -> bool {
        self.upload_enabled
    }

    pub fn get_application_id(&self) -> &str {
        // TODO: Error handling?
        self.application_id.as_ref().unwrap()
    }

    pub fn get_data_path(&self) -> &PathBuf {
        // TODO: Error handling?
        self.data_path.as_ref().unwrap()
    }
}
