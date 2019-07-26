// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::collections::HashMap;
use std::convert::TryFrom;
use std::os::raw::c_char;

use ffi_support::{
    define_handle_map_deleter, define_string_destructor, ConcurrentHandleMap, FfiStr,
};
use lazy_static::lazy_static;

use glean_core::Glean;

mod macros;

mod boolean;
mod counter;
mod datetime;
mod event;
mod handlemap_ext;
mod labeled;
mod ping_type;
mod string;
mod string_list;
mod timespan;
mod timing_distribution;
mod uuid;

use handlemap_ext::HandleMapExtension;
use ping_type::PING_TYPES;

lazy_static! {
    static ref GLEAN: ConcurrentHandleMap<Glean> = ConcurrentHandleMap::new();
}

type RawStringArray = *const *const c_char;
type RawIntArray = *const i32;
type RawInt64Array = *const i64;

/// Create a vector of strings from a raw C-like string array.
///
/// Returns an error if any of the strings contain invalid UTF-8 characters.
///
/// ## Safety
///
/// * We check the array pointer for validity (non-null).
/// * FfiStr checks each individual char pointer for validity (non-null).
/// * We discard invalid char pointers (null pointer).
/// * Invalid UTF-8 in any string will return an error from this function.
fn from_raw_string_array(arr: RawStringArray, len: i32) -> glean_core::Result<Vec<String>> {
    unsafe {
        if arr.is_null() || len == 0 {
            return Ok(vec![]);
        }

        let arr_ptrs = std::slice::from_raw_parts(arr, len as usize);
        arr_ptrs
            .iter()
            .map(|&p| {
                // Drop invalid strings
                FfiStr::from_raw(p)
                    .as_opt_str()
                    .map(|s| s.to_string())
                    .ok_or_else(glean_core::Error::utf8_error)
            })
            .collect()
    }
}

/// Create a HashMap<i32, String> from a pair of C int and string arrays.
///
/// Returns an error if any of the strings contain invalid UTF-8 characters.
///
/// ## Safety
///
/// * We check the array pointer for validity (non-null).
/// * FfiStr checks each individual char pointer for validity (non-null).
/// * We discard invalid char pointers (null pointer).
/// * Invalid UTF-8 in any string will return an error from this function.
fn from_raw_int_array_and_string_array(
    keys: RawIntArray,
    values: RawStringArray,
    len: i32,
) -> glean_core::Result<Option<HashMap<i32, String>>> {
    unsafe {
        if keys.is_null() || values.is_null() || len == 0 {
            return Ok(None);
        }

        let keys_ptrs = std::slice::from_raw_parts(keys, len as usize);
        let values_ptrs = std::slice::from_raw_parts(values, len as usize);

        let res: glean_core::Result<_> = keys_ptrs
            .iter()
            .zip(values_ptrs.iter())
            .map(|(&k, &v)| {
                FfiStr::from_raw(v)
                    .as_opt_str()
                    .map(|s| (k, s.to_string()))
                    .ok_or_else(glean_core::Error::utf8_error)
            })
            .collect();
        res.map(Some)
    }
}

/// Create a HashMap<String, String> from a pair of C string arrays.
///
/// Returns an error if any of the strings contain invalid UTF-8 characters.
///
/// ## Safety
///
/// * We check the array pointer for validity (non-null).
/// * FfiStr checks each individual char pointer for validity (non-null).
/// * We discard invalid char pointers (null pointer).
/// * Invalid UTF-8 in any string will return an error from this function.
fn from_raw_string_array_and_string_array(
    keys: RawStringArray,
    values: RawStringArray,
    len: i32,
) -> glean_core::Result<Option<HashMap<String, String>>> {
    unsafe {
        if keys.is_null() || values.is_null() || len == 0 {
            return Ok(None);
        }

        let keys_ptrs = std::slice::from_raw_parts(keys, len as usize);
        let values_ptrs = std::slice::from_raw_parts(values, len as usize);

        let res: glean_core::Result<_> = keys_ptrs
            .iter()
            .zip(values_ptrs.iter())
            .map(|(&k, &v)| {
                let k = FfiStr::from_raw(k)
                    .as_opt_str()
                    .map(|s| s.to_string())
                    .ok_or_else(glean_core::Error::utf8_error)?;

                let v = FfiStr::from_raw(v)
                    .as_opt_str()
                    .map(|s| s.to_string())
                    .ok_or_else(glean_core::Error::utf8_error)?;

                Ok((k, v))
            })
            .collect();
        res.map(Some)
    }
}

/// Create a Vec<u32> from a raw C uint64 array.
///
/// This will return an empty `Vec` if the input is empty.
///
/// ## Safety
///
/// * We check the array pointer for validity (non-null).
fn from_raw_int64_array(values: RawInt64Array, len: i32) -> Vec<i64> {
    unsafe {
        if values.is_null() || len == 0 {
            return vec![];
        }

        let value_slice = std::slice::from_raw_parts(values, len as usize);
        value_slice.to_vec()
    }
}

