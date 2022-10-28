# Custom pings

Applications can define metrics that are sent in custom pings. Unlike the built-in pings, custom pings are sent explicitly by the application.

This is useful when the scheduling of the built-in pings ([metrics](metrics.html), [baseline](baseline.html) and [events](events.html)) are not appropriate for your data.  Since the timing of the submission of custom pings is handled by the application, the measurement window is under the application's control.

This is especially useful when metrics need to be tightly related to one another, for example when you need to measure the distribution of frame paint times when a particular rendering backend is in use.  If these metrics were in different pings, with different measurement windows, it is much harder to do that kind of reasoning with much certainty.

## Defining a custom ping

Custom pings must be defined in a [`pings.yaml` file](../../reference/yaml/pings.md), placed in the same directory alongside your app's `metrics.yaml` file.

For example, to define a custom ping called `search` specifically for search information:

```YAML
$schema: moz://mozilla.org/schemas/glean/pings/2-0-0

search:
  description: >
    A ping to record search data.
  metadata:
    tags:
      - Search
  include_client_id: false
  notification_emails:
    - CHANGE-ME@example.com
  bugs:
    - http://bugzilla.mozilla.org/123456789/
  data_reviews:
    - http://example.com/path/to/data-review
```

Tags are an optional feature you can use to provide an additional layer of categorization to pings.
Any tags specified in the `metadata` section of a ping *must* have a corresponding entry in a [tags YAML registry](../../reference/yaml/tags.md) for your project.

Refer to the [pings YAML registry format](../../reference/yaml/pings.md) for a full reference
on the `pings.yaml` file structure.

## Sending metrics in a custom ping

To send a metric on a custom ping, you add the custom ping's name to the `send_in_pings` parameter in the `metrics.yaml` file.

{{#include ../../../shared/blockquote-warning.html}}

### Ping metadata must be loaded before sending!

> After defining a custom ping, before it can be used for sending data, its metadata must be [loaded into your application or library](../../reference/general/register-custom-pings.md).

For example, to define a new metric to record the default search engine, which is sent in a custom ping called `search`, put `search` in the `send_in_pings` parameter.  Note that it is an error to specify a ping in `send_in_pings` that does not also have an entry in `pings.yaml`.

```YAML
search.default:
  name:
    type: string
    description: >
      The name of the default search engine.
    send_in_pings:
      - search
```

If this metric should also be sent in the default ping for the given metric type, you can add the special value `default` to `send_in_pings`:

```YAML
    send_in_pings:
      - search
      - default
```

## The `glean.restarted` event

For custom pings that contain event metrics, the `glean.restarted` event is injected by Glean
on every application restart that may happen during the pings measurement window.

**Note**: All leading and trailing `glean.restarted` events are omitted from each ping.

### Event timestamps throughout application restarts

Event timestamps are always calculated relative to the first event in a ping. The first event
will always have timestamp `0` and subsequent events will have timestamps corresponding to the
elapsed amount of milliseconds since that first event.

That is also the case for events recorded throughout restarts.

#### Example

In the below example payload, there were two events recorded on the first application run.
The first event is timestamp `0` and the second event happens one second after the first one,
so it has timestamp `1000`.

The application is restarted one hour after the first event and a `glean.restarted` event is
recorded, timestamp `3600000`. Finally, an event is recorded during the second application run
two seconds after restart, timestamp `3602000`.

```json
{
  ...
  "events": [
    {
      "timestamp": 0,
      "category": "examples",
      "name": "event_example",
    },
    {
      "timestamp": 1000,
      "category": "examples",
      "name": "event_example"
    },
    {
      "timestamp": 3600000,
      "category": "glean",
      "name": "restarted"
    },
    {
      "timestamp": 3602000,
      "category": "examples",
      "name": "event_example"
    },
  ]
}
```

#### Caveat: Handling decreasing time offsets

For events recorded in a single application run, Glean relies on a monotonically increasing timer
to calculate event timestamps, while for calculating the time elapsed between application runs Glean
has to rely on the computer clock, which is not necessarily monotonically increasing.

In the case that timestamps in between application runs are not monotonically increasing, Glean
will take the value of the previous timestamp and add one millisecond, thus guaranteeing that
timestamps are always increasing.

{{#include ../../../shared/blockquote-info.html}}

##### Checking for decreasing time offsets between restarts

> When this edge case is hit, Glean records an [`InvalidValue` error](../error-reporting.md)
> for the `glean.restarted` metric. This metric may be consulted at analysis time.
> It is sent in the same ping where the error happened.

In the below example payload, the first and second application runs go exactly like in the
[example above](#example).

The only difference is that when the restart happens, the offset between the absolute time
of the first event and the absolute time of the restart is not enough to keep the timestamps increasing.
That may happen for many reasons, such as a change in timezones or simply a manual change in the clock
by the user.

In this case, Glean will ignore the incorrect timestamp and add one millisecond to the last timestamp
of the previous run, in order to keep the monotonically increasing nature of the timestamps.

```json
{
  ...
  "events": [
    {
      "timestamp": 0,
      "category": "examples",
      "name": "event_example",
    },
    {
      "timestamp": 1000,
      "category": "examples",
      "name": "event_example"
    },
    {
      "timestamp": 1001,
      "category": "glean",
      "name": "restarted"
    },
    {
      "timestamp": 3001,
      "category": "examples",
      "name": "event_example"
    },
  ]
}
```
