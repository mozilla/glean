# Payload format

The main sections of a Glean ping are described in [Ping Sections](../../../user/pings/index.md#Ping-sections).
This **Payload format** chapter describes details of the ping payload that are relevant for decoding Glean pings in the pipeline.
This is less relevant for end users of the Glean SDK.

## JSON Schema

Glean's ping payloads have a formal JSON schema defined in the [mozilla-pipeline-schemas](https://github.com/mozilla-services/mozilla-pipeline-schemas/) project.
It is written as a set of [templates](https://github.com/mozilla-services/mozilla-pipeline-schemas/tree/master/templates/include/glean) that are expanded by the mozilla-pipeline-schemas build infrastructure into a [fully expanded schema](https://github.com/mozilla-services/mozilla-pipeline-schemas/blob/master/schemas/glean/baseline/baseline.1.schema.json).

## Metric types

### Boolean

A [Boolean](../../../user/metrics/boolean.md) is represented by its boolean value.

#### Example

```json
true
```


### Counter

A [Counter](../../../user/metrics/counter.md) is represented by its integer value.

#### Example

```json
17
```

### String

A [String](../../../user/metrics/string.md) is represented by its string value.

#### Example

```json
"sample string"
```

### String list

A [String List](../../../user/metrics/string_list.md) is represented as an array of strings.

```json
["sample string", "another one"]
```

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

### Timing Distribution

A [Timing distribution](../../../user/metrics/timing_distribution.md) is represented as an object with the following fields.

| Field name | Type | Description |
|---|---|---|
| `sum` | Integer | The sum of all recorded values. |
| `values` | Map&lt;String, Integer&gt; | The values in each bucket. The key is the minimum value for the range of that bucket. Buckets with no values are not reported. |

#### Example:

```json
{
    "sum": 3,
    "values": {
        "0": 1,
        "1": 3,
    }
}
```

### UUID

A [UUID](../../../user/metrics/uuid.md) is represented by the string representation of the UUID.

#### Example

```json
"29711dc8-a954-11e9-898a-eb4ea7e8fd3f"
```

### Datetime

A [Datetime](../../../user/metrics/datetime.md) is represented by its ISO8601 string representation, truncated to the metric's time unit.
It always includes the timezone offset.

#### Example

```json
"2019-07-18T14:06:00.000+02:00"
```

### Event

[Events](../../../user/metrics/event.md) are represented as an array of objects, with one object for each event.
Each event object has the following keys:

| Field name | Type | Description |
|---|---|---|
| `timestamp` | Integer | A monotonically increasing timestamp value, in milliseconds. To avoid leaking absolute times, the first timestamp in the array is always zero, and subsequent timestamps in the array are relative to that reference point. |
| `category` | String | The event's category. This comes directly from the category under which the metric was defined in the `metrics.yaml` file. |
| `name` | String | The event's name, as defined in the `metrics.yaml` file. |
| `extra` | Object (optional) | Extra data associated with the event. Both the keys and values of this object are strings. The keys must be from the set defined for this event in the `metrics.yaml` file. The values have a maximum length of 50 bytes, when encoded as UTF-8. |

#### Example

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

### Custom Distribution

A [Custom distribution](../../../user/metrics/custom_distribution.md) is represented as an object with the following fields.

| Field name | Type | Description |
|---|---|---|
| `sum` | Integer | The sum of all recorded values. |
| `values` | Map&lt;String, Integer&gt; | The values in each bucket. The key is the minimum value for the range of that bucket. All buckets [0, max) are reported, so that the histograms can be aggregated in the pipeline without the pipeline knowing anything about the distribution of the buckets. | 

#### Example:

```json
{
    "sum": 3,
    "values": {
        "0": 1,
        "1": 3,
    }
}
```

### Labeled metrics

Currently several labeled metrics are supported:

* [Labeled Counters](../../../user/metrics/labeled_counters.md).
* [Labeled Strings](../../../user/metrics/labeled_strings.md).

All are on the top-level represented in the same way, as an object mapping the label to the metric's value.
See the individual metric types for details on the value payload:

* [Counter](#counter)
* [String](#string)

#### Example for Labeled Counters

```json
{
    "label1": 2,
    "label2": 17
}
```
