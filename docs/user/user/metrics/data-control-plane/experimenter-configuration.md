# Experimenter Configuration

The structure of this configuration is a key-value collection with the full metric identification of the Glean metric serving as the key in the format <metric_category.metric_name>.

The values of the key-value pair are booleans which represent whether the metric is enabled (`true`) or not (`false`).

In the example below `gleanMetricConfiguration` is the name of the variable defined in the Nimbus feature.

This configuration would be what is entered into the branch configuration setup in Experimenter when defining an experiment or rollout.

## Example Configuration:

```json
{
  "gleanMetricConfiguration": {
    "urlbar.abandonment": true,
    "urlbar.engagement": true,
    "urlbar.impression": true
  }
}
```
