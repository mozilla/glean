# Custom Distribution

Custom distributions are used to record the distribution of arbitrary values.

It should be used only when direct control over how the histogram buckets are computed is required.
Otherwise, look at the standard distribution metric types:

* [Timing Distributions](timing_distribution.md)
* [Memory Distributions](memory_distribution.md)

> **Note**: Custom distributions are currently not universally supported. See below for available APIs.

## Recording API

### `accumulateSamples`

Accumulate the provided samples in the metric.

{{#include ../../../shared/tab_header.md}}

<div data-lang="Kotlin" class="tab">

```Kotlin
import org.mozilla.yourApplication.GleanMetrics.Graphics

Graphics.checkerboardPeak.accumulateSamples([23])
```

</div>

<div data-lang="Java" class="tab">

```Java
import org.mozilla.yourApplication.GleanMetrics.Graphics;

Graphics.INSTANCE.checkerboardPeak().accumulateSamples(listOf(23));
```

</div>
<div data-lang="Swift" class="tab">

```Swift
Graphics.checkerboardPeak.accumulateSamples([23])
```

</div>
<div data-lang="Python" class="tab">

```Python
from glean import load_metrics
metrics = load_metrics("metrics.yaml")

metrics.graphics.checkerboard_peak.accumulate_samples([23])
```

</div>
<div data-lang="Rust" class="tab">

```Rust
use glean_metrics::graphics;

graphics::checkerboard_peak.accumulate_samples_signed(vec![23]);
```

</div>
<div data-lang="JavaScript" class="tab">

```js
import * as graphics from "./path/to/generated/files/graphics.js";

graphics.checkerboardPeak.accumulateSamples([23]);
```

</div>
<div data-lang="Firefox Desktop" class="tab">

**C++**

```cpp
#include "mozilla/glean/GleanMetrics.h"

mozilla::glean::graphics::checkerboard_peak.AccumulateSamples({ 23 });
```

**JavaScript**

```js
Glean.graphics.checkerboardPeak.accumulateSamples([23])
```

</div>

{{#include ../../../shared/tab_footer.md}}

#### Limits

* The maximum value of `bucket_count` is 100.
* Only non-negative values may be recorded (`>= 0`).

#### Recorded errors

* `invalid_value`: If recording a negative value.

## Testing API

### `testGetValue`

Gets the recorded value for a given counter metric.  
Returns a struct with counts per buckets and total sum if data is stored.  
Returns a language-specific empty/null value if no data is stored.

{{#include ../../../shared/tab_header.md}}

<div data-lang="Kotlin" class="tab">

```Kotlin
import org.mozilla.yourApplication.GleanMetrics.Graphics

// Get snapshot
val snapshot = Graphics.checkerboardPeak.testGetValue()

// Does the sum have the expected value?
assertEquals(23, snapshot.sum)

// Does the count have the expected value?
assertEquals(1L, snapshot.count)

// Buckets are indexed by their lower bound.
assertEquals(1L, snapshot.values[19])
```

</div>

<div data-lang="Java" class="tab">

```Java
import org.mozilla.yourApplication.GleanMetrics.Graphics;

// Get snapshot
val snapshot = Graphics.INSTANCE.checkerboardPeak().testGetValue();

// Does the sum have the expected value?
assertEquals(23, snapshot.sum);

// Does the count have the expected value?
assertEquals(1L, snapshot.count);
```

</div>
<div data-lang="Swift" class="tab">

```Swift
// Get snapshot.
let snapshot = graphics.checkerboardPeak.testGetValue()

// Does the sum have the expected value?
XCTAssertEqual(23, snapshot.sum)

// Does the count have the expected value?
XCTAssertEqual(1, snapshot.count)

// Buckets are indexed by their lower bound.
XCTAssertEqual(1L, snapshot.values[19])
```

</div>
<div data-lang="Python" class="tab">

```Python
from glean import load_metrics
metrics = load_metrics("metrics.yaml")

# Get snapshot.
snapshot = metrics.graphics.checkerboard_peak.test_get_value()

# Does the sum have the expected value?
assert 23 == snapshot.sum

# Does the count have the expected value?
assert 1 == snapshot.count

# Buckets are indexed by their lower bound.
assert 1 == snapshot.values[19]
```

</div>
<div data-lang="Rust" class="tab">

```Rust
use glean_metrics::graphics;

// Does the sum have the expected value?
assert_eq!(23, graphics::checkerboard_peak.test_get_value(None).unwrap().sum);

// Does the count have the expected value?
assert_eq!(1, graphics::checkerboard_peak.test_get_value(None).unwrap().count);

// Buckets are indexed by their lower bound.
assert_eq!(1, snapshot.values[19])
```

</div>
<div data-lang="JavaScript" class="tab">

```js
import * as graphics from "./path/to/generated/files/graphics.js";

// Get snapshot
const snapshot = await graphics.checkerboardPeak.testGetValue();

// Does the sum have the expected value?
assert.equal(23, snapshot.sum);

// Buckets are indexed by their lower bound.
assert.equal(1, snapshot.values[19]);
```

</div>
<div data-lang="Firefox Desktop" class="tab">

**C++**

```cpp
#include "mozilla/glean/GleanMetrics.h"

auto data = mozilla::glean::graphics::checkerboard_peak.TestGetValue().value();
ASSERT_EQ(23UL, data.sum);
```

**JavaScript**

```js
let data = Glean.graphics.checkerboardPeak.testGetValue();
Assert.equal(23, data.sum);
```

</div>

{{#include ../../../shared/tab_footer.md}}

### `testGetNumRecordedErrors`

Gets the number of errors recorded for a the custom distribution metric.

{{#include ../../../shared/tab_header.md}}

<div data-lang="Kotlin" class="tab">

```Kotlin
import org.mozilla.yourApplication.GleanMetrics.Graphics

/// Did the metric receive a negative value?
assertEquals(
    0,
    Graphics.checkerboardPeak.testGetNumRecordedErrors(ErrorType.INVALID_VALUE)
)
```

</div>

<div data-lang="Java" class="tab">

```Java
import org.mozilla.yourApplication.GleanMetrics.Graphics;

/// Did the metric receive a negative value?
assertEquals(
    0,
    Graphics.INSTANCE.checkerboardPeak().testGetNumRecordedErrors(
        ErrorType.INVALID_VALUE
    )
)
```

</div>
<div data-lang="Swift" class="tab">

```Swift
// Assert that no errors were recorded.
XCTAssertEqual(0, Graphics.checkerboardPeak.testGetNumRecordedErrors(.invalidValue))
```

</div>
<div data-lang="Python" class="tab">

```Python
from glean import load_metrics
metrics = load_metrics("metrics.yaml")

# Assert that no errors were recorded.
assert 0 == metrics.graphics.checkerboard_peak.test_get_num_recorded_errors(
    ErrorType.INVALID_VALUE
)
```

</div>
<div data-lang="Rust" class="tab">

```Rust
use glean::ErrorType;
use glean_metrics::graphics;

// Were any of the values negative and thus caused an error to be recorded?
assert_eq!(
    0,
    graphics::checkerboard_peak.test_get_num_recorded_errors(
        ErrorType::InvalidValue,
        None
    )
);
```

</div>
<div data-lang="JavaScript" class="tab">

```js
import * as graphics from "./path/to/generated/files/graphics.js";
import { ErrorType } from "@mozilla/glean/<platform>";

// Did the metric receive a negative value?
assert.equal(
    0,
    graphics.checkerboardPeak.testGetNumRecordedErrors(ErrorType.InvalidValue)
);
```

</div>
<div data-lang="Firefox Desktop" class="tab"></div>

{{#include ../../../shared/tab_footer.md}}

## Metric Parameters

Example custom distribution metric definition:

```YAML
graphics:
  checkerboard_peak:
    type: custom_distribution
    description: >
      Peak number of CSS pixels checkerboarded during a checkerboard event.
    range_min: 1
    range_max: 66355200
    bucket_count: 50
    histogram_type: exponential
    unit: pixels
    gecko_datapoint: CHECKERBOARD_PEAK
    bugs:
      - https://bugzilla.mozilla.org/000000
    data_reviews:
      - https://bugzilla.mozilla.org/show_bug.cgi?id=000000#c3
    notification_emails:
      - me@mozilla.com
    expires: 2020-10-01
```

### Extra metric parameters

Custom distributions have the following required parameters:

- `range_min`: (Integer) The minimum value of the first bucket
- `range_max`: (Integer) The minimum value of the last bucket
- `bucket_count`: (Integer) The number of buckets
- `histogram_type`:
  - `linear`: The buckets are evenly spaced
  - `exponential`: The buckets follow a natural logarithmic distribution

> **Note** Check out how these bucketing algorithms would behave on the [Custom distribution simulator](#simulator).

Custom distributions have the following optional parameters:

- `unit`: (String) The unit of the values in the metric. For documentation purposes only -- does not affect data collection.
- `gecko_datapoint`: (String) This is a Gecko-specific property.
  It is the name of the Gecko metric to accumulate the data from,
  when using a Glean SDK in a product using GeckoView.


## Reference

* [Rust API docs](../../../docs/glean/private/struct.CustomDistributionMetric.html)

## Simulator

<div id="custom-data-modal-overlay">
    <div id="custom-data-modal">
        <p>Please, insert your custom data below as a JSON array.</p>
        <textarea rows="30"></textarea>
    </div>
</div>

<div id="simulator-container">
    <div id="histogram-chart-container">
        <div id="histogram-chart"></div>
        <p id="histogram-chart-legend"><p>
    </div>
    <div id="data-options">
        <h3>Data options</h3>
        <div class="input-group">
            <label for="normally-distributed">Generate normally distributed data</label>
            <input name="data-options" value="normally-distributed" id="normally-distributed" type="radio" />
        </div>
        <div class="input-group">
            <label for="log-normally-distributed">Generate log-normally distributed data</label>
            <input name="data-options" value="log-normally-distributed" id="log-normally-distributed" type="radio" checked />
        </div>
        <div class="input-group">
            <label for="uniformly-distributed">Generate uniformly distributed data</label>
            <input name="data-options" value="uniformly-distributed" id="uniformly-distributed" type="radio" />
        </div>
        <div class="input-group" id="custom-data-input-group">
            <label for="custom">Use custom data</label>
            <input name="data-options" value="custom" id="custom" type="radio" />
        </div>
    </div>
    <div id="histogram-props">
        <h3>Properties</h3>
        <div class="input-group">
            <label for="kind">Histogram type (<code>histogram_type</code>)</label>
            <select id="kind" name="kind">
                <option value="exponential" selected>Exponential</option>
                <option value="linear">Linear</option>
            </select>
        </div>
        <div class="input-group">
            <label for="lower-bound">Range minimum (<code>range_min</code>)</label>
            <input name="lower-bound" id="lower-bound" type="number" value="1" />
        </div>
        <div class="input-group">
            <label for="upper-bound">Range maximum (<code>range_max</code>)</label>
            <input name="upper-bound" id="upper-bound" type="number" value="500" />
        </div>
        <div class="input-group">
            <label for="bucket-count">Bucket count (<code>bucket_count</code>)</label>
            <input name="bucket-count" id="bucket-count" type="number" value="20" />
        </div>
    </div>
</div>
