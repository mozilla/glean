# Payload format

The main sections of a Glean ping are described in [Ping Sections](../../user/pings/index.md#Ping-sections).
This **Payload format** chapter describes details of the ping payload that are less relevant to end users of Glean.

## JSON Schema

Glean's ping payloads have a formal JSON schema defined in the [mozilla-pipeline-schemas](https://github.com/mozilla-services/mozilla-pipeline-schemas/) project.
It is written as a set of [templates](https://github.com/mozilla-services/mozilla-pipeline-schemas/tree/master/templates/include/glean) that are expanded by the mozilla-pipeline-schemas build infrastructure into a [fully expanded schema](https://github.com/mozilla-services/mozilla-pipeline-schemas/blob/master/schemas/glean/baseline/baseline.1.schema.json).

## Metric types

TODO: Fill in the rest of the metric types.

### Event

Events are represented as an array of objects, with one object for each event.
Each event object has the following keys:

- `timestamp`: (integer) A monotonically increasing timestamp value, in milliseconds.  
  The first timestamp in the array is always zero, and subsequent timestamps in the array are relative to that reference point.
  
- `category`: (string) The event's category.
  This comes directly from the category under which the metric was defined in the `metrics.yaml` file.
  
- `name`: (string) The event's name, as defined in the `metrics.yaml` file.

- `extra`: (object, optional) Extra data associated with the event.
  Both the keys and values of this object are strings.
  The keys must be from the set defined for this event in the `metrics.yaml` file.
  The values have a maximum length of 50 bytes, when encoded as UTF-8.
  
Also see [the JSON schema for events](https://github.com/mozilla-services/mozilla-pipeline-schemas/blob/master/templates/include/glean/event.1.schema.json).

To avoid losing events when the application is killed by the operating system, events are queued on disk as they are recorded.
When the application starts up again, there is no good way to determine if the device has rebooted since the last run and therefore any timestamps recorded in the new run could not be guaranteed to be consistent with those recorded in the previous run.
To get around this, on application startup, any queued events are immediately collected into pings and then cleared.
These "startup-triggered pings" are likely to have a very short duration, as recorded in `ping_info.start_time` and `ping_info.end_time` (see [the `ping_info` section](../../user/pings/index.md#The-ping_info-section)).
The maximum timestamp of the events in these pings are quite likely to exceed the duration of the ping, but this is to be expected.
