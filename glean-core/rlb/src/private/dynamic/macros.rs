// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

/// Call the `glean_new_*` function for a specified metric type.
///
/// Automatically converts the common metric data to its FFI equivalent.
/// Additional parameters need to be converted and passed in FFI-compatible.
///
/// # Arguments
///
/// * `$fn`    - The constructor function to call, e.g. `glean_new_counter_metric`.
/// * `$meta`  - The `CommonMetricData` to pass in.
/// * `$extra` - (optional) Any optional extra arguments passed to the function. Need to be
///              FFI-compatible.
#[macro_export]
macro_rules! new_metric {
    ($fn:ident, $meta:ident $(, $extra:expr)*) => {{
        let category = ::std::ffi::CString::new($meta.category).unwrap();
        let name = ::std::ffi::CString::new($meta.name).unwrap();

        let send_in_pings_len = $meta.send_in_pings.len();
        let send_in_pings: Vec<::std::ffi::CString> = $meta
            .send_in_pings
            .into_iter()
            .map(|s| ::std::ffi::CString::new(s).unwrap())
            .collect();
        let send_in_pings_ptr: Vec<*const i8> = send_in_pings.iter().map(|s| s.as_ptr()).collect();

        let lifetime = $meta.lifetime;
        let disabled = $meta.disabled as u8;

        let id = crate::sys::with_glean(|glean| unsafe {
            glean.$fn(
                category.as_ptr(),
                name.as_ptr(),
                send_in_pings_ptr.as_ptr(),
                send_in_pings_len as i32,
                lifetime,
                disabled,
                $($extra ,)*
            )
        });
        id.unwrap_or(0)
    }}
}
