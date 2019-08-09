# Custom Distribution

Custom distributions are used to record the distribution of arbitrary values.

It should be used only when direct control over how the histogram buckets are computed is required.
Otherwise, look at the standard distribution metric types:

* [Timing Distributions](timing_distribution.md))

> Note: Custom distributions are currently only allowed for GeckoView metrics (the `gecko_datapoint` parameter is present).

## Configuration

Custom distributions have the following required parameters:

  - `range_min`: (Integer) The minimum value of the first bucket
  - `range_max`: (Integer) The minimum value of the last bucket
  - `bucket_count`: (Integer) The number of buckets
  - `histogram_type`: 
    - `linear`: The buckets are evenly spaced
    - `exponential`: The buckets follow a natural logarithmic distribution

In addition, the metric should specify:

  - `unit`: (String) The unit of the values in the metric. For documentation purposes only -- does not affect data collection.

If you wanted to create a custom distribution to the fraction of memory that is overhead as a percentage, first you need to add an entry for it to the `metrics.yaml` file:

```YAML
graphics:
  checkerboard_peak:
    type: custom_distribution
    description: >
      Peak number of CSS pixels checkerboarded during a checkerboard event.
    range_min: 1
    range_max: 66355200
    bucket_count: 50
    histogram_type: exponential
    unit: pixels
    gecko_datapoint: MEMORY_HEAP_OVERHEAD_FRACTION
    ...
```

## API

Now you can use the custom distribution from the application's code.

```Kotlin
import org.mozilla.yourApplication.GleanMetrics.Graphics

Graphics.checkerboardPeak.accumulateSamples([23])
```

There are test APIs available too.  For convenience, properties `sum` and `count` are exposed to facilitate validating that data was recorded correctly.

```Kotlin
import org.mozilla.yourApplication.GleanMetrics.Graphics

// Was anything recorded?
assertTrue(Graphics.checkerboardPeak.testHasValue())

// Get snapshot
val snapshot = Graphics.checkerboardPeak.testGetValue()

// Does the sum have the expected value?
assertEquals(11, snapshot.sum)

// Usually you don't know the exact timing values, but how many should have been recorded.
assertEquals(2L, snapshot.count())
```

## Limits

* The maximum value of `bucket_count` is 100.

* Only non-negative values may be recorded.

## Recorded errors

* `invalid_value`: If recording a negative value.

## Reference

* See [Kotlin API docs](../../../javadoc/glean/mozilla.telemetry.glean.private/-custom-distribution-metric-type/index.html)

 
