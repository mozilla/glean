# Custom Distribution

Custom distributions are used to record the distribution of arbitrary values.

It should be used only when direct control over how the histogram buckets are computed is required.
Otherwise, look at the standard distribution metric types:

* [Timing Distributions](timing_distribution.md)
* [Memory Distributions](memory_distribution.md)

> **Note**: Custom distributions are currently only allowed for GeckoView metrics (the `gecko_datapoint` parameter is present) and thus have only a Kotlin API.

## Configuration

Custom distributions have the following required parameters:

  - `range_min`: (Integer) The minimum value of the first bucket
  - `range_max`: (Integer) The minimum value of the last bucket
  - `bucket_count`: (Integer) The number of buckets
  - `histogram_type`:
    - `linear`: The buckets are evenly spaced
    - `exponential`: The buckets follow a natural logarithmic distribution

> **Note** Check out how these bucketing algorithms would behave on the [Custom distribution simulator](#simulator)

In addition, the metric should specify:

  - `unit`: (String) The unit of the values in the metric. For documentation purposes only -- does not affect data collection.

If you wanted to create a custom distribution of the peak number of pixels used during a checkerboard event, first you need to add an entry for it to the `metrics.yaml` file:

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
    gecko_datapoint: CHECKERBOARD_PEAK
    ...
```

## API

Now you can use the custom distribution from the application's code.

{{#include ../../tab_header.md}}

<div data-lang="Kotlin" class="tab">

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

/// Did the metric receive a negative value?
assertEquals(1, Graphics.checkerboardPeak.testGetNumRecordedErrors(ErrorType.InvalidValue))
```

</div>

<div data-lang="Rust" class="tab">

```rust
use glean_metrics;

graphics::checkerboard_peak.accumulate_samples_signed(vec![23]);
```

There are test APIs available too.

```rust
use glean::ErrorType;
use glean_metrics;

// Was anything recorded?
assert!(graphics::checkerboard_peak.test_get_value(None).is_some());
// Does it have the expected value?
assert_eq!(23, graphics::checkerboard_peak.test_get_value(None).unwrap().sum);

// Were any of the values negative and thus caused an error to be recorded?
assert_eq!(
    0,
    graphics::checkerboard_peak.test_get_num_recorded_errors(ErrorType::InvalidValue));
```

</div>

{{#include ../../tab_footer.md}}

## Limits

* The maximum value of `bucket_count` is 100.

* Only non-negative values may be recorded.

## Recorded errors

* `invalid_value`: If recording a negative value.

## Reference

* [Kotlin API docs](../../../javadoc/glean/mozilla.telemetry.glean.private/-custom-distribution-metric-type/index.html)

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
        <div class="input-group">
            <label for="kind">Histogram type (<code>histogram_type</code>)</label>
            <select id="kind" name="kind">
                <option value="exponential" selected>Exponential</option>
                <option value="linear">Linear</option>
            </select>
        </div>
        <div class="input-group">
            <label for="lower-bound">Range minimum (<code>range_min</code>)</label>
            <input name="lower-bound" id="lower-bound" type="number" value="1" />
        </div>
        <div class="input-group">
            <label for="upper-bound">Range maximum (<code>range_max</code>)</label>
            <input name="upper-bound" id="upper-bound" type="number" value="500" />
        </div>
        <div class="input-group">
            <label for="bucket-count">Bucket count (<code>bucket_count</code>)</label>
            <input name="bucket-count" id="bucket-count" type="number" value="20" />
        </div>
    </div>
</div>
