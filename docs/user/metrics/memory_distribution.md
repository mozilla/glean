# Memory Distribution

Memory distributions are used to accumulate and store memory sizes.

Memory distributions are recorded in a histogram where the buckets have an exponential distribution, specifically with 16 buckets for every power of 2.
That is, the function from a value \\( x \\) to a bucket index is:

\\[ \lfloor 16 \log_2(x) \rfloor \\]

This makes them suitable for measuring memory sizes on a number of different scales without any configuration.

> **Note** Check out how this bucketing algorithm would behave on our [Histogram Simulator](../../appendix/histograms.html)

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
