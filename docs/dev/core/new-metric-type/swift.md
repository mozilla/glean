# Adding a new metric type - Swift

## FFI

Swift can re-use the generated C header file.
Re-generate it with

```sh
make cbindgen
```

## Swift API

Finally, create a platform-specific metric type wrapper.
For Swift this would be `glean-core/ios/Glean/Metrics/CounterMetric.swift`:

```swift
public class CounterMetricType {
    let handle: UInt64
    let disabled: Bool
    let sendInPings: [String]

    /// The public constructor used by automatically generated metrics.
    public init(category: String, name: String, sendInPings: [String], lifetime: Lifetime, disabled: Bool) {
        self.disabled = disabled
        self.sendInPings = sendInPings
        self.handle = withArrayOfCStrings(sendInPings) { pingArray in
            glean_new_counter_metric(
                category,
                name,
                pingArray,
                Int32(sendInPings.count),
                lifetime.rawValue,
                disabled.toByte()
            )
        }
    }

    public func add(amount: Int32 = 1) {
        guard !self.disabled else { return }

        _ = Dispatchers.shared.launch {
            glean_counter_add(Glean.shared.handle, self.handle, amount)
        }
    }

    func testHasValue(_ pingName: String? = nil) -> Bool {
        let pingName = pingName ?? self.sendInPings[0]
        return glean_counter_test_has_value(Glean.shared.handle, self.handle, pingName) != 0
    }

    func testGetValue(_ pingName: String? = nil) throws -> Int32 {
        let pingName = pingName ?? self.sendInPings[0]

        if !testHasValue(pingName) {
            throw "Missing value"
        }

        return glean_counter_test_get_value(Glean.shared.handle, self.handle, pingName)
    }
}
```
