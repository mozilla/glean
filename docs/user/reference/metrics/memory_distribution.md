# Memory Distribution

Memory distributions are used to accumulate and store memory sizes.

Memory distributions are recorded in a histogram where the buckets have an exponential distribution, specifically with 16 buckets for every power of 2.
That is, the function from a value \\( x \\) to a bucket index is:

\\[ \lfloor 16 \log_2(x) \rfloor \\]

This makes them suitable for measuring memory sizes on a number of different scales without any configuration.

> **Note** Check out how this bucketing algorithm would behave on the [Simulator](#simulator).

## Recording API

### `accumulate`

Accumulates the provided sample in the metric.

{{#include ../../../shared/tab_header.md}}

<div data-lang="Kotlin" class="tab">

```Kotlin
import org.mozilla.yourApplication.GleanMetrics.Memory

fun allocateMemory(nbytes: Int) {
    // ...
    Memory.heapAllocated.accumulate(nbytes / 1024)
}
```

</div>
<div data-lang="Java" class="tab">

```Java
import org.mozilla.yourApplication.GleanMetrics.Memory;

fun allocateMemory(nbytes: Int) {
    // ...
    Memory.INSTANCE.heapAllocated().accumulate(nbytes / 1024);
}
```

</div>
<div data-lang="Swift" class="tab">

```Swift
import Glean

func allocateMemory(nbytes: UInt64) {
    // ...
    Memory.heapAllocated.accumulate(nbytes / 1024)
}
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

</div>
<div data-lang="Rust" class="tab">

```Rust
use glean_metrics::memory;

fn allocate_memory(bytes: u64) {
    // ...
    memory::heap_allocated.accumulate(bytes / 1024);
}
```

</div>
<div data-lang="JavaScript" class="tab">

```js
import * as memory from "./path/to/generated/files/memory.js";

function allocateMemory() {
    // ...
    memory.heapAllocated.accumulate(nbytes / 1024);
}
```

</div>
<div data-lang="Firefox Desktop" class="tab">

**C++**

```c++
#include "mozilla/glean/JsXpconnectMetrics.h"

mozilla::glean::memory::heap_allocated.Accumulate(bytes / 1024);
```

**JavaScript**

```js
Glean.memory.heapAllocated.accumulate(bytes / 1024);
```

</div>

{{#include ../../../shared/tab_footer.md}}

#### Recorded errors

* [`invalid_value`](../../user/metrics/error-reporting.md): If recording a negative memory size.
* [`invalid_value`](../../user/metrics/error-reporting.md): If recording a size larger than 1 TB.

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
use glean_metrics::memory;

let buffer = memory::heap_allocated.start_buffer();

for sample in used_memory.iter() {
  buffer.accumulate(sample);
}

// Explicit or implicit drop of `buffer` commits the data.
drop(buffer);
```

No data is recorded when the buffer is abandoned.

```Rust
use glean_metrics::memory;

let buffer = memory::heap_allocated.start_buffer();

for sample in used_memory.iter() {
  buffer.accumulate(sample);
}

buffer.abandon(); // No data is recorded.
```

</div>
<div data-lang="JavaScript" class="tab"></div>
<div data-lang="Firefox Desktop" class="tab"></div>

{{#include ../../../shared/tab_footer.md}}

#### Recorded errors

* [`invalid_value`](../../user/metrics/error-reporting.md): If recording a size larger than 1 TB.

## Testing API

### `testGetValue`

Gets the recorded value for a given memory distribution metric.  
Returns a struct with counts per buckets and total sum if data is stored.  
Returns a language-specific empty/null value if no data is stored.
Has an optional argument to specify the name of the ping you wish to retrieve data from, except
in Rust where it's required. `None` or no argument will default to the first value found for `send_in_pings`.

{{#include ../../../shared/tab_header.md}}

<div data-lang="Kotlin" class="tab">

```Kotlin
import org.mozilla.yourApplication.GleanMetrics.Memory

// Get snapshot
val snapshot = Memory.heapAllocated.testGetValue()

// Does the sum have the expected value?
assertEquals(11, snapshot.sum)

// Usually you don't know the exact memory values,
// but how many should have been recorded.
assertEquals(2L, snapshot.count)
```

</div>
<div data-lang="Java" class="tab">

```Java
import org.mozilla.yourApplication.GleanMetrics.Memory;

// Get snapshot
val snapshot = Memory.INSTANCE.heapAllocated().testGetValue();

// Does the sum have the expected value?
assertEquals(11, snapshot.sum);

// Usually you don't know the exact memory values,
// but how many should have been recorded.
assertEquals(2L, snapshot.getCount());
```

</div>
<div data-lang="Swift" class="tab">

```Swift
// Get snapshot
let snapshot = try! Memory.heapAllocated.testGetValue()

// Does the sum have the expected value?
XCTAssertEqual(11, snapshot.sum)

// Usually you don't know the exact memory values,
// but how many should have been recorded.
XCTAssertEqual(2, snapshot.count)
```

</div>
<div data-lang="Python" class="tab">

```Python
from glean import load_metrics
metrics = load_metrics("metrics.yaml")

# Get snapshot.
snapshot = metrics.memory.heap_allocated.test_get_value()

# Does the sum have the expected value?
assert 11 == snapshot.sum

# Usually you don't know the exact memory values,
# but how many should have been recorded.
assert 2 == snapshot.count
```

</div>
<div data-lang="Rust" class="tab">

```Rust
use glean::ErrorType;
use glean_metrics::memory;

// Get snapshot
let snapshot = memory::heap_allocated.test_get_value(None).unwrap();

// Does the sum have the expected value?
assert_eq!(11, snapshot.sum);

// Usually you don't know the exact timing values,
// but how many should have been recorded.
assert_eq!(2, snapshot.count);
```


</div>
<div data-lang="JavaScript" class="tab">

```js
import * as memory from "./path/to/generated/files/memory.js";

// Get snapshot
const snapshot = await memory.heapAllocated.testGetValue();

// Does the sum have the expected value?
assert.equal(11, snapshot.sum);

// Usually you don't know the exact memory values,
// but know how many should have been recorded.
assert.equal(2, snapshot.count);
```

</div>
<div data-lang="Firefox Desktop" class="tab">

**C++**

```c++
#include "mozilla/glean/JsXpconnectMetrics.h"

// Does it have an expected values?
const data = mozilla::glean::memory::heap_allocated.TestGetValue().value().unwrap()
ASSERT_EQ(11 * 1024, data.sum);
```

**JavaScript**

```js
const data = Glean.memory.heapAllocated.testGetValue();
Assert.equal(11 * 1024, data.sum);
```

</div>

{{#include ../../../shared/tab_footer.md}}

### `testGetNumRecordedErrors`

Gets the number of errors recorded for a given memory distribution metric.

{{#include ../../../shared/tab_header.md}}

<div data-lang="Kotlin" class="tab">

```Kotlin
import org.mozilla.yourApplication.GleanMetrics.Memory

// Did this record a negative value?
assertEquals(
    0,
    Memory.heapAllocated.testGetNumRecordedErrors(ErrorType.INVALID_VALUE)
)
```

</div>
<div data-lang="Java" class="tab">

```Java
import org.mozilla.yourApplication.GleanMetrics.Memory;

// Assert that no errors were recorded.
assertEquals(
    0,
    Memory.INSTANCE.heapAllocated().testGetNumRecordedErrors(ErrorType.INVALID_VALUE)
);
```

</div>
<div data-lang="Swift" class="tab">

```Swift
// Did this record a negative value?
XCTAssertEqual(0, Memory.heapAllocated.testGetNumRecordedErrors(.invalidValue))
```

</div>
<div data-lang="Python" class="tab">

```Python
from glean import load_metrics
metrics = load_metrics("metrics.yaml")

# Did this record a negative value?
assert 0 == metrics.memory.heap_allocated.test_get_num_recorded_errors(
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

```js
import * as memory from "./path/to/generated/files/memory.js";
import { ErrorType } from "@mozilla/glean/<platform>";

// Did this record a negative value?
assert.equal(
    0,
    await memory.heapAllocated.testGetNumRecordedErrors(ErrorType.InvalidValue)
);
```

</div>
<div data-lang="Firefox Desktop" class="tab"></div>

{{#include ../../../shared/tab_footer.md}}

## Metric parameters

Example memory distribution metric definition:

```YAML
memory:
  heap_allocated:
    type: memory_distribution
    memory_unit: kilobyte
    description: >
      The heap memory allocated
    bugs:
      - https://bugzilla.mozilla.org/000000
    data_reviews:
      - https://bugzilla.mozilla.org/show_bug.cgi?id=000000#c3
    notification_emails:
      - me@mozilla.com
    expires: 2020-10-01
```

### Extra metric parameters

#### `memory_unit`

Memory distributions have an optional `memory_unit` parameter, which specifies the unit the incoming memory size values are recorded in.
The allowed values for `memory_unit` are:

* `byte` (default)
* `kilobyte` (`= 2^10 = 1,024 bytes`)
* `megabyte` (`= 2^20 = 1,048,576 bytes`)
* `gigabyte` (`= 2^30 = 1,073,741,824 bytes`)

## Limits

* The maximum memory size that can be recorded is 1 Terabyte (2<sup>40</sup> bytes).
  Larger sizes will be truncated to 1 Terabyte.

## Data questions

* What is the distribution of the size of heap allocations?

## Reference

* [Python API docs](../../../python/glean/metrics/index.html#glean.metrics.TimingDistributionMetricType)
* [Rust API docs](../../../docs/glean/private/struct.MemoryDistributionMetric.html)
* [Swift API docs](../../../swift/Classes/MemoryDistributionMetric.html)

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
