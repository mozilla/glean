# Adding a new metric type - Kotlin

## Re-export generated API

By default a metric type gets an auto-generated API from the definition in `glean.udl`.
This API is exposed under the `internal` namespace.
If this API is sufficient it needs to be re-exported.

Create a new Kotlin file, e.g. `glean-core/android/src/main/java/mozilla/telemetry/glean/private/CounterMetricType.kt`:

```Kotlin
package mozilla.telemetry.glean.private

typealias CounterMetricType = mozilla.telemetry.glean.internal.CounterMetric
```

## Extend and modify API

If the generated API is not sufficient, convenient or needs additional language-specific constructs or conversions the generated API can be wrapped.

Create a new Kotlin file, e.g. `glean-core/android/src/main/java/mozilla/telemetry/glean/private/CounterMetricType.kt`.
Then create a new class, that delegates functionality to the metric type class from the `internal` namespace.

```Kotlin
package mozilla.telemetry.glean.private

import mozilla.telemetry.glean.internal.CounterMetric

class CounterMetricType(meta: CommonMetricData) {
    val inner = CounterMetric(meta)

    // Wrap existing functionality
    fun add(amount: Int = 1) {
        inner.add(amount)
    }

    // Add a new method
    fun addTwo() {
        inner.add(2)
    }
}
```

Remember to wrap all defined methods of the metric type.