/// Initialize the logging system based on the target platform. This ensures
/// that logging is shown when executing the Glean SDK unit tests.
#[no_mangle]
pub extern "C" fn glean_enable_logging() {
    #[cfg(target_os = "android")]
    {
        let _ = std::panic::catch_unwind(|| {
            android_logger::init_once(
                android_logger::Filter::default().with_min_level(log::Level::Debug),
                Some("libglean_ffi"),
            );
            log::debug!("Android logging should be hooked up!")
        });
    }
    // Make sure logging does something on non Android platforms as well. Use
    // the RUST_LOG environment variable to set the desired log level, e.g.
    // setting RUST_LOG=debug sets the log level to debug.
    #[cfg(not(target_os = "android"))]
    {
        match env_logger::try_init() {
            Ok(_) => log::debug!("stdout logging should be hooked up!"),
            // Please note that this is only expected to fail during unit tests,
            // where the logger might have already been initialized by a previous
            // test. So it's fine to print with the "logger".
            Err(_) => log::debug!("stdout was already initialized"),
        };
    }
}

/// Configuration over FFI.
///
/// **CAUTION**: This must match _exactly_ the definition on the Kotlin side.
/// If this side is changed, the Kotlin side need to be changed, too.
#[repr(C)]
pub struct FfiConfiguration<'a> {
    data_dir: FfiStr<'a>,
    package_name: FfiStr<'a>,
    upload_enabled: u8,
    max_events: Option<&'a i32>,
}

/// Convert the FFI-compatible configuration object into the proper Rust configuration object.
impl TryFrom<&FfiConfiguration<'_>> for glean_core::Configuration {
    type Error = glean_core::Error;

    fn try_from(cfg: &FfiConfiguration) -> Result<Self, Self::Error> {
        let data_path = cfg
            .data_dir
            .as_opt_str()
            .map(|s| s.to_string())
            .ok_or_else(glean_core::Error::utf8_error)?;
        let application_id = cfg
            .package_name
            .as_opt_str()
            .map(|s| s.to_string())
            .ok_or_else(glean_core::Error::utf8_error)?;
        let upload_enabled = cfg.upload_enabled != 0;
        let max_events = cfg.max_events.map(|m| *m as usize);

        Ok(Self {
            upload_enabled,
            data_path,
            application_id,
            max_events,
        })
    }
}

#[no_mangle]
pub unsafe extern "C" fn glean_initialize(cfg: *const FfiConfiguration) -> u64 {
    assert!(!cfg.is_null());

    GLEAN.insert_with_log(|| {
        // We can create a reference to the FfiConfiguration struct:
        // 1. We did a null check
        // 2. We're not holding on to it beyond this function
        //    and we copy out all data when needed.
        let glean_cfg = glean_core::Configuration::try_from(&*cfg)?;
        let glean = Glean::new(glean_cfg)?;
        log::info!("Glean initialized");
        Ok(glean)
    })
}

#[no_mangle]
pub extern "C" fn glean_on_ready_to_send_pings(glean_handle: u64) {
    GLEAN.call_infallible(glean_handle, |glean| glean.on_ready_to_send_pings())
}

#[no_mangle]
pub extern "C" fn glean_is_upload_enabled(glean_handle: u64) -> u8 {
    GLEAN.call_infallible(glean_handle, |glean| glean.is_upload_enabled())
}

#[no_mangle]
pub extern "C" fn glean_set_upload_enabled(glean_handle: u64, flag: u8) {
    GLEAN.call_infallible_mut(glean_handle, |glean| glean.set_upload_enabled(flag != 0));
    // The return value of set_upload_enabled is an implementation detail
    // that isn't exposed over FFI.
}

#[no_mangle]
pub extern "C" fn glean_send_ping(glean_handle: u64, ping_type_handle: u64, log_ping: u8) -> u8 {
    GLEAN.call_infallible(glean_handle, |glean| {
        PING_TYPES.call_with_log(ping_type_handle, |ping_type| {
            glean.send_ping(ping_type, log_ping != 0)
        })
    })
}

#[no_mangle]
pub extern "C" fn glean_send_ping_by_name(
    glean_handle: u64,
    ping_name: FfiStr,
    log_ping: u8,
) -> u8 {
    GLEAN.call_with_log(glean_handle, |glean| {
        glean.send_ping_by_name(ping_name.as_str(), log_ping != 0)
    })
}

