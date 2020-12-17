# Counter

Used to count how often something happens, say how often a certain button was pressed.
A counter always starts from `0`.
Each time you record to a counter, its value is incremented.
Unless incremented by a positive value, a counter will not be reported in pings.

> **IMPORTANT:** When using a counter metric, it is important to let the Glean metric do the counting. Using your own variable for counting and setting the counter yourself could be problematic because it will be difficult to reset the value at the exact moment that the value is sent in a ping. Instead, just use `counter.add` to increment the value and let Glean handle resetting the counter.

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
// Did the counter record a negative value?
assertEquals(
    1, Controls.refreshPressed.testGetNumRecordedErrors(ErrorType.InvalidValue)
)
```

</div>

<div data-lang="Java" class="tab">

```Java
import org.mozilla.yourApplication.GleanMetrics.Controls;

Controls.INSTANCE.refreshPressed.add(); // Adds 1 to the counter.
Controls.INSTANCE.refreshPressed.add(5); // Adds 5 to the counter.
```

There are test APIs available too:

```Java
import org.mozilla.yourApplication.GleanMetrics.Controls;

// Was anything recorded?
assertTrue(Controls.INSTANCE.refreshPressed.testHasValue());
// Does the counter have the expected value?
assertEquals(6, Controls.INSTANCE.refreshPressed.testGetValue());
// Did the counter record a negative value?
assertEquals(
    1, Controls.INSTANCE.refreshPressed.testGetNumRecordedErrors(ErrorType.InvalidValue)
);
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
# Did the counter record a negative value?
from glean.testing import ErrorType
assert 1 == metrics.controls.refresh_pressed.test_get_num_recorded_errors(
    ErrorType.INVALID_VALUE
)
```

</div>

<div data-lang="C#" class="tab">

```C#
using static Mozilla.YourApplication.GleanMetrics.Controls;

Controls.refreshPressed.Add(); // Adds 1 to the counter.
Controls.refreshPressed.Add(5); // Adds 5 to the counter.
```

There are test APIs available too:

```C#
using static Mozilla.YourApplication.GleanMetrics.Controls;

// Was anything recorded?
Assert.True(Controls.refreshPressed.TestHasValue());
// Does the counter have the expected value?
Assert.Equal(6, Controls.refreshPressed.TestGetValue());
// Did the counter record a negative value?
Assert.Equal(
    1, Controls.refreshPressed.TestGetNumRecordedErrors(ErrorType.InvalidValue)
);
```

</div>

<div data-lang="Rust" class="tab">

```rust
use glean_metrics;

controls::refresh_pressed.add(1); // Adds 1 to the counter.
controls::refresh_pressed.add(5); // Adds 5 to the counter.
```

There are test APIs available too:

```rust
use glean::ErrorType;

use glean_metrics;

// Was anything recorded?
assert!(controls::refresh_pressed.test_get_value(None).is_some());
// Does the counter have the expected value?
assert_eq!(6, controls::refresh_pressed.test_get_value(None).unwrap());
// Did the counter record an negative value?
assert_eq!(
  1,
  controls::refresh_pressed.test_get_num_recorded_errors(
    ErrorType::InvalidValue
  )
);
```

</div>

<div data-lang="C++" class="tab">

> **Note**: C++ APIs are only available in Firefox Desktop.

```c++
#include "mozilla/glean/GleanMetrics.h"

mozilla::glean::controls::refresh_pressed.Add(1);
mozilla::glean::controls::refresh_pressed.Add(5);
```

There are test APIs available too:

```c++
#include "mozilla/glean/GleanMetrics.h"

// Does the counter have the expected value?
ASSERT_EQ(6, mozilla::glean::controls::refresh_pressed.TestGetValue().value());
// Did it run across any errors?
// TODO: https://bugzilla.mozilla.org/show_bug.cgi?id=1683171
```

</div>

<div data-lang="JS" class="tab">

> **Note**: JS APIs are currently only available in Firefox Desktop.
> General JavaScript support is coming soon via [the Glean.js project](https://github.com/mozilla/glean.js/).

```js
Glean.controls.refreshPressed.add(1);
Glean.controls.refreshPressed.add(5);
```

There are test APIs available too:

```js
Assert.equal(6, Glean.controls.refreshPressed.testGetValue());
```

</div>

{{#include ../../tab_footer.md}}

## Limits

* Only increments, saturates at the largest value that can be represented as a 32-bit signed integer (`2147483647`).

## Examples

* How often was a certain button was pressed?

## Recorded errors

* `invalid_value`: If the counter is incremented by `0` or a negative value.

## Reference

* [Kotlin API docs](../../../javadoc/glean/mozilla.telemetry.glean.private/-counter-metric-type/index.html)
* [Swift API docs](../../../swift/Classes/CounterMetricType.html)
* [Python API docs](../../../python/glean/metrics/counter.html)
* [Rust API docs](../../../docs/glean/private/counter/struct.CounterMetric.html)
