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
