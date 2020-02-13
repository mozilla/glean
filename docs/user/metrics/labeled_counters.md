# Labeled Counters

Labeled counters are used to record different related counts that should sum up to a total.

## Configuration

For example, you may want to record a count of different types of crashes for your Android application, such as native code crashes and uncaught exceptions:

```YAML
stability:
  crash_count:
    type: labeled_counter
    description: >
      Counts the number of crashes that occur in the application. ...
    labels:
      - uncaught_exception
      - native_code_crash
    ...
```

> **Note:** removing or changing labels, including their order in the registry file, is permitted. Avoid reusing labels that were removed in the past. It is best practice to add documentation about removed labels to the description field so that analysts will know of their existence and meaning in historical data. Special care must be taken when changing GeckoView metrics sent through the Glean SDK, as the index of the labels is used to report Gecko data through the Glean SDK.

## API

Now you can use the labeled counter from the application's code:

{{#include ../../tab_header.md}}

<div data-lang="Kotlin" class="tab">

```Kotlin
import org.mozilla.yourApplication.GleanMetrics.Stability
Stability.crashCount["uncaught_exception"].add() // Adds 1 to the "uncaught_exception" counter.
Stability.crashCount["native_code_crash"].add(3) // Adds 3 to the "native_code_crash" counter.
```

There are test APIs available too:

```Kotlin
import org.mozilla.yourApplication.GleanMetrics.Stability
// Was anything recorded?
assertTrue(Stability.crashCount["uncaught_exception"].testHasValue())
assertTrue(Stability.crashCount["native_code_crash"].testHasValue())
// Do the counters have the expected values?
assertEquals(1, Stability.crashCount["uncaught_exception"].testGetValue())
assertEquals(3, Stability.crashCount["native_code_crash"].testGetValue())
// Did we record any invalid labels?
assertEquals(1, Stability.crashCount.testGetNumRecordedErrors(ErrorType.InvalidLabel))
```

</div>

<div data-lang="Swift" class="tab">

```Swift
Stability.crashCount["uncaught_exception"].add() // Adds 1 to the "uncaught_exception" counter.
Stability.crashCount["native_code_crash"].add(3) // Adds 3 to the "native_code_crash" counter.
```

There are test APIs available too:

```Swift
@testable import Glean

// Was anything recorded?
XCTAssert(Stability.crashCount["uncaught_exception"].testHasValue())
XCTAssert(Stability.crashCount["native_code_crash"].testHasValue())
// Do the counters have the expected values?
XCTAssertEqual(1, try Stability.crashCount["uncaught_exception"].testGetValue())
XCTAssertEqual(3, try Stability.crashCount["native_code_crash"].testGetValue())
```

</div>

<div data-lang="Python" class="tab">

```Python
from glean import load_metrics
metrics = load_metrics("metrics.yaml")

# Adds 1 to the "uncaught_exception" counter.
metrics.stability.crash_count["uncaught_exception"].add()
# Adds 3 to the "native_code_crash" counter.
metrics.stability.crash_count["native_code_crash"].add(3)
```

There are test APIs available too:

```Python
# Was anything recorded?
assert metrics.stability.crash_count["uncaught_exception"].test_has_value()
assert metrics.stability.crash_count["native_code_crash"].test_has_value()
# Do the counters have the expected values?
assert 1 == metrics.stability.crash_count["uncaught_exception"].test_get_value()
assert 3 == metrics.stability.crash_count["native_code_crash"].test_get_value()
# Did we record any invalid labels?
assert 1 == metrics.stability.crash_count.test_get_num_recorded_errors(
    ErrorType.INVALID_LABEL
)
```

</div>

{{#include ../../tab_footer.md}}

## Limits

* Labels must conform to the [label formatting regular expression](index.md#label-format).

* Labels support lowercase alphanumeric characters; they additionally allow for dots (`.`), underscores (`_`) and/or hyphens (`-`).

* Each label must have a maximum of 60 bytes, when encoded as UTF-8.

* If the labels are specified in the `metrics.yaml`, using a different label will be replaced with the special value `__other__`.

* If the labels aren't specified in the `metrics.yaml`, only 16 different dynamic labels may be used, after which the special value `__other__` will be used.

## Examples

* Record the number of times different kinds of crashes occurred.

## Recorded Errors

* `invalid_label`: If the label contains invalid characters.

* `invalid_label`: If the label exceeds the maximum number of allowed characters.

## Reference

* Kotlin API docs [`LabeledMetricType`](../../../javadoc/glean/mozilla.telemetry.glean.private/-labeled-metric-type/index.html), [`CounterMetricType`](../../../javadoc/glean/mozilla.telemetry.glean.private/-counter-metric-type/index.html)
* Swift API docs: [`LabeledMetricType`](../../../swift/Classes/LabeledMetricType.html), [`CounterMetricType`](../../../swift/Classes/CounterMetricType.html)
* [Python API docs](../../../python/glean/metrics/labeled.html)
