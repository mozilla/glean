# The `baseline` ping

## Description

This ping is intended to provide metrics that are managed by the library itself, and not explicitly
set by the application or included in the application's `metrics.yaml` file.

## Scheduling

The `baseline` ping is automatically sent when the application is moved to the [background](index.md#defining-background-state).

## Contents

---

*Not yet implemented*

---

| Field name | Type | Description |
|---|---|---|
| `duration` | Timespan | The duration, in seconds, of the last foreground session |
| `locale` | String | The locale of the application |

The `baseline` ping also includes the common [ping sections](pings.md) found in all pings.
