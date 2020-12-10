use std::ffi::CString;

use cfg_if::cfg_if;
use glean_ffi_sys::*;

cfg_if! {
    if #[cfg(windows)] {
        const LIB_NAME: &str = "libglean_ffi.dll";
    } else if #[cfg(target_os = "macos")] {
        const LIB_NAME: &str = "libglean_ffi.dylib";
    } else {
        const LIB_NAME: &str = "libglean_ffi.so";
    }
}

fn main() {
    unsafe {
        let glean = GleanSys::new(LIB_NAME).unwrap();
        glean.glean_enable_logging();

        let dir = CString::new("./tmp").unwrap();
        let name = CString::new("ffi.sys.usage").unwrap();
        let lang = CString::new("rust-ffi").unwrap();
        let cfg = FfiConfiguration {
            data_dir: dir.as_ptr(),
            package_name: name.as_ptr(),
            language_binding_name: lang.as_ptr(),
            upload_enabled: 1,
            max_events: &500 as *const _,
            delay_ping_lifetime_io: 0,
        };
        glean.glean_initialize(&cfg as *const _);
    }
}
