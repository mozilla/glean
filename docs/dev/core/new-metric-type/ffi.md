# Adding a new metric type - FFI layer

In order to use a new metric type over the FFI layer, it needs implementations in the FFI component.

## FFI component

The FFI component implementation can be found in `glean-core/ffi/src`.
Each metric type is implemented in its own module.

Add a new file named after your metric, e.g. `glean-core/ffi/src/counter.rs`, and declare it in `glean-core/ffi/src/lib.rs` with `mod counter;`.

In the metric type module define your metric type using the `define_metric` macro.
This allows referencing the metric name and defines the global map as well as some common functions such as the constructor and destructor.
Simple operations can be also defined in the same macro invocation:


```rust,noplaypen
use crate::{define_metric, handlemap_ext::HandleMapExtension, GLEAN};

define_metric!(CounterMetric => COUNTER_METRICS {
    new           -> glean_new_counter_metric(),
    destroy       -> glean_destroy_counter_metric,

    add -> glean_counter_add(amount: i32),
});
```

More complex operations need to be defined as plain functions.
For example the test helper function for a counter metric can be defined as:

```rust,noplaypen
#[no_mangle]
pub extern "C" fn glean_counter_test_has_value(
    glean_handle: u64,
    metric_id: u64,
    storage_name: FfiStr,
) -> u8 {
    GLEAN.call_infallible(glean_handle, |glean| {
        COUNTER_METRICS.call_infallible(metric_id, |metric| {
            metric
                .test_get_value(glean, storage_name.as_str())
                .is_some()
        })
    })
}
```
