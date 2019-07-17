# Payload format

The main sections of a Glean ping are described in [Ping Sections](../../../user/pings/index.md#Ping-sections).
This **Payload format** chapter describes details of the ping payload that are relevant for decoding Glean pings in the pipeline.
This is less relevant for end users of the Glean SDK.

## JSON Schema

Glean's ping payloads have a formal JSON schema defined in the [mozilla-pipeline-schemas](https://github.com/mozilla-services/mozilla-pipeline-schemas/) project.
It is written as a set of [templates](https://github.com/mozilla-services/mozilla-pipeline-schemas/tree/master/templates/include/glean) that are expanded by the mozilla-pipeline-schemas build infrastructure into a [fully expanded schema](https://github.com/mozilla-services/mozilla-pipeline-schemas/blob/master/schemas/glean/baseline/baseline.1.schema.json).

## Metric types

TODO: Fill in the rest of the metric types. https://bugzilla.mozilla.org/show_bug.cgi?id=1566854

### Timespan

A [Timespan](../../../user/metrics/timespan.md) is represented as an object of their duration as an integer and the time unit.

| Field name | Type | Description |
|---|---|---|
| `value` | Integer | The value in the marked time unit. |
| `time_unit` | String | The time unit, see the [timespan's configuration](../../../user/metrics/timespan.md#configuration) for valid values. |

#### Example

```json
{
    "time_unit": "milliseconds",
    "value": 10
}
```

## Timing Distribution

A [Timing distribution](../../../user/metrics/timing_distribution.md) is represented as an object with the following fields.

| Field name | Type | Description |
|---|---|---|
| `bucket_count` | Integer | The bucket count of the histogram. |
| `range` | Array&lt;Integer&gt; | The range indicated by its minimum and maxium value. |
| `sum` | Integer | The sum of all recorded values. |
| `time_unit` | String | The timespan's time unit, see the [time distribution's configuration](../../../user/metrics/timing_distribution.md#configuration) for valid values. |
| `values` | Map&lt;String, Integer&gt; | The values in each bucket. The key is the minimum value for the range of that bucket. Buckets with no values are not reported. |

#### Example:

```json
{
    "bucket_count": 100,
    "range": [0, 60000],
    "histogram_type": "expontential",
    "sum": 3,
    "time_unit": "milliseconds",
    "values": {
        "0": 1,
        "1": 3,
    }
}
```

### Event

[Events](../../../user/metrics/event.md) are represented as an array of objects, with one object for each event.
Each event object has the following keys:

- `timestamp`: (integer) A monotonically increasing timestamp value, in milliseconds.
  To avoid leaking absolute times, the first timestamp in the array is always zero, and subsequent timestamps in the array are relative to that reference point.

- `category`: (string) The event's category.
  This comes directly from the category under which the metric was defined in the `metrics.yaml` file.

- `name`: (string) The event's name, as defined in the `metrics.yaml` file.

- `extra`: (object, optional) Extra data associated with the event.
  Both the keys and values of this object are strings.
  The keys must be from the set defined for this event in the `metrics.yaml` file.
  The values have a maximum length of 50 bytes, when encoded as UTF-8.

For example:

```json
[
  {
    "timestamp": 0,
    "category": "app",
    "name": "ss_menu_opened"
  },
  {
    "timestamp": 124,
    "category": "search",
    "name": "performed_search",
    "extra": {
      "source": "default.action"
    }
  }
]
```

Also see [the JSON schema for events](https://github.com/mozilla-services/mozilla-pipeline-schemas/blob/master/templates/include/glean/event.1.schema.json).

To avoid losing events when the application is killed by the operating system, events are queued on disk as they are recorded.
When the application starts up again, there is no good way to determine if the device has rebooted since the last run and therefore any timestamps recorded in the new run could not be guaranteed to be consistent with those recorded in the previous run.
To get around this, on application startup, any queued events are immediately collected into pings and then cleared.
These "startup-triggered pings" are likely to have a very short duration, as recorded in `ping_info.start_time` and `ping_info.end_time` (see [the `ping_info` section](../../../user/pings/index.md#The-ping_info-section)).
The maximum timestamp of the events in these pings are quite likely to exceed the duration of the ping, but this is to be expected.
