# Labeled Counters 

Labeled counters are used to record different related counts that should sum up to a total.

## Configuration

For example, you may want to record a count of different types of crashes for your Android application, such as native code crashes and uncaught exceptions:

```YAML
stability:
  crash_count:
    type: labeled_counter
    description: >
      Counts the number of crashes that occur in the application. ...
    labels:
      - uncaught_exception
      - native_code_crash
    ...
```

## API

Now you can use the labeled counter from the application's code:

```Kotlin
import org.mozilla.yourApplication.GleanMetrics.Stability
Stability.crashCount["uncaught_exception"].add() // Adds 1 to the "uncaught_exception" counter.
Stability.crashCount["native_code_crash"].add(3) // Adds 3 to the "native_code_crash" counter.
```

There are test APIs available too:

```Kotlin
import org.mozilla.yourApplication.GleanMetrics.Stability
Glean.enableTestingMode()
// Was anything recorded?
assertTrue(Stability.crashCount["uncaught_exception"].testHasValue())
assertTrue(Stability.crashCount["native_code_crash"].testHasValue())
// Do the counters have the expected values?
assertEquals(1, Stability.crashCount["uncaught_exception"].testGetValue())
assertEquals(3, Stability.crashCount["native_code_crash"].testGetValue())
```

## Limits

* Labels support alphanumeric characters; they additionally allow for dots (`.`), underscores (`_`) and/or hyphens (`-`).

* Labels are limited to starting with either a letter or an underscore character.

* Each label must have a maximum of 60 characters.

* If the labels are specified in the `metrics.yaml`, using a different label will be replaced with the special value `__other__`.

* If the labels aren't specified in the `metrics.yaml`, only 16 different dynamic labels may be used, after which the special value `__other__` will be used.

## Examples

* Record the number of times different kinds of crashes occurred.

## Recorded Errors

* `invalid_label`: If the label contains invalid characters.

* `invalid_label`: If the label exceeds the maximum number of allowed characters.

## Reference

* Kotlin API docs [LabeledMetricType](../../../javadoc/glean/mozilla.telemetry.glean.private/-labeled-metric-type/index.html), [CounterMetricType](../../../javadoc/glean/mozilla.telemetry.glean.private/-counter-metric-type/index.html)
