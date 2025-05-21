# Labeled Booleans

Labeled booleans are used to record different related boolean flags.

## Recording API

### `set`

Sets one of the labels in a labeled boolean metric to a specific value.

{{#include ../../../shared/tab_header.md}}

<div data-lang="Kotlin" class="tab">

```Kotlin
import org.mozilla.yourApplication.GleanMetrics.Accessibility

Accessibility.features["screen_reader"].set(isScreenReaderEnabled())
Accessibility.features["high_contrast"].set(isHighContrastEnabled())
```
</div>

<div data-lang="Java" class="tab">

```Java
import org.mozilla.yourApplication.GleanMetrics.Accessibility;

Acessibility.INSTANCE.features()["screen_reader"].set(isScreenReaderEnabled());
Acessibility.INSTANCE.features()["high_contrast"].set(isHighContrastEnabled());
```
</div>

<div data-lang="Swift" class="tab">

```Swift
Accessibility.features["screen_reader"].set(isScreenReaderEnabled())
Accessibility.features["high_contrast"].set(isHighContrastEnabled())
```
</div>

<div data-lang="Python" class="tab">

```Python
from glean import load_metrics
metrics = load_metrics("metrics.yaml")

metrics.accessibility.features["screen_reader"].set(is_screen_reader_enabled())
metrics.accessibility.features["high_contrast"].set(is_high_contrast_enabled())
```
</div>

<div data-lang="Rust" class="tab">

```Rust
use glean_metrics::accessibility;

accessibility::features.get("screen_reader").set(is_screen_reader_enabled());
accessibility::features.get("high_contrast").set(is_high_contrast_enabled());
```
</div>

<div data-lang="JavaScript" class="tab">

```js
import * as acessibility from "./path/to/generated/files/acessibility.js";

acessibility.features["screen_reader"].set(this.isScreenReaderEnabled());
acessibility.features["high_contrast"].set(this.isHighContrastEnabled());
```
</div>

<div data-lang="Firefox Desktop" class="tab">

**C++**
```cpp
#include "mozilla/glean/AccessibleMetrics.h"

mozilla::glean::accessibility::features.Get("screen_reader"_ns).Set(true);
mozilla::glean::accessibility::features.Get("high_contrast"_ns).Set(false);
```

**JavaScript**
```js
Glean.accessibility.features.screen_reader.set(true);
Glean.accessibility.features["high_contrast"].set(false);
```
</div>

{{#include ../../../shared/tab_footer.md}}

#### Recorded Errors

* [`invalid_type`](../../user/metrics/error-reporting.md): if a non-boolean value is given.
{{#include ../../_includes/label-errors.md}}

#### Limits

{{#include ../../_includes/label-limits.md}}

## Testing API

### `testGetValue`

Gets the recorded value for a given label in a labeled boolean metric.  
Returns the count if data is stored.  
Returns a language-specific empty/null value if no data is stored.
Has an optional argument to specify the name of the ping you wish to retrieve data from, except
in Rust where it's required. `None` or no argument will default to the first value found for `send_in_pings`.

{{#include ../../../shared/tab_header.md}}

<div data-lang="Kotlin" class="tab">

```Kotlin
import org.mozilla.yourApplication.GleanMetrics.Accessibility

// Do the booleans have the expected values?
assertEquals(True, Accessibility.features["screen_reader"].testGetValue())
assertEquals(False, Accessibility.features["high_contrast"].testGetValue())
```
</div>

<div data-lang="Java" class="tab">

```Java
import org.mozilla.yourApplication.GleanMetrics.Accessibility;

// Do the booleans have the expected values?
assertEquals(True, Acessibility.INSTANCE.features()["screen_reader"].testGetValue());
assertEquals(False, Acessibility.INSTANCE.features()["high_contrast"].testGetValue());
```
</div>

<div data-lang="Swift" class="tab">

```Swift
// Do the booleans have the expected values?
XCTAssertEqual(true, Accessibility.features["screen_reader"].testGetValue())
XCTAssertEqual(false, Accessibility.features["high_contrast"].testGetValue())
```
</div>

<div data-lang="Python" class="tab">

```Python
from glean import load_metrics
metrics = load_metrics("metrics.yaml")

# Do the booleans have the expected values?
assert metrics.accessibility.features["screen_reader"].test_get_value()
assert not metrics.accessibility.features["high_contrast"].test_get_value()
```
</div>

<div data-lang="Rust" class="tab">

```Rust
use glean_metrics::accessibility;

// Do the booleans have the expected values?
assert!(accessibility::features.get("screen_reader").test_get_value(None).unwrap());
assert!(!accessibility::features.get("high_contrast").test_get_value(None).unwrap());
```
</div>

<div data-lang="JavaScript" class="tab">

```js
import * as accessibility from "./path/to/generated/files/acessibility.js";

assert(await accessibility.features["screen_reader"].testGetValue());
assert(!(await accessibility.features["high_contrast"].testGetValue()));
```
</div>

<div data-lang="Firefox Desktop" class="tab">

**C++**
```cpp
#include "mozilla/glean/AccessibleMetrics.h"

ASSERT_EQ(
    true,
    mozilla::glean::accessibility::features.Get("screen_reader"_ns).TestGetValue().unwrap().ref());
ASSERT_EQ(
    false,
    mozilla::glean::accessibility::features.Get("high_contrast"_ns).TestGetValue().unwrap().ref());
```

**JavaScript**
```js
Assert.equal(true, Glean.accessibility.features["screen_reader"].testGetValue());
Assert.equal(false, Glean.accessibility.features.high_contrast.testGetValue());
```
</div>

{{#include ../../../shared/tab_footer.md}}

### `testGetNumRecordedErrors`

Gets the number of errors recorded for a given labeled boolean metric in total.

{{#include ../../../shared/tab_header.md}}

<div data-lang="Kotlin" class="tab">

```Kotlin
import org.mozilla.yourApplication.GleanMetrics.Accessibility

// Did we record any invalid labels?
assertEquals(
    0,
    Accessibility.features.testGetNumRecordedErrors(ErrorType.INVALID_LABEL)
)
```
</div>

<div data-lang="Java" class="tab">

```Java
import org.mozilla.yourApplication.GleanMetrics.Accessibility;

// Did we record any invalid labels?
assertEquals(
    0,
    Acessibility.INSTANCE.features().testGetNumRecordedErrors(ErrorType.INVALID_LABEL)
);
```
</div>

<div data-lang="Swift" class="tab">

```Swift
// Were there any invalid labels?
XCTAssertEqual(0, Accessibility.features.testGetNumRecordedErrors(.invalidLabel))
```
</div>

<div data-lang="Python" class="tab">

```Python
from glean import load_metrics
metrics = load_metrics("metrics.yaml")

# Did we record any invalid labels?
assert 0 == metrics.accessibility.features.test_get_num_recorded_errors(
    ErrorType.INVALID_LABEL
)
```
</div>

<div data-lang="Rust" class="tab">

```Rust
use glean::ErrorType;
use glean_metrics::accessibility;

// Did we record any invalid labels?
assert_eq!(
  1,
  accessibility::features.test_get_num_recorded_errors(
    ErrorType::InvalidLabel
  )
);
```
</div>

<div data-lang="JavaScript" class="tab">

```js
import * as accessibility from "./path/to/generated/files/acessibility.js";
import { ErrorType } from "@mozilla/glean/error";

assert(
  1,
  await accessibility.features.testGetNumRecordedErrors(ErrorType.InvalidLabel)
);
```
</div>

<div data-lang="Firefox Desktop" class="tab"></div>

{{#include ../../../shared/tab_footer.md}}

## Metric parameters

Example labeled boolean metric definition:

```YAML
accessibility:
  features:
    type: labeled_boolean
    description: >
      a11y features enabled on the device.
    bugs:
      - https://bugzilla.mozilla.org/000000
    data_reviews:
      - https://bugzilla.mozilla.org/show_bug.cgi?id=000000#c3
    notification_emails:
      - me@mozilla.com
    expires: 2020-10-01
    labels:
      - screen_reader
      - high_contrast
      ...
```

### Extra metric parameters

{{#include ../../_includes/labels-parameter.md}}

## Data questions

* Which accessibility features are enabled?

## Reference

* Python API docs: [`LabeledBooleanMetricType`](../../../python/glean/metrics/labeled.html#glean.metrics.labeled.LabeledBooleanMetricType), [`BooleanMetricType`](../../../python/glean/metrics/index.html#glean.metrics.BooleanMetric)
* Rust API docs: [`LabeledMetric`](../../../docs/glean/private/struct.LabeledMetric.html), [`BooleanMetric`](../../../docs/glean/private/struct.BooleanMetric.html)
* Swift API docs: [`LabeledMetricType`](../../../swift/Classes/LabeledMetricType.html), [`BooleanMetric`](../../../swift/Classes/BooleanMetric.html)
