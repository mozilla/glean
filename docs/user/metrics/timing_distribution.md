# Timing Distribution

Timing distributions are used to accumulate and store time measurement, for analyzing distributions of the timing data.

To measure the distribution of multiple timespans, see [Timing Distributions](timing_distribution.md). To record absolute times, see [Datetimes](datetime.md).

## Configuration

Timing distributions have a required `time_unit` parameter to specify the resolution and range of the values that are recorded. The allowed values for `time_unit` are:

  - `nanosecond`
  - `microsecond`
  - `millisecond`
  - `second`
  - `minute`
  - `hour`
  - `day`

Which range of values is recorded in detail depends on the `time_unit`, e.g. for milliseconds, all values greater 60000 are recorded as overflow values.

If you wanted to create a timing distribution to measure page load times, first you need to add an entry for it to the `metrics.yaml` file:

```YAML
pages:
  page_load:
    type: timing_distribution
    time_unit: millisecond
    description: >
      Counts how long each page takes to load
    ...
```

## API

Now you can use the timing distribution from the application's code.
Starting a timer returns a timer ID that needs to be used to stop or cancel the timer at a later point.
Multiple intervals can be measured concurrently.
For example, to measure page load time on a number of tabs that are loading at the same time, each tab object needs to store the running timer ID.

```Kotlin
import mozilla.components.service.glean.timing.GleanTimerId
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

// Get snapshot
val snapshot = pages.pageLoad.testGetValue()

// Does the sum have the expected value?
assertEquals(11, snapshot.sum)

// Usually you don't know the exact timing values, but how many should have been recorded.
assertEquals(2L, snapshot.count())
```

## Limits

* Which range of values is recorded in detail depends on the `time_unit`, e.g. for milliseconds, all values greater 60000 are recorded as overflow values.

## Examples

* How long does it take a page to load?

## Recorded errors

* `invalid_value`: If recording a negative timespan.
* `invalid_value`: If a non-existing/stopped timer is stopped again.

## Reference

* See [Kotlin API docs](../../../javadoc/glean/mozilla.telemetry.glean.private/-timing-distribution-metric-type/index.html)

 
