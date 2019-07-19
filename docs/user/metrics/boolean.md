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

```Kotlin
import org.mozilla.yourApplication.GleanMetrics.Flags

Flags.a11yEnabled.set(System.isAccesibilityEnabled())
```

There are test APIs available too:

```Kotlin
import org.mozilla.yourApplication.GleanMetrics.Flags
Glean.enableTestingMode()

// Was anything recorded?
assertTrue(Flags.a11yEnabled.testHasValue())
// Does it have the expected value?
assertTrue(Flags.a11yEnabled.testGetValue())
```

## Limits

* None.

## Examples

* Is a11y enabled?

## Recorded errors

* None.

## Reference

* [Kotlin API docs](../../../javadoc/glean/mozilla.telemetry.glean.private/-boolean-metric-type/index.html)

