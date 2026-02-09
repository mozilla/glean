# Max Events

By default, Glean batches events together to submit on a single events ping.
The `event_threshold` Server Knob controls how many events Glean will collect before submitting an events ping.

For instance, if you wanted to disable batching in order to transmit an events ping after every event is recorded you could set `event_threshold: 1`.

## Example Configuration:

```json
{
  "gleanMetricConfiguration": {
    "event_threshold": 1
  }
}
```

## Server Knobs Configuration in Pings

When Server Knobs configuration is applied through `applyServerKnobsConfig`, the entire configuration is automatically recorded as an `ObjectMetric` and included in the `ping_info` section of all pings. This makes it easier to identify which metrics are being controlled by Server Knobs and to calculate effective sampling rates in analysis.

The configuration is stored using a standard `ObjectMetric` (at `glean.internal.server_knobs_config`), which provides schema definition support for downstream tooling and requires minimal changes to ingestion pipeline schemas.

### How It Appears in Pings

The complete Server Knobs configuration is included in `ping_info.server_knobs_config`:

```json
{
  "ping_info": {
    "seq": 123,
    "start_time": "2024-01-01T00:00:00Z",
    "end_time": "2024-01-01T01:00:00Z",
    "server_knobs_config": {
      "metrics_enabled": {
        "urlbar.engagement": true,
        "urlbar.impression": true
      },
      "pings_enabled": {},
      "event_threshold": null
    }
  },
  "metrics": {
    "counter": {
      "urlbar.engagement": 5,
      "urlbar.impression": 2
    }
  }
}
```

