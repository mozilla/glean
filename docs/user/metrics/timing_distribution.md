# Timing Distribution

Timing distributions are used to accumulate and store time measurement, for analyzing distributions of the timing data.

To measure the distribution of single timespans, see [Timespans](timespan.md). To record absolute times, see [Datetimes](datetime.md).

Timing distributions are recorded in a histogram where the buckets have an exponential distribution, specifically with 8 buckets for every power of 2.
That is, the function from a value \\( x \\) to a bucket index is:

\\[ \lfloor 8 \log_2(x) \rfloor \\]

This makes them suitable for measuring timings on a number of time scales without any configuration.

> **Note** Check out how this bucketing algorithm would behave on the [Simulator](#simulator)

Timings always span the full length between `start` and `stopAndAccumulate`.
If the Glean upload is disabled when calling `start`, the timer is still started.
If the Glean upload is disabled at the time `stopAndAccumulate` is called, nothing is recorded.

Multiple concurrent timespans in different threads may be measured at the same time.

Timings are always stored and sent in the payload as nanoseconds. However, the `time_unit` parameter
controls the minimum and maximum values that will recorded:

  - `nanosecond`: 1ns <= x <= 10 minutes
  - `microsecond`: 1μs <= x <= ~6.94 days
  - `millisecond`: 1ms <= x <= ~19 years

Overflowing this range is considered an error and is reported through the error reporting mechanism. Underflowing this range is not an error and the value is silently truncated to the minimum value.

Additionally, when a metric comes from GeckoView (the `geckoview_datapoint` parameter is present), the `time_unit` parameter specifies the unit that the samples are in when passed to Glean. Glean will convert all of the incoming samples to nanoseconds internally.

## Configuration

If you wanted to create a timing distribution to measure page load times, first you need to add an entry for it to the `metrics.yaml` file:

```YAML
pages:
  page_load:
    type: timing_distribution
    description: >
      Counts how long each page takes to load
    ...
```

## API

Now you can use the timing distribution from the application's code.
Starting a timer returns a timer ID that needs to be used to stop or cancel the timer at a later point.
Multiple intervals can be measured concurrently.
For example, to measure page load time on a number of tabs that are loading at the same time, each tab object needs to store the running timer ID.

{{#include ../../tab_header.md}}

<div data-lang="Kotlin" class="tab">

```Kotlin
import mozilla.components.service.glean.GleanTimerId
import org.mozilla.yourApplication.GleanMetrics.Pages

val timerId : GleanTimerId

fun onPageStart(e: Event) {
    timerId = Pages.pageLoad.start()
}

fun onPageLoaded(e: Event) {
    Pages.pageLoad.stopAndAccumulate(timerId)
}
```

There are test APIs available too.  For convenience, properties `sum` and `count` are exposed to facilitate validating that data was recorded correctly.

Continuing the `pageLoad` example above, at this point the metric should have a `sum == 11` and a `count == 2`:

```Kotlin
import org.mozilla.yourApplication.GleanMetrics.Pages

// Was anything recorded?
assertTrue(Pages.pageLoad.testHasValue())

// Get snapshot.
val snapshot = Pages.pageLoad.testGetValue()

// Usually you don't know the exact timing values, but how many should have been recorded.
assertEquals(1L, snapshot.count)

// Assert that no errors were recorded.
assertEquals(0, Pages.pageLoad.testGetNumRecordedErrors(ErrorType.InvalidValue))
```

</div>

<div data-lang="Java" class="tab">

```Java
import mozilla.components.service.glean.GleanTimerId;
import org.mozilla.yourApplication.GleanMetrics.Pages;

GleanTimerId timerId;

void onPageStart(Event e) {
    timerId = Pages.INSTANCE.pageLoad.start();
}

void onPageLoaded(Event e) {
    Pages.INSTANCE.pageLoad.stopAndAccumulate(timerId);
}
```

There are test APIs available too.  For convenience, properties `sum` and `count` are exposed to facilitate validating that data was recorded correctly.

Continuing the `pageLoad` example above, at this point the metric should have a `sum == 11` and a `count == 2`:

```Java
import org.mozilla.yourApplication.GleanMetrics.Pages;

// Was anything recorded?
assertTrue(pages.INSTANCE.pageLoad.testHasValue());

// Get snapshot.
DistributionData snapshot = pages.INSTANCE.pageLoad.testGetValue();

// Usually you don't know the exact timing values, but how many should have been recorded.
assertEquals(1L, snapshot.getCount);

// Assert that no errors were recorded.
assertEquals(
    0,
    pages.INSTANCE.pageLoad.testGetNumRecordedErrors(
        ErrorType.InvalidValue
    )
);
```

</div>


<div data-lang="Swift" class="tab">

```Swift
import Glean

var timerId : GleanTimerId

func onPageStart() {
    timerId = Pages.pageLoad.start()
}

func onPageLoaded() {
    Pages.pageLoad.stopAndAccumulate(timerId)
}
```

There are test APIs available too.  For convenience, properties `sum` and `count` are exposed to facilitate validating that data was recorded correctly.

Continuing the `pageLoad` example above, at this point the metric should have a `sum == 11` and a `count == 2`:

```Swift
@testable import Glean

// Was anything recorded?
XCTAssert(pages.pageLoad.testHasValue())

// Get snapshot.
let snapshot = try! pages.pageLoad.testGetValue()

// Usually you don't know the exact timing values, but how many should have been recorded.
XCTAssertEqual(1, snapshot.count)

// Assert that no errors were recorded.
XCTAssertEqual(0, pages.pageLoad.testGetNumRecordedErrors(.invalidValue))
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
        # ...
        self.timer_id = metrics.pages.page_load.start()

    def on_page_loaded(self, event):
        # ...
        metrics.pages.page_load.stop_and_accumulate(self.timer_id)
```

The Python bindings also have a context manager for measuring time:

```Python
with metrics.pages.page_load.measure():
    # Load a page ...
```

There are test APIs available too.  For convenience, properties `sum` and `count` are exposed to facilitate validating that data was recorded correctly.

Continuing the `page_load` example above, at this point the metric should have a `sum == 11` and a `count == 2`:

```Python
# Was anything recorded?
assert metrics.pages.page_load.test_has_value()

# Get snapshot.
snapshot = metrics.pages.page_load.test_get_value()

# Usually you don't know the exact timing values, but how many should have been recorded.
assert 1 == snapshot.count

# Assert that no errors were recorded.
assert 0 == metrics.pages.page_load.test_get_num_recorded_errors(
    ErrorType.INVALID_VALUE
)
```

</div>

<div data-lang="C#" class="tab">

```C#
using static Mozilla.YourApplication.GleanMetrics.Pages;

var timerId;

void onPageStart(Event e) {
    timerId = Pages.pageLoad.Start();
}

void onPageLoaded(Event e) {
    Pages.pageLoad.StopAndAccumulate(timerId);
}
```

There are test APIs available too.  For convenience, properties `sum` and `count` are exposed to facilitate validating that data was recorded correctly.

Continuing the `pageLoad` example above, at this point the metric should have a `sum == 11` and a `count == 2`:

```C#
using static Mozilla.YourApplication.GleanMetrics.Pages;

// Was anything recorded?
Assert.True(Pages.pageLoad.TestHasValue());

// Get snapshot.
var snapshot = Pages.pageLoad.TestGetValue();

// Usually you don't know the exact timing values, but how many should have been recorded.
Assert.Equal(1, snapshot.Values.Count);

// Assert that no errors were recorded.
Assert.Equal(0, Pages.pageLoad.TestGetNumRecordedErrors(ErrorType.InvalidValue));
```

</div>

<div data-lang="Rust" class="tab">

```rust
use glean_metrics;

fn on_page_start() {
    self.timer_id = pages::page_load.start();
}

fn on_page_loaded() {
    pages::page_load.stop_and_accumulate(self.timer_id);
}
```

There are test APIs available too.

```rust
use glean::ErrorType;
use glean_metrics;

// Was anything recorded?
assert!(pages::page_load.test_get_value(None).is_some());

// Assert no errors were recorded.
let errors = [
    ErrorType::InvalidValue,
    ErrorType::InvalidState,
    ErrorType::InvalidOverflow
];
for error in errors {
    assert_eq!(0, pages::page_load.test_get_num_recorded_errors(error));
}
```

</div>

<div data-lang="C++" class="tab">

> **Note**: C++ APIs are only available in Firefox Desktop.

```c++
#include "mozilla/glean/GleanMetrics.h"

auto timerId = mozilla::glean::pages::page_load.Start();
PR_Sleep(PR_MillisecondsToInterval(10));
mozilla::glean::pages::page_load.StopAndAccumulate(timerId);
```

There are test APIs available too:

```c++
#include "mozilla/glean/GleanMetrics.h"

// Does it have an expected values?
const data = mozilla::glean::pages::page_load.TestGetValue().value();
ASSERT_TRUE(data.sum > 0);
// Did it run across any errors?
// TODO: https://bugzilla.mozilla.org/show_bug.cgi?id=1683171
```

</div>

<div data-lang="JS" class="tab">

> **Note**: JS APIs are only available in Firefox Desktop.

```js
const timerId = Glean.pages.pageLoad.start();
await sleep(10);
Glean.pages.pageLoad.stopAndAccumulate(timerId);
```

There are test APIs available too:

```js
Assert.ok(Glean.pages.pageLoad.testGetValue().sum > 0);
// Did it run across any errors?
// TODO: https://bugzilla.mozilla.org/show_bug.cgi?id=1683171
```

</div>

{{#include ../../tab_footer.md}}

## Limits

* Timings are recorded in nanoseconds.

  * On Android, the [`SystemClock.elapsedRealtimeNanos()`](https://developer.android.com/reference/android/os/SystemClock.html#elapsedRealtimeNanos()) function is used, so it is limited by the accuracy and performance of that timer. The time measurement includes time spent in sleep.

  * On iOS, the [`mach_absolute_time`](https://developer.apple.com/library/archive/documentation/Darwin/Conceptual/KernelProgramming/services/services.html) function is used,
    so it is limited by the accuracy and performance of that timer.
    The time measurement does not include time spent in sleep.

  * On Python 3.7 and later, [`time.monotonic_ns()`](https://docs.python.org/3/library/time.html#time.monotonic_ns) is used.  On earlier versions of Python, [`time.monotonics()`](https://docs.python.org/3/library/time.html#time.monotonic) is used, which is not guaranteed to have nanosecond resolution.

  * In Rust,
    [`time::precise_time_ns()`](https://docs.rs/time/0.1.42/time/fn.precise_time_ns.html)
    is used.


* The maximum timing value that will be recorded depends on the `time_unit` parameter:

  - `nanosecond`: 1ns <= x <= 10 minutes
  - `microsecond`: 1μs <= x <= ~6.94 days
  - `millisecond`: 1ms <= x <= ~19 years

  Longer times will be truncated to the maximum value and an error will be recorded.

## Examples

* How long does it take a page to load?

## Recorded errors

* `invalid_value`: If recording a negative timespan.
* `invalid_state`: If a non-existing/stopped timer is stopped again.
* `invalid_overflow`: If recording a time longer than the maximum for the given unit.

## Reference

* [Kotlin API docs](../../../javadoc/glean/mozilla.telemetry.glean.private/-timing-distribution-metric-type/index.html)
* [Swift API docs](../../../swift/Classes/TimingDistributionMetricType.html)
* [Python API docs](../../../python/glean/metrics/timing_distribution.html)

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

