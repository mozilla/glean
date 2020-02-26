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
                let uuid = CString::new(request.uuid()).unwrap();
                let path = CString::new(request.path()).unwrap();
                let body = CString::new(request.body()).unwrap();
                let headers = CString::new(request.headers()).unwrap();
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
