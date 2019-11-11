# Custom pings

Applications can define metrics that are sent in custom pings. Unlike the built-in pings, custom pings are sent explicitly by the application.

## Defining a custom ping

Custom pings must be defined in a `pings.yaml` file, which is in the same directory alongside your app's `metrics.yaml` file.

Each ping has the following parameters:

- `include_client_id` (required): A boolean indicating whether to include the
  `client_id` in the [`client_info` section](index.md#The-client_info-section)).

In addition to these parameters, pings also support the parameters related to data review and expiration defined in [common metric parameters](../adding-new-metrics.md#common-metric-parameters): `description`, `notification_emails`, `bugs`, and `data_reviews`.

For example, to define a custom ping called `search` specifically for search information:

```YAML
# Required to indicate this is a `pings.yaml` file
$schema: moz://mozilla.org/schemas/glean/pings/1-0-0

search:
  description: >
    A ping to record search data.
  include_client_id: false
  notification_emails:
    - CHANGE-ME@example.com
  bugs:
    - 123456789
  data_reviews:
    - http://example.com/path/to/data-review
```

> Note: the names `baseline`, `metrics`, `events` and `all_pings` are reserved and may not be used as the name of a custom ping.

## Loading custom ping metadata into your application or library

The Glean SDK build generates code from `pings.yaml` in a `Pings` object, which must be instantiated so Glean can send pings by name.

{{#include ../../tab_header.md}}

<div data-lang="Kotlin" class="tab">

In Kotlin, this object must be registered with Glean from your startup code (such as in your application's `onCreate` method or a function called from that method).

```Kotlin
import org.mozilla.yourApplication.GleanMetrics.Pings

...

override fun onCreate() {
    ...
    Glean.registerPings(Pings)
    ...
}
```

</div>

<div data-lang="Python" class="tab">

```
from glean import load_pings

pings = load_pings("pings.yaml")
```

</div>

{{#include ../../tab_footer.md}}

## Sending metrics in a custom ping

To send a metric on a custom ping, you add the custom ping's name to the `send_in_pings` parameter in the `metrics.yaml` file.

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

## Sending a custom ping

To send a custom ping, call the `send` method on the `PingType` object that the Glean SDK generated for your ping.

For example, to send the custom ping defined above:

{{#include ../../tab_header.md}}

<div data-lang="Kotlin" class="tab">

```kotlin
import org.mozilla.yourApplication.GleanMetrics.Pings
Pings.search.send()
```

</div>

<div data-lang="Python" class="tab">

```Python
pings.search.send()
```

</div>

{{#include ../../tab_footer.md}}
