# The `events` ping

## Description

The events ping's purpose is to transport event metric information.

If the application crashes, an `events` ping is generated next time the application starts with events that were not sent before the crash.

### Platform availability

| SDK | Kotlin | Swift | Python | Rust | JavaScript | Firefox Desktop |
|-:|:-:|:-:|:-:|:-:|:-:|:-:|
| [`events` ping](events.md) | ✅ | ✅ | ✅  | ✅ | ✅ | ✅ |

## Scheduling

The `events` ping is automatically submitted under the following circumstances:

1. If there are any recorded events to send when the application becomes inactive (on mobile, this means going to [background](sent-by-glean.md#defining-foreground-and-background-state)).

2. When the queue of events exceeds `Glean.configuration.maxEvents` (default 500). This configuration
option can be changed at [initialization](../../reference/general/initializing.md).

3. If there are any unsent events found on disk when starting the application. _(This results in this ping never containing the [`glean.restarted`](./custom.md#the-gleanrestarted-event) event.)_

{{#include ../../../shared/blockquote-info.html}}

### Python and JavaScript caveats

> Since the Glean Python and JavaScript SDKs don't have a generic concept of "inactivity",
> case (1) above cannot be handled automatically.
>
> On Python, users can call the [`handle_client_inactive`](../../../python/glean/#glean.Glean.handle_client_inactive)
> API to let Glean know the app is inactive and that will trigger submission of the `events` ping.
>
> On JavaScript there is no such API and only cases (2) and (3) apply.

## Contents

At the top-level, this ping contains the following keys:

- `client_info`: The information [common to all pings](index.md#the-client_info-section).

- `ping_info`: The information [common to all pings](index.md#the-ping_info-section).

- `events`: An array of all of the events that have occurred since the last time the `events` ping was sent.

Each entry in the `events` array is an object with the following properties:

- `"timestamp"`: The milliseconds relative to the first event in the ping.

- `"category"`: The category of the event, as defined by its location in the `metrics.yaml` file.

- `"name"`: The name of the event, as defined in the `metrics.yaml` file.

- `"extra"` (optional): A mapping of strings to strings providing additional data about the event. The keys are restricted to 40 characters and values in this map will never exceed 100 characters.
  
### Example event JSON

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
    "end_time": "2019-03-29T10:02-04:00"
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
  "events": [
    {
      "timestamp": 0,
      "category": "examples",
      "name": "event_example",
      "extra": {
        "metadata1": "extra",
        "metadata2": "more_extra"
      }
    },
    {
      "timestamp": 1000,
      "category": "examples",
      "name": "event_example"
    }
  ]
}
```
