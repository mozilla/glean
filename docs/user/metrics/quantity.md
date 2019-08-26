# Quantity

Used to record a single non-negative integer value.
For example, the width of the display in pixels.

> Quantities are only available for metrics that come from Gecko.

## Configuration

Say you're adding a new quantity for the width of the display in pixels. First you need to add an entry for the quantity to the `metrics.yaml` file:

```YAML
gfx:
  display_width:
    type: quantity
    description: >
      The width of the display, in pixels.
    unit: pixels
    gecko_datapoint: DISPLAY_W_PIXELS
    ...
```

Note that quantities have a required `unit` parameter, which is a free-form string for documentation purposes.

## API

```Kotlin
import org.mozilla.yourApplication.GleanMetrics.Gfx

Gfx.displayWidth.set(width)
```

There are test APIs available too:

```Kotlin
import org.mozilla.yourApplication.GleanMetrics.Gfx

// Was anything recorded?
assertTrue(Gfx.displayWidth.testHasValue())
// Does the quantity have the expected value?
assertEquals(6, Gfx.displayWidth.testGetValue())
```

## Limits

* Quantities must be non-negative integers.

* Quantities are only available for metrics that come from Gecko.

## Examples

* What is the width of the display, in pixels?

## Recorded errors

* `invalid_value`: If a negative value is passed in.

## Reference

* [Kotlin API docs](../../../javadoc/glean/mozilla.telemetry.glean.private/-quantity-metric-type/index.html)
