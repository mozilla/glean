// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Policies for ping storage, uploading and requests.

const MAX_RECOVERABLE_FAILURES: u32 = 3;
const MAX_WAIT_ATTEMPTS: u32 = 3;
const MAX_PING_BODY_SIZE: usize = 1024 * 1024; // 1 MB
const MAX_PENDING_PINGS_DIRECTORY_SIZE: u64 = 10 * 1024 * 1024; // 10MB

/// A struct holding the values for all the policies related to ping storage, uploading and requests.
#[derive(Debug)]
pub struct Policy {
    /// The maximum recoverable failures allowed per uploading window.
    ///
    /// Limiting this is necessary to avoid infinite loops on requesting upload tasks.
    max_recoverable_failures: Option<u32>,
    /// The maximum of [`PingUploadTask::Wait`] responses a user may get in a row
    /// when calling [`get_upload_task`].
    ///
    /// Limiting this is necessary to avoid infinite loops on requesting upload tasks.
    max_wait_attempts: Option<u32>,
    /// The maximum size in bytes a ping body may have to be eligible for upload.
    max_ping_body_size: Option<usize>,
    /// The maximum size in byte the pending pings directory may have on disk.
    max_pending_pings_directory_size: Option<u64>,
}

impl Default for Policy {
    fn default() -> Self {
        Policy {
            max_recoverable_failures: Some(MAX_RECOVERABLE_FAILURES),
            max_wait_attempts: Some(MAX_WAIT_ATTEMPTS),
            max_ping_body_size: Some(MAX_PING_BODY_SIZE),
            max_pending_pings_directory_size: Some(MAX_PENDING_PINGS_DIRECTORY_SIZE),
        }
    }
}

impl Policy {
    pub fn max_recoverable_failures(&self) -> u32 {
        match &self.max_recoverable_failures {
            Some(v) => *v,
            None => u32::MAX,
        }
    }

    #[cfg(test)]
    pub fn set_max_recoverable_failures(&mut self, v: Option<u32>) {
        self.max_recoverable_failures = v;
    }

    pub fn max_wait_attempts(&self) -> u32 {
        match &self.max_wait_attempts {
            Some(v) => *v,
            None => u32::MAX,
        }
    }

    #[cfg(test)]
    pub fn set_max_wait_attempts(&mut self, v: Option<u32>) {
        self.max_wait_attempts = v;
    }

    pub fn max_ping_body_size(&self) -> usize {
        match &self.max_ping_body_size {
            Some(v) => *v,
            None => usize::MAX,
        }
    }

    #[cfg(test)]
    pub fn set_max_ping_body_size(&mut self, v: Option<usize>) {
        self.max_ping_body_size = v;
    }

    pub fn max_pending_pings_directory_size(&self) -> u64 {
        match &self.max_pending_pings_directory_size {
            Some(v) => *v,
            None => u64::MAX,
        }
    }

    #[cfg(test)]
    pub fn set_max_pending_pings_directory_size(&mut self, v: Option<u64>) {
        self.max_pending_pings_directory_size = v;
    }
}
