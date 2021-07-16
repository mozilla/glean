# Custom pings

Applications can define metrics that are sent in custom pings. Unlike the built-in pings, custom pings are sent explicitly by the application.

This is useful when the scheduling of the built-in pings ([metrics](metrics.html), [baseline](baseline.html) and [events](events.html)) are not appropriate for your data.  Since the timing of the submission of custom pings is handled by the application, the measurement window is under the application's control.

This is especially useful when metrics need to be tightly related to one another, for example when you need to measure the distribution of frame paint times when a particular rendering backend is in use.  If these metrics were in different pings, with different measurement windows, it is much harder to do that kind of reasoning with much certainty.

## Defining a custom ping

Custom pings must be defined in a [`pings.yaml` file](https://mozilla.github.io/glean_parser/pings-yaml.html), placed in the same directory alongside your app's `metrics.yaml` file.

For example, to define a custom ping called `search` specifically for search information:

```YAML
$schema: moz://mozilla.org/schemas/glean/pings/2-0-0

search:
  description: >
    A ping to record search data.
  include_client_id: false
  notification_emails:
    - CHANGE-ME@example.com
  bugs:
    - http://bugzilla.mozilla.org/123456789/
  data_reviews:
    - http://example.com/path/to/data-review
```

Refer to the [pings YAML registry format](../../reference/yaml/pings.md) for a full reference
on the `pings.yaml` file structure.

## Sending metrics in a custom ping

To send a metric on a custom ping, you add the custom ping's name to the `send_in_pings` parameter in the `metrics.yaml` file.

{{#include ../../../shared/blockquote-warning.html}}

##### Ping metadata must be loaded before sending!

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
