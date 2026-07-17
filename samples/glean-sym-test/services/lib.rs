// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

/// 256 allocations should be more than enough for us.
/// Chosen empirically.
/// If this crashes with `oom` increase it.
#[global_allocator]
static ALLOCATOR: local_allocator::Allocator::<256> = local_allocator::Allocator::new("services");

#[allow(clippy::all)] // Don't lint generated code.
pub mod glean_metrics {
    include!(concat!(env!("OUT_DIR"), "/glean_metrics.rs"));
}

#[unsafe(no_mangle)]
unsafe extern "C" fn record(amount: i32) {
    env_logger::init();
    let _ = &*glean_metrics::services_info;
    log::info!("Record invoked");

    // A timer ID is passed through a `RustBuffer`,
    // so it needs to be copied and freed correctly.
    let tid = glean_metrics::dylib::timing.start();

    log::info!("new LoginStore! Recording a metric");
    glean_metrics::dylib::counting.add(amount);
    log::info!("Metric recorded.");

    glean_metrics::dylib::data.set(String::from("value"));
    // `StringMetric#test_get_value` returns a string, which is passed through a `RustBuffer`,
    // which needs to be copied and freed correctly.
    let stored = glean_metrics::dylib::data.test_get_value(None).unwrap();
    assert_eq!("value", stored);

    glean_metrics::dylib::event.record(None);
    let extra = glean_metrics::dylib::EventWithExtrasExtra { is_set: Some(true) };
    glean_metrics::dylib::event_with_extras.record(extra);

    glean_metrics::dylib::timing.stop_and_accumulate(tid);

    glean_metrics::services_info.submit(Some("recorded"));
}
