# Quantity

Used to record a single non-negative integer value or 0.
For example, the width of the display in pixels.

> **IMPORTANT** If you need to _count_ something (e.g. number of tabs open or number of times a button is pressed) prefer using the [Counter](./counter.md) metric type, which has a specific API for counting things and also takes care of resetting the count at the correct time.

## Configuration

Say you're adding a new quantity for the width of the display in pixels. First you need to add an entry for the quantity to the `metrics.yaml` file:

```YAML
display:
  width:
    type: quantity
    description: >
      The width of the display, in pixels.
    unit: pixels
    ...
```

Note that quantities have a required `unit` parameter, which is a free-form string for documentation purposes.

## API

{{#include ../../tab_header.md}}

<div data-lang="Kotlin" class="tab">

```Kotlin
import org.mozilla.yourApplication.GleanMetrics.Display

Display.width.set(width)
```

There are test APIs available too:

```Kotlin
import org.mozilla.yourApplication.GleanMetrics.Display

// Was anything recorded?
assertTrue(Display.width.testHasValue())
// Does the quantity have the expected value?
assertEquals(6, Display.width.testGetValue())
// Did it record an error due to a negative value?
assertEquals(1, Display.width.testGetNumRecordedErrors(ErrorType.InvalidValue))
```

</div>

<div data-lang="Java" class="tab">

```Java
import org.mozilla.yourApplication.GleanMetrics.Display;

Display.INSTANCE.width.set(width);
```

There are test APIs available too:

```Java
import org.mozilla.yourApplication.GleanMetrics.Display;

// Was anything recorded?
assertTrue(Display.INSTANCE.width.testHasValue());
// Does the quantity have the expected value?
assertEquals(6, Display.INSTANCE.width.testGetValue());
// Did the quantity record a negative value?
assertEquals(
    1, Display.INSTANCE.width.testGetNumRecordedErrors(ErrorType.InvalidValue)
);
```

</div>

<div data-lang="Swift" class="tab">

```Swift
Display.width.set(width)
```

There are test APIs available too:

```Swift
@testable import Glean

// Was anything recorded?
XCTAssert(Display.width.testHasValue())
// Does the quantity have the expected value?
XCTAssertEqual(6, try Display.width.testGetValue())
// Did the quantity record a negative value?
XCTAssertEqual(1, Display.width.testGetNumRecordedErrors(.invalidValue))
```

</div>

<div data-lang="Python" class="tab">

```Python
from glean import load_metrics
metrics = load_metrics("metrics.yaml")

metrics.display.width.set(width)
```

There are test APIs available too:

```Python
# Was anything recorded?
assert metrics.display.width.test_has_value()
# Does the quantity have the expected value?
assert 6 == metrics.display.width.test_get_value()
# Did the quantity record an negative value?
from glean.testing import ErrorType
assert 1 == metrics.display.width.test_get_num_recorded_errors(
    ErrorType.INVALID_VALUE
)
```

</div>

<div data-lang="C#" class="tab">

```C#
using static Mozilla.YourApplication.GleanMetrics.Display;

Display.width.Set(width);
```

There are test APIs available too:

```C#
using static Mozilla.YourApplication.GleanMetrics.Display;

// Was anything recorded?
Assert.True(Display.width.TestHasValue());
// Does the counter have the expected value?
Assert.Equal(6, Display.width.TestGetValue());
// Did the counter record an negative value?
Assert.Equal(
    1, Display.width.TestGetNumRecordedErrors(ErrorType.InvalidValue)
);
```

</div>

<div data-lang="Rust" class="tab">

```rust
use glean_metrics;

display::width.set(width);
```

There are test APIs available too:

```rust
use glean_metrics;

// Was anything recorded?
assert!(display::width.test_get_value(None).is_some());
// Does it have the expected value?
assert_eq!(width, display::width.test_get_value(None).unwrap());
```

</div>

{{#include ../../tab_footer.md}}

## Limits

* Quantities must be non-negative integers or 0.

## Examples

* What is the width of the display, in pixels?

## Recorded errors

* `invalid_value`: If a negative value is passed in.

## Reference

* [Kotlin API docs](../../../javadoc/glean/mozilla.telemetry.glean.private/-quantity-metric-type/index.html)
* [Swift API docs](../../../swift/Classes/QuantityMetricType.html)
* [Python API docs](../../../python/glean/metrics/quantity.html)
* [Rust API docs](../../../docs/glean/private/quantity/struct.QuantityMetric.html)
