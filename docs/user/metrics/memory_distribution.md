# Memory Distribution

Memory distributions are used to accumulate and store memory sizes.

Memory distributions are recorded in a histogram where the buckets have an exponential distribution, specifically with 16 buckets for every power of 2.
This makes them suitable for measuring memory sizes on a number of different scales without any configuration.

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

For example, to measure page load time on a number of tabs that are loading at the same time, each tab object needs to store the running timer ID.

```Kotlin
import org.mozilla.yourApplication.GleanMetrics.Memory

fun allocateMemory(nbytes: Int) {
    Memory.heapAllocated.accumulate(nbytes / 1024)
}
```

There are test APIs available too.  For convenience, properties `sum` and `count` are exposed to facilitate validating that data was recorded correctly.

Continuing the `pageLoad` example above, at this point the metric should have a `sum == 11` and a `count == 2`:

```Kotlin
import org.mozilla.yourApplication.GleanMetrics.Memory

// Was anything recorded?
assertTrue(Memory.heapAllocated.testHasValue())

// Get snapshot
val snapshot = Memory.heapAllocated.testGetValue()

// Does the sum have the expected value?
assertEquals(11, snapshot.sum)

// Usually you don't know the exact memory values, but how many should have been recorded.
assertEquals(2L, snapshot.count())
```

## Limits

* The maxmimum memory size that can be recorded is 1 Terabyte (2^40 bytes). Longer times will be truncated to 1 Terabyte.

## Examples

* How much memory was allocated by a given process?

## Recorded errors

* `invalid_value`: If recording a negative memory size.
* `invalid_value`: If recording a size larger than 1TB. 

## Reference

* See [Kotlin API docs](../../../javadoc/glean/mozilla.telemetry.glean.private/-memory-distribution-metric-type/index.html)
