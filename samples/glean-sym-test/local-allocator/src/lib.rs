// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! A local allocator that does not free memory and aborts if pointers are freed that haven't been
//! allocated by it.
//!
//! The allocator uses a stack-allocated array to keep track of pointers it handed out.
//! On `dealloc` it checks that the pointer is known and if not it aborts the process.
//!
//! If it runs out of tracking slots as defined by `N` at allocation time it aborts the process.

use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::RwLock;
use std::sync::atomic::{AtomicUsize, Ordering};

// Non-allocating logger.
//
// Uses libc's `snprintf` and `write` with a stack-allocated buffer to format and print to `stderr`.
// This avoids any allocation and thus makes it safe to use within the allocator itself.
macro_rules! elog {
    () => { elog!("") };
    ($fmt:expr) => { elog!($fmt, ()) };
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

pub struct Allocator<const N: usize> {
    name: &'static str,
    map: RwLock<[usize; N]>,
    idx: AtomicUsize,
}

unsafe impl<const N: usize> Sync for Allocator<N> {}

impl<const N: usize> Allocator<N> {
    /// Create a new allocator with space for `N` allocations.
    pub const fn new(name: &'static str) -> Self {
        Allocator {
            name,
            map: RwLock::new([0; N]),
            idx: AtomicUsize::new(0),
        }
    }
}

unsafe impl<const N: usize> GlobalAlloc for Allocator<N> {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        unsafe {
            let ptr = System.alloc(layout);

            {
                let idx = self.idx.fetch_add(1, Ordering::SeqCst);
                let map = &mut *self.map.write().unwrap();
                if idx >= map.len() {
                    elog!("oom");
                    // Immediately abort the process.
                    std::process::abort();
                }
                map[idx] = ptr as usize;
            }

            ptr
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        unsafe {
            let max_idx = self.idx.load(Ordering::SeqCst);
            let map = &*self.map.read().unwrap();

            let found = (0..max_idx).any(|idx| map[idx] == ptr as usize);
            if !found {
                elog!(
                    "%.*s: Trying to dealloc=%p. Pointer wasn't allocated here.",
                    self.name.len() as i32,
                    self.name.as_ptr(),
                    ptr
                );

                // Immediately abort the process.
                std::process::abort();
            }

            // Intentionally not deallocating any memory.
            // Memory will not be freed, pointers will not be reused.
        }
    }
}
