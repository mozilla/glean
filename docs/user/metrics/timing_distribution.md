# Timing Distribution

Timing distributions are used to accumulate and store time measurement, for analyzing distributions of the timing data.

To measure the distribution of single timespans, see [Timespans](timespan.md). To record absolute times, see [Datetimes](datetime.md).

Timing distributions are recorded in a histogram where the buckets have an exponential distribution, specifically with 8 buckets for every power of 2.
This makes them suitable for measuring timings on a number of time scales without any configuration.

Timings always span the full length between `start` and `stopAndAccumulate`.
If the Glean upload is disabled when calling `start`, the timer is still started.
If the Glean upload is disabled at the time `stopAndAccumulate` is called, nothing is recorded.

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

> Note: Timing distributions have an optional `time_unit` parameter that is only used when samples are provided directly from an external tool in a unit other than nanoseconds.

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
assertTrue(pages.pageLoad.testHasValue())

// Get snapshot.
val snapshot = pages.pageLoad.testGetValue()

// Does the sum have the expected value?
assertEquals(11, snapshot.sum)

// Usually you don't know the exact timing values, but how many should have been recorded.
assertEquals(2L, snapshot.count)

// Was an error recorded?
assertEquals(1, pages.pageLoad.testGetNumRecordedErrors(ErrorType.InvalidValue))
```

</div>

<div data-lang="Java" class="tab">

```Java
import mozilla.components.service.glean.GleanTimerId
import org.mozilla.yourApplication.GleanMetrics.Pages

val timerId : GleanTimerId

fun onPageStart(e: Event) {
    timerId = Pages.INSTANCE.pageLoad.start()
}

fun onPageLoaded(e: Event) {
    Pages.INSTANCE.pageLoad.stopAndAccumulate(timerId)
}
```

There are test APIs available too.  For convenience, properties `sum` and `count` are exposed to facilitate validating that data was recorded correctly.

Continuing the `pageLoad` example above, at this point the metric should have a `sum == 11` and a `count == 2`:

```Java
import org.mozilla.yourApplication.GleanMetrics.Pages

// Was anything recorded?
assertTrue(pages.INSTANCE.pageLoad.testHasValue())

// Get snapshot.
val snapshot = pages.INSTANCE.pageLoad.testGetValue()

// Does the sum have the expected value?
assertEquals(11, snapshot.getSum)

// Usually you don't know the exact timing values, but how many should have been recorded.
assertEquals(2L, snapshot.getCount)

// Was an error recorded?
assertEquals(
    1, 
    pages.INSTANCE.pageLoad.testGetNumRecordedErrors(
        ErrorType.InvalidValue
    )
)
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

// Does the sum have the expected value?
XCTAssertEqual(11, snapshot.sum)

// Usually you don't know the exact timing values, but how many should have been recorded.
XCTAssertEqual(2, snapshot.count)

// Was an error recorded?
XCTAssertEqual(1, pages.pageLoad.testGetNumRecordedErrors(.invalidValue))
```

</div>

{{#include ../../tab_footer.md}}

## Limits

* Timings are recorded in nanoseconds.
  On Android, the [`SystemClock.getElapsedNanos()`](https://developer.android.com/reference/android/os/SystemClock.html#elapsedRealtimeNanos()) function is used, so it is limited by the accuracy and performance of that timer.

* The maximum timing value that will be recorded is 10 minutes. Longer times will be truncated to 10 minutes and an error will be recorded.

## Examples

* How long does it take a page to load?

## Recorded errors

* `invalid_value`: If recording a negative timespan.
* `invalid_state`: If a non-existing/stopped timer is stopped again.
* `invalid_overflow`: If recording a time longer than 10 minutes.

## Reference

* [Kotlin API docs](../../../javadoc/glean/mozilla.telemetry.glean.private/-timing-distribution-metric-type/index.html)
* [Swift API docs](../../../swift/Classes/TimingDistributionMetricType.html)
