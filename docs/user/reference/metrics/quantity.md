# Quantity

Used to record a single non-negative integer value or 0.
For example, the width of the display in pixels.

{{#include ../../../shared/blockquote-warning.html}}

## Do not use Quantity for counting

> If you need to _count_ something (e.g. the number of times a button is pressed)
> prefer using the [Counter](./counter.md) metric type, which has a specific API for counting things.

## Recording API

### `set`

Sets a quantity metric to a specific value.

{{#include ../../../shared/tab_header.md}}

<div data-lang="Kotlin" class="tab">

```Kotlin
import org.mozilla.yourApplication.GleanMetrics.Display

Display.width.set(width)
```
</div>

<div data-lang="Java" class="tab">

```Java
import org.mozilla.yourApplication.GleanMetrics.Display;

Display.INSTANCE.width.set(width);
```
</div>

<div data-lang="Swift" class="tab">

```Swift
Display.width.set(width)
```
</div>

<div data-lang="Python" class="tab">

```Python
from glean import load_metrics
metrics = load_metrics("metrics.yaml")

metrics.display.width.set(width)
```
</div>

<div data-lang="JavaScript" class="tab">

```js
import * as display from "./path/to/generated/files/display.js";

display.width.set(window.innerHeight);
```
</div>

<div data-lang="Rust" class="tab">

```rust
use glean_metrics;

display::width.set(width);
```
</div>

<div data-lang="Firefox Desktop" class="tab">

**C++**

```cpp
#include "mozilla/glean/GleanMetrics.h"

mozilla::glean::display::width.Set(innerHeight);
```

**JavaScript**

```js
Glean.display.width.set(innerHeight);
```
</div>

{{#include ../../../shared/tab_footer.md}}

#### Limits

* Quantities must be non-negative integers or 0.

#### Recorded errors

* [`invalid_value`](../../user/metrics/error-reporting.md): if a negative value is passed in.
* [`invalid_type`](../../user/metrics/error-reporting.md): if a floating point or non-number value is given.

## Testing API

### `testGetValue`

Gets the recorded value for a given quantity metric.

{{#include ../../../shared/tab_header.md}}

<div data-lang="Kotlin" class="tab">

```Kotlin
import org.mozilla.yourApplication.GleanMetrics.Display

// Does the quantity have the expected value?
assertEquals(6, Display.width.testGetValue())
```
</div>

<div data-lang="Java" class="tab">

```Java
import org.mozilla.yourApplication.GleanMetrics.Display;

// Was anything recorded?
assertTrue(Display.INSTANCE.width.testHasValue());
// Does the quantity have the expected value?
assertEquals(6, Display.INSTANCE.width.testGetValue());
```
</div>

<div data-lang="Swift" class="tab">

```Swift
// Does the quantity have the expected value?
XCTAssertEqual(6, try Display.width.testGetValue())
```

</div>

<div data-lang="Python" class="tab">

```Python
from glean import load_metrics
metrics = load_metrics("metrics.yaml")

# Does the quantity have the expected value?
assert 6 == metrics.display.width.test_get_value()
```
</div>

<div data-lang="JavaScript" class="tab">

```js
import * as display from "./path/to/generated/files/display.js";

assert.strictEqual(433, await display.width.testGetValue());
```

</div>

<div data-lang="Rust" class="tab">

```rust
use glean_metrics;

// Was anything recorded?
assert!(display::width.test_get_value(None).is_some());
```
</div>

<div data-lang="Firefox Desktop" class="tab">

**C++**

```cpp
#include "mozilla/glean/GleanMetrics.h"

ASSERT_TRUE(mozilla::glean::display::width.TestGetValue().isOk());
ASSERT_EQ(433, mozilla::glean::display::width.TestGetValue().unwrap().value());
```

**JavaScript**

```js
// testGetValue will throw NS_ERROR_LOSS_OF_SIGNIFICANT_DATA on error.
Assert.equal(433, Glean.display.width.testGetValue());
```
</div>

{{#include ../../../shared/tab_footer.md}}

### `testHasValue`

Whether or not **any** value was recorded for a given quantity metric.

{{#include ../../../shared/tab_header.md}}

<div data-lang="Kotlin" class="tab">

```Kotlin
import org.mozilla.yourApplication.GleanMetrics.Display

// Was anything recorded?
assertTrue(Display.width.testHasValue())
```
</div>

<div data-lang="Java" class="tab">

```Java
import org.mozilla.yourApplication.GleanMetrics.Display;

// Was anything recorded?
assertTrue(Display.INSTANCE.width.testHasValue());
);
```
</div>

<div data-lang="Swift" class="tab">

```Swift
// Was anything recorded?
XCTAssert(Display.width.testHasValue())
```

</div>

<div data-lang="Python" class="tab">

```Python
from glean import load_metrics
metrics = load_metrics("metrics.yaml")

# Was anything recorded?
assert metrics.display.width.test_has_value()
```
</div>

<div data-lang="JavaScript" class="tab"></div>

<div data-lang="Rust" class="tab"></div>

<div data-lang="Firefox Desktop" class="tab"></div>

{{#include ../../../shared/tab_footer.md}}

### `testGetNumRecordedErrors`

Gets number of errors recorded for a given quantity metric.

{{#include ../../../shared/tab_header.md}}

<div data-lang="Kotlin" class="tab">

```Kotlin
import org.mozilla.yourApplication.GleanMetrics.Display

// Did it record an error due to a negative value?
assertEquals(1, Display.width.testGetNumRecordedErrors(ErrorType.InvalidValue))
```
</div>

<div data-lang="Java" class="tab">

```Java
import org.mozilla.yourApplication.GleanMetrics.Display;

// Did the quantity record a negative value?
assertEquals(
    1, Display.INSTANCE.width.testGetNumRecordedErrors(ErrorType.InvalidValue)
);
```
</div>

<div data-lang="Swift" class="tab">

```Swift
// Did the quantity record a negative value?
XCTAssertEqual(1, Display.width.testGetNumRecordedErrors(.invalidValue))
```

</div>

<div data-lang="Python" class="tab">

```Python
from glean import load_metrics
metrics = load_metrics("metrics.yaml")

# Did the quantity record an negative value?
from glean.testing import ErrorType
assert 1 == metrics.display.width.test_get_num_recorded_errors(
    ErrorType.INVALID_VALUE
)
```
</div>

<div data-lang="JavaScript" class="tab">

```js
import * as display from "./path/to/generated/files/display.js";
import { ErrorType } from "@mozilla/glean/<platform>";

assert.strictEqual(
  0,
  await display.width.testGetNumRecordedErrors(ErrorType.InvalidValue)
);
```

</div>

<div data-lang="Rust" class="tab">

```rust
use glean::ErrorType;

use glean_metrics;

assert_eq!(
  1,
  display::width.test_get_num_recorded_errors(
    ErrorType::InvalidValue
  )
);
```
</div>

<div data-lang="Firefox Desktop" class="tab" data-info="Firefox Desktop uses testGetValue to communicate errors"></div>

{{#include ../../../shared/tab_footer.md}}

## Metric parameters

Example quantity metric definition:

```yaml
controls:
  refresh_pressed:
    type: quantity
    description: >
      The width of the display, in pixels.
    bugs:
      - https://bugzilla.mozilla.org/000000
    data_reviews:
      - https://bugzilla.mozilla.org/show_bug.cgi?id=000000#c3
    notification_emails:
      - me@mozilla.com
    expires: 2020-10-01
    unit: pixels
```

For a full reference on metrics parameters common to all metric types,
refer to the [metrics YAML registry format](../yaml/metrics.md) reference page.

### Extra metric parameters

#### `unit`

Quantities have the required `unit` parameter, which is a free-form string for documentation purposes.

## Data questions

* What is the width of the display, in pixels?

## Reference

* [Swift API docs](../../../swift/Classes/QuantityMetricType.html)
* [Python API docs](../../../python/glean/metrics/quantity.html)
* [Rust API docs](../../../docs/glean/private/quantity/struct.QuantityMetric.html)
* [JavaScript API docs](https://mozilla.github.io/glean.js/classes/core_metrics_types_quantity.default.html#set)
