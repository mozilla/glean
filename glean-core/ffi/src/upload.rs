use std::ffi::CString;
use std::os::raw::c_char;

use ffi_support::IntoFfi;

use crate::glean_str_free;
use glean_core::upload::PingUploadTask;

/// A FFI-compatible representation for the PingUploadTask
#[repr(u8)]
pub enum FfiPingUploadTask {
    Wait,
    Upload {
        uuid: *mut c_char,
        path: *mut c_char,
        body: *mut c_char,
        headers: *mut c_char,
    },
    Done,
}

impl From<PingUploadTask> for FfiPingUploadTask {
    fn from(task: PingUploadTask) -> Self {
        match task {
            PingUploadTask::Wait => FfiPingUploadTask::Wait,
            PingUploadTask::Upload(request) => {
                // Safe unwraps:
                // 1. CString::new(..) should not fail as we are the ones that created the strings being transformed;
                // 2. serde_json::to_string(&request.body) should not fail as request.body is a JsonValue;
                // 3. serde_json::to_string(&request.headers) should not fail as request.headers is a HashMap of Strings.
                let uuid = CString::new(request.uuid.to_owned()).unwrap();
                let path = CString::new(request.path.to_owned()).unwrap();
                let body = CString::new(serde_json::to_string(&request.body).unwrap()).unwrap();
                let headers =
                    CString::new(serde_json::to_string(&request.headers).unwrap()).unwrap();
                FfiPingUploadTask::Upload {
                    uuid: uuid.into_raw(),
                    path: path.into_raw(),
                    body: body.into_raw(),
                    headers: headers.into_raw(),
                }
            }
            PingUploadTask::Done => FfiPingUploadTask::Done,
        }
    }
}

impl Drop for FfiPingUploadTask {
    fn drop(&mut self) {
        if let FfiPingUploadTask::Upload {
            uuid,
            path,
            body,
            headers,
        } = self
        {
            // We need to free the previously allocated strings before dropping.
            unsafe {
                glean_str_free(*uuid);
                glean_str_free(*path);
                glean_str_free(*body);
                glean_str_free(*headers);
            }
        }
    }
}

unsafe impl IntoFfi for FfiPingUploadTask {
    type Value = FfiPingUploadTask;

    #[inline]
    fn ffi_default() -> FfiPingUploadTask {
        FfiPingUploadTask::Done
    }

    #[inline]
    fn into_ffi_value(self) -> FfiPingUploadTask {
        self
    }
}
