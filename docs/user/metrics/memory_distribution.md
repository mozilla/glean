# Memory Distribution

Memory distributions are used to accumulate and store memory sizes.

Memory distributions are recorded in a histogram where the buckets have an exponential distribution, specifically with 16 buckets for every power of 2.
That is, the function from a value \\( x \\) to a bucket index is:

\\[ \lfloor 16 \log_2(x) \rfloor \\]

This makes them suitable for measuring memory sizes on a number of different scales without any configuration.

> **Note** Check out how this bucketing algorithm would behave on the [Simulator](#simulator)

## Configuration

If you wanted to create a memory distribution to measure the amount of heap memory allocated, first you need to add an entry for it to the `metrics.yaml` file:

```YAML
memory:
  heap_allocated:
    type: memory_distribution
    description: >
      The heap memory allocated
    memory_unit: kilobyte
    ...
```

## API

Now you can use the memory distribution from the application's code.

> **Note** The data _provided_ to the `accumulate` method is in the configured memory unit specified in the `metrics.yaml` file. The data _recorded_, on the other hand, is always in **bytes**.

For example, to measure the distribution of heap allocations:

{{#include ../../tab_header.md}}

<div data-lang="Kotlin" class="tab">

```Kotlin
import org.mozilla.yourApplication.GleanMetrics.Memory

fun allocateMemory(nbytes: Int) {
    // ...
    Memory.heapAllocated.accumulate(nbytes / 1024)
}
```

There are test APIs available too.  For convenience, properties `sum` and `count` are exposed to facilitate validating that data was recorded correctly.

Continuing the `heapAllocated` example above, at this point the metric should have a `sum == 11` and a `count == 2`:

```Kotlin
import org.mozilla.yourApplication.GleanMetrics.Memory

// Was anything recorded?
assertTrue(Memory.heapAllocated.testHasValue())

// Get snapshot
val snapshot = Memory.heapAllocated.testGetValue()

// Does the sum have the expected value?
assertEquals(11, snapshot.sum)

// Usually you don't know the exact memory values, but how many should have been recorded.
assertEquals(2L, snapshot.count)

// Did this record a negative value?
assertEquals(1, Memory.heapAllocated.testGetNumRecordedErrors(ErrorType.InvalidValue))
```

</div>

<div data-lang="Swift" class="tab">

```Swift
func allocateMemory(nbytes: UInt64) {
    // ...
    Memory.heapAllocated.accumulate(nbytes / 1024)
}
```

There are test APIs available too.  For convenience, properties `sum` and `count` are exposed to facilitate validating that data was recorded correctly.

Continuing the `heapAllocated` example above, at this point the metric should have a `sum == 11` and a `count == 2`:

```Swift
@testable import Glean

// Was anything recorded?
XCTAssert(Memory.heapAllocated.testHasValue())

// Get snapshot
let snapshot = try! Memory.heapAllocated.testGetValue()

// Does the sum have the expected value?
XCTAssertEqual(11, snapshot.sum)

// Usually you don't know the exact memory values, but how many should have been recorded.
XCTAssertEqual(2, snapshot.count)

// Did this record a negative value?
XCTAssertEqual(1, Memory.heapAllocated.testGetNumRecordedErrors(.invalidValue))
```

</div>

<div data-lang="Python" class="tab">

```Python
from glean import load_metrics
metrics = load_metrics("metrics.yaml")

def allocate_memory(nbytes):
    # ...
    metrics.memory.heap_allocated.accumulate(nbytes / 1024)
```

There are test APIs available too.  For convenience, properties `sum` and `count` are exposed to facilitate validating that data was recorded correctly.

Continuing the `heapAllocated` example above, at this point the metric should have a `sum == 11` and a `count == 2`:

```Python
# Was anything recorded?
assert metrics.memory.head_allocated.test_has_value()

# Get snapshot
snapshot = metrics.memory.heap_allocated.test_get_value()

# Does the sum have the expected value?
assert 11 == snapshot.sum

# Usually you don't know the exact memory values, but how many should have been recorded.
assert 2 == snapshot.count

# Did this record a negative value?
assert 1 == metrics.memory.heap_allocated.test_get_num_recorded_errors(
    ErrorType.INVALID_VALUE
)
```

</div>

<div data-lang="C#" class="tab">

```C#
using static Mozilla.YourApplication.GleanMetrics.Memory;

fun allocateMemory(ulong nbytes) {
    // ...
    Memory.heapAllocated.Accumulate(nbytes / 1024);
}
```

There are test APIs available too.  For convenience, properties `Sum` and `Count` are exposed to facilitate validating that data was recorded correctly.

Continuing the `heapAllocated` example above, at this point the metric should have a `Sum == 11` and a `Count == 2`:

```C#
using static Mozilla.YourApplication.GleanMetrics.Memory;

// Was anything recorded?
Assert.True(Memory.heapAllocated.TestHasValue());

// Get snapshot
var snapshot = Memory.heapAllocated.TestGetValue();

// Does the sum have the expected value?
Assert.Equal(11, snapshot.Sum);

// Usually you don't know the exact memory values, but how many should have been recorded.
Assert.Equal(2L, snapshot.Count);

// Did this record a negative value?
Assert.Equal(1, Memory.heapAllocated.TestGetNumRecordedErrors(ErrorType.InvalidValue));
```

</div>

<div data-lang="Rust" class="tab">

```rust
use glean_metrics;

fn allocate_memory(bytes: u64) {
    memory::heap_allocated.accumulate(bytes / 1024);
}
```

There are test APIs available too:

```rust
use glean::{DistributionData, ErrorType};
use glean_metrics;

// Was anything recorded?
assert!(memory::heap_allocated.test_get_value(None).is_some());

// Is the sum as expected?
let data = memory::heap_allocated.test_get_value(None).unwrap();
assert_eq!(11, data.sum)
// The actual buckets and counts live in `data.values`.

// Were there any errors?
assert_eq!(1, memory::heap_allocated.test_get_num_recorded_errors(InvalidValue));

```

</div>

<div data-lang="C++" class="tab">

> **Note**: C++ APIs are only available in Firefox Desktop.

```c++
#include "mozilla/glean/GleanMetrics.h"

mozilla::glean::memory::heap_allocated.Accumulate(bytes / 1024);
```

There are test APIs available too:

```c++
#include "mozilla/glean/GleanMetrics.h"

// Does it have the expected value?
ASSERT_EQ(11 * 1024, mozilla::glean::memory::heap_allocated.TestGetValue().value().sum);
// Did it run across any errors?
// TODO: https://bugzilla.mozilla.org/show_bug.cgi?id=1683171
```

</div>

<div data-lang="JS" class="tab">

> **Note**: JS APIs are only available in Firefox Desktop.

```js
Glean.memory.heapAllocated.accumulate(bytes / 1024);
```

There are test APIs available too:

```js
const data = Glean.memory.heapAllocated.testGetValue();
Assert.equal(11 * 1024, data.sum);
// Does it have the right number of samples?
Assert.equal(1, Object.entries(data.values).reduce(([bucket, count], sum) => count + sum, 0));
// Did it run across any errors?
// TODO: https://bugzilla.mozilla.org/show_bug.cgi?id=1683171
```

</div>

{{#include ../../tab_footer.md}}

## Limits

* The maximum memory size that can be recorded is 1 Terabyte (2<sup>40</sup> bytes). Larger sizes will be truncated to 1 Terabyte.

## Examples

* What is the distribution of the size of heap allocations?

## Recorded errors

* `invalid_value`: If recording a negative memory size.
* `invalid_value`: If recording a size larger than 1TB.

## Reference

* [Kotlin API docs](../../../javadoc/glean/mozilla.telemetry.glean.private/-memory-distribution-metric-type/index.html)
* [Swift API docs](../../../swift/Classes/MemoryDistributionMetricType.html)
* [Python API docs](../../../python/glean/metrics/timing_distribution.html)
* [Rust API docs](../../../docs/glean/private/struct.MemoryDistributionMetric.html)

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
            <input id="buckets-per-magnitude" name="buckets-per-magnitude" type="number" value="16" disabled />
        </div>
        <div class="input-group hide">
            <label for="maximum-value">Maximum value</label>
            <input id="maximum-value" name="maximum-value" type="number" value="1099511627776" disabled />
        </div>
        <div class="input-group">
            <label for="memory-unit">Memory unit (<code>memory_unit</code>)</label>
            <select id="memory-unit" name="memory-unit">
                <option value="byte" selected>Byte</option>
                <option value="kilobyte">Kilobyte</option>
                <option value="megabyte">Megabyte</option>
                <option value="gigabyte">Gigabyte</option>
            </select>
        </div>
    </div>
</div>

> **Note** The data _provided_, is assumed to be in the configured memory unit. The data _recorded_, on the other hand, is always in **bytes**.
> This means that, if the configured memory unit is not `byte`, the data will be transformed before being recorded. Notice this, by using the select field above to change the memory unit and see the mean of the data recorded changing.
