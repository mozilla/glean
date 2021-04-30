# Counter

Used to count how often something happens, say how often a certain button was pressed.
A counter always starts from `0`.
Each time you record to a counter, its value is incremented.
Unless incremented by a positive value, a counter will not be reported in pings.

If you find that you need to control the actual value sent in the ping, you may be measuring something,
not just counting something, and a [Quantity metric](quantity.html) may be a better choice.

{{#include ../../../shared/blockquote-warning.html}}

## Let the Glean metric do the counting

> When using a counter metric, it is important to let the Glean metric do the counting.
> Using your own variable for counting and setting the counter yourself could be problematic because
> it will be difficult to reset the value at the exact moment that the value is sent in a ping.
> Instead, just use `counter.add` to increment the value and let Glean handle resetting the counter.

## Recording API

### `add`

Increases the counter by a certain amount. If no amount is passed it defaults to `1`.

{{#include ../../../shared/tab_header.md}}
<div data-lang="Kotlin" class="tab">

  ```Kotlin
  import org.mozilla.yourApplication.GleanMetrics.Controls

  Controls.refreshPressed.add() // Adds 1 to the counter.
  Controls.refreshPressed.add(5) // Adds 5 to the counter.
  ```
</div>
<div data-lang="Java" class="tab">

  ```Java
  import org.mozilla.yourApplication.GleanMetrics.Controls;

  Controls.INSTANCE.refreshPressed.add(); // Adds 1 to the counter.
  Controls.INSTANCE.refreshPressed.add(5); // Adds 5 to the counter.
  ```
</div>
<div data-lang="Swift" class="tab">

  ```Swift
  Controls.refreshPressed.add() // Adds 1 to the counter.
  Controls.refreshPressed.add(5) // Adds 5 to the counter.
  ```
</div>
<div data-lang="Python" class="tab">

  ```Python
  from glean import load_metrics
  metrics = load_metrics("metrics.yaml")

  metrics.controls.refresh_pressed.add()  # Adds 1 to the counter.
  metrics.controls.refresh_pressed.add(5) # Adds 5 to the counter.
  ```
</div>
<div data-lang="Rust" class="tab">

  ```Rust
  use glean_metrics;

  controls::refresh_pressed.add(1); // Adds 1 to the counter.
  controls::refresh_pressed.add(5); // Adds 5 to the counter.
  ```
</div>
<div data-lang="Javascript" class="tab">

  ```js
  import * as controls from "./path/to/generated/files/controls.js";

  controls.refreshPressed.add(1); // Adds 1 to the counter.
  controls.refreshPressed.add(5); // Adds 5 to the counter.
  ```
</div>
<div data-lang="Firefox Desktop" class="tab">

  **C++**

  ```cpp
  #include "mozilla/glean/GleanMetrics.h"

  mozila::glean::controls::refresh_pressed.Add(1);
  mozilla::glean::controls::refresh_pressed.Add(5);
  ```

  **Javascript**

  ```js
  Glean.controls.refreshPressed.add(1);
  Glean.controls.refreshPressed.add(5);
  ```
</div>
{{#include ../../../shared/tab_footer.md}}

#### Recorded errors

* [`invalid_value`](../../user/metrics/error-reporting.md): If the counter is incremented by `0` or a negative value.

#### Limits

* Only increments;
* Saturates at the largest value that can be represented as a 32-bit signed integer (`2147483647`).

## Testing API

### `testGetValue`

Gets the recorded value for a given counter metric.

{{#include ../../../shared/tab_header.md}}

<div data-lang="Kotlin" class="tab">

```Kotlin
import org.mozilla.yourApplication.GleanMetrics.Controls

assertEquals(6, Controls.refreshPressed.testGetValue())
```

</div>

<div data-lang="Java" class="tab">

```Java
import org.mozilla.yourApplication.GleanMetrics.Controls;

assertEquals(6, Controls.INSTANCE.refreshPressed.testGetValue());
```

</div>


<div data-lang="Swift" class="tab">

```Swift
@testable import Glean

XCTAssertEqual(6, try Controls.refreshPressed.testGetValue())
```

</div>

<div data-lang="Python" class="tab">

```Python
from glean import load_metrics
metrics = load_metrics("metrics.yaml")

assert 6 == metrics.controls.refresh_pressed.test_get_value()
```

</div>

<div data-lang="Rust" class="tab">

```rust
use glean_metrics;

assert_eq!(6, controls::refresh_pressed.test_get_value(None).unwrap());
```

</div>

<div data-lang="Javascript" class="tab">

  ```js
  import * as controls from "./path/to/generated/files/controls.js";

  assert.strictEqual(6, await controls.refreshPressed.testGetValue());
  ```
</div>

<div data-lang="Firefox Desktop" class="tab">

**C++**

```cpp
#include "mozilla/glean/GleanMetrics.h"

ASSERT_EQ(6, mozilla::glean::controls::refresh_pressed.TestGetValue().value());
```

**Javascript**

```js
Assert.equal(6, Glean.controls.refreshPressed.testGetValue());
```

</div>

{{#include ../../../shared/tab_footer.md}}

### `testHasValue`

Whether or not **any** value was recorded for a given counter metric.

{{#include ../../../shared/tab_header.md}}

<div data-lang="Kotlin" class="tab">

```Kotlin
import org.mozilla.yourApplication.GleanMetrics.Controls

assertTrue(Controls.refreshPressed.testHasValue())
```

</div>

<div data-lang="Java" class="tab">

```Java
import org.mozilla.yourApplication.GleanMetrics.Controls;

assertTrue(Controls.INSTANCE.refreshPressed.testHasValue());
```

</div>


<div data-lang="Swift" class="tab">

```Swift
@testable import Glean

XCTAssert(Controls.refreshPressed.testHasValue())
```

</div>

<div data-lang="Python" class="tab">

```Python
from glean import load_metrics
metrics = load_metrics("metrics.yaml")

assert metrics.controls.refresh_pressed.test_has_value()
```

</div>

<div data-lang="Rust" class="tab"></div>

<div data-lang="Javascript" class="tab"></div>

<div data-lang="Firefox Desktop" class="tab"></div>

{{#include ../../../shared/tab_footer.md}}

### `testGetNumRecordedErrors`

Gets number of errors recorded for a given counter metric.

{{#include ../../../shared/tab_header.md}}

<div data-lang="Kotlin" class="tab">

```Kotlin
import org.mozilla.yourApplication.GleanMetrics.Controls

assertEquals(
    1, Controls.refreshPressed.testGetNumRecordedErrors(ErrorType.InvalidValue)
)
```

</div>

<div data-lang="Java" class="tab">

```Java
import org.mozilla.yourApplication.GleanMetrics.Controls;

assertEquals(
    1, Controls.INSTANCE.refreshPressed.testGetNumRecordedErrors(ErrorType.InvalidValue)
);
```

</div>


<div data-lang="Swift" class="tab">

```Swift
XCTAssertEqual(1, Controls.refreshPressed.testGetNumRecordedErrors(.invalidValue))
```

</div>

<div data-lang="Python" class="tab">

```Python
from glean import load_metrics
metrics = load_metrics("metrics.yaml")

from glean.testing import ErrorType

assert 1 == metrics.controls.refresh_pressed.test_get_num_recorded_errors(
    ErrorType.INVALID_VALUE
)
```
</div>

<div data-lang="Rust" class="tab">

```rust
use glean::ErrorType;

use glean_metrics;

assert_eq!(
  1,
  controls::refresh_pressed.test_get_num_recorded_errors(
    ErrorType::InvalidValue
  )
);
```

</div>

<div data-lang="Javascript" class="tab"></div>

<div data-lang="Firefox Desktop" class="tab" data-bug="1683171"></div>

{{#include ../../../shared/tab_footer.md}}

## Metric parameters

Example counter metric definition:

```yaml
controls:
  refresh_pressed:
    type: counter
    description: >
      Counts how often the refresh button is pressed.
    bugs:
      - https://bugzilla.mozilla.org/000000
    data_reviews:
      - https://bugzilla.mozilla.org/show_bug.cgi?id=000000#c3
    notification_emails:
      - me@mozilla.com
    expires: 2020-10-01
```

For a full reference on metrics parameters common to all metric types,
refer to the metrics [YAML format](../yaml/index.md) reference page.

### Extra metric parameters

N/A

## Data questions

* How often was a certain button pressed?

## Reference

* [Kotlin API docs](../../../javadoc/glean/mozilla.telemetry.glean.private/-counter-metric-type/index.html)
* [Swift API docs](../../../swift/Classes/CounterMetricType.html)
* [Python API docs](../../../python/glean/metrics/counter.html)
* [Rust API docs](../../../docs/glean/private/counter/struct.CounterMetric.html)
