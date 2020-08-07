# The `baseline` ping

## Description

This ping is intended to provide metrics that are managed by the Glean SDK itself, and not explicitly set by the application or included in the application's `metrics.yaml` file.

> **Note:** As the `baseline` ping was specifically designed for mobile operating systems, it is not sent when using the Glean Python bindings.

## Scheduling

The `baseline` ping is automatically submitted with a `reason: foreground` when the application is moved to the [foreground](index.md#defining-foreground-and-background-state).  These baseline pings do not contain `duration`.

The `baseline` ping is automatically submitted with a `reason: background` when the application is moved to the [background](index.md#defining-foreground-and-background-state).
Occasionally, the `baseline` ping may fail to send when going to background (e.g. the process is killed quickly).  In that case, it will be submitted at startup with a `reason: dirty_startup`, if the previous session was not cleanly closed. This only happens from the second start onward.

See also the [ping schedules and timing overview](ping-schedules-and-timings.html).

## Contents

The baseline ping includes the following fields:

| Field name | Type | Description |
|---|---|---|
| `duration` | Timespan | The duration, in seconds, of the last foreground session. Only available if `reason: background`. [^1] |

[^1]: See also the [ping schedules and timing overview](ping-schedules-and-timings.html) for how the `duration` metric relates to other sources of timing in the `baseline` ping.

The `baseline` ping also includes the common [ping sections](index.md#ping-sections) found in all pings.

### Querying ping contents

A quick note about querying ping contents (i.e. for [sql.telemetry.mozilla.org](https://sql.telemetry.mozilla.org)):  Each metric in the baseline ping is organized by its metric type, and uses a namespace of `glean.baseline`. For instance, in order to select `duration` you would use `metrics.timespan['glean.baseline.duration']`. If you were trying to select a String based metric such as `os`, then you would use `metrics.string['glean.baseline.os']`

## Example baseline ping

```json
{
  "ping_info": {
    "experiments": {
      "third_party_library": {
        "branch": "enabled"
      }
    },
    "seq": 0,
    "start_time": "2019-03-29T09:50-04:00",
    "end_time": "2019-03-29T09:53-04:00",
    "reason": "foreground"
  },
  "client_info": {
    "telemetry_sdk_build": "0.49.0",
    "first_run_date": "2019-03-29-04:00",
    "os": "Android",
    "android_sdk_version": "27",
    "os_version": "8.1.0",
    "device_manufacturer": "Google",
    "device_model": "Android SDK built for x86",
    "architecture": "x86",
    "app_build": "1",
    "app_display_version": "1.0",
    "client_id": "35dab852-74db-43f4-8aa0-88884211e545"
  },
  "metrics": {
    "timespan": {
      "glean.baseline.duration": {
        "value": 52,
        "time_unit": "second"
      }
    }
  }
}
```
