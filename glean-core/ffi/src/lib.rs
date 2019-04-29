use std::os::raw::c_char;

use lazy_static::lazy_static;
use ffi_support::{
    IntoFfi,
    FfiStr,
    ConcurrentHandleMap,
    ExternError,
    define_handle_map_deleter,
    define_string_destructor,
    call_with_output,
};

use glean_core::{
    Glean,
    CommonMetricData,
    metrics::*,
};

lazy_static! {
    static ref BOOLEAN_METRICS: ConcurrentHandleMap<BooleanMetric> = ConcurrentHandleMap::new();
    static ref STRING_METRICS: ConcurrentHandleMap<StringMetric> = ConcurrentHandleMap::new();
    static ref COUNTER_METRICS: ConcurrentHandleMap<CounterMetric> = ConcurrentHandleMap::new();
}

#[no_mangle]
pub extern fn glean_initialize() {
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

    Glean::singleton().initialize("/data/user/0/org.mozilla.samples.glean_rs/data");
    log::info!("Glean.rs initialized");
}

#[no_mangle]
pub extern fn glean_is_initialized() -> u8 {
    Glean::singleton().is_initialized().into_ffi_value()
}

#[no_mangle]
pub extern fn glean_is_upload_enabled() -> u8 {
    Glean::singleton().is_upload_enabled().into_ffi_value()
}

#[no_mangle]
pub extern fn glean_set_upload_enabled(flag: u8) {
    let flag = flag != 0;
    Glean::singleton().set_upload_enabled(flag);
}

#[no_mangle]
pub extern fn glean_new_boolean_metric(name: FfiStr, category: FfiStr, err: &mut ExternError) -> u64 {
    BOOLEAN_METRICS.insert_with_output(err, || {
        BooleanMetric::new(CommonMetricData {
            name: name.into_string(),
            category: category.into_string(),
            send_in_pings: vec!["core".into()],
            .. Default::default()
        })
    })
}

#[no_mangle]
pub extern fn glean_new_string_metric(name: FfiStr, category: FfiStr, err: &mut ExternError) -> u64 {
    STRING_METRICS.insert_with_output(err, || {
        StringMetric::new(CommonMetricData {
            name: name.into_string(),
            category: category.into_string(),
            send_in_pings: vec!["core".into()],
            .. Default::default()
        })
    })
}

#[no_mangle]
pub extern fn glean_new_counter_metric(name: FfiStr, category: FfiStr, err: &mut ExternError) -> u64 {
    COUNTER_METRICS.insert_with_output(err, || {
        CounterMetric::new(CommonMetricData {
            name: name.into_string(),
            category: category.into_string(),
            send_in_pings: vec!["core".into()],
            .. Default::default()
        })
    })
}

#[no_mangle]
pub extern fn glean_counter_add(metric_id: u64, amount: u64, error: &mut ExternError) {
    COUNTER_METRICS.call_with_output(error, metric_id, |metric| {
        metric.add(amount as u32);
        ()
    })
}

#[no_mangle]
pub extern fn glean_ping_collect(ping_name: FfiStr, error: &mut ExternError) -> *mut c_char {
    call_with_output(error, || {
        let ping_maker = glean_core::ping::PingMaker::new();
        let ping = ping_maker.collect_string(ping_name.as_str());
        ping
    })
}

define_handle_map_deleter!(BOOLEAN_METRICS, glean_destroy_boolean_metric);
define_string_destructor!(glean_str_free);
