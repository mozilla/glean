# Flows Enabled

A [flow](../../flows/index.md) is instrumentation that can be wrong: boundaries declared over the
wrong events, a path missing from `ends_on`, a funnel that turns out to measure something other than
what was intended. The `flows_enabled` Server Knob switches individual flows on or off without
shipping a build.

Keys are flow names, and values override the flow's `disabled` setting from `flows.yaml`, in either
direction. A flow shipped with `disabled: true` can be switched on remotely. A flow absent from the
map keeps its `flows.yaml` setting, and a key that names no flow is ignored.

## When it takes effect

The setting is checked when an entry event would open a new instance, and only then:

- Switching a flow off stops new instances from opening. An instance that is already open runs to
  its normal end and records its `glean.flow_end` as usual. Its events stay tagged throughout.
- Entry events recorded while the flow is off open nothing, and aren't counted in
  `glean.flows_not_started`, since no instance was lost to timing.
- Switching a flow back on takes effect at its next entry event.

The delay before a switch-off fully takes effect is the remaining life of at most one open instance
per flow, bounded by its `auto_close_timeout` or the end of its session. That's one more reason to
give flows an `auto_close_timeout`.

A misspelled flow name fails silently and leaves the flow running. After throwing a switch, check both
that the applied configuration (below) contains the key, and that new instances of the flow have
stopped appearing.

## Example Configuration:

```json
{
  "gleanMetricConfiguration": {
    "flows_enabled": {
      "onboarding": false
    }
  }
}
```

## Disabling a flow's entry or exit event instead

Turning off a flow's entry or exit *event* through
[`metrics_enabled`](../metrics/index.md) is not a way to turn off the flow. An entry event that isn't
recorded can't open an instance, and an exit event that isn't recorded can't close one, so every
instance then ends as `timeout` or `session_end`. In the data, that looks like a healthy funnel
suddenly being abandoned, with no error anywhere. Glean logs a warning when a configuration does this.
To stop a flow, use `flows_enabled`.

{{#include ../../../_includes/server-knobs-config-in-pings.md}}
