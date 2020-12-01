# Labeled Booleans

Labeled booleans are used to record different related boolean flags.

## Configuration

For example, you may want to record a set of flags related to accessibility (a11y).

```YAML
accessibility:
  features:
    type: labeled_boolean
    description: >
      a11y features enabled on the device. ...
    labels:
      - screen_reader
      - high_contrast
    ...
```

> **Note:** removing or changing labels, including their order in the registry file, is permitted. Avoid reusing labels that were removed in the past. It is best practice to add documentation about removed labels to the description field so that analysts will know of their existence and meaning in historical data. Special care must be taken when changing GeckoView metrics sent through the Glean SDK, as the index of the labels is used to report Gecko data through the Glean SDK.

## API

Now you can use the labeled boolean from the application's code:

{{#include ../../tab_header.md}}

<div data-lang="Kotlin" class="tab">

```Kotlin
import org.mozilla.yourApplication.GleanMetrics.Accessibility
Accessibility.features["screen_reader"].set(isScreenReaderEnabled())
Accessibility.features["high_contrast"].set(isHighContrastEnabled())
```

There are test APIs available too:

```Kotlin
import org.mozilla.yourApplication.GleanMetrics.Accessibility
// Was anything recorded?
assertTrue(Accessibility.features["screen_reader"].testHasValue())
assertTrue(Accessibility.features["high_contrast"].testHasValue())
// Do the booleans have the expected values?
assertEquals(True, Accessibility.features["screen_reader"].testGetValue())
assertEquals(False, Accessibility.features["high_contrast"].testGetValue())
// Did we record any invalid labels?
assertEquals(0, Accessibility.features.testGetNumRecordedErrors(ErrorType.InvalidLabel))
```

</div>

<div data-lang="Swift" class="tab">

```Swift
Accessibility.features["screen_reader"].set(isScreenReaderEnabled())
Accessibility.features["high_contrast"].set(isHighContrastEnabled())
```

There are test APIs available too:

```Swift
@testable import Glean

// Was anything recorded?
XCTAssert(Accessibility.features["screen_reader"].testHasValue())
XCTAssert(Accessibility.features["high_contrast"].testHasValue())
// Do the booleans have the expected values?
XCTAssertEqual(true, try Accessibility.features["screen_reader"].testGetValue())
XCTAssertEqual(false, try Accessibility.features["high_contrast"].testGetValue())
// Were there any invalid labels?
XCTAssertEqual(0, Accessibility.features.testGetNumRecordedErrors(.invalidLabel))
```

</div>

<div data-lang="Python" class="tab">

```Python
from glean import load_metrics
metrics = load_metrics("metrics.yaml")

metrics.accessibility.features["screen_reader"].set(
    is_screen_reader_enabled()
)
metrics.accessibility.features["high_contrast"].set(
    is_high_contrast_enabled()
)
```

There are test APIs available too:

```Python
# Was anything recorded?
assert metrics.accessibility.features["screen_reader"].test_has_value()
assert metrics.accessibility.features["high_contrast"].test_has_value()
# Do the booleans have the expected values?
assert metrics.accessibility.features["screen_reader"].test_get_value()
assert not metrics.accessibility.features["high_contrast"].test_get_value()
# Did we record any invalid labels?
assert 0 == metrics.accessibility.features.test_get_num_recorded_errors(
    ErrorType.INVALID_LABEL
)
```

</div>

<div data-lang="C#" class="tab">

```C#
using static Mozilla.YourApplication.GleanMetrics.Accessibility;

Accessibility.features["screen_reader"].Set(isScreenReaderEnabled());
Accessibility.features["high_contrast"].Set(isHighContrastEnabled());
```

There are test APIs available too:

```C#
using static Mozilla.YourApplication.GleanMetrics.Accessibility;
// Was anything recorded?
Assert.True(Accessibility.features["screen_reader"].TestHasValue());
Assert.True(Accessibility.features["high_contrast"].TestHasValue());
// Do the booleans have the expected values?
Assert.Equal(true, Accessibility.features["screen_reader"].TestGetValue());
Assert.Equal(false, Accessibility.features["high_contrast"].TestGetValue());
// Did we record any invalid labels?
Assert.Equal(0, Accessibility.features.TestGetNumRecordedErrors(ErrorType.InvalidLabel));
```

</div>

<div data-lang="Rust" class="tab">

```rust
use glean_metrics;

accessibility::features.get("screen_reader").set(is_screen_reader_enabled());
accessibility::features.get("high_contrast").set(is_high_contrast_enabled());
```

There are test APIs available too:

```rust
use glean::ErrorType;

use glean_metrics;

// Was anything recorded?
assert!(accessibility::features.get("screen_reader").test_get_value(None).is_some());
assert!(accessibility::features.get("high_contrast").test_get_value(None).is_some());
// Do the booleans have the expected values?
assert!(accessibility::features.get("screen_reader").test_get_value(None).unwrap());
assert!(!accessibility::features.get("high_contrast").test_get_value(None).unwrap());
// Did we record any invalid labels?
assert_eq!(
  1,
  accessibility::features.test_get_num_recorded_errors(
    ErrorType::InvalidLabel
  )
);
```

</div>

{{#include ../../tab_footer.md}}

## Limits

* Labels must conform to the [label formatting regular expression](index.md#label-format).

* Labels support lowercase alphanumeric characters; they additionally allow for dots (`.`), underscores (`_`) and/or hyphens (`-`).

* Each label must have a maximum of 60 bytes, when encoded as UTF-8.

* If the labels are specified in the `metrics.yaml`, using any label not listed in that file will be replaced with the special value `__other__`.

* If the labels aren't specified in the `metrics.yaml`, only 16 different dynamic labels may be used, after which the special value `__other__` will be used.

## Examples

* Record a related set of boolean flags.

## Recorded Errors

* `invalid_label`: If the label contains invalid characters. Data is still recorded to the special label `__other__`.

* `invalid_label`: If the label exceeds the maximum number of allowed characters. Data is still recorded to the special label `__other__`.

## Reference

* Kotlin API docs [`LabeledMetricType`](../../../javadoc/glean/mozilla.telemetry.glean.private/-labeled-metric-type/index.html), [`BooleanMetricType`](../../../javadoc/glean/mozilla.telemetry.glean.private/-boolean-metric-type/index.html)
* Swift API docs: [`LabeledMetricType`](../../../swift/Classes/LabeledMetricType.html), [`BooleanMetricType`](../../../swift/Classes/BooleanMetricType.html)
* Python API docs: [`LabeledMetricBase`](../../../python/glean/metrics/labeled.html), [`BooleanMetricType`](../../../python/glean/metrics/boolean.html)
