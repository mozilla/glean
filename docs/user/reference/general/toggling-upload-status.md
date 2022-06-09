# Toggling upload status

The Glean SDKs provide an API for toggling Glean's upload status after initialization.

Applications instrumented with Glean
[are expected to](../../user/adding-glean-to-your-project/index.md#glean-integration-checklist)
provide some form of user interface to allow for toggling the upload status.

## Disabling upload

When upload is disabled, the Glean SDK will perform the following tasks:

1. Submit a [`deletion-request`](../../user/pings/deletion-request.md) ping.
2. Cancel scheduled ping uploads.
3. Clear metrics and pings data from the client, except for the
  [`first_run_date` and `first_run_hour`](../../user/pings/index.html#the-client_info-section) metrics.

While upload is disabled, metrics aren't recorded and no data is uploaded.

## Enabling upload

When upload is enabled, the Glean SDK will re-initialize its [core metrics](../../user/collected-metrics/metrics.md).
The only core metrics that are not re-initialized are the [`first_run_date` and `first_run_hour`](../../user/pings/index.html#the-client_info-section) metrics.

While upload is enabled all metrics are recorded as expected
and pings are sent to the telemetry servers.

## API

### `Glean.setUploadEnabled(boolean)`

Enables or disables upload.

If called prior to initialize this function is a no-op.

If the upload state is not actually changed in between calls to this function, it is also a no-op.

{{#include ../../../shared/tab_header.md}}

<div data-lang="Kotlin" class="tab">

```Kotlin
import mozilla.telemetry.glean.Glean

open class MainActivity : AppCompatActivity() {
    override fun onCreate() {
        // ...

        uploadSwitch.setOnCheckedChangeListener { _, isChecked ->
            if (isChecked) {
                Glean.setUploadEnabled(true)
            } else {
                Glean.setUploadEnabled(false)
            }
        }
    }
}
```

</div>
<div data-lang="Java" class="tab">

```Java
import mozilla.telemetry.glean.Glean

Glean.INSTANCE.setUploadEnabled(false);
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
        Glean.shared.setUploadEnabled(enableSwitch.isOn)
    }
}
```


</div>
<div data-lang="Python" class="tab">

```python
from glean import Glean

Glean.set_upload_enabled(false)
```

</div>
<div data-lang="Rust" class="tab">

```Rust
use glean;

glean::set_upload_enabled(false);
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

</div>
<div data-lang="Firefox Desktop" class="tab" data-info="On Firefox Desktop data collection is toggled internally."></div>

{{#include ../../../shared/tab_footer.md}}

## Reference

* [Swift API docs](../../../swift/Classes/Glean.html#/s:5GleanAAC16setUploadEnabledyySbF)
* [Python API docs](../../../python/glean/index.html#glean.Glean.set_upload_enabled)
* [Rust API docs](../../../docs/glean/fn.set_upload_enabled.html)
* [JavaScript API docs](https://mozilla.github.io/glean.js/classes/core_glean.default.html#setUploadEnabled)
