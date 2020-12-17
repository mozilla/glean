# Timespan

Timespans are used to make a measurement of how much time is spent in a particular task.

To measure the distribution of multiple timespans, see [Timing Distributions](timing_distribution.md). To record absolute times, see [Datetimes](datetime.md).

It is not recommended to use timespans in multiple threads, since calling `start` or `stop` out of order will be recorded as an `invalid_state` error.

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

{{#include ../../tab_header.md}}

<div data-lang="Kotlin" class="tab">

```Kotlin
import org.mozilla.yourApplication.GleanMetrics.Auth

fun onShowLogin() {
    Auth.loginTime.start()
    // ...
}

fun onLogin() {
    Auth.loginTime.stop()
    // ...
}

fun onLoginCancel() {
    Auth.loginTime.cancel()
    // ...
}
```

The time reported in the telemetry ping will be timespan recorded during the lifetime of the ping.

There are test APIs available too:

```Kotlin
import org.mozilla.yourApplication.GleanMetrics.Auth

// Was anything recorded?
assertTrue(Auth.loginTime.testHasValue())
// Does the timer have the expected value
assertTrue(Auth.loginTime.testGetValue() > 0)
// Was the timing recorded incorrectly?
assertEquals(1, Auth.loginTime.testGetNumRecordedErrors(ErrorType.InvalidValue))
```

</div>

<div data-lang="Java" class="tab">

```Java
import org.mozilla.yourApplication.GleanMetrics.Auth;

void onShowLogin() {
    Auth.INSTANCE.loginTime.start();
    // ...
}

void onLogin() {
    Auth.INSTANCE.loginTime.stop();
    // ...
}

void onLoginCancel() {
    Auth.INSTANCE.loginTime.cancel();
    // ...
}
```

The time reported in the telemetry ping will be timespan recorded during the lifetime of the ping.

There are test APIs available too:

```Java
import org.mozilla.yourApplication.GleanMetrics.Auth;

// Was anything recorded?
assertTrue(Auth.INSTANCE.loginTime.testHasValue());
// Does the timer have the expected value
assertTrue(Auth.INSTANCE.loginTime.testGetValue() > 0);
// Was the timing recorded incorrectly?
assertEquals(
    1,
    Auth.INSTANCE.loginTime.testGetNumRecordedErrors(
        ErrorType.InvalidValue
    )
);
```

</div>

<div data-lang="Swift" class="tab">

```Swift
func onShowLogin() {
    Auth.loginTime.start()
    // ...
}

func onLogin() {
    Auth.loginTime.stop()
    // ...
}

func onLoginCancel() {
    Auth.loginTime.cancel()
    // ...
}
```

The time reported in the telemetry ping will be timespan recorded during the lifetime of the ping.

There are test APIs available too:

```Swift
@testable import Glean

// Was anything recorded?
XCTAssert(Auth.loginTime.testHasValue())
// Does the timer have the expected value
XCTAssert(try Auth.loginTime.testGetValue() > 0)
// Was the timing recorded incorrectly?
XCTAssertEqual(1, Auth.loginTime.testGetNumRecordedErrors(.invalidValue))
```

</div>

<div data-lang="Python" class="tab">

```Python
from glean import load_metrics
metrics = load_metrics("metrics.yaml")

def on_show_login():
    metrics.auth.login_time.start()
    # ...

def on_login():
    metrics.auth.login_time.stop()
    # ...

def on_login_cancel():
    metrics.auth.login_time.cancel()
    # ...
```

The Python bindings also have a context manager for measuring time:

```Python
with metrics.auth.login_time.measure():
    # ... Do the login ...
```

The time reported in the telemetry ping will be timespan recorded during the lifetime of the ping.

There are test APIs available too:

```Python
# Was anything recorded?
assert metrics.auth.login_time.test_has_value()
# Does the timer have the expected value
assert metrics.auth.login_time.test_get_value() > 0
# Was the timing recorded incorrectly?
assert 1 == metrics.auth.local_time.test_get_num_recorded_errors(
    ErrorType.INVALID_VALUE
)
```

</div>

<div data-lang="C#" class="tab">

```csharp
using static Mozilla.YourApplication.GleanMetrics.Auth;

void OnShowLogin()
{
    Auth.loginTime.Start();
    // ...
}

void OnLogin()
{
    Auth.loginTime.Stop();
    // ...
}

void OnLoginCancel()
{
    Auth.loginTime.Cancel();
    // ...
}
```

The time reported in the telemetry ping will be timespan recorded during the lifetime of the ping.

There are test APIs available too:

```csharp
using static Mozilla.YourApplication.GleanMetrics.Auth;

// Was anything recorded?
Assert.True(Auth.loginTime.TestHasValue());
// Does the timer have the expected value
Assert.True(Auth.loginTime.TestGetValue() > 0);
// Was the timing recorded incorrectly?
Assert.Equals(1, Auth.loginTime.TestGetNumRecordedErrors(ErrorType.InvalidValue));
```

</div>

<div data-lang="Rust" class="tab">

```rust
fn show_login() {
    metrics::auth::login_time.start();
    // ...
}

fn login() {
    metrics::auth::login_time.stop();
    // ...
}

fn login_cancel() {
    metrics::auth::login_time.cancel();
    // ...
}
```

The time reported in the telemetry ping will be timespan recorded during the lifetime of the ping.

There are test APIs available too:

```rust
use metrics::auth::login_time;

// Was anything recorded?
assert!(login_time.test_get_value().is_some());
assert!(login_time.test_get_value().unwrap() > 0);
// Was the timing recorded incorrectly?
assert_eq!(1, login_time.test_get_num_recorded_errors(ErrorType::InvalidValue));
```

</div>

<div data-lang="C++" class="tab">

> **Note**: C++ APIs are only available in Firefox Desktop.

```c++
#include "mozilla/glean/GleanMetrics.h"

mozilla::glean::auth::login_time.Start();
PR_Sleep(PR_MillisecondsToInterval(10));
mozilla::glean::auth::login_time.Stop();
```

There are test APIs available too:

```c++
#include "mozilla/glean/GleanMetrics.h"

// Does it have an expected values?
ASSERT_TRUE(mozilla::glean::auth::login_time.TestGetValue().value() > 0);
// Did it run across any errors?
// TODO: https://bugzilla.mozilla.org/show_bug.cgi?id=1683171
```

</div>

<div data-lang="JS" class="tab">

> **Note**: JS APIs are only available in Firefox Desktop.

```js
Glean.auth.loginTime.start();
await sleep(10);
Glean.auth.loginTime.stop();
```

There are test APIs available too:

```js
Assert.ok(Glean.auth.loginTime.testGetValue() > 0);
// Did it run across any errors?
// TODO: https://bugzilla.mozilla.org/show_bug.cgi?id=1683171
```

</div>

{{#include ../../tab_footer.md}}

## Raw API

> **Note**: The raw API was designed to support a specific set of use-cases.
> Please consider using the higher level APIs listed above.

It's possible to explicitly set the timespan value, in nanoseconds.
This API should only be used if your library or application requires recording times in a way that can not make use of `start`/`stop`/`cancel`.

The raw API will not overwrite a running timer or existing timespan value.

{{#include ../../tab_header.md}}

<div data-lang="Kotlin" class="tab">

```Kotlin
import org.mozilla.yourApplication.GleanMetrics.HistorySync

val duration = SyncResult.status.syncs.took.toLong()
HistorySync.setRawNanos(duration)
```

</div>

<div data-lang="Swift" class="tab">

```Swift
let duration = SyncResult.status.syncs.took.toLong()
HistorySync.setRawNanos(duration)
```

</div>

<div data-lang="Python" class="tab">

```Python
import org.mozilla.yourApplication.GleanMetrics.HistorySync

val duration = SyncResult.status.syncs.took.toLong()
HistorySync.setRawNanos(duration)
```

</div>

<div data-lang="C#" class="tab">

TODO. To be implemented in [bug 1648442](https://bugzilla.mozilla.org/show_bug.cgi?id=1648442).

</div>

<div data-lang="Rust" class="tab">

The raw API is not supported in Rust. See [bug 1680225](https://bugzilla.mozilla.org/show_bug.cgi?id=1680225).

</div>

{{#include ../../tab_footer.md}}

## Limits

* Timings are recorded in nanoseconds.

  * On Android, the [`SystemClock.elapsedRealtimeNanos()`](https://developer.android.com/reference/android/os/SystemClock.html#elapsedRealtimeNanos()) function is used, so it is limited by the accuracy and performance of that timer. The time measurement includes time spent in sleep.

  * On iOS, the [`mach_absolute_time`](https://developer.apple.com/library/archive/documentation/Darwin/Conceptual/KernelProgramming/services/services.html) function is used,
    so it is limited by the accuracy and performance of that timer.
    The time measurement does not include time spent in sleep.

  * On Python 3.7 and later, [`time.monotonic_ns()`](https://docs.python.org/3/library/time.html#time.monotonic_ns) is used.  On earlier versions of Python, [`time.monotonics()`](https://docs.python.org/3/library/time.html#time.monotonic) is used, which is not guaranteed to have nanosecond resolution.

  * On other platforms it uses [`time::precise_time_ns`](https://docs.rs/time/0.1.40/time/fn.precise_time_ns.html), which uses a high-resolution performance counter in nanoseconds provided by the underlying platform.

## Examples

* How much time is spent rendering the UI?

## Recorded errors

* `invalid_value`
    * If recording a negative timespan.
* `invalid_state`
    * If starting a timer while a previous timer is running.
    * If stopping a timer while it is not running.
    * If trying to set a raw timespan while a timer is running.
    * If trying to record a timespan again while a previous value is still stored.

## Reference

* [Kotlin API docs](../../../javadoc/glean/mozilla.telemetry.glean.private/-timespan-metric-type/index.html)
* [Swift API docs](../../../swift/Classes/TimespanMetricType.html)
* [Python API docs](../../../python/glean/metrics/timespan.html)
