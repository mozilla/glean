# Payload format

The main sections of a Glean ping are described in [Ping Sections](../../../book/user/pings/index.md#payload-structure).
This **Payload format** chapter describes details of the ping payload that are relevant for decoding Glean pings in the pipeline.

> NOTE: The payload format is an implementation detail of the Glean SDK and subject to change at any time.
> External users should not rely on this information.
> It is provided as a reference for contributing to Glean only.

## JSON Schema

Glean's ping payloads have a formal JSON schema defined in the [mozilla-pipeline-schemas](https://github.com/mozilla-services/mozilla-pipeline-schemas/) project.
It is written as a set of [templates](https://github.com/mozilla-services/mozilla-pipeline-schemas/tree/HEAD/templates/include/glean) that are expanded by the mozilla-pipeline-schemas build infrastructure into a [fully expanded schema](https://github.com/mozilla-services/mozilla-pipeline-schemas/blob/HEAD/schemas/glean/baseline/baseline.1.schema.json).

## Metric types

### Boolean

A [Boolean](../../../book/reference/metrics/boolean.md) is represented by its boolean value.

#### Example

```json
true
```


### Counter

A [Counter](../../../book/reference/metrics/counter.md) is represented by its integer value.

#### Example

```json
17
```

### Quantity

A [Quantity](../../../book/reference/metrics/quantity.md) is represented by its integer value.

#### Example

```json
42
```

### String

A [String](../../../book/reference/metrics/string.md) is represented by its string value.

#### Example

```json
"sample string"
```

### String list

A [String List](../../../book/reference/metrics/string_list.md) is represented as an array of strings.

#### Example

```json
["sample string", "another one"]
```

An empty list is accepted and sent as:

```json
[]
```

### Timespan

A [Timespan](../../../book/reference/metrics/timespan.md) is represented as an object of their duration as an integer and the time unit.

| Field name | Type | Description |
|---|---|---|
| `value` | Integer | The value in the marked time unit. |
| `time_unit` | String | The [time unit](../../../book/reference/metrics/timespan.md#time_unit). |

#### Example

```json
{
    "time_unit": "milliseconds",
    "value": 10
}
```

### Timing Distribution

A [Timing distribution](../../../book/reference/metrics/timing_distribution.md) is represented as an object with the following fields.

| Field name | Type | Description |
|---|---|---|
| `sum` | Integer | The sum of all recorded values. |
| `values` | Map&lt;String, Integer&gt; | The values in each bucket. The key is the minimum value for the range of that bucket. |

A contiguous range of buckets is always sent, so that the server can aggregate and visualize distributions, without knowing anything about the specific bucketing function used.
This range starts with the first bucket with a non-zero accumulation, and ends at one bucket beyond the last bucket with a non-zero accumulation (so that the upper bound on the last bucket is retained).

For example, the following shows the recorded values vs. what is sent in the payload.

```
recorded:  1024: 2, 1116: 1,                   1448: 1,
sent:      1024: 2, 1116: 1, 1217: 0, 1327: 0, 1448: 1, 1579: 0
```

#### Example:

```json
{
    "sum": 4612,
    "values": {
        "1024": 2,
        "1116": 1,
        "1217": 0,
        "1327": 0,
        "1448": 1,
        "1579": 0
    }
}
```

### Memory Distribution

A [Memory distribution](../../../book/reference/metrics/memory_distribution.md) is represented as an object with the following fields.

| Field name | Type | Description |
|---|---|---|
| `sum` | Integer | The sum of all recorded values. |
| `values` | Map&lt;String, Integer&gt; | The values in each bucket. The key is the minimum value for the range of that bucket. |

A contiguous range of buckets is always sent.
See [timing distribution](#timing-distribution) for more details.

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

A [UUID](../../../book/reference/metrics/uuid.md) is represented by the string representation of the UUID.

#### Example

```json
"29711dc8-a954-11e9-898a-eb4ea7e8fd3f"
```

### URL

A [URL](../../../book/reference/metrics/url.md) is represented by the encoded string representation of the URL.

#### Example

```json
"https://mysearchengine.com/?query=%25s"
```

### Datetime

A [Datetime](../../../book/reference/metrics/datetime.md) is represented by its ISO8601 string representation, truncated to the metric's time unit.
It always includes the timezone offset.

#### Example

```json
"2019-07-18T14:06:00.000+02:00"
```

### Event

[Events](../../../book/reference/metrics/event.md) are represented as an array of objects, with one object for each event.
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

Also see [the JSON schema for events](https://github.com/mozilla-services/mozilla-pipeline-schemas/blob/HEAD/templates/include/glean/event.1.schema.json).

To avoid losing events when the application is killed by the operating system, events are queued on disk as they are recorded.
When the application starts up again, there is no good way to determine if the device has rebooted since the last run and therefore any timestamps recorded in the new run could not be guaranteed to be consistent with those recorded in the previous run.
To get around this, on application startup, any queued events are immediately collected into pings and then cleared.
These "startup-triggered pings" are likely to have a very short duration, as recorded in `ping_info.start_time` and `ping_info.end_time` (see [the `ping_info` section](../../../book/user/pings/index.md#the-ping_info-section)).
The maximum timestamp of the events in these pings are quite likely to exceed the duration of the ping, but this is to be expected.

### Custom Distribution

A [Custom distribution](../../../book/reference/metrics/custom_distribution.md) is represented as an object with the following fields.

| Field name | Type | Description |
|---|---|---|
| `sum` | Integer | The sum of all recorded values. |
| `values` | Map&lt;String, Integer&gt; | The values in each bucket. The key is the minimum value for the range of that bucket. |

From Glean v66.0.0 on the `values` only includes filled buckets.
Empty buckets are not sent.

For example, suppose you had a custom distribution defined by the following parameters:

  - `range_min`: 10
  - `range_max`: 200
  - `bucket_count`: 80
  - `histogram_type`: `'linear'`

The following shows the recorded values vs. what is sent in the payload.

```
recorded: 12: 2, 22: 1
sent:     12: 2, 22: 1
```

Up until Glean v65.0.3 a contiguous range of buckets was always sent, so that the server can aggregate and visualize distributions, without knowing anything about the specific bucketing function used.
This range started with the first bucket (as specified in the `range_min` parameter), and ends at one bucket beyond the last bucket with a non-zero accumulation (so that the upper bound on the last bucket is retained).

#### Example:

```json
{
    "sum": 3,
    "values": {
        "12": 2,
        "22": 1,
    }
}
```

### Labeled metrics

Currently several labeled metrics are supported:

* [Labeled Counters](../../../book/reference/metrics/labeled_counters.md).
* [Labeled Strings](../../../book/reference/metrics/labeled_strings.md).

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

### Dual Labeled metrics

Currently only counters are supported:

* [Dual Labeled Counters](../../../book/reference/metrics/dual_labeled_counters.md).

All are on the nested represented in the same way, with "key" as the top level key and "category" as an
object mapping the category to the metric's value.

See the [Counter](#counter) metric type for details on the value payload.

#### Example for Dual Labeled Counters

```json
{
    "key1": {
        "category1": 2,
        "category2": 17
    },
    "key2": {
        "category1": 2,
        "category2": 17
    }
}
```

### Rate

A [Rate](../../../book/reference/metrics/rate.md) is represented by its `numerator` and `denominator`.

#### Example

```json
{
    "numerator": 22,
    "denominator": 7,
}
```

### Text

A [Text](../../../book/reference/metrics/text.md) is represented by its string value.

#### Example

```json
"sample string"
```

### Object

An [Object](../../../book/reference/metrics/object.md) is represented as either a JSON array or JSON object,
depending on the allowed structure.
Missing values (`null`) and empty arrays will not be serialized in the payload.

#### Example

Object:

```json
{
  "colour": "red",
  "diameter": 5
}
```

Array with objects:

```json
[
  {
    "type": "ERROR",
    "address": "0x000000"
  },
  {
    "type": "ERROR"
  }
]
```
