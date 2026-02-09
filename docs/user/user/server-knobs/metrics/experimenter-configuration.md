# Experimenter Configuration

The structure of this configuration is a key-value collection with the full metric identification of the Glean metric serving as the key in the format <metric_category.metric_name>.

The values of the key-value pair are booleans which represent whether the metric is enabled (`true`) or not (`false`).

In the example below `gleanMetricConfiguration` is the name of the variable defined in the Nimbus feature.

This configuration would be what is entered into the branch configuration setup in Experimenter when defining an experiment or rollout.

## Example Configuration:

```json
{
  "gleanMetricConfiguration": {
    "metrics_enabled": {
      "urlbar.abandonment": true,
      "urlbar.engagement": true,
      "urlbar.impression": true
    }
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

