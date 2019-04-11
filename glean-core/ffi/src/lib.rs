use lazy_static::lazy_static;
use ffi_support::{
    FfiStr,
    ConcurrentHandleMap,
    ExternError,
    define_handle_map_deleter,
    define_string_destructor,
};

use glean_core::{
    Glean,
    CommonMetricData,
    metrics::BooleanMetric,
};

lazy_static! {
    static ref BOOLEAN_METRICS: ConcurrentHandleMap<BooleanMetric> = ConcurrentHandleMap::new();
}

#[no_mangle]
pub extern fn glean_initialize() {
    Glean::initialize();
}

#[no_mangle]
pub extern fn glean_new_boolean_metric(name: FfiStr<'_>, category: FfiStr<'_>, err: &mut ExternError) -> u64 {
    BOOLEAN_METRICS.insert_with_output(err, || {
        BooleanMetric::new(CommonMetricData {
            name: name.into_string(),
            category: category.into_string(),
            .. Default::default()
        })
    })
}

define_handle_map_deleter!(BOOLEAN_METRICS, glean_destroy_boolean_metric);
define_string_destructor!(glean_destroy_string);
