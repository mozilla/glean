use std::io;
use std::thread::{self, JoinHandle};

#[cfg(all(feature = "gecko", not(target_os = "android")))]
mod bindings {
    extern "C" {
        pub fn gecko_profiler_register_thread(name: *const std::ffi::c_char);
        pub fn gecko_profiler_unregister_thread();
    }
}

/// Register a thread with the Gecko Profiler.
#[cfg(all(feature = "gecko", not(target_os = "android")))]
fn register_thread(thread_name: &str) {
    let name = std::ffi::CString::new(thread_name).unwrap();
    unsafe {
        // gecko_profiler_register_thread copies the passed name here.
        bindings::gecko_profiler_register_thread(name.as_ptr());
    }
}

#[cfg(any(not(feature = "gecko"), target_os = "android"))]
fn register_thread(_thread_name: &str) {}

/// Unregister a thread with the Gecko Profiler.
#[cfg(all(feature = "gecko", not(target_os = "android")))]
fn unregister_thread() {
    unsafe {
        bindings::gecko_profiler_unregister_thread();
    }
}
#[cfg(any(not(feature = "gecko"), target_os = "android"))]
fn unregister_thread() {}

/// Spawns a new thread, returning a [`JoinHandle`] for it.
///
/// Wrapper around [`std::thread::spawn`], but automatically naming the thread.
pub fn spawn<F, T>(name: &'static str, f: F) -> Result<JoinHandle<T>, io::Error>
where
    F: FnOnce() -> T + Send + 'static,
    T: Send + 'static,
{
    thread::Builder::new()
        .name(name.to_string())
        .spawn(move || {
            register_thread(name);
            let res = f();
            unregister_thread();
            res
        })
}
