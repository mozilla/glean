# Labeled Quantities

Labeled quantity metrics are used to record different related non-negative integer values.
For example, the width and height of the display in pixels.

{{#include ../../../shared/blockquote-warning.html}}

## Do not use Labeled Quantity metrics for counting

> If you need to _count_ some things (e.g. the number of times buttons are pressed)
> prefer using the [Labeled Counter](./labeled_counters.md) metric type, which has a specific API for counting things.

## Recording API

### `set`

Sets a quantity metric to a specific value for the given label.

{{#include ../../../shared/tab_header.md}}

<div data-lang="Kotlin" class="tab">
 
```Kotlin
import org.mozilla.yourApplication.GleanMetrics.Gfx

Gfx.display["width"].set(width)
Gfx.display["heigth"].set(height)
```

</div>
<div data-lang="Java" class="tab">

```Java
import org.mozilla.yourApplication.GleanMetrics.Gfx;

Gfx.INSTANCE.display()["width"].set(width)
Gfx.INSTANCE.display()["heigth"].set(height)
```

</div>
<div data-lang="Swift" class="tab">

```Swift
Gfx.display["width"].set(width)
Gfx.display["heigth"].set(heigth)
```

</div>
<div data-lang="Python" class="tab">

```Python
from glean import load_metrics
metrics = load_metrics("metrics.yaml")

metrics.gfx.display["width"].set(width)
metrics.gfx.display["height"].set(heigth)
```

</div>
<div data-lang="Rust" class="tab">

```Rust
use glean_metrics::gfx;

gfx::display.get("width").set(width);
gfx::display.get("height").set(height);
```
</div>

<div data-lang="JavaScript" class="tab"></div>
<div data-lang="Firefox Desktop" class="tab">

**C++**
```cpp
#include "mozilla/glean/GfxMetrics.h"

mozilla::glean::gfx::display.Get("width"_ns").Set(aWidth);
mozilla::glean::gfx::display.Get("height"_ns").Set(aHeight);
```

**JavaScript**
```js
Glean.gfx.display.width.set(aWidth);
Glean.gfx.display["height"].set(aHeight);
```
</div>

{{#include ../../../shared/tab_footer.md}}

#### Limits

* Quantities must be non-negative integers or 0.

#### Recorded errors

* [`invalid_value`](../../user/metrics/error-reporting.md): if a negative value is passed in.
* [`invalid_type`](../../user/metrics/error-reporting.md): if a floating point or non-number value is given.
{{#include ../../_includes/label-errors.md}}

## Testing API

### `testGetValue`

Gets the recorded value for a given label in a labeled quantity metric.  
Returns the quantity value if data is stored. The Glean SDK will return a map of each label with a
stored value to its value.   
Returns a language-specific empty/null value if no data is stored. The Glean SDK will always
return a map, but it will be empty if no data is stored. 
Has an optional argument to specify the name of the ping you wish to retrieve data from, except  
in Rust where it's required. `None` or no argument will default to the first value found for `send_in_pings`.

{{#include ../../../shared/tab_header.md}}

<div data-lang="Kotlin" class="tab">

```Kotlin
import org.mozilla.yourApplication.GleanMetrics.Gfx

val values = Gfx.display.testGetValue()
assertEquals(433, values["width"])
assertEquals(42, values["height"])
```

</div>
<div data-lang="Java" class="tab">org

```Java
import org.mozilla.yourApplication.GleanMetrics.Gfx;

Map<String, ?> values = 
assertEquals(433, values["width"]);
assertEquals(42, values["height"]);
```
</div>
<div data-lang="Swift" class="tab">

```Swift
let values = Gfx.display.testGetValue()
XCTAssertEqual(433, values["width"])
XCTAssertEqual(42, values["heigth"])
```

</div>
<div data-lang="Python" class="tab">

```Python
from glean import load_metrics
metrics = load_metrics("metrics.yaml")

values = metrics.gfx.display.test_get_value()
assert 433 == values["width"]
assert 42 == values["height"]
```

</div>
<div data-lang="Rust" class="tab">

```Rust
use glean_metrics::gfx;

let values = gfx::display.test_get_value(None).unwrap();
// Was anything recorded?
assert_eq!(433, values["width"]);
assert_eq!(42, values["height"]);
```
</div>

<div data-lang="JavaScript" class="tab"></div>
<div data-lang="Firefox Desktop" class="tab">

**C++**
```cpp
#include "mozilla/glean/GfxMetrics.h"

ASSERT_EQ(
    433,
    mozilla::glean::gfx::display.Get("width"_ns).TestGetValue().unwrap().ref));
ASSERT_EQ(
    42,
    mozilla::glean::gfx::display.Get("height"_ns).TestGetValue().unwrap().ref));
```

**JavaScript**
```js
Assert.equal(433, Glean.gfx.display.width.testGetValue());
Assert.equal(42, Glean.gfx.display["height"].testGetValue());
```

{{#include ../../../shared/tab_footer.md}}

### `testGetNumRecordedErrors`

Gets the number of errors recorded for a given label in a labeled quantity metric.

{{#include ../../../shared/tab_header.md}}

<div data-lang="Kotlin" class="tab">

```Kotlin
import org.mozilla.yourApplication.GleanMetrics.Gfx

assertEquals(
    0,
    Gfx.display["width"].testGetNumRecordedErrors(ErrorType.INVALID_VALUE)
)
assertEquals(
    42,
    Gfx.display["height"].testGetNumRecordedErrors(ErrorType.INVALID_VALUE)
)
```

</div>
<div data-lang="Java" class="tab">org

```Java
import org.mozilla.yourApplication.GleanMetrics.Gfx;

assertEquals(
    0,
    Gfx.INSTANCE.display()["width"].testGetNumRecordedErrors(ErrorType.INVALID_VALUE)
);
assertEquals(
    42,
    Gfx.INSTANCE.display()["height"].testGetNumRecordedErrors(ErrorType.INVALID_VALUE)
);
```
</div>
<div data-lang="Swift" class="tab">

```Swift
XCTAssertEqual(0, Gfx.display["width"].testGetNumRecordedErrors(.invalidValue))
XCTAssertEqual(0, Gfx.display["heigth"].testGetNumRecordedErrors(.invalidValue))
```

</div>
<div data-lang="Python" class="tab">

```Python
from glean import load_metrics
metrics = load_metrics("metrics.yaml")

assert 0 == metrics.gfx.display["width"].test_get_num_recorded_errors(
    ErrorType.INVALID_VALUE
)
assert 0 == metrics.gfx.display["height"].test_get_num_recorded_errors(
    ErrorType.INVALID_VALUE
)
```

</div>
<div data-lang="Rust" class="tab">

```Rust
use glean::ErrorType;
use glean_metrics::gfx;

assert_eq!(
  0,
  gfx::display.get("width").test_get_num_recorded_errors(
    ErrorType::InvalidValue,
    None
  )
);
assert_eq!(
  0,
  gfx::display.get("height").test_get_num_recorded_errors(
    ErrorType::InvalidValue,
    None
  )
);
```
</div>

<div data-lang="JavaScript" class="tab"></div>
<div data-lang="Firefox Desktop" class="tab" data-info="Firefox Desktop uses testGetValue to communicate errors"></div>

{{#include ../../../shared/tab_footer.md}}

## Metric parameters

Example quantity metric definition:

```yaml
gfx:
  display:
    type: labeled_quantity
    description: >
      The dimensions of the display, in pixels.
      For one-dimensional displays, uses only "width".
      Two-dimensional displays add "height".
      3D displays gain "depth".
    bugs:
      - https://bugzilla.mozilla.org/000000
    data_reviews:
      - https://bugzilla.mozilla.org/show_bug.cgi?id=000000#c3
    notification_emails:
      - me@mozilla.com
    expires: 2020-10-01
    unit: pixels
    labels:
      - width
      - height
      - depth
```

For a full reference on metrics parameters common to all metric types,
refer to the [metrics YAML registry format](../yaml/metrics.md) reference page.

### Extra metric parameters

#### `unit`

Labeled Quantities have the required `unit` parameter, which is a free-form string for documentation purposes.

{{#include ../../_includes/labels-parameter.md}}

## Data questions

* What are the dimensions of the display, in pixels?

## Limits

{{#include ../../_includes/label-limits.md}}

## Reference

* Python API docs: [`LabeledQuantityMetricType`](../../../python/glean/metrics/labeled.html#glean.metrics.labeled.LabeledQuantityMetricType), [`QuantityMetricType`](../../../python/glean/metrics/index.html#glean.metrics.QuantityMetric)
* Rust API docs: [`LabeledMetric`](../../../docs/glean/private/struct.LabeledMetric.html), [`QuantityMetric`](../../../docs/glean/private/struct.QuantityMetric.html)
