# Labeled Counters

Labeled counters are used to record different related counts that should sum up to a total.
Each counter always starts from `0`.
Each time you record to a labeled counter, its value is incremented.
Unless incremented by a positive value, a counter will not be reported in pings,
that means: the value `0` is never sent in a ping.

## Recording API

### `add`

Increases one of the labels in a labeled counter metric by a certain amount.
If no amount is passed it defaults to `1`.

{{#include ../../../shared/tab_header.md}}

<div data-lang="Kotlin" class="tab">

```Kotlin
import org.mozilla.yourApplication.GleanMetrics.Stability

Stability.crashCount["uncaught_exception"].add() // Adds 1 to the "uncaught_exception" counter.
Stability.crashCount["native_code_crash"].add(3) // Adds 3 to the "native_code_crash" counter.
```
</div>

<div data-lang="Java" class="tab">

```Java
import org.mozilla.yourApplication.GleanMetrics.Stability;

Stability.INSTANCE.crashCount()["uncaught_exception"].add(); // Adds 1 to the "uncaught_exception" counter.
Stability.INSTANCE.crashCount()["native_code_crash"].add(3); // Adds 3 to the "native_code_crash" counter.
```
</div>

<div data-lang="Swift" class="tab">

```Swift
Stability.crashCount["uncaught_exception"].add() // Adds 1 to the "uncaught_exception" counter.
Stability.crashCount["native_code_crash"].add(3) // Adds 3 to the "native_code_crash" counter.
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
</div>

<div data-lang="Rust" class="tab">

```Rust
use glean_metrics::stability;

stability::crash_count.get("uncaught_exception").add(1); // Adds 1 to the "uncaught_exception" counter.
stability::crash_count.get("native_code_crash").add(3); // Adds 3 to the "native_code_crash" counter.
```
</div>

<div data-lang="JavaScript" class="tab">

```js
import * as stability from "./path/to/generated/files/stability.js";

// Adds 1 to the "uncaught_exception" counter.
stability.crashCount["uncaught_exception"].add();
// Adds 3 to the "native_code_crash" counter.
stability.crashCount["native_code_crash"].add(3);
```
</div>

<div data-lang="Firefox Desktop" class="tab">

**C++**
```cpp
#include "mozilla/glean/GleanMetrics.h"

mozilla::glean::stability::crash_count.Get("uncaught_exception"_ns).Add(1);
mozilla::glean::stability::crash_count.Get("native_code_crash"_ns).Add(3);
```

**JavaScript**
```js
Glean.stability.crashCount.uncaught_exception.add(1);
Glean.stability.crashCount["native_code_crash"].add(3);
```
</div>

{{#include ../../../shared/tab_footer.md}}

#### Recorded Errors

* [`invalid_value`](../../user/metrics/error-reporting.md): if the counter is incremented by `0` or a negative value.
* [`invalid_type`](../../user/metrics/error-reporting.md): if a floating point or non-number value is given.
{{#include ../../_includes/label-errors.md}}

#### Limits

* Only increments
* Saturates at the largest value that can be represented as a 32-bit signed integer.
{{#include ../../_includes/label-limits.md}}

## Testing API

### `testGetValue`

Gets the recorded value for a given label in a labeled counter metric.  
Returns the count if data is stored.  
Returns a language-specific empty/null value if no data is stored.

{{#include ../../../shared/tab_header.md}}

<div data-lang="Kotlin" class="tab">

```Kotlin
import org.mozilla.yourApplication.GleanMetrics.Stability

// Do the counters have the expected values?
assertEquals(1, Stability.crashCount["uncaught_exception"].testGetValue())
assertEquals(3, Stability.crashCount["native_code_crash"].testGetValue())
```
</div>

<div data-lang="Java" class="tab">

```Java
import org.mozilla.yourApplication.GleanMetrics.Stability;

// Do the counters have the expected values?
assertEquals(1, Stability.INSTANCE.crashCount()["uncaught_exception"].testGetValue());
assertEquals(3, Stability.INSTANCE.crashCount()["native_code_crash"].testGetValue());
```
</div>

<div data-lang="Swift" class="tab">

```Swift
// Do the counters have the expected values?
XCTAssertEqual(1, Stability.crashCount["uncaught_exception"].testGetValue())
XCTAssertEqual(3, Stability.crashCount["native_code_crash"].testGetValue())
```
</div>

<div data-lang="Python" class="tab">

```Python
from glean import load_metrics
metrics = load_metrics("metrics.yaml")

# Do the counters have the expected values?
assert 1 == metrics.stability.crash_count["uncaught_exception"].test_get_value()
assert 3 == metrics.stability.crash_count["native_code_crash"].test_get_value()
```
</div>

<div data-lang="Rust" class="tab">

```Rust
use glean_metrics::stability;

// Do the counters have the expected values?
assert_eq!(1, stability::crash_count.get("uncaught_exception").test_get_value().unwrap());
assert_eq!(3, stability::crash_count.get("native_code_crash").test_get_value().unwrap());
```
</div>

<div data-lang="JavaScript" class="tab">

```js
import * as stability from "./path/to/generated/files/stability.js";

// Do the counters have the expected values?
assert.strictEqual(1, await stability.crashCount["uncaught_exception"].testGetValue());
assert.strictEqual(3, await stability.crashCount["native_code_crash"].testGetValue());
```
</div>

<div data-lang="Firefox Desktop" class="tab">

**C++**
```cpp
#include "mozilla/glean/GleanMetrics.h"

ASSERT_EQ(
    1,
    mozilla::glean::stability::crash_count.Get("uncaught_exception"_ns).TestGetValue().unwrap().ref());
ASSERT_EQ(
    3,
    mozilla::glean::stability::crash_count.Get("native_code_crash"_ns).TestGetValue().unwrap().ref());
```

**JavaScript**
```js
Assert.equal(1, Glean.stability.crashCount["uncaught_exception"].testGetValue());
Assert.equal(3, Glean.stability.crashCount.native_code_crash.testGetValue());
```
</div>

{{#include ../../../shared/tab_footer.md}}

### `testGetNumRecordedErrors`

Gets the number of errors recorded for a given labeled counter metric in total.

{{#include ../../../shared/tab_header.md}}

<div data-lang="Kotlin" class="tab">

```Kotlin
import org.mozilla.yourApplication.GleanMetrics.Stabilit

// Were there any invalid labels?
assertEquals(
    0,
    Stability.crashCount.testGetNumRecordedErrors(ErrorType.INVALID_LABEL)
)
```
</div>

<div data-lang="Java" class="tab">

```Java
import org.mozilla.yourApplication.GleanMetrics.Stability;

// Were there any invalid labels?
assertEquals(
    0,
    Stability.INSTANCE.crashCount().testGetNumRecordedErrors(ErrorType.INVALID_LABEL)
);
```
</div>

<div data-lang="Swift" class="tab">

```Swift
// Were there any invalid labels?
XCTAssertEqual(0, Stability.crashCount.testGetNumRecordedErrors(.invalidLabel))
```
</div>

<div data-lang="Python" class="tab">

```Python
from glean import load_metrics
metrics = load_metrics("metrics.yaml")

# Were there any invalid labels?
assert 0 == metrics.stability.crash_count.test_get_num_recorded_errors(
    ErrorType.INVALID_LABEL
)
```
</div>

<div data-lang="Rust" class="tab">

```Rust
use glean::ErrorType;

use glean_metrics::stability;

// Were there any invalid labels?
assert_eq!(
  0,
  stability::crash_count.test_get_num_recorded_errors(
    ErrorType::InvalidLabel
  )
);
```
</div>

<div data-lang="JavaScript" class="tab">

```js
import * as stability from "./path/to/generated/files/stability.js";
import { ErrorType } from "@mozilla/glean/error";

// Were there any invalid labels?
assert(
  0,
  await stability.crashCount.testGetNumRecordedErrors(ErrorType.InvalidLabel)
);
```
</div>

<div data-lang="Firefox Desktop" class="tab"></div>

{{#include ../../../shared/tab_footer.md}}

## Metric parameters

Example labeled counter metric definition:

```YAML
accessibility:
  features:
    type: labeled_counter
    description: >
      Counts the number of crashes that occur in the application.
    bugs:
      - https://bugzilla.mozilla.org/000000
    data_reviews:
      - https://bugzilla.mozilla.org/show_bug.cgi?id=000000#c3
    notification_emails:
      - me@mozilla.com
    expires: 2020-10-01
    labels:
      - uncaught_exception
      - native_code_crash
      ...
```

### Extra metric parameters

{{#include ../../_includes/labels-parameter.md}}

## Data questions

* How many times did different types of crashes occur?

## Reference

* Swift API docs: [`LabeledMetricType`](../../../swift/Classes/LabeledMetricType.html), [`CounterMetricType`](../../../swift/Classes/CounterMetricType.html)
* Python API docs: [`LabeledMetricBase`](../../../python/glean/metrics/labeled.html), [`CounterMetricType`](../../../python/glean/metrics/counter.html)
* Rust API docs: [`LabeledMetric`](../../../docs/glean/private/struct.LabeledMetric.html), [`CounterMetricType`](../../../docs/glean/private/struct.CounterMetric.html)
* JavaScript API docs: [`LabeledMetricType`](https://mozilla.github.io/glean.js/classes/core_metrics_types_labeled.default.html), [`CounterMetricType`](https://mozilla.github.io/glean.js/classes/core_metrics_types_counter.default.html)
