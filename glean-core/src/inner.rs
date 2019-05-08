use crate::database::Database;

#[derive(Debug)]
pub struct Inner {
    initialized: bool,
    upload_enabled: bool,
    pub data_store: Database,
}

impl Inner {
    pub fn new() -> Self {
        log::info!("Creating new Inner glean");

        Self {
            initialized: false,
            upload_enabled: true,
            data_store: Database::new(),
        }
    }

    pub fn initialize(&mut self, data_path: &str) {
        self.data_store.initialize(data_path);
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
}
