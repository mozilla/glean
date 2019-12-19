# Custom pings

Applications can define metrics that are sent in custom pings. Unlike the built-in pings, custom pings are sent explicitly by the application.

## Defining a custom ping

Custom pings must be defined in a `pings.yaml` file, which is in the same directory alongside your app's `metrics.yaml` file.

Each ping has the following parameters:

- `include_client_id` (required): A boolean indicating whether to include the
  `client_id` in the [`client_info` section](index.md#The-client_info-section)).
- `send_if_empty` (optional, default: false): A boolean indicating if the ping is sent if it contains no metric data.

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

> Note: the names `baseline`, `metrics`, `events` and `all-pings` are reserved and may not be used as the name of a custom ping.

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

<div data-lang="Swift" class="tab">

In Swift, this object must be registered with Glean from your startup code
(such as in your application's `application` method or a function called from that method).

```swift
import Glean

@UIApplicationMain
class AppDelegate: UIResponder, UIApplicationDelegate {
func application(_: UIApplication, didFinishLaunchingWithOptions _: [UIApplication.LaunchOptionsKey: Any]?) -> Bool {
    // ...
    Glean.shared.registerPings(GleanMetrics.Pings)
    // ...
}
}
```

</div>

<div data-lang="Python" class="tab">

For Python, the `pings.yaml` file must be available and loaded at runtime.

If your project is a script (i.e. just Python files in a directory), you can load the `pings.yaml` using:

```
from glean import load_pings

pings = load_pings("pings.yaml")
```

If your project is a distributable Python package, you need to include the `metrics.yaml` file using [one of the myriad ways to include data in a Python package](https://setuptools.readthedocs.io/en/latest/setuptools.html#including-data-files) and then use [`package_resources.resource_filename()`](https://setuptools.readthedocs.io/en/latest/pkg_resources.html#resource-extraction) to get the filename at runtime.

```Python
from glean import load_pings
from package_resources import resource_filename

pings = load_pings(resource_filename(__name__, "pings.yaml"))
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

## Submitting a custom ping

To collect and queue a custom ping for eventual uploading, call the `submit` method on the `PingType` object that the Glean SDK generated for your ping.

For example, to submit the custom ping defined above:

{{#include ../../tab_header.md}}

<div data-lang="Kotlin" class="tab">

```kotlin
import org.mozilla.yourApplication.GleanMetrics.Pings
Pings.search.submit()
```

</div>

<div data-lang="Swift" class="tab">

```swift
import Glean

GleanMetrics.Pings.shared.search.submit()
```

</div>

<div data-lang="Python" class="tab">

```Python
from glean import load_pings

pings = load_pings("pings.yaml")

pings.search.submit()
```

</div>

{{#include ../../tab_footer.md}}

If none of the metrics for the ping contain data the ping is not sent (unless `send_if_empty` is set to true in the definition file)
