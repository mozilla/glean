# Adding a new metric type - Kotlin

## FFI

The platform-specific FFI wrapper needs the definitions of these new functions.
For Kotlin this is in `glean-core/android/src/main/java/mozilla/telemetry/glean/rust/LibGleanFFI.kt`:

```kotlin
fun glean_new_counter_metric(category: String, name: String, send_in_pings: StringArray, send_in_pings_len: Int, lifetime: Int, disabled: Byte): Long
fun glean_destroy_counter_metric(handle: Long)
fun glean_counter_add(glean_handle: Long, metric_id: Long, amount: Int)
```

## Kotlin API

Finally, create a platform-specific metric type wrapper.
For Kotlin this would be `glean-core/android/src/main/java/mozilla/telemetry/glean/private/CounterMetricType.kt`:

```kotlin
class CounterMetricType(
    private var handle: Long,
    private val disabled: Boolean,
    private val sendInPings: List<String>
) {
    /**
     * The public constructor used by automatically generated metrics.
     */
    constructor(
        disabled: Boolean,
        category: String,
        lifetime: Lifetime,
        name: String,
        sendInPings: List<String>
    ) : this(handle = 0, disabled = disabled, sendInPings = sendInPings) {
        val ffiPingsList = StringArray(sendInPings.toTypedArray(), "utf-8")
        this.handle = LibGleanFFI.INSTANCE.glean_new_counter_metric(
                category = category,
                name = name,
                send_in_pings = ffiPingsList,
                send_in_pings_len = sendInPings.size,
                lifetime = lifetime.ordinal,
                disabled = disabled.toByte())
    }

    fun add(amount: Int = 1) {
        if (disabled) {
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
