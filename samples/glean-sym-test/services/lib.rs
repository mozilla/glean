// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

macro_rules! eprintln {
    () => { eprintln!("") };
    ($fmt:expr) => { eprintln!($fmt, ()) };
    ($fmt:expr, $($arg:expr),*) => {{
        use std::ffi::{c_char, c_int};
        #[allow(non_camel_case_types)]
        type c_size_t = usize;
        #[allow(non_camel_case_types)]
        type c_ssize_t = isize;
        unsafe extern "C" {
            fn snprintf(buf: *mut c_char, buf_size: c_size_t, format: *const c_char, ...) -> c_int;
            fn write(fildes: c_int, buf: *const c_char, nbyte: c_size_t) -> c_ssize_t;
        }
        let mut buf = [0; 128];
        let fmt = concat!($fmt, "\n\0");
        let n = snprintf(buf.as_mut_ptr(), buf.len(), fmt.as_ptr() as _, $($arg),*);
        debug_assert!((n as usize) < buf.len());
        _ = write(2, buf.as_ptr(), n as c_size_t);

    }};
}

use std::alloc::{Layout, GlobalAlloc, System};

#[global_allocator]
static ALLOCATOR: LogAlloc = LogAlloc;

struct LogAlloc;

unsafe impl GlobalAlloc for LogAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        unsafe {
            let ptr = System.alloc(layout);
            eprintln!("services.alloc=%p", ptr);
            ptr
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe {
            eprintln!("services.dealloc=%p", ptr);
            System.dealloc(ptr, layout)
        }
    }
}

#[allow(clippy::all)] // Don't lint generated code.
pub mod glean_metrics {
    include!(concat!(env!("OUT_DIR"), "/glean_metrics.rs"));
}

#[unsafe(no_mangle)]
unsafe extern "C" fn record(amount: i32) {
    env_logger::init();
    log::info!("Record invoked");

    let tid = glean_metrics::dylib::timing.start();

    log::info!("new LoginStore! Recording a metric");
    glean_metrics::dylib::counting.add(amount);
    log::info!("Metric recorded.");

    glean_metrics::dylib::timing.stop_and_accumulate(tid);
}
