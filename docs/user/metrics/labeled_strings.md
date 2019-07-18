# Labeled Strings 

Labeled strings record multiple Unicode string values, each under a different label.

## Configuration

For example to record which kind of error occured in different stages of a login process - `"RuntimeException"` in the `"server_auth"` stage or `"invalid_string"` in the `"enter_email"` stage:

```YAML
login:
  errors_by_stage:
    type: labeled_string
    description: Records the error type, if any, that occur in different stages of the login process.
    labels:
      - server_auth
      - enter_email
    ...
```

## API

Now you can use the labeled string from the application's code:

```Kotlin
import org.mozilla.yourApplication.GleanMetrics.Login

Login.errorsByStage["server_auth"].set("Invalid password")
```

There are test APIs available too:

```Kotlin
import org.mozilla.yourApplication.GleanMetrics.Login
Glean.enableTestingMode()

// Was anything recorded?
assertTrue(Login.errorsByStage["server_auth"].testHasValue())
```

## Limits


* Labels support alphanumeric characters; they additionally allow for dots (`.`), underscores (`_`) and/or hyphens (`-`).

* Labels are limited to starting with either a letter or an underscore character.

* Each label must have a maximum of 60 characters.

* If the labels are specified in the `metrics.yaml`, using a different label will be replaced with the special value `__other__`.

* If the labels aren't specified in the `metrics.yaml`, only 16 different dynamic labels may be used, after which the special value `__other__` will be used.

## Examples

* What kind of errors occurred at each step in the login process? 

## Recorded Errors

* `invalid_label`: If the label contains invalid characters.

* `invalid_label`: If the label exceeds the maximum number of allowed characters.

## Reference

* Kotlin API docs: [LabeledMetricType](../../../javadoc/glean/mozilla.telemetry.glean.private/-labeled-metric-type/index.html), [StringMetricType](../../../javadoc/glean/mozilla.telemetry.glean.private/-string-metric-type/index.html)
