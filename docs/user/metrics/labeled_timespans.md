# Labeled Timespans 

Used to measure how much time is spent in a set of related tasks.

## Configuration

For example, to record the time spent in different stages in a login process:

```YAML
auth:
  times_per_stage:
    type: labeled_timespan
    description: The time spent in the different stages of the login process.
    labels:
      - fill_form
      - auth_with_server
      - load_next_view
    ...
```

## API

Now you can use the labeled timespan from the application's code. 
Each time interval that the metric records must be associated with an object provided by the user. 
This is so that intervals can be measured concurrently. 
In our example, using time in different stages of the login process, this might be an object representing the login UI page.

```Kotlin
import org.mozilla.yourApplication.GleanMetrics.Auth
fun onShowLoginForm(e: Event) {
    Auth.timesPerStage["fill_form"].start(e.target)
    // ...
}
fun onLoginFormSubmitted(e: Event) {
    Auth.timesPerStage["fill_form"].stopAndSum(e.target)
    Auth.timesPerStage["auth_with_server"].start(e.target)
    // ...
}
// ... etc.
fun onLoginCancel(e: Event) {
    Auth.timesPerStage["fill_form"].cancel(e.target)
    // ...
}
```

The times reported in the Glean ping will be the sum of all of these timespans recorded during the lifetime of the ping.

There are test APIs available too:

```Kotlin
import org.mozilla.yourApplication.GleanMetrics.Auth
Glean.enableTestingMode()
// Was anything recorded?
assertTrue(Auth.timesPerStage["fill_form"].testHasValue())
assertTrue(Auth.timesPerStage["auth_with_server"].testHasValue())
// Does the timer have the expected value
assertTrue(Auth.timesPerStage["fill_form"].testGetValue() > 0)
assertTrue(Auth.timesPerStage["auth_with_server"].testGetValue() > 0)Now you can use the labeled counter from the application's code:
```

## Limits


* Labels support lowercase alphanumeric characters; they additionally allow for dots (`.`), underscores (`_`) and/or hyphens (`-`).

* Labels are limited to starting with either a letter or an underscore character.

* Each label must have a maximum of 60 characters.

* If the labels are specified in the `metrics.yaml`, using a different label will be replaced with the special value `__other__`.

* If the labels aren't specified in the `metrics.yaml`, only 16 different dynamic labels may be used, after which the special value `__other__` will be used.

## Examples

* Record the time spent in different stages in a login process.

## Recorded Errors

* `invalid_label`: If the label contains invalid characters.

* `invalid_label`: If the label exceeds the maximum number of allowed characters.

## Reference

* Kotlin API docs [LabeledMetricType](../../../javadoc/glean/mozilla.telemetry.glean.private/-labeled-metric-type/index.html), [TimespanMetricType](../../../javadoc/glean/mozilla.telemetry.glean.private/-timespan-metric-type/index.html)
