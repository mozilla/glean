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

<div data-lang="Swift" class="tab">

```Swift
Flags.a11yEnabled.set(self.isAccesibilityElement)
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

{{#include ../../tab_footer.md}}

## Limits

* None.

## Examples

* Is a11y enabled?

## Recorded errors

* None.

## Reference

* [Kotlin API docs](../../../javadoc/glean/mozilla.telemetry.glean.private/-boolean-metric-type/index.html)

