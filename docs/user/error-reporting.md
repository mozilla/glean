# Error reporting

The Glean SDK records the number of errors that occur when metrics are passed invalid data or are otherwise used incorrectly.
This information is reported back in special labeled counter metrics in the `glean.error` category.
Error metrics are included in the same pings as the metric that caused the error.
Additionally, error metrics are always sent in the [`metrics` ping](pings/metrics.md) ping.

The following categories of errors are recorded:

- `invalid_value`: The metric value was invalid.
- `invalid_label`: The label on a labeled metric was invalid.
- `invalid_state`: The metric caught an invalid state while recording.
- `invalid_overflow`: The metric value to be recorded overflows the metric-specific upper range.

For example, if you had a string metric and passed it a string that was too long:

```Kotlin
MyMetrics.stringMetric.set("this_string_is_longer_than_the_limit_for_string_metrics")
```

The following error metric counter would be incremented:

```Kotlin
Glean.error.invalidOverflow["my_metrics.string_metric"].add(1)
```

Resulting in the following keys in the ping:

```json
{
  "metrics": {
    "labeled_counter": {
      "glean.error.invalid_overflow": {
        "my_metrics.string_metric": 1
      }
    }
  }
}
```

If you have a debug build of the Glean SDK, details about the errors being recorded are included in the logs. This detailed information is not included in Glean pings.

