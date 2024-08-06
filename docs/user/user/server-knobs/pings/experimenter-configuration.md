# Experimenter Configuration

The structure of this configuration is a key-value collection with the name of the Glean ping serving as the keys and the values are booleans representing whether the ping is enabled (`true`) or not (`false`).

In the example below, `gleanMetricConfiguration` is the name of the variable defined in the Nimbus feature.

This configuration would be what is entered into the branch configuration setup in Experimenter when defining an experiment or rollout.

## Example Configuration:

```json
{
  "gleanMetricConfiguration": {
    "pings_enabled": {
      "baseline": false,
      "events": false,
      "metrics": false
    }
  }
}
```
