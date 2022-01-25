# Initializing

The following steps are required for applications using the Glean SDK, but not libraries.

{{#include ../../../shared/blockquote-info.html}}

##### Note

> The `initialize` function _must_ be called, even if telemetry upload is disabled.
> Glean needs to perform maintenance tasks even when telemetry is disabled, and because Glean
> does this as part of its initialization, it is _required_ to always call the `initialize`
> function. Otherwise, Glean won't be able to clean up collected data, disable queuing of pre-init
> tasks, or perform other required operations.
>
> This does not apply to special builds where telemetry is disabled at build time. In that case, it is acceptable to not call `initialize` at all.

{{#include ../../../shared/blockquote-stop.html}}

##### Initialize Glean with the correct value for `uploadEnabled`!

> `Glean.initialize` must **always** be called with real values.
> Always pass the user preference, e.g. `Glean.initialize(upload=userSettings.telemetry_enabled)` or the equivalent for your application.
> Never call `Glean.initialize(upload=true)` if `true` is a placeholder value that later gets reset by `Glean.setUploadEnabled(false)`.
> Depending on the provided placeholder value, this might trigger the generation of new client ids or the submission of bogus [`deletion-request` pings](../../user/pings/deletion-request.md).

{{#include ../../../shared/tab_header.md}}

<div data-lang="Kotlin" class="tab">

{{#include ../../../shared/blockquote-info.html}}

##### Multiple processes support

> The Glean Kotlin SDK does not support use across multiple processes, and must only be initialized on the application's main process. Initializing in other processes is a no-op.
> Additionally, Glean must be initialized on the main (UI) thread of the applications main process. Failure to do so will throw an `IllegalThreadStateException`.

An excellent place to initialize Glean is within the `onCreate` method of the class that extends Android's `Application` class.

```Kotlin
import org.mozilla.yourApplication.GleanMetrics.GleanBuildInfo
import org.mozilla.yourApplication.GleanMetrics.Pings

class SampleApplication : Application() {

    override fun onCreate() {
        super.onCreate()

        // If you have custom pings in your application, you must register them
        // using the following command. This command should be omitted for
        // applications not using custom pings.
        Glean.registerPings(Pings)

        // Initialize the Glean library.
        Glean.initialize(
            applicationContext,
            // Here, `settings()` is a method to get user preferences, specific to
            // your application and not part of the Glean API.
            uploadEnabled = settings().isTelemetryEnabled,
            buildInfo = GleanBuildInfo.buildInfo
        )
    }
}
```

The Glean Kotlin SDK should be initialized as soon as possible, and importantly, before any other libraries in the application start using Glean.
Library code should never call `Glean.initialize`, since it should be called exactly once per application.

{{#include ../../../shared/blockquote-warning.html}}

##### Uploads when using Android Components

> When the Glean Kotlin SDK is consumed through Android Components, it is required to configure an HTTP client to be used for upload.
> For example:
>
> ```Kotlin
> // Requires `org.mozilla.components:concept-fetch`
> import mozilla.components.concept.fetch.Client
> // Requires `org.mozilla.components:lib-fetch-httpurlconnection`.
> // This can be replaced by other implementations, e.g. `lib-fetch-okhttp`
> // or an implementation from `browser-engine-gecko`.
> import mozilla.components.lib.fetch.httpurlconnection.HttpURLConnectionClient
>
> import mozilla.components.service.glean.config.Configuration
> import mozilla.components.service.glean.net.ConceptFetchHttpUploader
>
> val httpClient = ConceptFetchHttpUploader(lazy { HttpURLConnectionClient() as Client })
> val config = Configuration(httpClient = httpClient)
> Glean.initialize(
>    context,
>    uploadEnabled = true,
>    configuration = config,
>    buildInfo = GleanBuildInfo.buildInfo
>)
>```

</div>

<div data-lang="Swift" class="tab">

{{#include ../../../shared/blockquote-info.html}}

##### Multiple processes support

> The Glean Swift SDK does not support use across multiple processes, and must only be initialized on the application's main process.

An excellent place to initialize Glean is within the `application(_:)` method of the class that extends the `UIApplicationDelegate` class.

```Swift
import Glean
import UIKit

@UIApplicationMain
class AppDelegate: UIResponder, UIApplicationDelegate {
    func application(_: UIApplication, didFinishLaunchingWithOptions _: [UIApplication.LaunchOptionsKey: Any]?) -> Bool {
        // If you have custom pings in your application, you must register them
        // using the following command. This command should be omitted for
        // applications not using custom pings.
        Glean.shared.registerPings(GleanMetrics.Pings)

        // Initialize the Glean library.
        Glean.shared.initialize(
            // Here, `Settings` is a method to get user preferences specific to
            // your application, and not part of the Glean API.
            uploadEnabled = Settings.isTelemetryEnabled,
            buildInfo = GleanMetrics.GleanBuild.info
        )
    }
}
```

The Glean Swift SDK should be initialized as soon as possible, and importantly, before any other libraries in the application start using Glean.
Library code should never call `Glean.shared.initialize`, since it should be called exactly once per application.

</div>

<div data-lang="Python" class="tab">

The main control for the Glean Python SDK is on the `glean.Glean` singleton.

The Glean Python SDK should be initialized as soon as possible, and importantly, before any other libraries in the application start using Glean.
Library code should never call `Glean.initialize`, since it should be called exactly once per application.


```python
from glean import Glean

Glean.initialize(
    application_id="my-app-id",
    application_version="0.1.0",
    upload_enabled=True,
)
```

Additional configuration is available on the `glean.Configuration` object, which can be passed into `Glean.initialize()`.

Unlike Android and Swift, the Python SDK does not automatically send any pings.
See the [custom pings documentation](../../user/pings/custom.md) about adding custom pings and sending them.

</div>

<div data-lang="Rust" class="tab">

{{#include ../../../shared/blockquote-info.html}}

##### Multiple processes support

> The Glean Rust SDK does not support use across multiple processes, and must only be initialized on the application's main process.

The Glean Rust SDK should be initialized as soon as possible,
and importantly, before any other libraries in the application start using Glean.
Library code should never call Glean.initialize,
since it should be called exactly once per application.

```Rust
use glean::{ClientInfoMetrics, Configuration};
let cfg = Configuration {
    data_path,
    application_id: "my-app-id".into(),
    upload_enabled: true,
    max_events: None,
    delay_ping_lifetime_io: false,
    channel: None,
    server_endpoint: Some("https://incoming.telemetry.mozilla.org".into()),
    uploader: None,
    use_core_mps: true,
};

let client_info = ClientInfoMetrics {
    app_build: env!("CARGO_PKG_VERSION").to_string(),
    app_display_version: env!("CARGO_PKG_VERSION").to_string(),
};

glean::initialize(cfg, client_info);
```

Unlike in other implementations, the Rust SDK does not provide a default uploader.
See [`PingUploader`](../../../docs/glean/net/trait.PingUploader.html) for details.

</div>

<div data-lang="JavaScript" class="tab">

The main control for Glean is on the `Glean` singleton.

The Glean JavaScript SDK should be initialized as soon as possible when the product using it is started.

```js
// Glean.js for webextensions: the following code
// runs in the background script.
import Glean from "@mozilla/glean/webext";

Glean.initialize(
    "my-app-id",
    true, // uploadEnabled
    {
      appDisplayVersion: "0.1.0"
    }
);
```
</div>

<div data-lang="Firefox Desktop" class="tab" data-info="On Firefox Desktop Glean initialization is handled internally."></div>

{{#include ../../../shared/tab_footer.md}}

## What happens after Glean is initialized?

Once initialized, if upload is enabled, Glean will automatically start collecting [baseline metrics](../../user/pings/metrics.md).
If upload is disabled, any persisted metrics, events and pings (other than `first_run_date` and `first_run_hour`) are cleared, and subsequent calls to record metrics will be no-ops.

## Release channels

If the application has the concept of release channels and knows which channel it is on at run-time,
then it can provide the Glean SDKs with this information by setting it as part of the `Configuration` object parameter of the `initialize` method.

## Behavior when uninitialized

Metric recording that happens before Glean is initialized is queued and applied at initialization.
To avoid unbounded memory growth the queue is bounded (currently to a maximum of 100 tasks), and further recordings are dropped.
The number of recordings dropped, if any, is recorded in the `glean.error.preinit_tasks_overflow` metric.

Custom ping submission will not fail before initialization.
Collection and upload of the custom ping is delayed until Glean is initialized.
Built-in pings are only available after initialization.