#[no_mangle]
pub extern "C" fn glean_ping_collect(glean_handle: u64, ping_type_handle: u64) -> *mut c_char {
    GLEAN.call_infallible(glean_handle, |glean| {
        PING_TYPES.call_infallible(ping_type_handle, |ping_type| {
            let ping_maker = glean_core::ping::PingMaker::new();
            let data = ping_maker
                .collect_string(glean, ping_type)
                .unwrap_or_else(|| String::from(""));
            log::info!("Ping({}): {}", ping_type.name.as_str(), data);
            data
        })
    })
}

#[no_mangle]
pub extern "C" fn glean_set_experiment_active(
    glean_handle: u64,
    experiment_id: FfiStr,
    branch: FfiStr,
    extra_keys: RawStringArray,
    extra_values: RawStringArray,
    extra_len: i32,
) {
    GLEAN.call_with_log(glean_handle, |glean| {
        let extra = from_raw_string_array_and_string_array(extra_keys, extra_values, extra_len)?;
        glean.set_experiment_active(
            experiment_id.as_str().to_string(),
            branch.as_str().to_string(),
            extra,
        );
        Ok(())
    })
}

#[no_mangle]
pub extern "C" fn glean_set_experiment_inactive(glean_handle: u64, experiment_id: FfiStr) {
    GLEAN.call_infallible(glean_handle, |glean| {
        glean.set_experiment_inactive(experiment_id.as_str().to_string());
    })
}

#[no_mangle]
pub extern "C" fn glean_experiment_test_is_active(glean_handle: u64, experiment_id: FfiStr) -> u8 {
    GLEAN.call_infallible(glean_handle, |glean| {
        glean.test_is_experiment_active(experiment_id.as_str().to_string())
    })
}

#[no_mangle]
pub extern "C" fn glean_experiment_test_get_data(
    glean_handle: u64,
    experiment_id: FfiStr,
) -> *mut c_char {
    GLEAN.call_infallible(glean_handle, |glean| {
        glean.test_get_experiment_data_as_json(experiment_id.as_str().to_string())
    })
}

define_handle_map_deleter!(GLEAN, glean_destroy_glean);
define_string_destructor!(glean_str_free);

#[cfg(test)]
mod test {
    use super::*;
    use std::ffi::CString;

    mod raw_string_array {
        use super::*;

        #[test]
        fn parsing_valid_array() {
            let expected = vec!["first", "second"];
            let array: Vec<CString> = expected
                .iter()
                .map(|&s| CString::new(&*s).unwrap())
                .collect();
            let ptr_array: Vec<*const _> = array.iter().map(|s| s.as_ptr()).collect();

            let list = from_raw_string_array(ptr_array.as_ptr(), expected.len() as i32).unwrap();
            assert_eq!(expected, list);
        }

        #[test]
        fn parsing_empty_array() {
            let expected: Vec<String> = vec![];

            // Testing a null pointer (length longer to ensure the null pointer is checked)
            let list = from_raw_string_array(std::ptr::null(), 2).unwrap();
            assert_eq!(expected, list);

            // Need a (filled) vector to obtain a valid pointer.
            let array = vec![CString::new("glean").unwrap()];
            let ptr_array: Vec<*const _> = array.iter().map(|s| s.as_ptr()).collect();

            // Check the length with a valid pointer.
            let list = from_raw_string_array(ptr_array.as_ptr(), 0).unwrap();
            assert_eq!(expected, list);
        }

        #[test]
        fn parsing_invalid_utf8_fails() {
            // CAREFUL! We're manually constructing nul-terminated

            // Need a (filled) vector to obtain a valid pointer.
            let array = vec![
                // -1 is definitely an invalid UTF-8 codepoint
                // Let's not break anything and append the nul terminator
                vec![0x67, 0x6c, -1, 0x65, 0x61, 0x6e, 0x00],
            ];
            let ptr_array: Vec<*const _> = array.iter().map(|s| s.as_ptr()).collect();

            let list = from_raw_string_array(ptr_array.as_ptr(), array.len() as i32);
            assert!(list.is_err());
        }
    }

    mod raw_int_string_array {
        use super::*;

        #[test]
        fn parsing_valid_array() {
            let mut expected_map = HashMap::new();
            expected_map.insert(7, "seven".to_string());
            expected_map.insert(8, "eight".to_string());

            let int_array = vec![7, 8];
            let str_array = vec![
                CString::new("seven").unwrap(),
                CString::new("eight").unwrap(),
            ];
            let ptr_array: Vec<*const _> = str_array.iter().map(|s| s.as_ptr()).collect();

            let map = from_raw_int_array_and_string_array(
                int_array.as_ptr(),
                ptr_array.as_ptr(),
                expected_map.len() as i32,
            )
            .unwrap();
            assert_eq!(Some(expected_map), map);
        }

