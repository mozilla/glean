# Adding Glean to your project

## Before using Glean

Products using the Glean SDK to collect telemetry **must**:

- add documentation for any new metric collected with the library in its repository (see [an example](pings/index.md));
- go through data review for the newly collected data by following [this process](https://wiki.mozilla.org/Firefox/Data_Collection);
- provide a way for users to turn data collection off (e.g. providing settings to control
  `Glean.setUploadEnabled()`).

## Usage

### Integrating with your project

{{#include ../tab_header.md}}

<div data-lang="Kotlin" class="tab">

#### Setting up the dependency

Glean is published on [maven.mozilla.org](https://maven.mozilla.org/).
To use it, you need to add the following to your project's top-level build file,
in the `allprojects` block (see e.g. [Glean's own `build.gradle`](https://github.com/mozilla/glean/blob/master/build.gradle)):

```Groovy
repositories {
    maven {
       url "https://maven.mozilla.org/maven2"
    }
}
```

Each module that uses Glean needs to specify it in its build file, in the `dependencies` block.
Add this to your Gradle configuration:

```Groovy
implementation "org.mozilla.components:service-glean:{latest-version}"
```

> **Important:** the `{latest-version}` placeholder in the above link should be replaced with the version number of the Glean SDK used by the project.

For example, if version *6.0.2* is used, then the include directive becomes:

```Groovy
implementation "org.mozilla.components:service-glean:6.0.2"
```

The Glean SDK is released as part of [android-components](https://github.com/mozilla-mobile/android-components).  Therefore, it follows android-components' versions.
The [android-components release page](https://github.com/mozilla-mobile/android-components/releases/) can be used to determine the latest version.

#### Integrating with the build system

In order for the Glean SDK to generate an API for your metrics, two Gradle plugins must be included in your build:

- The [Glean Gradle plugin](https://github.com/mozilla/glean/tree/master/gradle-plugin/)
- JetBrains' [Python envs plugin](https://github.com/JetBrains/gradle-python-envs/)

The Glean Gradle plugin is distributed through Mozilla's Maven, so we need to tell your build where to look for it by adding the following to the top of your `build.gradle`:

```
buildscript {
    repositories {
        maven {
            url "https://maven.mozilla.org/maven2"
        }

        dependencies {
            classpath "org.mozilla.telemetry:glean-gradle-plugin:{latest-version}"
        }
    }
}
```

> **Important:** as above, the `{latest-version}` placeholder in the above link should be replaced with the version number of the Glean SDK used by the project.

The JetBrains Python plugin is distributed in the Gradle plugin repository, so it can be included with:

```Groovy
plugins {
    id "com.jetbrains.python.envs" version "0.0.26"
}
```

Right before the end of the same file, we need to apply the Glean Gradle plugin:

```Groovy
apply plugin: "org.mozilla.telemetry.glean-gradle-plugin"
```

There are [additional parameters](android-build-configuration-options.md) that can be set to control the behavior of the Glean Gradle plugin, but they are rarely needed for normal use.

> **Note:** Earlier versions of Glean used a Gradle script (`sdk_generator.gradle`) rather than a Gradle plugin. Its use is deprecated and projects should be updated to use the Gradle plugin as described above.

</div>

<div data-lang="Swift" class="tab">

#### Setting up the dependency

Glean can be consumed through [Carthage](https://github.com/Carthage/Carthage), a dependency manager for macOS and iOS.
For consuming the latest version of Glean, add the following line to your `Cartfile`::

```
github "mozilla/glean" "master"
```

#### Integrating with the build system

> **Note:** This is not yet documented, as the exact mechanism is not finalized. See [Bug 1589383](https://bugzilla.mozilla.org/show_bug.cgi?id=1589383) for more information.

</div>

<div data-lang="Python" class="tab">

It is recommended that you use a virtual environment for your work to isolate the dependencies for your project. There are many popular abstractions on top of virtual environments in the Python ecosystem which can help manage your project dependencies.

The Python Glean bindings currently have [prebuilt wheels on PyPI for x86_64 Linux only](https://pypi.org/project/glean-sdk/#files).

If you're running that platform and have your virtual environment set up and activated, you can install Glean into it using:

```bash
$ python -m pip install glean_sdk
```

If you are not on x86_64 Linux, you will need to build the Glean Python bindings from source using [these instructions](../dev/python/setting-up-python-build-environment.html).

</div>

{{#include ../tab_footer.md}}

### Adding new metrics

All metrics that your project collects must be defined in a `metrics.yaml` file.

The format of that file is documented [with `glean_parser`](https://mozilla.github.io/glean_parser/metrics-yaml.html).
To learn more, see [adding new metrics](adding-new-metrics.md).

> **Important**: as stated [before](adding-glean-to-your-project.md#before-using-glean), any new data collection requires documentation and data-review.
> This is also required for any new metric automatically collected by the Glean SDK.

{{#include ../tab_header.md}}

<div data-lang="Swift" class="tab">

On iOS, add this file to your project and define it as an input file for the `sdk_generator.sh` script in the `Run Script` step defined before.

</div>

<div data-lang="Python" class="tab">

> **Important:** The Python bindings are already being used internally for testing of Mozilla's data platform.
> However, they are not ready for production or inclusion in applications that are distributed to end users.
> Progress on this support is being [tracked in this bug](https://bugzilla.mozilla.org/show_bug.cgi?id=1598673).

For Python, the `metrics.yaml` file must be available and loaded at runtime.

If your project is a script (i.e. just Python files in a directory), you can load the `metrics.yaml` using:

```Python
from glean import load_metrics

metrics = load_metrics("metrics.yaml")

# Use a metric on the returned object
metrics.your_category.your_metric.set("value")
```

If your project is a distributable Python package, you need to include the `metrics.yaml` file using [one of the myriad ways to include data in a Python package](https://setuptools.readthedocs.io/en/latest/setuptools.html#including-data-files) and then use [`package_resources.resource_filename()`](https://setuptools.readthedocs.io/en/latest/pkg_resources.html#resource-extraction) to get the filename at runtime.

```Python
from glean import load_metrics
from package_resources import resource_filename

metrics = load_metrics(resource_filename(__name__, "metrics.yaml"))

# Use a metric on the returned object
metrics.your_category.your_metric.set("value")
```

</div>

{{#include ../tab_footer.md}}

### Adding custom pings

Please refer to the [custom pings documentation](pings/custom.md).

> **Important**: as stated [before](adding-glean-to-your-project.md#before-using-glean), any new data collection requires documentation and data-review.
> This is also required for any new metric automatically collected by the Glean SDK.

### Testing metrics

In order to make testing metrics easier 'out of the box', all metrics include a set of test API functions in order to facilitate unit testing.  These include functions to test whether a value has been stored, and functions to retrieve the stored value for validation.  For more information, please refer to [Unit testing Glean metrics](testing-metrics.md).

### Adding metadata about your project to the pipeline

In order for data to be collected from your project, metadata must be added to `probe_scraper`.

These specific steps are described in [the `probe_scraper` documentation](https://github.com/mozilla/probe-scraper#adding-a-new-glean-repository).

## Application-specific steps

The following steps are required for applications using the Glean SDK, but not libraries.

> **Important:** The Glean SDK should only be initialized from the main application, not individual libraries.

If you are adding Glean support to a library, you can safely skip this section.

{{#include ../tab_header.md}}

<div data-lang="Kotlin" class="tab">

### Initializing the Glean SDK

Please also note that the Glean SDK does not support use across multiple processes, and must only be initialized on the application's main process. Initializing in other processes is a no-op.
Additionally, Glean must be initialized on the main (UI) thread of the applications main process. Failure to do so will throw an `IllegalThreadStateException`.

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

### Enabling and disabling metrics

`Glean.setUploadEnabled()` should be called in response to the user enabling or disabling telemetry.
This method should also be called at least once prior to calling `Glean.initialize()`.

The application should provide some form of user interface to call this method.

When going from enabled to disabled, all pending events, metrics and pings are cleared, except for [`first_run_date`](pings/index.html#the-client_info-section).
When re-enabling, core Glean metrics will be recomputed at that time.

</div>

<div data-lang="Swift" class="tab">

### Initializing the Glean SDK

Please also note that the Glean SDK does not support use across multiple processes, and must only be initialized on the application's main process.

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

### Enabling and disabling metrics

`Glean.shared.setUploadEnabled()` should be called in response to the user enabling or disabling telemetry.
This method should also be called at least once prior to calling `Glean.shared.initialize()`.

The application should provide some form of user interface to call this method.

When going from enabled to disabled, all pending events, metrics and pings are cleared, except for [`first_run_date`](pings/index.html#the-client_info-section).
When re-enabling, core Glean metrics will be recomputed at that time.

</div>

<div data-lang="Python" class="tab">

### Initializing the Glean SDK

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

### Enabling and disabling metrics

`Glean.set_upload_enabled()` should be called in response to the user enabling or disabling telemetry.
This method should also be called at least once prior to calling `Glean.initialize()`.

The application should provide some form of user interface to call this method.

When going from enabled to disabled, all pending events, metrics and pings are cleared, except for [`first_run_date`](pings/index.html#the-client_info-section).
When re-enabling, core Glean metrics will be recomputed at that time.

</div>

{{#include ../tab_footer.md}}
