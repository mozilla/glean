# Adding a new metric type - Swift

## Re-export generated API

By default a metric type gets an auto-generated API from the definition in `glean.udl`.
If this API is sufficient it needs to be re-exported.

Create a new Swift file, e.g. `glean-core/ios/Glean/Metrics/CounterMetric.swift`:

```Swift
public typealias CounterMetricType = CounterMetric
```

## Extend and modify API

If the generated API is not sufficient, convenient or needs additional language-specific constructs or conversions the generated API can be wrapped.

Create a new Swift file, e.g. `glean-core/ios/Glean/Metrics/CounterMetric.swift`.
Then create a new class, that delegates functionality to the generated metric type class.

```Swift
public class CounterMetricType {
    let inner: CounterMetric

    // Wrap existing functionality
    public func add(_ amount: Int = 1) {
        inner.add(amount)
    }

    // Add a new method
    public func addTwo() {
        inner.add(2)
    }
}
```

Remember to wrap all defined methods of the metric type.