        #[test]
        fn parsing_empty_array() {
            // Testing a null pointer (length longer to ensure the null pointer is checked)
            let result =
                from_raw_int_array_and_string_array(std::ptr::null(), std::ptr::null(), 2).unwrap();
            assert_eq!(None, result);

            // Need a (filled) vector to obtain a valid pointer.
            let int_array = vec![1];
            let result =
                from_raw_int_array_and_string_array(int_array.as_ptr(), std::ptr::null(), 2)
                    .unwrap();
            assert_eq!(None, result);

            let array = vec![CString::new("glean").unwrap()];
            let ptr_array: Vec<*const _> = array.iter().map(|s| s.as_ptr()).collect();
            let result =
                from_raw_int_array_and_string_array(std::ptr::null(), ptr_array.as_ptr(), 2)
                    .unwrap();
            assert_eq!(None, result);

            // Check the length with valid pointers.
            let result =
                from_raw_int_array_and_string_array(int_array.as_ptr(), ptr_array.as_ptr(), 0)
                    .unwrap();
            assert_eq!(None, result);
        }

        #[test]
        fn parsing_invalid_utf8_fails() {
            // CAREFUL! We're manually constructing nul-terminated

            // Need a (filled) vector to obtain a valid pointer.
            let int_array = vec![1];
            let array = vec![
                // -1 is definitely an invalid UTF-8 codepoint
                // Let's not break anything and append the nul terminator
                vec![0x67, 0x6c, -1, 0x65, 0x61, 0x6e, 0x00],
            ];
            let ptr_array: Vec<*const _> = array.iter().map(|s| s.as_ptr()).collect();

            let map = from_raw_int_array_and_string_array(
                int_array.as_ptr(),
                ptr_array.as_ptr(),
                array.len() as i32,
            );
            assert!(map.is_err());
        }
    }

    mod raw_string_string_array {
        use super::*;

        #[test]
        fn parsing_valid_array() {
            let mut expected_map = HashMap::new();
            expected_map.insert("one".to_string(), "seven".to_string());
            expected_map.insert("two".to_string(), "eight".to_string());

            let key_array = vec![CString::new("one").unwrap(), CString::new("two").unwrap()];
            let ptr_key_array: Vec<*const _> = key_array.iter().map(|s| s.as_ptr()).collect();

            let str_array = vec![
                CString::new("seven").unwrap(),
                CString::new("eight").unwrap(),
            ];
            let ptr_array: Vec<*const _> = str_array.iter().map(|s| s.as_ptr()).collect();

            let map = from_raw_string_array_and_string_array(
                ptr_key_array.as_ptr(),
                ptr_array.as_ptr(),
                expected_map.len() as i32,
            )
            .unwrap();
            assert_eq!(Some(expected_map), map);
        }

        #[test]
        fn parsing_empty_array() {
            // Testing a null pointer (length longer to ensure the null pointer is checked)
            let result =
                from_raw_string_array_and_string_array(std::ptr::null(), std::ptr::null(), 2)
                    .unwrap();
            assert_eq!(None, result);

            // Need a (filled) vector to obtain a valid pointer.
            let key_array = vec![CString::new("one").unwrap()];
            let ptr_key_array: Vec<*const _> = key_array.iter().map(|s| s.as_ptr()).collect();

            let str_array = vec![CString::new("seven").unwrap()];
            let ptr_array: Vec<*const _> = str_array.iter().map(|s| s.as_ptr()).collect();
            let result =
                from_raw_string_array_and_string_array(ptr_key_array.as_ptr(), std::ptr::null(), 2)
                    .unwrap();
            assert_eq!(None, result);

            let result =
                from_raw_int_array_and_string_array(std::ptr::null(), ptr_array.as_ptr(), 2)
                    .unwrap();
            assert_eq!(None, result);

            // Check the length with valid pointers.
            let result = from_raw_string_array_and_string_array(
                ptr_key_array.as_ptr(),
                ptr_array.as_ptr(),
                0,
            )
            .unwrap();
            assert_eq!(None, result);
        }

        #[test]
        fn parsing_invalid_utf8_fails() {
            // CAREFUL! We're manually constructing nul-terminated

            // Need a (filled) vector to obtain a valid pointer.
            let key_array = vec![CString::new("one").unwrap()];
            let ptr_key_array: Vec<*const _> = key_array.iter().map(|s| s.as_ptr()).collect();
            let array = vec![
                // -1 is definitely an invalid UTF-8 codepoint
                // Let's not break anything and append the nul terminator
                vec![0x67, 0x6c, -1, 0x65, 0x61, 0x6e, 0x00],
            ];
            let ptr_array: Vec<*const _> = array.iter().map(|s| s.as_ptr()).collect();

            let map = from_raw_string_array_and_string_array(
                ptr_key_array.as_ptr(),
                ptr_array.as_ptr(),
                array.len() as i32,
            );
            assert!(map.is_err());
        }
    }
}
