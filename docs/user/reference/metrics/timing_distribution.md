# Timing Distribution

Timing distributions are used to accumulate and store time measurement, for analyzing distributions of the timing data.

To measure the distribution of single timespans, see [Timespans](timespan.md).
To record absolute times, see [Datetimes](datetime.md).

Timing distributions are recorded in a histogram where the buckets have an exponential distribution, specifically with 8 buckets for every power of 2.
That is, the function from a value \\( x \\) to a bucket index is:

\\[ \lfloor 8 \log_2(x) \rfloor \\]

This makes them suitable for measuring timings on a number of time scales without any configuration.

> **Note** Check out how this bucketing algorithm would behave on the [Simulator](#simulator).

Timings always span the full length between `start` and `stopAndAccumulate`.
If the Glean upload is disabled when calling `start`, the timer is still started.
If the Glean upload is disabled at the time `stopAndAccumulate` is called, nothing is recorded.

Multiple concurrent timings in different threads may be measured at the same time.

Timings are always stored and sent in the payload as nanoseconds. However, the `time_unit` parameter
controls the minimum and maximum values that will recorded:

  - `nanosecond`: 1ns <= x <= 10 minutes
  - `microsecond`: 1μs <= x <= ~6.94 days
  - `millisecond`: 1ms <= x <= ~19 years

Overflowing this range is considered an error and is reported through the error reporting mechanism.
Underflowing this range is not an error and the value is silently truncated to the minimum value.

Additionally, when a metric comes from GeckoView (the `geckoview_datapoint` parameter is present),
the `time_unit` parameter specifies the unit that the samples are in when passed to Glean.
Glean will convert all of the incoming samples to nanoseconds internally.

## Recording API

### `start`

Start tracking time for the provided metric.
Multiple timers can run simultaneously.
Returns a unique `TimerId` for the new timer.

{{#include ../../../shared/tab_header.md}}

<div data-lang="Kotlin" class="tab">

```Kotlin
import mozilla.components.service.glean.GleanTimerId
import org.mozilla.yourApplication.GleanMetrics.Pages

val timerId : GleanTimerId

fun onPageStart(e: Event) {
    timerId = Pages.pageLoad.start()
}
```

</div>
<div data-lang="Java" class="tab">

```Java
import mozilla.components.service.glean.GleanTimerId;
import org.mozilla.yourApplication.GleanMetrics.Pages;

GleanTimerId timerId;

void onPageStart(Event e) {
    timerId = Pages.INSTANCE.pageLoad().start();
}
```

</div>
<div data-lang="Swift" class="tab">

```Swift
import Glean

var timerId : GleanTimerId

func onPageStart() {
    timerId = Pages.pageLoad.start()
}
```

</div>
<div data-lang="Python" class="tab">

```Python
from glean import load_metrics
metrics = load_metrics("metrics.yaml")

class PageHandler:
    def __init__(self):
        self.timer_id = None

    def on_page_start(self, event):
        self.timer_id = metrics.pages.page_load.start()
```

</div>
<div data-lang="Rust" class="tab">

```Rust
use glean_metrics::pages;

fn on_page_start() {
    self.timer_id = pages::page_load.start();
}
```

</div>
<div data-lang="JavaScript" class="tab">

```Javascript
import * as pages from "./path/to/generated/files/pages.js";

function onPageStart() {
    // store this ID, you will need it later to stop or cancel your timer
    const timerId = pages.pageLoad.start();
}
```

</div>
<div data-lang="Firefox Desktop" class="tab">

**C++**

```c++
#include "mozilla/glean/DomMetrics.h"

auto timerId = mozilla::glean::pages::page_load.Start();
```

**JavaScript**

```js
let timerId = Glean.pages.pageLoad.start();
```

</div>

{{#include ../../../shared/tab_footer.md}}

### `stopAndAccumulate`

Stops tracking time for the provided metric and associated timer id.

Adds a count to the corresponding bucket in the timing distribution.
This will record an error if `start` was not called.

{{#include ../../../shared/tab_header.md}}

<div data-lang="Kotlin" class="tab">

```Kotlin
import org.mozilla.yourApplication.GleanMetrics.Pages

fun onPageLoaded(e: Event) {
    Pages.pageLoad.stopAndAccumulate(timerId)
}
```

</div>
<div data-lang="Java" class="tab">

```Java
import org.mozilla.yourApplication.GleanMetrics.Pages;

void onPageLoaded(Event e) {
    Pages.INSTANCE.pageLoad().stopAndAccumulate(timerId);
}
```

</div>
<div data-lang="Swift" class="tab">

```Swift
import Glean

func onPageLoaded() {
    Pages.pageLoad.stopAndAccumulate(timerId)
}
```

</div>
<div data-lang="Python" class="tab">

```Python
from glean import load_metrics
metrics = load_metrics("metrics.yaml")

class PageHandler:
    def on_page_loaded(self, event):
        metrics.pages.page_load.stop_and_accumulate(self.timer_id)
```

</div>
<div data-lang="Rust" class="tab">

```Rust
use glean_metrics::pages;

fn on_page_loaded() {
    pages::page_load.stop_and_accumulate(self.timer_id);
}
```

</div>
<div data-lang="JavaScript" class="tab">

```Javascript
import * as pages from "./path/to/generated/files/pages.js";

function onPageLoaded() {
    pages.pageLoad.stopAndAccumulate(timerId);
}
```

</div>
<div data-lang="Firefox Desktop" class="tab">

**C++**

```c++
#include "mozilla/glean/DomMetrics.h"

mozilla::glean::pages::page_load.StopAndAccumulate(std::move(timerId));
```

**JavaScript**

```js
Glean.pages.pageLoad.stopAndAccumulate(timerId);
```

</div>

{{#include ../../../shared/tab_footer.md}}

### `accumulateSamples`

Accumulates the provided signed samples in the metric.
This is required so that the platform-specific code can provide us with
64 bit signed integers if no `u64` comparable type is available. This
will take care of filtering and reporting errors for any provided negative
sample.

Please note that this assumes that the provided samples are already in
the "unit" declared by the instance of the metric type (e.g. if the
instance this method was called on is using `TimeUnit::Second`, then
`samples` are assumed to be in that unit).

{{#include ../../../shared/tab_header.md}}

<div data-lang="Kotlin" class="tab">

```Kotlin
import org.mozilla.yourApplication.GleanMetrics.Pages

fun onPageLoaded(e: Event) {
    Pages.pageLoad.accumulateSamples(samples)
}
```

</div>
<div data-lang="Java" class="tab">

```Java
import org.mozilla.yourApplication.GleanMetrics.Pages;

void onPageLoaded(Event e) {
    Pages.INSTANCE.pageLoad().accumulateSamples(samples);
}
```

</div>
<div data-lang="Swift" class="tab"></div>
<div data-lang="Python" class="tab">

```Python
from glean import load_metrics
metrics = load_metrics("metrics.yaml")

class PageHandler:
    def on_page_loaded(self, event):
        metrics.pages.page_load.accumulate_samples(samples)
```

</div>
<div data-lang="Rust" class="tab">

```Rust
use glean_metrics::pages;

fn on_page_loaded() {
    pages::page_load.accumulate_samples(samples);
}
```

</div>
<div data-lang="JavaScript" class="tab">

```Javascript
import * as pages from "./path/to/generated/files/pages.js";

function onPageLoaded() {
    pages.pageLoad.accumulateSamples(samples);
}
```

</div>
<div data-lang="Firefox Desktop" class="tab">

**C++**

```c++
#include "mozilla/glean/DomMetrics.h"

mozilla::glean::pages::page_load.AccumulateRawSamples(samples);
```

**JavaScript**

```js
Glean.pages.pageLoad.accumulateSamples(samples);
```

</div>

{{#include ../../../shared/tab_footer.md}}

#### Limits

- Samples are limited to the maximum value for the given time unit.
- Only non-negative values may be recorded (`>= 0`).
- Negative values are discarded and an `ErrorType::InvalidValue` is generated for each instance.
- Samples that are longer than maximum sample time for the given unit generate an `ErrorType::InvalidOverflow` error for each instance.

#### Recorded errors

* [`invalid_value`](../../user/metrics/error-reporting.md): If recording a negative timespan.
* [`invalid_state`](../../user/metrics/error-reporting.md): If a non-existing/stopped timer is stopped again.
* [`invalid_overflow`](../../user/metrics/error-reporting.md): If recording a time longer than the maximum for the given unit.

### `accumulateSingleSample`

Accumulates a single signed sample and appends it to the metric. Prefer this
for the common use case of having a single value to avoid having to pass
a collection over a foreign language interface.

A signed value is required so that the platform-specific code can provide
us with a 64 bit signed integer if no `u64` comparable type is available.
This will take care of filtering and reporting errors for a negative
sample.

Please note that this assumes that the provided sample is already in
the "unit" declared by the instance of the metric type (e.g. if the
instance this method was called on is using `TimeUnit::Second`, then
`sample` is assumed to be in that unit).

{{#include ../../../shared/tab_header.md}}

<div data-lang="Kotlin" class="tab">

```Kotlin
import org.mozilla.yourApplication.GleanMetrics.Pages

fun onPageLoaded(e: Event) {
    Pages.pageLoad.accumulateSingleSample(sample)
}
```

</div>
<div data-lang="Java" class="tab">

```Java
import org.mozilla.yourApplication.GleanMetrics.Pages;

void onPageLoaded(Event e) {
    Pages.INSTANCE.pageLoad().accumulateSingleSample(sample);
}
```

</div>
<div data-lang="Swift" class="tab"></div>
<div data-lang="Python" class="tab">

```Python
from glean import load_metrics
metrics = load_metrics("metrics.yaml")

class PageHandler:
    def on_page_loaded(self, event):
        metrics.pages.page_load.accumulate_single_sample(sample)
```

</div>
<div data-lang="Rust" class="tab">

```Rust
use glean_metrics::pages;

fn on_page_loaded() {
    pages::page_load.accumulate_single_sample(sample);
}
```

</div>
<div data-lang="JavaScript" class="tab">

```Javascript
import * as pages from "./path/to/generated/files/pages.js";

function onPageLoaded() {
    pages.pageLoad.accumulateSingleSample(sample);
}
```

</div>
<div data-lang="Firefox Desktop" class="tab">

**C++**

```c++
#include "mozilla/glean/DomMetrics.h"

mozilla::glean::pages::page_load.AccumulateRawDuration(aDuration);
```

**JavaScript**

```js
Glean.pages.pageLoad.accumulateSingleSample(sample);
```

</div>

{{#include ../../../shared/tab_footer.md}}

#### Limits

- Samples are limited to the maximum value for the given time unit.
- Only non-negative values may be recorded (`>= 0`).
- Negative values are discarded and an `ErrorType::InvalidValue` is generated for each instance.
- Samples that are longer than maximum sample time for the given unit generate an `ErrorType::InvalidOverflow` error for each instance.

#### Recorded errors

* [`invalid_value`](../../user/metrics/error-reporting.md): If recording a negative timespan.
* [`invalid_state`](../../user/metrics/error-reporting.md): If a non-existing/stopped timer is stopped again.
* [`invalid_overflow`](../../user/metrics/error-reporting.md): If recording a time longer than the maximum for the given unit.

### `measure`

For convenience one can measure the time of a function or block of code.

{{#include ../../../shared/tab_header.md}}

<div data-lang="Kotlin" class="tab">

```Kotlin
import org.mozilla.yourApplication.GleanMetrics.Pages

Pages.pageLoad.measure {
    // Load a page
}
```

</div>
<div data-lang="Java" class="tab"></div>
<div data-lang="Swift" class="tab">

```Swift
import Glean

Pages.pageLoad.measure {
    // Load a page
}
```

</div>
<div data-lang="Python" class="tab">

```Python
from glean import load_metrics
metrics = load_metrics("metrics.yaml")

with metrics.pages.page_load.measure():
    # Load a page
```

</div>
<div data-lang="Rust" class="tab"></div>
<div data-lang="JavaScript" class="tab"></div>
<div data-lang="Firefox Desktop" class="tab">

**C++**

```c++
#include "mozilla/glean/DomMetrics.h"

{ // Scope for RAII
  auto timer = mozilla::glean::pages::page_load.Measure();
  // load the page
}
```

**JavaScript**

*Not currently implemented.*

</div>

{{#include ../../../shared/tab_footer.md}}

### `cancel`

Aborts a previous `start` call.
No error is recorded if `start` was not called.

{{#include ../../../shared/tab_header.md}}

<div data-lang="Kotlin" class="tab">

```Kotlin
import org.mozilla.yourApplication.GleanMetrics.Pages

fun onPageError(e: Event) {
    Pages.pageLoad.cancel(timerId)
}
```

</div>
<div data-lang="Java" class="tab">

```Java
import org.mozilla.yourApplication.GleanMetrics.Pages;

fun onPageError(e: Event) {
    Pages.INSTANCE.pageLoad().cancel(timerId);
}
```

</div>
<div data-lang="Swift" class="tab">

```Swift
import Glean

func onPageError() {
    Pages.pageLoad.cancel(timerId)
}
```

</div>
<div data-lang="Python" class="tab">

```Python
from glean import load_metrics
metrics = load_metrics("metrics.yaml")

class PageHandler:
    def on_page_error(self, event):
        metrics.pages.page_load.cancel(self.timer_id)
```

</div>
<div data-lang="Rust" class="tab">

```Rust
use glean_metrics::pages;

fn on_page_error() {
    pages::page_load.cancel(self.timer_id);
}
```

</div>
<div data-lang="JavaScript" class="tab">

```Javascript
import * as pages from "./path/to/generated/files/pages.js";

function onPageError() {
    pages.pageLoad.cancel(timerId);
}
```

</div>
<div data-lang="Firefox Desktop" class="tab">

**C++**

```c++
#include "mozilla/glean/DomMetrics.h"

mozilla::glean::pages::page_load.Cancel(std::move(timerId));
```

**JavaScript**

```js
Glean.pages.pageLoad.cancel(timerId);
```

</div>

{{#include ../../../shared/tab_footer.md}}

### `startBuffer`

**Experimental:**
Start a new histogram buffer associated with this custom distribution metric.

A histogram buffer accumulates in-memory.
Data is recorded into the metric when committed.
No data is collected if the buffer is abandoned.

{{#include ../../../shared/tab_header.md}}

<div data-lang="Kotlin" class="tab"></div>
<div data-lang="Java" class="tab"></div>
<div data-lang="Swift" class="tab"></div>
<div data-lang="Python" class="tab"></div>
<div data-lang="Rust" class="tab">

Data is automatically committed on drop.

```Rust
use glean_metrics::pages;

let buffer = pages::page_load.start_buffer();

for sample in load_times.iter() {
  buffer.accumulate(sample);
}

// Explicit or implicit drop of `buffer` commits the data.
drop(buffer);
```

No data is recorded when the buffer is abandoned.

```Rust
use glean_metrics::pages;

let buffer = pages::page_load.start_buffer();

for sample in all_frames.iter() {
  buffer.accumulate(sample);
}

buffer.abandon(); // No data is recorded.
```

</div>
<div data-lang="JavaScript" class="tab"></div>
<div data-lang="Firefox Desktop" class="tab"></div>

{{#include ../../../shared/tab_footer.md}}

#### Limits

- Samples are limited to the maximum value for the given time unit.
- Samples that are longer than maximum sample time for the given unit generate an `ErrorType::InvalidOverflow` error for each instance.

#### Recorded errors

* [`invalid_overflow`](../../user/metrics/error-reporting.md): If recording a time longer than the maximum for the given unit.

## Testing API

### `testGetValue`

Gets the recorded value for a given timing distribution metric.  
Returns a struct with counts per buckets and total sum if data is stored.  
Returns a language-specific empty/null value if no data is stored.
Has an optional argument to specify the name of the ping you wish to retrieve data from, except
in Rust where it's required. `None` or no argument will default to the first value found for `send_in_pings`.

{{#include ../../../shared/tab_header.md}}

<div data-lang="Kotlin" class="tab">

```Kotlin
import org.mozilla.yourApplication.GleanMetrics.Pages

// Get snapshot.
val snapshot = Pages.pageLoad.testGetValue()

// Does the sum have the expected value?
assertEquals(11, snapshot.sum)

// Usually you don't know the exact timing values,
// but how many should have been recorded.
assertEquals(2L, snapshot.count)
```

</div>
<div data-lang="Java" class="tab">

```Java
import org.mozilla.yourApplication.GleanMetrics.Pages;

// Get snapshot.
DistributionData snapshot = pages.INSTANCE.pageLoad().testGetValue();

// Does the sum have the expected value?
assertEquals(11, snapshot.sum);

// Usually you don't know the exact timing values,
// but how many should have been recorded.
assertEquals(2L, snapshot.getCount());
```

</div>
<div data-lang="Swift" class="tab">

```Swift
// Get snapshot.
let snapshot = pages.pageLoad.testGetValue()

// Does the sum have the expected value?
XCTAssertEqual(11, snapshot.sum)

// Usually you don't know the exact timing values,
// but how many should have been recorded.
XCTAssertEqual(2, snapshot.count)
```

</div>
<div data-lang="Python" class="tab">

```Python
from glean import load_metrics
metrics = load_metrics("metrics.yaml")

# Get snapshot.
snapshot = metrics.pages.page_load.test_get_value()

# Does the sum have the expected value?
assert 11 == snapshot.sum

# Usually you don't know the exact timing values,
# but how many should have been recorded.
assert 2 == snapshot.count
```

</div>
<div data-lang="Rust" class="tab">

```Rust
use glean::ErrorType;
use glean_metrics::pages;

// Get snapshot
let snapshot = pages::page_load.test_get_value(None).unwrap();

// Does the sum have the expected value?
assert_eq!(11, snapshot.sum);

// Usually you don't know the exact timing values,
// but how many should have been recorded.
assert_eq!(2, snapshot.count);
```


</div>
<div data-lang="JavaScript" class="tab">

```Javascript
import * as pages from "./path/to/generated/files/pages.js";

const snapshot = await pages.pageLoad.testGetValue();

// Usually you don't know the exact timing values,
// but how many should have been recorded.
assert.equal(1, snapshot.count);
```

</div>
<div data-lang="Firefox Desktop" class="tab">

**C++**

```c++
#include "mozilla/glean/DomMetrics.h"

// Does it have an expected values?
const data = mozilla::glean::pages::page_load.TestGetValue().value().unwrap();
ASSERT_TRUE(data.sum > 0);
```

**JavaScript**

```js
Assert.ok(Glean.pages.pageLoad.testGetValue().sum > 0);
```

</div>

{{#include ../../../shared/tab_footer.md}}

### `testGetNumRecordedErrors`

Gets the number of errors recorded for a given timing distribution metric.

{{#include ../../../shared/tab_header.md}}

<div data-lang="Kotlin" class="tab">

```Kotlin
import org.mozilla.yourApplication.GleanMetrics.Pages

// Assert that no errors were recorded.
assertEquals(
    0,
    Pages.pageLoad.testGetNumRecordedErrors(ErrorType.INVALID_VALUE)
)
```

</div>
<div data-lang="Java" class="tab">

```Java
import org.mozilla.yourApplication.GleanMetrics.Pages;

// Assert that no errors were recorded.
assertEquals(
    0,
    Pages.INSTANCE.pageLoad().testGetNumRecordedErrors(ErrorType.INVALID_VALUE)
);
```

</div>
<div data-lang="Swift" class="tab">

```Swift
// Assert that no errors were recorded.
XCTAssertEqual(0, Pages.pageLoad.testGetNumRecordedErrors(.invalidValue))
```

</div>
<div data-lang="Python" class="tab">

```Python
from glean import load_metrics
metrics = load_metrics("metrics.yaml")

# Assert that no errors were recorded.
assert 0 == metrics.pages.page_load.test_get_num_recorded_errors(
    ErrorType.INVALID_VALUE
)
```

</div>
<div data-lang="Rust" class="tab">

```Rust
use glean::ErrorType;
use glean_metrics::pages;

assert_eq!(
    0,
    pages::page_load.test_get_num_recorded_errors(ErrorType::InvalidValue)
);
```

</div>
<div data-lang="JavaScript" class="tab">

```Javascript
import * as pages from "./path/to/generated/files/pages.js";
import { ErrorType } from "@mozilla/glean/<platform>";

assert.equal(1, await pages.pageLoad.testGetNumRecordedErrors(ErrorType.InvalidValue));
```

</div>
<div data-lang="Firefox Desktop" class="tab"></div>

{{#include ../../../shared/tab_footer.md}}

## Metric parameters

Example timing distribution metric definition:

```YAML
pages:
  page_load:
    type: timing_distribution
    time_unit: millisecond
    description: >
      Counts how long each page takes to load
    bugs:
      - https://bugzilla.mozilla.org/000000
    data_reviews:
      - https://bugzilla.mozilla.org/show_bug.cgi?id=000000#c3
    notification_emails:
      - me@mozilla.com
    expires: 2020-10-01
```

### Extra metric parameters

#### `time_unit`

Timing distributions have an optional `time_unit` parameter to specify the smallest unit of resolution that the timespan will record.
The allowed values for `time_unit` are:

* `nanosecond` (default)
* `microsecond`
* `millisecond`
* `second`
* `minute`
* `hour`
* `day`

## Limits

* Timings are recorded in nanoseconds.
  * On Android, the [`SystemClock.elapsedRealtimeNanos()`](https://developer.android.com/reference/android/os/SystemClock.html#elapsedRealtimeNanos()) function is used,
    so it is limited by the accuracy and performance of that timer. The time measurement includes time spent in sleep.

  * On iOS, the [`mach_absolute_time`](https://developer.apple.com/library/archive/documentation/Darwin/Conceptual/KernelProgramming/services/services.html) function is used,
    so it is limited by the accuracy and performance of that timer.
    The time measurement does not include time spent in sleep.

  * On Python 3.7 and later, [`time.monotonic_ns()`](https://docs.python.org/3/library/time.html#time.monotonic_ns) is used.
    On earlier versions of Python, [`time.monotonics()`](https://docs.python.org/3/library/time.html#time.monotonic) is used,
    which is not guaranteed to have nanosecond resolution.
  * In Rust, [`time::precise_time_ns()`](https://docs.rs/time/0.1.42/time/fn.precise_time_ns.html) is used.

* The maximum timing value that will be recorded depends on the `time_unit` parameter:

  - `nanosecond`: 1ns <= x <= 10 minutes
  - `microsecond`: 1μs <= x <= ~6.94 days
  - `millisecond`: 1ms <= x <= ~19 years

  Longer times will be truncated to the maximum value and an error will be recorded.

## Data questions

* How long does it take a page to load?

## Reference

* [Python API docs](../../../python/glean/metrics/index.html#glean.metrics.TimingDistributionMetricType)
* [Rust API docs](../../../docs/glean/private/struct.TimingDistributionMetric.html)

## Simulator

<div id="custom-data-modal-overlay">
    <div id="custom-data-modal">
        <p>Please, insert your custom data below as a JSON array.</p>
        <textarea rows="30"></textarea>
    </div>
</div>

<div id="simulator-container">
    <div id="histogram-chart-container">
        <div id="histogram-chart"></div>
        <p id="histogram-chart-legend"><p>
    </div>
    <div id="data-options">
        <h3>Data options</h3>
        <div class="input-group">
            <label for="normally-distributed">Generate normally distributed data</label>
            <input name="data-options" value="normally-distributed" id="normally-distributed" type="radio" />
        </div>
        <div class="input-group">
            <label for="log-normally-distributed">Generate log-normally distributed data</label>
            <input name="data-options" value="log-normally-distributed" id="log-normally-distributed" type="radio" checked />
        </div>
        <div class="input-group">
            <label for="uniformly-distributed">Generate uniformly distributed data</label>
            <input name="data-options" value="uniformly-distributed" id="uniformly-distributed" type="radio" />
        </div>
        <div class="input-group" id="custom-data-input-group">
            <label for="custom">Use custom data</label>
            <input name="data-options" value="custom" id="custom" type="radio" />
        </div>
    </div>
    <div id="histogram-props">
        <h3>Properties</h3>
        <div class="input-group hide">
            <label for="kind">Histogram type</label>
            <select id="kind" name="kind" disabled>
                <option value="functional" selected>Functional</option>
            </select>
        </div>
        <div class="input-group hide">
            <label for="log-base">Log base</label>
            <input id="log-base" name="log-base" type="number" value="2" disabled />
        </div>
        <div class="input-group hide">
            <label for="buckets-per-magnitude">Buckets per magnitude</label>
            <input id="buckets-per-magnitude" name="buckets-per-magnitude" type="number" value="8" disabled />
        </div>
        <div class="input-group hide">
            <label for="maximum-value">Maximum value</label>
            <input id="maximum-value" name="maximum-value" type="number" value="600000000000" disabled />
        </div>
        <div class="input-group">
            <label for="time-unit">Time unit (<code>time_unit</code>)</label>
            <select id="time-unit" name="time-unit">
                <option value="nanoseconds" selected>Nanoseconds</option>
                <option value="microseconds">Microseconds</option>
                <option value="milliseconds">Milliseconds</option>
            </select>
        </div>
    </div>
</div>

> **Note** The data _provided_, is assumed to be in the configured time unit. The data _recorded_, on the other hand, is always in **nanoseconds**.
> This means that, if the configured time unit is not `nanoseconds`, the data will be transformed before being recorded. Notice this, by using the select field above to change the time unit and see the mean of the data recorded changing.

