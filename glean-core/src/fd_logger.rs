// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::fs::File;
use std::io::Write;
use std::sync::RwLock;

#[cfg(target_os = "windows")]
use std::os::windows::io::FromRawHandle;

#[cfg(target_os = "windows")]
use std::ffi::c_void;

#[cfg(not(target_os = "windows"))]
use std::os::unix::io::FromRawFd;

use serde::Serialize;

/// An implementation of log::Log that writes log messages in JSON format to a
/// file descriptor/handle. The logging level is ignored in this implementation:
/// it is up to the receiver of these log messages (on the language binding
/// side) to filter the log messages based on their level.
/// The JSON payload of each message in an object with the following keys:
///    - `level` (string): One of the logging levels defined here:
///      <https://docs.rs/log/0.4.11/log/enum.Level.html>
///    - `message` (string): The logging message.
pub struct FdLogger {
    pub file: RwLock<File>,
}

#[derive(Serialize)]
struct FdLoggingRecord {
    level: String,
    message: String,
    target: String,
}

/// Obtain a `File` from a raw handle.
///
/// # Safety
///
/// The safety requirements of [`File::from_raw_handle`] apply:
///
/// The handled passed in must:
///
/// * be an owned handle; in particular it must be open
/// * be a handle for a resource that may be freed via `CloseHandle`
///
/// # Returns
///
/// Some file valid for use.
#[cfg(target_os = "windows")]
unsafe fn get_file_from_fd(handle: u64) -> Option<File> {
    Some(File::from_raw_handle(handle as *mut c_void))
}

/// Obtain a `File` from a raw file descriptor.
///
/// # Safety
///
/// The safety requirements of [`File::from_raw_handle`] apply:
///
/// The `fd` passed in must be an owned file descriptor; in particular, it must be open.
///
/// # Returns
///
/// Some file valid for use or `None` if the passed `fd` cannot represent a file descriptor.
#[cfg(not(target_os = "windows"))]
unsafe fn get_file_from_fd(fd: u64) -> Option<File> {
    if let Ok(raw_fd) = i32::try_from(fd) {
        Some(File::from_raw_fd(raw_fd))
    } else {
        None
    }
}

impl FdLogger {
    pub unsafe fn new(fd: u64) -> Option<Self> {
        Some(FdLogger {
            file: RwLock::new(get_file_from_fd(fd)?),
        })
    }
}

impl log::Log for FdLogger {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        // This logger always emits logging messages of any level, and the
        // language binding consuming these messages is responsible for
        // filtering and routing them.
        true
    }

    fn log(&self, record: &log::Record) {
        // Normally, classes implementing the Log trait would filter based on
        // the log level here. But in this case, we want to emit all log
        // messages and let the logging system in the language binding filter
        // and route them.
        let payload = FdLoggingRecord {
            level: record.level().to_string(),
            message: record.args().to_string(),
            target: record.target().to_string(),
        };
        let mut file = self.file.write().unwrap(); // safe unwrap, only error case is poisoning
        let _ = serde_json::to_writer(&mut *file, &payload);
    }

    fn flush(&self) {
        let _ = self.file.write().unwrap().flush();
    }
}
