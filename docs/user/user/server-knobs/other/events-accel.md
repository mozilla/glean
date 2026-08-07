# Events Ping Acceleration Factor

By default, Glean batches events together to submit on a single ["events" ping](../../pings/events.md).
The number of events Glean will collect before submitting an "events" ping is determined at
[Glean initialization](../../../reference/general/initializing.md) via the `maxEvents`
configuration option and can be changed remotely via the
[`event_threshold` Server Knob](./max-events.md).

This Server Knob overrides the `eventsPingAccelerationFactor` configuration option from Glean initialization.
It takes effect immediately, but if the number of "events" pings already submitted this session exceeds the factor,
it might have no practical effect.

## Example Configuration:

```json
{
  "gleanMetricConfiguration": {
    "events_ping_acceleration_factor": 5
  }
}
```

{{#include ../../../_includes/server-knobs-config-in-pings.md}}
