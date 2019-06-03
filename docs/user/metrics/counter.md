# Counter

Used to count how often something happens, say how often a certain button was
pressed. A counter always starts from `0`. Each time you record to a counter,
its value is incremented.

## Configuration

Say you're adding a new counter for how often the refresh button is pressed.
First you need to add an entry for the counter to the `metrics.yaml` file:

```YAML
controls:
  refresh_pressed:
    type: counter
    description: >
      Counts how often the refresh button is pressed.
    ...
```

## API

```Kotlin
import org.mozilla.yourApplication.GleanMetrics.Controls

Controls.refreshPressed.add() // Adds 1 to the counter.
Controls.refreshPressed.add(5) // Adds 5 to the counter.
```

There are test APIs available too:

```Kotlin
import org.mozilla.yourApplication.GleanMetrics.Controls
Glean.enableTestingMode()

// Was anything recorded?
assertTrue(Controls.refreshPressed.testHasValue())
// Does the counter have the expected value?
assertEquals(6, Controls.refreshPressed.testGetValue())
```

## Limits

* Only increments, saturates at the limits of a 32-bit signed integer.

## Examples

* How often a certain button was pressed?

## Recorded errors

* None.

## Reference

* [Kotlin
  API docs](../../../javadoc/glean/mozilla.telemetry.glean.private/-counter-metric-type/index.html)

