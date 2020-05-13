# Boolean

Booleans are used for simple flags, for example "is a11y enabled"?.

## Configuration

Say you're adding a boolean to record whether a11y is enabled on the device. First you need to add an entry for the boolean to the `metrics.yaml` file:

```YAML
flags:
  a11y_enabled:
    type: boolean
    description: >
      Records whether a11y is enabled on the device.
    lifetime: application
    ...
```

## API

{{#include ../../tab_header.md}}

<div data-lang="Kotlin" class="tab">

```Kotlin
import org.mozilla.yourApplication.GleanMetrics.Flags

Flags.a11yEnabled.set(System.isAccesibilityEnabled())
```

There are test APIs available too:

```Kotlin
import org.mozilla.yourApplication.GleanMetrics.Flags

// Was anything recorded?
assertTrue(Flags.a11yEnabled.testHasValue())
// Does it have the expected value?
assertTrue(Flags.a11yEnabled.testGetValue())
```

</div>

<div data-lang="Java" class="tab">

```Java
import org.mozilla.yourApplication.GleanMetrics.Flags;

Flags.INSTANCE.a11yEnabled.set(System.isAccessibilityEnabled());
```

There are test APIs available too:

```Java
import org.mozilla.yourApplication.GleanMetrics.Flags;

// Was anything recorded?
assertTrue(Flags.INSTANCE.a11yEnabled.testHasValue());
// Does it have the expected value?
assertTrue(Flags.INSTANCE.a11yEnabled.testGetValue());
```

</div>


<div data-lang="Swift" class="tab">

```Swift
Flags.a11yEnabled.set(self.isAccessibilityEnabled)
```

There are test APIs available too:

```Swift
@testable import Glean

// Was anything recorded?
XCTAssertTrue(Flags.a11yEnabled.testHasValue())
// Does the counter have the expected value?
XCTAssertTrue(try Flags.a11yEnabled.testGetValue())
```

</div>

<div data-lang="Python" class="tab">

```Python
from glean import load_metrics
metrics = load_metrics("metrics.yaml")

metrics.flags.a11y_enabled.set(is_accessibility_enabled())
```

There are test APIs available too:

```Python
# Was anything recorded?
assert metrics.flags.a11y_enabled.test_has_value()
# Does it have the expected value?
assert True is metrics.flags.a11y_enabled.test_get_value()
```

</div>

{{#include ../../tab_footer.md}}

## Limits

* None.

## Examples

* Is a11y enabled?

## Recorded errors

* None.

## Reference

* [Kotlin API docs](../../../javadoc/glean/mozilla.telemetry.glean.private/-boolean-metric-type/index.html)
* [Swift API docs](../../../swift/Classes/BooleanMetricType.html)
* [Python API docs](../../../python/glean/metrics/boolean.html)
