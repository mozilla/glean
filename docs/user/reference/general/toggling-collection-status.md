# Toggling collection status

The Glean SDKs provide an API for toggling Glean's collection status after initialization.

Applications instrumented with Glean
[are expected to](../../user/adding-glean-to-your-project/index.md#glean-integration-checklist)
provide some form of user interface to allow for toggling the collection status.

{{#include ../../../shared/blockquote-info.html}}

## `setUploadEnabled` is deprecated since Glean v63.0.0

> Prior to Glean v63.0.0 this API was called `setUploadEnabled`.
> `setUploadEnabled` is now deprecated and replaced by `setCollectionEnabled`.
> It behaves the same way with respect to built-in pings and custom pings,
> unless those are marked with `follows_collection_enabled: false`.
> See [TODO: the collection-enabled documentation for details]().

## Disabling collection

When collection is disabled, the Glean SDK will perform the following tasks:

1. Submit a [`deletion-request`](../../user/pings/deletion-request.md) ping.
2. Cancel scheduled ping uploads.
3. Clear metrics and pings data from the client, except for the
  [`first_run_date`](../../user/pings/index.html#the-client_info-section) metric.

While collection is disabled, metrics aren't recorded and no data is uploaded.

## Enabling collection

When collection is enabled, the Glean SDK will re-initialize its [core metrics](../../user/collected-metrics/metrics.md).
The only core metric that is not re-initialized is the [`first_run_date`](../../user/pings/index.html#the-client_info-section) metric.

While collection is enabled all metrics are recorded as expected
and pings are sent to the telemetry servers.

## API

### `Glean.setCollectionEnabled(boolean)`

Enables or disables collection.

If called prior to initialize this function is a no-op.

If the collection state is not actually changed in between calls to this function, it is also a no-op.

{{#include ../../../shared/tab_header.md}}

<div data-lang="Kotlin" class="tab">

```Kotlin
import mozilla.telemetry.glean.Glean

open class MainActivity : AppCompatActivity() {
    override fun onCreate() {
        // ...

        uploadSwitch.setOnCheckedChangeListener { _, isChecked ->
            if (isChecked) {
                Glean.setCollectionEnabled(true)
            } else {
                Glean.setCollectionEnabled(false)
            }
        }
    }
}
```

</div>
<div data-lang="Java" class="tab">

```Java
import mozilla.telemetry.glean.Glean

Glean.INSTANCE.setCollectionEnabled(false);
```

</div>
<div data-lang="Swift" class="tab">


```Swift
import Glean
import UIKit

class ViewController: UIViewController {
    @IBOutlet var enableSwitch: UISwitch!

    // ...

    @IBAction func enableToggled(_: Any) {
        Glean.shared.setCollectionEnabled(enableSwitch.isOn)
    }
}
```


</div>
<div data-lang="Python" class="tab">

```python
from glean import Glean

Glean.set_collection_enabled(false)
```

</div>
<div data-lang="Rust" class="tab">

```Rust
use glean;

glean::set_collection_enabled(false);
```

</div>
<div data-lang="JavaScript" class="tab">

```js
import Glean from "@mozilla/glean/web";

const uploadSwitch = document.querySelector("input[type=checkbox].upload-switch");
uploadSwitch.addEventListener("change", event => {
    if (event.target.checked) {
        Glean.setUploadEnabled(true);
    } else {
        Glean.setUploadEnabled(false);
    }
});
```

{{#include ../../../shared/blockquote-info.html}}

## Glean.js still uses `setUploadEnabled`

> Glean.js did not yet switch to the new naming and continues to use `setUploadEnabled` unchanged.
> See [Bug 1956280](https://bugzilla.mozilla.org/show_bug.cgi?id=1956280) for more information.

</div>
<div data-lang="Firefox Desktop" class="tab" data-info="On Firefox Desktop data collection is toggled internally."></div>

{{#include ../../../shared/tab_footer.md}}

## Reference

* [Swift API docs](../../../swift/Classes/Glean.html#/s:5GleanAAC16setCollectionEnabledyySbF)
* [Python API docs](../../../python/glean/index.html#glean.Glean.set_collection_enabled)
* [Rust API docs](../../../docs/glean/fn.set_collection_enabled.html)
* [JavaScript API docs](https://mozilla.github.io/glean.js/reference/uploaders/#uploadenabled)
