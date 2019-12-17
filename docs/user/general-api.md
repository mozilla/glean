# The General API

Glean has a minimal API available on its top-level `Glean` object.
This API allows to enable and disable upload, register [custom pings][] and set [experiment data][experiments-api].

[custom-pings]: pings/custom.md
[experiments-api]: experiments-api.md

The following steps are required for applications using the Glean SDK, but not libraries.

> **Important:** The Glean SDK should only be initialized from the main application, not individual libraries.

If you are adding Glean support to a library, you can safely skip this section.

## The API

The Glean SDK provides a general API that supports the following operations. See below for language-specific details.

| Operation | Description | Notes |
| --------- | ----------- | ----- |
| `initialize` | Configure and initialize the Glean SDK. | [Initializing the Glean SDK](#initializing-the-glean-sdk) |
| `setUploadEnabled` | Enable or disable Glean collection and upload. | [Enabling and disabling Metrics](#enabling-and-disabling-metrics) |
| `getUploadEnabled` | Get whether or not Glean is allowed to record and upload data. | |
| `registerPings` | Register custom pings generated from `pings.yaml`. | [Custom pings][custom pings] |
| `setExperimentActive` | Indicate that an experiment is running. | [Using the Experiments API][experiments-api] |
| `setExperimentInactive` | Indicate that an experiment is no longer running.. | [Using the Experiments API][experiments-api] |

## Initializing the Glean SDK

{{#include ../tab_header.md}}

<div data-lang="Kotlin" class="tab">

> **Note**: The Glean SDK does not support use across multiple processes, and must only be initialized on the application's main process. Initializing in other processes is a no-op.
> Additionally, Glean must be initialized on the main (UI) thread of the applications main process. Failure to do so will throw an `IllegalThreadStateException`.

An excellent place to initialize Glean is within the `onCreate` method of the class that extends Android's `Application` class.

```Kotlin
import org.mozilla.yourApplication.GleanMetrics.Pings

class SampleApplication : Application() {

    override fun onCreate() {
        super.onCreate()

        // If you have custom pings in your application, you must register them
        // using the following command. This command should be omitted for
        // applications not using custom pings.
        Glean.registerPings(Pings)

        // Call setUploadEnabled first, since Glean.initialize
        // might send pings if there are any metrics queued up
        // from a previous run.
        Glean.setUploadEnabled(Settings.isTelemetryEnabled)

        // Initialize the Glean library.
        Glean.initialize(applicationContext)
    }
}
```

Once initialized, if collection is enabled, the Glean SDK will automatically start collecting [baseline metrics](pings/metrics.md) and sending its [pings](pings/index.md).

The Glean SDK should be initialized as soon as possible, and importantly, before any other libraries in the application start using Glean.
Library code should never call `Glean.initialize`, since it should be called exactly once per application.

> **Note**: if the application has the concept of release channels and knows which channel it is on at run-time, then it can provide the Glean SDK with this information by setting it as part of the `Configuration` object parameter of the `Glean.initialize` method. For example:

```Kotlin
Glean.initialize(applicationContext, Configuration(channel = "beta"))
```

</div>

<div data-lang="Swift" class="tab">

> **Note**: The Glean SDK does not support use across multiple processes, and must only be initialized on the application's main process.

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

        // Call setUploadEnabled first, since Glean.initialize
        // might send pings if there are any metrics queued up
        // from a previous run.
        Glean.shared.setUploadEnabled(Settings.isTelemetryEnabled)

        // Initialize the Glean library.
        Glean.shared.initialize()
    }
}
```

Once initialized, if collection is enabled, the Glean SDK will automatically start collecting [baseline metrics](pings/metrics.md) and sending its [pings](pings/index.md).

The Glean SDK should be initialized as soon as possible, and importantly, before any other libraries in the application start using Glean.
Library code should never call `Glean.shared.initialize`, since it should be called exactly once per application.

> **Note**: if the application has the concept of release channels and knows which channel it is on at run-time,
>  then it can provide the Glean SDK with this information by setting it as part of the `Configuration` object parameter of the `Glean.shared.initialize` method. For example:

```Swift
Glean.shared.initialize(Configuration(channel: "beta"))
```

</div>

<div data-lang="Python" class="tab">

The main control for Glean is on the `glean.Glean` singleton.

The Glean SDK should be initialized as soon as possible, and importantly, before any other libraries in the application start using Glean.
Library code should never call `Glean.initialize`, since it should be called exactly once per application.


```python
from glean import Glean

# Call Glean.set_upload_enabled first, since Glean.initialize might send pings
# if there are any metrics queued up from a previous run.
Glean.set_upload_enabled(True)

Glean.initialize(
    application_id="my-app-id",
    application_version="0.1.0",
)
```

Additonal configuration is available on the `glean.Configuration` object, which can be passed into `Glean.initialize()`.

Unlike Android and Swift, the Python bindings do not automatically send any pings.
See the [custom pings documentation](pings/custom.md) about adding custom pings and sending them.

</div>

{{#include ../tab_footer.md}}

## Enabling and disabling metrics

{{#include ../tab_header.md}}

<div data-lang="Kotlin" class="tab">

`Glean.setUploadEnabled()` should be called in response to the user enabling or disabling telemetry.
This method should also be called at least once prior to calling `Glean.initialize()`.

The application should provide some form of user interface to call this method.

When going from enabled to disabled, all pending events, metrics and pings are cleared, except for [`first_run_date`](pings/index.html#the-client_info-section).
When re-enabling, core Glean metrics will be recomputed at that time.

</div>

<div data-lang="Swift" class="tab">

`Glean.shared.setUploadEnabled()` should be called in response to the user enabling or disabling telemetry.
This method should also be called at least once prior to calling `Glean.shared.initialize()`.

The application should provide some form of user interface to call this method.

When going from enabled to disabled, all pending events, metrics and pings are cleared, except for [`first_run_date`](pings/index.html#the-client_info-section).
When re-enabling, core Glean metrics will be recomputed at that time.

</div>

<div data-lang="Python" class="tab">

`Glean.set_upload_enabled()` should be called in response to the user enabling or disabling telemetry.
This method should also be called at least once prior to calling `Glean.initialize()`.

The application should provide some form of user interface to call this method.

When going from enabled to disabled, all pending events, metrics and pings are cleared, except for [`first_run_date`](pings/index.html#the-client_info-section).
When re-enabling, core Glean metrics will be recomputed at that time.

</div>

{{#include ../tab_footer.md}}
