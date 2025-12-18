# Counter

Used to count how often something happens, say how often a certain button was pressed.
A counter always starts from `0`.
Each time you record to a counter, its value is incremented.
Unless incremented by a positive value, a counter will not be reported in pings,
that means: the value `0` is never sent in a ping.

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

Controls.INSTANCE.refreshPressed().add(); // Adds 1 to the counter.
Controls.INSTANCE.refreshPressed().add(5); // Adds 5 to the counter.
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
use glean_metrics::controls;

controls::refresh_pressed.add(1); // Adds 1 to the counter.
controls::refresh_pressed.add(5); // Adds 5 to the counter.
```

</div>
<div data-lang="JavaScript" class="tab">

```js
import * as controls from "./path/to/generated/files/controls.js";

controls.refreshPressed.add(); // Adds 1 to the counter.
controls.refreshPressed.add(5); // Adds 5 to the counter.
```

</div>

<div data-lang="Firefox Desktop" class="tab">

**C++**

```cpp
#include "mozilla/glean/UrlbarMetrics.h"

mozilla::glean::controls::refresh_pressed.Add(1);
mozilla::glean::controls::refresh_pressed.Add(5);
```

**JavaScript**

```js
Glean.controls.refreshPressed.add(1);
Glean.controls.refreshPressed.add(5);
```

</div>
{{#include ../../../shared/tab_footer.md}}

#### Recorded errors

* [`invalid_value`](../../user/metrics/error-reporting.md): If the counter is incremented by a negative value
  (or, in versions up to and including 54.0.0, `0`).
* [`invalid_type`](../../user/metrics/error-reporting.md): If a floating point or non-number value is given.

#### Limits

* Only increments;
* Saturates at the largest value that can be represented as a 32-bit signed integer (`2147483647`).

## Testing API

### `testGetValue`

Gets the recorded value for a given counter metric.  
Returns the count if data is stored.  
Returns a language-specific empty/null value if no data is stored.
Has an optional argument to specify the name of the ping you wish to retrieve data from, except
in Rust where it's required. `None` or no argument will default to the first value found for `send_in_pings`.

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

assertEquals(6, Controls.INSTANCE.refreshPressed().testGetValue());
```

</div>


<div data-lang="Swift" class="tab">

```Swift
XCTAssertEqual(6, Controls.refreshPressed.testGetValue())
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

```Rust
use glean_metrics::controls;

assert_eq!(6, controls::refresh_pressed.test_get_value(None).unwrap());
```

</div>

<div data-lang="JavaScript" class="tab">

```js
import * as controls from "./path/to/generated/files/controls.js";

assert.strictEqual(6, await controls.refreshPressed.testGetValue());
```

</div>

<div data-lang="Firefox Desktop" class="tab">

**C++**

```cpp
#include "mozilla/glean/UrlbarMetrics.h"

ASSERT_TRUE(mozilla::glean::controls::refresh_pressed.TestGetValue().isOk());
ASSERT_EQ(6, mozilla::glean::controls::refresh_pressed.TestGetValue().unwrap().value());
```

**JavaScript**

```js
// testGetValue will throw NS_ERROR_LOSS_OF_SIGNIFICANT_DATA on error.
Assert.equal(6, Glean.controls.refreshPressed.testGetValue());
```

</div>

{{#include ../../../shared/tab_footer.md}}

### `testGetNumRecordedErrors`

Gets the number of errors recorded for a given counter metric.

{{#include ../../../shared/tab_header.md}}

<div data-lang="Kotlin" class="tab">

```Kotlin
import org.mozilla.yourApplication.GleanMetrics.Controls

assertEquals(
    0,
    Controls.refreshPressed.testGetNumRecordedErrors(ErrorType.INVALID_VALUE)
)
```

</div>

<div data-lang="Java" class="tab">

```Java
import org.mozilla.yourApplication.GleanMetrics.Controls;

assertEquals(
    0,
    Controls.INSTANCE.refreshPressed().testGetNumRecordedErrors(ErrorType.INVALID_VALUE)
);
```

</div>


<div data-lang="Swift" class="tab">

```Swift
XCTAssertEqual(0, Controls.refreshPressed.testGetNumRecordedErrors(.invalidValue))
```

</div>

<div data-lang="Python" class="tab">

```Python
from glean import load_metrics
metrics = load_metrics("metrics.yaml")

from glean.testing import ErrorType

assert 0 == metrics.controls.refresh_pressed.test_get_num_recorded_errors(
    ErrorType.INVALID_VALUE
)
```
</div>

<div data-lang="Rust" class="tab">

```Rust
use glean::ErrorType;
use glean_metrics::controls;

assert_eq!(
  0,
  controls::refresh_pressed.test_get_num_recorded_errors(
    ErrorType::InvalidValue
  )
);
```

</div>

<div data-lang="JavaScript" class="tab">

```js
import * as controls from "./path/to/generated/files/controls.js";
import { ErrorType } from "@mozilla/glean/error";
assert.strictEqual(
  0,
  await controls.refreshPressed.testGetNumRecordedErrors(ErrorType.InvalidValue)
);
```
</div>

<div data-lang="Firefox Desktop" class="tab" data-info="Firefox Desktop uses testGetValue to communicate errors"></div>

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
refer to the [metrics YAML registry format](../yaml/metrics.md) reference page.

### Extra metric parameters

N/A

## Data questions

* How often was a certain button pressed?

## Reference

* [Python API docs](../../../python/glean/metrics/index.html#glean.metrics.CounterMetric)
* [Rust API docs](../../../docs/glean/private/struct.CounterMetric.html)
