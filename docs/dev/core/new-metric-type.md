# Adding a new metric type

Data in the Glean SDK is stored in so-called metrics.
You can find the full list of implemented metric types [in the user overview](../../user/metrics/index.md).

Adding a new metric type involves defining the metric type's API, its persisted and in-memory storage as well as its serialization into the ping payload.

## The metric type's API

A metric type implementation is defined in its own file under `glean-core/src/metrics/`, e.g. `glean-core/src/metrics/counter.rs` for a [Counter](../../user/metrics/counter.md).

Start by defining a structure to hold the metric's metadata:

```rust,noplaypen
#[derive(Clone, Debug)]
pub struct CounterMetric {
    meta: CommonMetricData
}
```

Implement the `MetricType` trait to create a metric from the meta data as well as expose the meta data.
This also gives you a `should_record` method on the metric type.

```rust,noplaypen
impl MetricType for CounterMetric {
    fn meta(&self) -> &CommonMetricData {
        &self.meta
    }

    fn meta_mut(&mut self) -> &mut CommonMetricData {
        &mut self.meta
    }
}
```

Its implementation should have a way to create a new metric from the common metric data. It should be the same for all metric types.

```rust,noplaypen
impl CounterMetric {
    pub fn new(meta: CommonMetricData) -> Self {
        Self { meta }
    }
}
```

Implement each method for the type. The first argument to accept should always be `glean: &Glean`, that is: a reference to the `Glean` object, used to access the storage:

```rust,noplaypen
impl CounterMetric { // same block as above
    pub fn add(&self, glean: &Glean, amount: i32) {
        // Always include this check!
        if !self.should_record() {
            return;
        }

        // Do error handling here

        glean
            .storage()
            .record_with(&self.meta, |old_value| match old_value {
                Some(Metric::Counter(old_value)) => Metric::Counter(old_value + amount),
                _ => Metric::Counter(amount),
            })
    }
}
```

Use `glean.storage().record()` to record a fixed value or `glean.storage.record_with()` to construct a new value from the currently stored one.

The storage operation makes use of the metric's variant of the `Metric` enumeration.

## The `Metric` enumeration

Persistence and in-memory serialization as well as ping payload serialization are handled through the `Metric` enumeration.
This is defined in `glean-core/src/metrics/mod.rs`.
Variants of this enumeration are used in the storage implementation of the metric type.

To add a new metric type, include the metric module and declare its use, then add a new variant to the `Metric` enum:

```rust,noplaypen

mod counter;

// ...

pub use self::counter::CounterMetric;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Metric {
    // ...
    Counter(i32),
}
```

Then modify the below implementation and define the right ping section name for the new type. This will be used in the ping payload:

```rust,noplaypen
impl Metric {
    pub fn ping_section(&self) -> &'static str {
        match self {
            // ...
            Metric::Counter(_) => "counter",
        }
    }
}
```

Finally, define the ping payload serialization (as JSON).
In the simple cases where the in-memory representation maps to its JSON representation it is enough to call the `json!` macro.

```rust,noplaypen
impl Metric { // same block as above
    pub fn as_json(&self) -> JsonValue {
        match self {
            // ...
            Metric::Counter(c) => json!(c),
        }
    }
}
```

For more complex serialization consider implementing serialization logic as a function returning a [`serde_json::Value`](https://docs.rs/serde_json/*/serde_json/enum.Value.html)
or another object that can be serialized.

For example, the `DateTime` serializer has the following entry, where `get_iso_time_string` is a function to convert from the `DateTime` metric representation to a string:

```rust,noplaypen
Metric::Datetime(d, time_unit) => json!(get_iso_time_string(*d, *time_unit)),
```

## FFI layer

In order to use a new metric type over the FFI layer, it needs implementations in the FFI component and the platform-part.

### FFI component

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
    should_record -> glean_counter_should_record,

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

### Platform-part (Kotlin)

The platform-specific FFI wrapper needs the definitions of these new functions.
For Kotlin this is in `glean-core/android/src/main/java/mozilla/telemetry/glean/rust/LibGleanFFI.kt`:

```kotlin
fun glean_new_counter_metric(category: String, name: String, send_in_pings: StringArray, send_in_pings_len: Int, lifetime: Int, disabled: Byte): Long
fun glean_destroy_counter_metric(handle: Long, error: RustError.ByReference)
fun glean_counter_add(glean_handle: Long, metric_id: Long, amount: Int)
fun glean_counter_should_record(glean_handle: Long, metric_id: Long): Byte
```

Finally, create a platform-specific metric type wrapper.
For Kotlin this would be `glean-core/android/src/main/java/mozilla/telemetry/glean/private/CounterMetricType.kt`:

```kotlin
class CounterMetricType(
    disabled: Boolean,
    category: String,
    lifetime: Lifetime,
    name: String,
    val sendInPings: List<String>
) {
    private var handle: Long

    init {
        val ffiPingsList = StringArray(sendInPings.toTypedArray(), "utf-8")
        this.handle = LibGleanFFI.INSTANCE.glean_new_counter_metric(
                category = category,
                name = name,
                send_in_pings = ffiPingsList,
                send_in_pings_len = sendInPings.size,
                lifetime = lifetime.ordinal,
                disabled = disabled.toByte())
    }

    protected fun finalize() {
        if (this.handle != 0L) {
            val error = RustError.ByReference()
            LibGleanFFI.INSTANCE.glean_destroy_counter_metric(this.handle, error)
        }
    }

    fun shouldRecord(): Boolean {
        // Don't record metrics if we aren't initialized
        if (!Glean.isInitialized()) {
            return false
        }

        return LibGleanFFI.INSTANCE.glean_counter_should_record(Glean.handle, this.handle).toBoolean()
    }

    fun add(amount: Int = 1) {
        if (!shouldRecord()) {
            return
        }

        @Suppress("EXPERIMENTAL_API_USAGE")
        Dispatchers.API.launch {
            LibGleanFFI.INSTANCE.glean_counter_add(
                Glean.handle,
                this@CounterMetricType.handle,
                amount)
        }
    }

    @VisibleForTesting(otherwise = VisibleForTesting.NONE)
    fun testHasValue(pingName: String = sendInPings.first()): Boolean {
        @Suppress("EXPERIMENTAL_API_USAGE")
        Dispatchers.API.assertInTestingMode()

        val res = LibGleanFFI.INSTANCE.glean_counter_test_has_value(Glean.handle, this.handle, pingName)
        return res.toBoolean()
    }
}
```
