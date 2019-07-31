/// Define the global handle map, constructor and destructor functions and any user-defined
/// functions for a new metric
///
/// This allows to define most common functionality and simple operations for a metric type.
/// More complex operations should be written as plain functions directly.
///
/// ## Arguments
///
/// * `$metric_type` - metric type to use from glean_core, e.g. `CounterMetric`.
/// * `$metric_map` - name to use for the global name, should be all uppercase, e.g. `COUNTER_METRICS`.
/// * `$new_fn(...)` - (optional) name of the constructor function, followed by all additional (non-common) arguments.
/// * `$destroy` - name of the destructor function.
///
/// Additional simple functions can be define as a mapping `$op -> $op_fn`:
///
/// * `$op` - function on the metric type to call.
/// * `$op_fn` - FFI function name for the operation, followed by its arguments.
///              Arguments are converted into the target type using `TryFrom::try_from`.
#[macro_export]
macro_rules! define_metric {
    ($metric_type:ident => $metric_map:ident {
        $(new -> $new_fn:ident($($new_argname:ident: $new_argtyp:ty),* $(,)*),)?
        destroy -> $destroy_fn:ident,

        $(
            $op:ident -> $op_fn:ident($($op_argname:ident: $op_argtyp:ty),* $(,)*)
        ),* $(,)*
    }) => {
        lazy_static::lazy_static! {
            pub static ref $metric_map: ffi_support::ConcurrentHandleMap<glean_core::metrics::$metric_type> = ffi_support::ConcurrentHandleMap::new();
        }
        ffi_support::define_handle_map_deleter!($metric_map, $destroy_fn);

        $(
        #[no_mangle]
        pub extern "C" fn $new_fn(
            category: ffi_support::FfiStr,
            name: ffi_support::FfiStr,
            send_in_pings: crate::RawStringArray,
            send_in_pings_len: i32,
            lifetime: i32,
            disabled: u8,
            $($new_argname: $new_argtyp),*
        ) -> u64 {
            $metric_map.insert_with_log(|| {
                let send_in_pings = crate::from_raw_string_array(send_in_pings, send_in_pings_len)?;
                let lifetime = std::convert::TryFrom::try_from(lifetime)?;

                $(
                    let $new_argname = std::convert::TryFrom::try_from($new_argname)?;
                )*

                Ok(glean_core::metrics::$metric_type::new(glean_core::CommonMetricData {
                    name: name.into_string(),
                    category: category.into_string(),
                    send_in_pings,
                    lifetime,
                    disabled: disabled != 0,
                }, $($new_argname),*))
            })
        }
        )?

        $(
            #[no_mangle]
            pub extern "C" fn $op_fn(glean_handle: u64, metric_id: u64, $($op_argname: $op_argtyp),*) {
                crate::handlemap_ext::HandleMapExtension::call_infallible(&*crate::GLEAN, glean_handle, |glean| {
                    crate::handlemap_ext::HandleMapExtension::call_infallible(&*$metric_map, metric_id, |metric| {
                        metric.$op(glean, $($op_argname),*);
                    })
                })
            }
        )*
    }
}
