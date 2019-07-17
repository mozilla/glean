# Timespan

Timespans are used to make a measurement of how much time is spent in a particular task.

To measure the distribution of multiple timespans, see [Timing Distributions](timing_distribution.md). To record absolute times, see [Datetimes](datetime.md).

## Configuration

Timespans have a required `time_unit` parameter to specify the smallest unit of resolution that the timespan will record. The allowed values for `time_unit` are:

   - `nanosecond`
   - `microsecond`
   - `millisecond`
   - `second`
   - `minute`
   - `hour`
   - `day`

Consider the resolution that is required by your metric, and use the largest possible value that will provide useful information so as to not leak too much fine-grained information from the client. It is important to note that the value sent in the ping is truncated down to the nearest unit. Therefore, a measurement of 500 nanoseconds will be truncated to 0 microseconds.

Say you're adding a new timespan for the time spent logging into the app. First you need to add an entry for the counter to the `metrics.yaml` file:

```YAML
auth:
  login_time:
    type: timespan
    description: >
      Measures the time spent logging in.
    time_unit: milliseconds
    ...
```

## API

Each time interval that the timespan metric records must be associated with an object provided by the user. This is so that intervals can be measured concurrently. In our example using login time, this might be an object representing the login UI page.

```Kotlin
import org.mozilla.yourApplication.GleanMetrics.Auth

fun onShowLogin(e: Event) {
    Auth.loginTime.start()
    // ...
}

fun onLogin(e: Event) {
    Auth.loginTime.stop()
    // ...
}

fun onLoginCancel(e: Event) {
    Auth.loginTime.cancel()
    // ...
}
```

The time reported in the telemetry ping will be timespan recorded during the lifetime of the ping.

There are test APIs available too:

```Kotlin
import org.mozilla.yourApplication.GleanMetrics.Auth
Glean.enableTestingMode()

// Was anything recorded?
assertTrue(Auth.loginTime.testHasValue())
// Does the timer have the expected value
assertTrue(Auth.loginTime.testGetValue() > 0)
```

### Raw API

> **Note**: The raw API was designed to support a specific set of use-cases.
> Please consider using the higher level APIs listed above.

It's possible to explicitly set the timespan value, in nanoseconds.
This API should only be used if your library or application requires recording times in a way that can not make use of `start`/`stop`/`cancel`.

```Kotlin
import org.mozilla.yourApplication.GleanMetrics.HistorySync

val duration = SyncResult.status.syncs.took.toLong()
HistorySync.setRawNanos(duration)
```

The raw API will not overwrite a running timer or existing timespan value.

## Limits

* None.

## Examples

* How much time is spent rendering the UI?

## Recorded errors

* `invalid_value`
    * If recording a negative timespan.
    * If starting a timer while a previous timer is running.
    * If stopping a timer while it is not running.
    * If trying to set a raw timespan while a timer is running.

## Reference

* [Kotlin API docs](../../../javadoc/glean/mozilla.telemetry.glean.private/-timespan-metric-type/index.html)
