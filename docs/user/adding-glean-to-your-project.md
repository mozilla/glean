# Adding Glean to your project

## Before using the library
Products using Glean to collect telemetry **must**:

- add documentation for any new metric collected with the library in its repository (see [an example](pings/index.md));
- go through data review for the newly collected data by following [this process](https://wiki.mozilla.org/Firefox/Data_Collection);
- provide a way for users to turn data collection off (e.g. providing settings to control
  `Glean.setUploadEnabled()`).

## Usage

### Setting up the dependency

Use Gradle to download the library from [maven.mozilla.org](https://maven.mozilla.org/)
([Setup repository](../../../README.md#maven-repository)):

```Groovy
implementation "org.mozilla.components:service-glean:{latest-version}"
```

### Integrating with the build system

In order for Glean to work, a Python environment must be accessible at build time.
This is done automatically by the [`com.jetbrains.python.envs`](https://github.com/JetBrains/gradle-python-envs/) plugin. 
The plugin **must** be manually enabled by adding the following `plugins` block at the top of the `build.gradle` file for your app module.

```Groovy
plugins {
    id "com.jetbrains.python.envs" version "0.0.26"
}
```

Right before the end of the same file, the Glean build script must be included.
This script can be referenced directly from the GitHub repo, as shown below:

```Groovy
apply from: 'https://github.com/mozilla/glean/raw/v{latest-version}/glean-core/android/sdk_generator.gradle'
```

**Important:** the `{latest-version}` placeholder in the above link should be replaced with the version number of the Glean library used by the project. 
For example, if version *0.34.2* is used, then the include directive becomes:

```Groovy
apply from: 'https://github.com/mozilla/glean/raw/v0.34.2/glean-core/android/sdk_generator.gradle'
```

### Initializing Glean

Before any data collection can take place, Glean **must** be initialized from the application.
An excellent place to perform this operation is within the `onCreate` method of the class that extends Android's `Application` class.

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

Once initialized, if collection is enabled, Glean will automatically start collecting [baseline metrics](pings/metrics.md) and sending its [pings](pings/index.md).

Glean should be initialized as soon as possible, and importantly, before any other libraries in the application start using Glean. 
Library code should never call `Glean.initialize`, since it should be called exactly once per application.

**Note**: if the application has the concept of release channels and knows which channel it is on at run-time, then it can provide Glean with this information by setting it as part of the `Configuration` object parameter of the `Glean.initialize` method. For example:

```Kotlin
Glean.initialize(applicationContext, Configuration(channel = "beta"))
```

### Enabling and disabling metrics

`Glean.setUploadEnabled()` should be called in response to the user enabling or disabling telemetry.  
This method should also be called at least once prior to calling `Glean.initialize()`.

The application should provide some form of user interface to call this method.

When going from enabled to disabled, all pending events, metrics and pings are cleared, except for `first_run_date`.
When re-enabling, core Glean metrics will be recomputed at that time.

### Adding new metrics

All metrics that your application collects must be defined in a `metrics.yaml` file. 
This file should be at the root of the application module (the same directory as the `build.gradle` file you updated). 
The format of that file is documented [here](https://mozilla.github.io/glean_parser/metrics-yaml.html).
To learn more, see [adding new metrics](adding-new-metrics.md).

**Important**: as stated [here](#before-using-the-library), any new data collection requires documentation and data-review. This is also required for any new metric automatically collected by Glean.

### Adding custom pings

Please refer to the custom pings documentation [here](pings/custom.md).

**Important**: as stated [here](#before-using-the-library), any new data collection, including new custom pings, requires documentation and data-review. This is also required for any new ping automatically collected by Glean.

### Testing metrics

In order to make testing metrics easier 'out of the box', all metrics include a set of test API functions in order to facilitate unit testing.  These include functions to test whether a value has been stored, and functions to retrieve the stored value for validation.  For more information, please refer to [Unit testing Glean metrics](testing-metrics.md).

