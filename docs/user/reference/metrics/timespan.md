# Timespan

Timespans are used to make a measurement of how much time is spent in a particular task.
Irrespective of the timespan's `lifetime`, both `start` and `stop` must occur within the same application session.

To measure the distribution of multiple timespans,
see [Timing Distributions](timing_distribution.md).
To record absolute times, see [Datetimes](datetime.md).

It is not recommended to use timespans in multiple threads,
since calling `start` or `stop` out of order will be recorded as an `invalid_state` error.

## Recording API

### `start`

Starts tracking time. Uses an internal monotonic timer.

{{#include ../../../shared/tab_header.md}}
<div data-lang="Kotlin" class="tab">

```Kotlin
import org.mozilla.yourApplication.GleanMetrics.Auth

fun onShowLogin() {
    Auth.loginTime.start()
    // ...
}
```
</div>
<div data-lang="Java" class="tab">

```Java
import org.mozilla.yourApplication.GleanMetrics.Auth;

void onShowLogin() {
    Auth.INSTANCE.loginTime().start();
    // ...
}
```
</div>
<div data-lang="Swift" class="tab">

```Swift
func onShowLogin() {
    Auth.loginTime.start()
    // ...
}
```
</div>
<div data-lang="Python" class="tab">

```Python
from glean import load_metrics
metrics = load_metrics("metrics.yaml")

def on_show_login():
    metrics.auth.login_time.start()
    # ...
```
</div>
<div data-lang="Rust" class="tab">

```Rust
use glean_metrics::auth;

fn show_login() {
    auth::login_time.start();
    // ...
}
```
</div>
<div data-lang="JavaScript" class="tab">

```js
import * as auth from "./path/to/generated/files/auth.js";

function onShowLogin() {
    auth.loginTime.start();
    // ...
}
```
</div>
<div data-lang="Firefox Desktop" class="tab">

**C++**
```c++
#include "mozilla/glean/PasswordmgrMetrics.h"
void OnShowLogin() {
  mozilla::glean::auth::login_time.Start();
  // ...
}
```
**JavaScript**
```js
function onShowLogin() {
  Glean.auth.loginTime.start();
  // ...
}
```
</div>
{{#include ../../../shared/tab_footer.md}}

#### Recorded errors
* [`invalid_state`](../../user/metrics/error-reporting.md):
  If the metric is already tracking time (`start` has already been called and not `cancel`ed).

#### Limits
* The maximum resolution of the elapsed duration is limited by the clock used on each platform.
* This also determines the behavior of a timespan over sleep:
  * On Android, the
    [`SystemClock.elapsedRealtimeNanos()`](https://developer.android.com/reference/android/os/SystemClock.html#elapsedRealtimeNanos())
    function is used, so it is limited by the accuracy and performance of that timer.
    The time measurement includes time spent in sleep.
  * On iOS, the
    [`mach_absolute_time`](https://developer.apple.com/library/archive/documentation/Darwin/Conceptual/KernelProgramming/services/services.html)
    function is used, so it is limited by the accuracy and performance of that timer.
    The time measurement does not include time spent in sleep.
  * On Python 3.7 and later,
    [`time.monotonic_ns()`](https://docs.python.org/3/library/time.html#time.monotonic_ns) is used.
    On earlier versions of Python,
    [`time.monotonics()`](https://docs.python.org/3/library/time.html#time.monotonic) is used,
    which is not guaranteed to have nanosecond resolution.
  * On other platforms
    [`time::precise_time_ns`](https://docs.rs/time/0.1.40/time/fn.precise_time_ns.html) is used,
    which uses a high-resolution performance counter in nanoseconds provided by the underlying platform.

### `stop`

Stops tracking time.
The metric value is set to the elapsed time.

{{#include ../../../shared/tab_header.md}}
<div data-lang="Kotlin" class="tab">

```Kotlin
import org.mozilla.yourApplication.GleanMetrics.Auth

fun onLogin() {
    Auth.loginTime.stop()
    // ...
}
```
</div>
<div data-lang="Java" class="tab">

```Java
import org.mozilla.yourApplication.GleanMetrics.Auth;

void onLogin() {
    Auth.INSTANCE.loginTime().stop();
    // ...
}
```
</div>
<div data-lang="Swift" class="tab">

```Swift
func onLogin() {
    Auth.loginTime.stop()
    // ...
}
```
</div>
<div data-lang="Python" class="tab">

```Python
from glean import load_metrics
metrics = load_metrics("metrics.yaml")

def on_login():
    metrics.auth.login_time.stop()
    # ...
```
</div>
<div data-lang="Rust" class="tab">

```Rust
use glean_metrics::auth;;

fn login() {
    auth::login_time.stop();
    // ...
}
```
</div>
<div data-lang="JavaScript" class="tab">

```js
import * as auth from "./path/to/generated/files/auth.js";

function onLogin() {
    auth.login_time.stop();
    // ...
}
```
</div>
<div data-lang="Firefox Desktop" class="tab">

**C++**
```c++
#include "mozilla/glean/PasswordmgrMetrics.h"
void OnLogin() {
  mozilla::glean::auth::login_time.Stop();
  // ...
}
```
**JavaScript**
```js
function onLogin() {
  Glean.auth.loginTime.stop();
  // ...
}
```
</div>
{{#include ../../../shared/tab_footer.md}}

#### Recorded errors

* [`invalid_state`](../../user/metrics/error-reporting.md):
  Calling `stop` without calling `start` first,
  e.g. if the `start` happened on a previous application run.

### `cancel`

Cancels a previous `start`.
No error is recorded if there was no previous `start`.

{{#include ../../../shared/tab_header.md}}
<div data-lang="Kotlin" class="tab">

```Kotlin
import org.mozilla.yourApplication.GleanMetrics.Auth

fun onLoginCancel() {
    Auth.loginTime.cancel()
    // ...
}
```
</div>
<div data-lang="Java" class="tab">

```Java
import org.mozilla.yourApplication.GleanMetrics.Auth;

void onLoginCancel() {
    Auth.INSTANCE.loginTime().cancel();
    // ...
}
```
</div>
<div data-lang="Swift" class="tab">

```Swift
func onLoginCancel() {
    Auth.loginTime.cancel()
    // ...
}
```
</div>
<div data-lang="Python" class="tab">

```Python
from glean import load_metrics
metrics = load_metrics("metrics.yaml")

def on_login_cancel():
    metrics.auth.login_time.cancel()
    # ...
```
</div>
<div data-lang="Rust" class="tab">

```Rust
use glean_metrics::auth;

fn login_cancel() {
    auth::login_time.cancel();
    // ...
}
```
</div>
<div data-lang="JavaScript" class="tab">

```js
import * as auth from "./path/to/generated/files/auth.js";

function onLoginCancel() {
    auth.login_time.cancel();
    // ...
}
```
</div>
<div data-lang="Firefox Desktop" class="tab">

**C++**
```c++
#include "mozilla/glean/PasswordmgrMetrics.h"
void OnLoginCancel() {
  mozilla::glean::auth::login_time.Cancel();
  // ...
}
```
**JavaScript**
```js
function onLoginCancel() {
  Glean.auth.loginTime.cancel();
  // ...
}
```
</div>
{{#include ../../../shared/tab_footer.md}}

### `measure`

Some languages support convenient auto timing of blocks of code.
`measure` is treated as a `start` and `stop` pair for the purposes of error recording.
Exceptions (if present in the language) are treated as a `cancel`.

{{#include ../../../shared/tab_header.md}}
<div data-lang="Kotlin" class="tab">

```Kotlin
import org.mozilla.yourApplication.GleanMetrics.Auth

Auth.loginTime.measure {
    // Process login flow
}
```
</div>
<div data-lang="Java" class="tab">

```Java
import org.mozilla.yourApplication.GleanMetrics.Auth

Auth.INSTANCE.loginTime().measure() -> {
    // Process login flow
    return null;
});
```
</div>
<div data-lang="Swift" class="tab">

```Swift
Auth.loginTime.measure {
    // Process login flow
}
```
</div>
<div data-lang="Python" class="tab">

```Python
from glean import load_metrics
metrics = load_metrics("metrics.yaml")

with metrics.auth.login_time.measure():
    # ... Do the login ...
```
</div>
<div data-lang="Rust" class="tab"></div>
<div data-lang="JavaScript" class="tab"></div>
<div data-lang="Firefox Desktop" class="tab"></div>

{{#include ../../../shared/tab_footer.md}}

### `setRawNanos`

Explicitly sets the timespan's value.

Regardless of the time unit chosen for the metric, this API expects the raw value to be in **nanoseconds**.

{{#include ../../../shared/blockquote-warning.html}}

## Only use this if you have to

> This API should only be used if the code being instrumented cannot make use of
> `start`, `stop`, and `cancel` or `measure`.
> Time is hard, and this API can't help you with it.

{{#include ../../../shared/tab_header.md}}
<div data-lang="Kotlin" class="tab">

```Kotlin
import org.mozilla.yourApplication.GleanMetrics.Auth

fun afterLogin(loginElapsedNs: Long) {
    Auth.loginTime.setRawNanos(loginElapsedNs)
    // ...
}
```
</div>
<div data-lang="Java" class="tab">

```Java
import org.mozilla.yourApplication.GleanMetrics.Auth;

void afterLogin(long loginElapsedNs) {
    Auth.INSTANCE.loginTime().setRawNanos(loginElapsedNs);
    // ...
}
```
</div>
<div data-lang="Swift" class="tab">

```Swift
func afterLogin(_ loginElapsedNs: UInt64) {
    Auth.loginTime.setRawNanos(loginElapsedNs)
    // ...
}
```
</div>
<div data-lang="Python" class="tab">

```Python
from glean import load_metrics
metrics = load_metrics("metrics.yaml")

def after_login(login_elapsed_ns):
    metrics.auth.login_time.set_raw_nanos(login_elapsed_ns)
    # ...
```
</div>
<div data-lang="Rust" class="tab">

```Rust
use std::time::duration;
use glean_metrics::auth;

fn after_login(login_elapsed: Duration) {
    auth::login_time.set_raw(login_elapsed);
    // ...
}
```
</div>
<div data-lang="JavaScript" class="tab">

```js
import * as auth from "./path/to/generated/files/auth.js";

function onAfterLogin(loginElapsedNs) {
    auth.loginTime.setRawNanos(loginElapsedNs);
    // ...
}
```
</div>
<div data-lang="Firefox Desktop" class="tab">

{{#include ../../../shared/blockquote-warning.html}}

## These are different

> Firefox Desktop's `setRaw` uses the units specified in the metric definition.
> e.g. if the Timespan's `time_unit` is `millisecond`,
> then the duration parameter is a count of milliseconds.

**C++**
```c++
#include "mozilla/glean/PasswordmgrMetrics.h"

void AfterLogin(uint32_t aDuration) {
  mozilla::glean::auth::login_time.SetRaw(aDuration);
  // ...
}
```
**JavaScript**
```js
function afterLogin(aDuration) {
  Glean.auth.loginTime.setRaw(aDuration);
  // ...
}
```
</div>
{{#include ../../../shared/tab_footer.md}}

#### Recorded errors

* [`invalid_value`](../../user/metrics/error-reporting.md): if attempting to record a negative elapsed duration.
* [`invalid_state`](../../user/metrics/error-reporting.md): if this method is called after calling `start` or this method is called multiple times.
* [`invalid_type`](../../user/metrics/error-reporting.md): if a negative, floating point or non-number value is given.

## Testing API

### `testGetValue`

Get the currently-stored value.  
Returns the timespan as a integer in the metric's time unit if data is stored.  
Returns a language-specific empty/null value if no data is stored.
Has an optional argument to specify the name of the ping you wish to retrieve data from, except
in Rust where it's required. `None` or no argument will default to the first value found for `send_in_pings`.

{{#include ../../../shared/tab_header.md}}
<div data-lang="Kotlin" class="tab">

```Kotlin
import org.mozilla.yourApplication.GleanMetrics.Auth

assertTrue(Auth.loginTime.testGetValue() > 0)
```
</div>
<div data-lang="Java" class="tab">

```Java
import org.mozilla.yourApplication.GleanMetrics.Auth;

assertTrue(Auth.INSTANCE.loginTime().testGetValue() > 0);
```
</div>
<div data-lang="Swift" class="tab">

```Swift
XCTAssert(Auth.loginTime.testGetValue() > 0)
```
</div>
<div data-lang="Python" class="tab">

```Python
from glean import load_metrics
metrics = load_metrics("metrics.yaml")

assert metrics.auth.login_time.test_get_value() > 0
```
</div>
<div data-lang="Rust" class="tab">

```Rust
use glean_metrics::auth;

assert!(auth::login_time.test_get_value(None).unwrap() > 0);
```
</div>
<div data-lang="JavaScript" class="tab">

```js
import * as auth from "./path/to/generated/files/auth.js";

assert(await auth.loginTime.testGetValue() > 0);
```
</div>
<div data-lang="Firefox Desktop" class="tab">

**C++**
```c++
#include "mozilla/glean/PasswordmgrMetrics.h"

ASSERT_TRUE(mozilla::glean::auth::login_time.TestGetValue().isOk());
ASSERT_GE(mozilla::glean::auth::login_time.TestGetValue().unwrap().value(), 0);
```
**JavaScript**
```js
// testGetValue will throw NS_ERROR_LOSS_OF_SIGNIFICANT_DATA on error.
Assert.ok(Glean.auth.loginTime.testGetValue() > 0);
```
</div>
{{#include ../../../shared/tab_footer.md}}

### `testGetNumRecordedErrors`

Gets the number of errors recorded during operations on this metric.

{{#include ../../../shared/tab_header.md}}
<div data-lang="Kotlin" class="tab">

```Kotlin
import org.mozilla.yourApplication.GleanMetrics.Auth

assertEquals(
    0,
    Auth.loginTime.testGetNumRecordedErrors(ErrorType.INVALID_VALUE)
)
```

</div>
<div data-lang="Java" class="tab">

```Java
import org.mozilla.yourApplication.GleanMetrics.Auth;

assertEquals(
    0,
    Auth.INSTANCE.loginTime().testGetNumRecordedErrors(ErrorType.INVALID_VALUE)
);
```

</div>
<div data-lang="Swift" class="tab">

```Swift
XCTAssertEqual(0, Auth.loginTime.testGetNumRecordedErrors(.invalidValue))
```

</div>
<div data-lang="Python" class="tab">

```Python
from glean import load_metrics
metrics = load_metrics("metrics.yaml")

assert 0 == metrics.auth.local_time.test_get_num_recorded_errors(
    ErrorType.INVALID_VALUE
)
```

</div>
<div data-lang="Rust" class="tab">

```Rust
use glean_metrics::auth;

assert_eq!(1, auth::login_time.test_get_num_recorded_errors(ErrorType::InvalidValue));
```

</div>
<div data-lang="JavaScript" class="tab">

```js
import * as auth from "./path/to/generated/files/auth.js";
import { ErrorType } from "@mozilla/glean/error";;

assert.strictEqual(
  1,
  await auth.loginTime.testGetNumRecordedErrors(ErrorType.InvalidValue)
);
```
</div>
<div data-lang="Firefox Desktop" class="tab" data-info="Firefox Desktop uses testGetValue to communicate errors"></div>

{{#include ../../../shared/tab_footer.md}}

## Metric parameters

Example timespan metric definition:

```YAML
auth:
  login_time:
    type: timespan
    description: >
      Measures the time spent logging in.
    time_unit: millisecond
    bugs:
      - https://bugzilla.mozilla.org/show_bug.cgi?id=000000
    data_reviews:
      - https://bugzilla.mozilla.org/show_bug.cgi?id=000000#c3
    notification_emails:
      - me@mozilla.com
    expires: 2020-01-01
    data_sensitivity:
      - interaction
```

For a full reference on metrics parameters common to all metric types,
refer to the [metrics YAML registry format](../yaml/metrics.md) reference page.

### Extra metric parameters

#### `time_unit`

Timespans have an optional `time_unit` parameter to specify the smallest unit of resolution that the timespan will record.
The allowed values for `time_unit` are:

* `nanosecond`
* `microsecond`
* `millisecond` (default)
* `second`
* `minute`
* `hour`
* `day`

Consider the resolution that is required by your metric,
and use the largest possible value that will provide useful information so as to not leak too much fine-grained information from the client.

{{#include ../../../shared/blockquote-warning.html}}

## Values are truncated

> It is important to note that the value sent in the ping is truncated down to the nearest unit.
> Therefore, a measurement of 500 nanoseconds will be truncated to 0 microseconds.

## Data questions

* How long did it take for the user to log in?

## Reference

* [Python API docs](../../../python/glean/metrics/index.html#glean.metrics.TimespanMetricType)
* [Rust API docs](../../../docs/glean/private/struct.TimespanMetric.html)
* [Swift API docs](../../../swift/Classes/TimespanMetric.html)
