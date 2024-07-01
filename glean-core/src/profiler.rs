#[cfg(feature = "gecko")]
use std::ffi::CString;

#[cfg(feature = "gecko")]
mod bindings {
    use std::ffi::c_char;

    extern "C" {
        pub fn gecko_profiler_register_thread(name: *const c_char);
        pub fn gecko_profiler_unregister_thread();
    }
}

/// Register a thread with the Gecko Profiler.
#[cfg(feature = "gecko")]
pub fn register_thread(thread_name: &str) {
    let name = CString::new(thread_name).unwrap();
    unsafe {
        // gecko_profiler_register_thread copies the passed name here.
        bindings::gecko_profiler_register_thread(name.as_ptr());
    }
}

#[cfg(not(feature = "gecko"))]
pub fn register_thread(_thread_name: &str) {}

/// Unregister a thread with the Gecko Profiler.
#[cfg(feature = "gecko")]
pub fn unregister_thread() {
    unsafe {
        bindings::gecko_profiler_unregister_thread();
    }
}
#[cfg(not(feature = "gecko"))]
pub fn unregister_thread() {}
