# Counter

Used to count how often something happens, say how often a certain button was pressed. A counter always starts from `0`. Each time you record to a counter, its value is incremented.

> **IMPORTANT:** When using a counter metric, it is important to let the it do the counting. Specifically, use your own variable for counting and reset the counter yourself. It will be difficult to reset at the correct moment relative to when the counter value is sent in a ping. By just using the `counter.add` API and letting Glean handle resetting the counter when it is sent in a ping, Glean will ensure that counts aren't duplicated across pings.

## Configuration

Say you're adding a new counter for how often the refresh button is pressed. First you need to add an entry for the counter to the `metrics.yaml` file:

```YAML
controls:
  refresh_pressed:
    type: counter
    description: >
      Counts how often the refresh button is pressed.
    ...
```

## API

{{#include ../../tab_header.md}}

<div data-lang="Kotlin" class="tab">

```Kotlin
import org.mozilla.yourApplication.GleanMetrics.Controls

Controls.refreshPressed.add() // Adds 1 to the counter.
Controls.refreshPressed.add(5) // Adds 5 to the counter.
```

There are test APIs available too:

```Kotlin
import org.mozilla.yourApplication.GleanMetrics.Controls

// Was anything recorded?
assertTrue(Controls.refreshPressed.testHasValue())
// Does the counter have the expected value?
assertEquals(6, Controls.refreshPressed.testGetValue())
// Did the counter record an negative value?
assertEquals(
    1, Controls.refreshPressed.testGetNumRecordedErrors(ErrorType.InvalidValue)
)
```

</div>

<div data-lang="Java" class="tab">

```Java
import org.mozilla.yourApplication.GleanMetrics.Controls

Controls.INSTANCE.refreshPressed.add() // Adds 1 to the counter.
Controls.INSTANCE.refreshPressed.add(5) // Adds 5 to the counter.
```

There are test APIs available too:

```Java
import org.mozilla.yourApplication.GleanMetrics.Controls

// Was anything recorded?
assertTrue(Controls.INSTANCE.refreshPressed.testHasValue())
// Does the counter have the expected value?
assertEquals(6, Controls.INSTANCE.refreshPressed.testGetValue())
// Did the counter record an negative value?
assertEquals(
    1, Controls.INSTANCE.refreshPressed.testGetNumRecordedErrors(ErrorType.InvalidValue)
)
```

</div>

<div data-lang="Swift" class="tab">

```Swift
Controls.refreshPressed.add() // Adds 1 to the counter.
Controls.refreshPressed.add(5) // Adds 5 to the counter.
```

There are test APIs available too:

```Swift
@testable import Glean

// Was anything recorded?
XCTAssert(Controls.refreshPressed.testHasValue())
// Does the counter have the expected value?
XCTAssertEqual(6, try Controls.refreshPressed.testGetValue())
// Did the counter record a negative value?
XCTAssertEqual(1, Controls.refreshPressed.testGetNumRecordedErrors(.invalidValue))
```

</div>

<div data-lang="Python" class="tab">

```Python
from glean import load_metrics
metrics = load_metrics("metrics.yaml")

metrics.controls.refresh_pressed.add()  # Adds 1 to the counter.
metrics.controls.refresh_pressed.add(5) # Adds 5 to the counter.
```

There are test APIs available too:

```Python
# Was anything recorded?
assert metrics.controls.refresh_pressed.test_has_value()
# Does the counter have the expected value?
assert 6 == metrics.controls.refresh_pressed.test_get_value()
# Did the counter record an negative value?
from glean.testing import ErrorType
assert 1 == metrics.controls.refresh_pressed.test_get_num_recorded_errors(
    ErrorType.INVALID_VALUE
)
```

</div>

{{#include ../../tab_footer.md}}

## Limits

* Only increments, saturates at the limits of a 32-bit signed integer.

## Examples

* How often a certain button was pressed?

## Recorded errors

* `invalid_value`: If the counter is incremented by a non-positive value.

## Reference

* [Kotlin API docs](../../../javadoc/glean/mozilla.telemetry.glean.private/-counter-metric-type/index.html)
* [Swift API docs](../../../swift/Classes/CounterMetricType.html)
* [Python API docs](../../../python/glean/metrics/counter.html)
