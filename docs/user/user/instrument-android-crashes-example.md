# Instrumenting Android crashes with the Glean SDK

One of the things that might be useful to collect data on in an Android application is crashes.  This guide will walk through a
basic strategy for instrumenting an Android application with crash telemetry using a custom ping.

**Note:**  This is a _very_ simple example of instrumenting crashes using the Glean SDK.  There will be challenges to
using this approach in a production application that should be considered.  For instance, when an app crashes it can be in an
unknown state and may not be able to do things like upload data to a server.  The recommended way of instrumenting crashes with
Android Components is called [lib-crash](https://github.com/mozilla-mobile/android-components/tree/HEAD/components/lib/crash), which takes into consideration things like multiple processes and persistence.

## Before You Start

There are a few things that need to be installed in order to proceed, mainly [Android Studio](https://developer.android.com/studio/).  If you include the Android SDK,
Android Studio can take a little while to download and get installed.  This walk-through assumes some knowledge of Android
application development.  Knowing where to go to create a new project and how to add dependencies to a Gradle file will be
helpful in following this guide.

## Setup Build Configuration

Please follow the instruction in the ["Adding Glean to your project"](adding-glean-to-your-project.md) chapter in order to set up
Glean in an Android project.

### Add A Custom Metric

Since crashes will be instrumented with some custom metrics, the next step will be to add a `metrics.yaml` file to define the
metrics used to record the crash information and a `pings.yaml` file to define a custom ping which will give some control over
the scheduling of the uploading.  See ["Adding new metrics"](adding-new-metrics.md) for more information about adding metrics.

What metric type should be used to represent the crash data?  While this could be implemented several ways, an [event](metrics/event.md) is an
excellent choice, simply because events capture information in a nice concise way and they have a built-in way of passing
additional information using the `extras` field.  If it is necessary to pass along the cause of the exception or a few lines of
description, events let us do that easily (with [some limitations](metrics/event.md#limits)).

Now that a metric type has been chosen to represent the metric, the next step is creating the `metrics.yaml`.  Inside of the
root application folder of the Android Studio project create a new file named `metrics.yaml`.  After adding the schema
definition and event metric definition, the `metrics.yaml` should look like this:

```YAML
# Required to indicate this is a `metrics.yaml` file
$schema: moz://mozilla.org/schemas/glean/metrics/1-0-0

crash:
  exception:
    type: event
    description: |
      Event to record crashes caused by unhandled exceptions
    notification_emails:
      - crashes@example.com
    bugs:
      - https://bugzilla.mozilla.org/show_bug.cgi?id=1582479
    data_reviews:
      - https://bugzilla.mozilla.org/show_bug.cgi?id=1582479
    expires:
      2021-01-01
    send_in_pings:
      - crash
    extra_keys:
      cause:
        description: The cause of the crash
      message:
        description: The exception message
```

As a brief explanation, this creates a metric called `exception` within a metric category called `crash`.  There is a text
`description` and the required `notification_emails`, `bugs`, `data_reviews`, and `expires` fields.  The `send_in_pings` field
is important to note here that it has a value of `- crash`.  This means that the crash event metric will be sent via a custom
ping named `crash` (which hasn't been created yet).  Finally, note the `extra_keys` field which has two keys defined, `cause`
and `message`.  This allows for sending additional information along with the event to be associated with these keys.

**Note:**  For Mozilla applications, a mandatory [data review](https://github.com/mozilla/data-review/blob/HEAD/request.md) is required in order to collect information with the Glean SDK.

### Add A Custom Ping

Define the custom ping that will help control the upload scheduling by creating a `pings.yaml` file in the same directory as
the `metrics.yaml` file.  For more information about adding custom pings, see the section on [custom pings](pings/custom.md).  
The name of the ping will be `crash`, so the `pings.yaml` file should look like this:

```YAML
# Required to indicate this is a `pings.yaml` file
$schema: moz://mozilla.org/schemas/glean/pings/1-0-0

crash:
  description: >
    A ping to transport crash data
  include_client_id: true
  notification_emails:
    - crash@example.com
  bugs:
    - https://bugzilla.mozilla.org/show_bug.cgi?id=1582479
  data_reviews:
    - https://bugzilla.mozilla.org/show_bug.cgi?id=1582479
```

Before the newly defined metric or ping can be used, the application must first be built.  This will cause the [glean_parser](https://github.com/mozilla/glean_parser/)
to execute and generate the API files that represent the metric and ping that were newly defined.

**Note:** If changes to the YAML files aren't showing up in the project, try running the clean task on the project before
building any time one of the Glean YAML files has been modified.  

It is recommended that Glean be initialized as early in the application startup as possible, which is why it's good to use a
custom `Application`, like the Glean Sample App [`GleanApplication.kt`](https://github.com/mozilla/glean/blob/main/samples/android/app/src/main/java/org/mozilla/samples/glean/GleanApplication.kt).

Initializing Glean in the `Application.onCreate()` is ideal for this purpose.  Start by adding the import statement to allow
the usage of the custom ping that was created, adding the following to the top of the file:

```Kotlin
import org.mozilla.gleancrashexample.GleanMetrics.Pings
```

Next, register the custom ping by calling `Glean.registerPings(Pings)` in the `onCreate()` function, preferably before calling
`Glean.initialize()`.  The completed function should look something like this:

```Kotlin
override fun onCreate() {
  super.onCreate()

  // Register the application's custom pings.
  Glean.registerPings(Pings)

  // Initialize the Glean library
  Glean.initialize(applicationContext)
}

```

This completes the registration of the custom ping with the Glean SDK so that it knows about it and can manage the storage and
other important details of it like sending it when `send()` is called.

### Instrument The App To Record The Event

In order to make the custom `Application` class handle uncaught exceptions, extend the class definition by adding
`Thread.UncaughtExceptionHandler` as an inherited class like this:

```Kotlin
class MainActivity : AppCompatActivity(), Thread.UncaughtExceptionHandler {
    ...
}
```

As part of implementing the `Thread.UncaughtExceptionHandler` interface, the custom `Application` needs to implement the
override of the `uncaughtException()` function.  An example of this override that records data and sends the ping could look
something like this:

```Kotlin
override fun uncaughtException(thread: Thread, exception: Throwable) {
    Crash.exception.record(
        mapOf(
            Crash.exceptionKeys.cause to exception.cause!!.toString(),
            Crash.exceptionKeys.message to exception.message!!
        )
    )
    Pings.crash.submit()
}
```

This records data to the `Crash.exception` metric from the `metrics.yaml`.  The category of the metric is `crash` and the name
is `exception` so it is accessed it by calling `record()` on the `Crash.exception` object.  The extra information for the
`cause` and the `message` is set as well.  Finally, calling `Pings.crash.submit()` forces the `crash` ping to be scheduled to be
sent.

The final step is to register the custom `Application` as the default uncaught exception handler by adding the following to the
`onCreate()` function after `Glean.initialize(this)`:

```Kotlin
Thread.setDefaultUncaughtExceptionHandler(this)
```

### Next Steps

This information didn't really get recorded by anything, as it would be rejected by the telemetry pipeline unless the
application was already known.  In order to collect telemetry from a new application, there is additional work that is
necessary that is beyond the scope of this example.  In order for data to be collected from your project, metadata must be
added to the `probe_scraper`.  The instructions for accomplishing this can be found in the [`probe_scraper` documentation](https://github.com/mozilla/probe-scraper#adding-a-new-glean-repository).
