// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! **THIS IS HIGHLY UNSAFE**
//!
//! We deliberately override pthread symbols here to control how threads are launched,
//! in order to simulate failures in this thread launching.
//! It works on Linux and macOS only.
//! It's only a test.
//!
//! Launching a thread can fail for various reasons, e.g. not enough memory resources available.
//! We saw that issue on low-powered machines running 32-bit versions of Windows.
//!
//! You want to run this example with `RUSTFLAGS="-C panic=abort"` to make sure it aborts.

use std::env;
use std::path::PathBuf;

use once_cell::sync::Lazy;
use tempfile::Builder;

use glean::{private::PingType, ClientInfoMetrics, ConfigurationBuilder};

#[cfg(unix)]
mod unix {
    use std::os::raw::{c_int, c_void};
    use std::sync::atomic::{AtomicU32, Ordering};

    /// Tracking how many threads have already been spawned.
    static ALLOW_THREAD_SPAWNED: AtomicU32 = AtomicU32::new(0);

    #[allow(non_camel_case_types)]
    type pthread_ft = extern "C" fn(
        native: *mut libc::pthread_t,
        attr: *const libc::pthread_attr_t,
        f: extern "C" fn(*mut c_void) -> *mut c_void,
        value: *mut c_void,
    ) -> c_int;

    #[no_mangle]
    pub unsafe extern "C" fn pthread_create(
        native: *mut libc::pthread_t,
        attr: *const libc::pthread_attr_t,
        f: extern "C" fn(*mut c_void) -> *mut c_void,
        value: *mut c_void,
    ) -> c_int {
        let name = c"pthread_create".as_ptr();
        let symbol = libc::dlsym(libc::RTLD_NEXT, name);
        if symbol.is_null() {
            panic!("dlsym failed to load `pthread_create` name. Nothing we can do, we abort.");
        }

        let real_pthread_create = *(&symbol as *const *mut _ as *const pthread_ft);

        // thread 1 = glean.initialize
        // thread 2 = ping directory processor
        // thread 3 = MPS
        // thread 4 = uploader for first metrics ping <- this is the one we want to fail
        // thread 5 = uploader for prototype ping <- this is the one we want to fail
        // thread 6 = post-init uploader <- this needs to fail, too
        // thread 7 = shutdown wait thread
        let spawned = ALLOW_THREAD_SPAWNED.fetch_add(1, Ordering::SeqCst);
        if spawned == 4 || spawned == 5 || spawned == 6 {
            return -1;
        }

        real_pthread_create(native, attr, f, value)
    }
}

pub mod glean_metrics {
    use glean::{private::BooleanMetric, CommonMetricData, Lifetime};

    #[allow(non_upper_case_globals)]
    pub static sample_boolean: once_cell::sync::Lazy<BooleanMetric> =
        once_cell::sync::Lazy::new(|| {
            BooleanMetric::new(CommonMetricData {
                name: "sample_boolean".into(),
                category: "test.metrics".into(),
                send_in_pings: vec!["prototype".into()],
                disabled: false,
                lifetime: Lifetime::Ping,
                ..Default::default()
            })
        });
}

#[allow(non_upper_case_globals)]
pub static PrototypePing: Lazy<PingType> = Lazy::new(|| {
    PingType::new(
        "prototype",
        true,
        true,
        true,
        true,
        true,
        vec![],
        vec![],
        true,
        vec![],
    )
});

fn main() {
    env_logger::init();

    let mut args = env::args().skip(1);

    let data_path = if let Some(path) = args.next() {
        PathBuf::from(path)
    } else {
        let root = Builder::new().prefix("simple-db").tempdir().unwrap();
        root.path().to_path_buf()
    };

    _ = &*PrototypePing;
    let cfg = ConfigurationBuilder::new(true, data_path, "org.mozilla.glean_core.example")
        .with_server_endpoint("invalid-test-host")
        .with_use_core_mps(true)
        .build();

    let client_info = ClientInfoMetrics {
        app_build: env!("CARGO_PKG_VERSION").to_string(),
        app_display_version: env!("CARGO_PKG_VERSION").to_string(),
        channel: None,
        locale: None,
    };

    glean::initialize(cfg, client_info);

    glean_metrics::sample_boolean.set(true);

    PrototypePing.submit(None);

    glean::shutdown();
}
