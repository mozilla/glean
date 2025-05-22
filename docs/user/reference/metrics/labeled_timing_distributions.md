# Labeled Timing Distributions

Labeled timing distributions are used to record different related distributions of time measurements.

See [the Timing Distribution reference](timing_distribution.md) for details on bucket distribution,
specifics about how Glean records time, and a histogram simulator.

## Recording API

### `start`

Start tracking time for the provided metric for the given label.
Multiple timers for multiple labels can run simultaneously.

Returns a unique `TimerId` for the new timer.

{{#include ../../../shared/tab_header.md}}

<div data-lang="Kotlin" class="tab"></div>
<div data-lang="Java" class="tab"></div>
<div data-lang="Swift" class="tab"></div>
<div data-lang="Python" class="tab"></div>
<div data-lang="Rust" class="tab">

```Rust
use glean_metrics::devtools;

self.start = devtools::cold_toolbox_open_delay
    .get(toolbox_id)
    .start();
```

</div>
<div data-lang="JavaScript" class="tab"></div>
<div data-lang="Firefox Desktop" class="tab">

**C++**

```c++
#include "mozilla/glean/DevtoolsClientFrameworkMetrics.h"

auto timerId = mozilla::glean::devtools::cold_toolbox_open_delay
    .Get(toolbox_id)
    .Start();
```

**JavaScript**

```js
const timerId = Glean.devtools.coldToolboxOpenDelay[toolbox_id].start();
```

</div>

{{#include ../../../shared/tab_footer.md}}

#### Recorded Errors

{{#include ../../_includes/label-errors.md}}

### `stopAndAccumulate`

Stops tracking time for the provided timer from the metric for the given label.

Adds a count to the corresponding bucket in the label's timing distribution.

Do not use the provided `TimerId` after passing it to this method.

{{#include ../../../shared/tab_header.md}}

<div data-lang="Kotlin" class="tab"></div>
<div data-lang="Java" class="tab"></div>
<div data-lang="Swift" class="tab"></div>
<div data-lang="Python" class="tab"></div>
<div data-lang="Rust" class="tab">

```Rust
use glean_metrics::devtools;

devtools::cold_toolbox_open_delay
    .get(toolbox_id)
    .stop_and_accumulate(self.start);
```

</div>
<div data-lang="JavaScript" class="tab"></div>
<div data-lang="Firefox Desktop" class="tab">

**C++**

```c++
#include "mozilla/glean/DevtoolsClientFrameworkMetrics.h"

mozilla::glean::devtools::cold_toolbox_open_delay
    .Get(toolbox_id)
    .StopAndAccumulate(std::move(timerId));
```

**JavaScript**

```js
Glean.devtools.coldToolboxOpenDelay[toolbox_id].stopAndAccumulate(timerId);
```

</div>

{{#include ../../../shared/tab_footer.md}}

#### Recorded errors

* [`invalid_state`](../../user/metrics/error-reporting.md): If a non-existing, cancelled, or already-stopped timer is stopped again.
{{#include ../../_includes/label-errors.md}}

### `cancel`

Aborts a previous `start` call, consuming the supplied timer id.

{{#include ../../../shared/tab_header.md}}

<div data-lang="Kotlin" class="tab"></div>
<div data-lang="Java" class="tab"></div>
<div data-lang="Swift" class="tab"></div>
<div data-lang="Python" class="tab"></div>
<div data-lang="Rust" class="tab">

```Rust
use glean_metrics::devtools;

devtools::cold_toolbox_open_delay
    .get(toolbox_id)
    .cancel(self.start);
```

</div>
<div data-lang="JavaScript" class="tab"></div>
<div data-lang="Firefox Desktop" class="tab">

**C++**

```c++
#include "mozilla/glean/DevtoolsClientFrameworkMetrics.h"

mozilla::glean::devtools::cold_toolbox_open_delay
    .Get(toolbox_id)
    .Cancel(std::move(timerId));
```

**JavaScript**

```js
Glean.devtools.coldToolboxOpenDelay[toolbox_id].cancel(timerId);
```

</div>

{{#include ../../../shared/tab_footer.md}}

#### Recorded errors

{{#include ../../_includes/label-errors.md}}

### `accumulateSamples`

Accumulates the provided, signed samples in the metric for a given label.
Where possible, have Glean do the timing for you and don't use methods like this one.
If you are doing timing yourself,
ensure your time source is monotonic and behaves consistently across platforms.

This is required so that the platform-specific code can provide us with
64 bit signed integers if no `u64` comparable type is available. This
will take care of filtering and reporting errors for any provided negative
sample.

Please note that this assumes that the provided samples are already in
the "unit" declared by the instance of the metric type (e.g. if the
instance this method was called on is using `TimeUnit::Second`, then
`samples` are assumed to be in that unit).

{{#include ../../../shared/tab_header.md}}

<div data-lang="Kotlin" class="tab"></div>
<div data-lang="Java" class="tab"></div>
<div data-lang="Swift" class="tab"></div>
<div data-lang="Python" class="tab"></div>
<div data-lang="Rust" class="tab">

```Rust
use glean_metrics::devtools;

devtools::cold_toolbox_open_delay
    .get(toolbox_id)
    .accumulate_samples(samples);
```

</div>
<div data-lang="JavaScript" class="tab"></div>
<div data-lang="Firefox Desktop" class="tab">

**C++**

```c++
#include "mozilla/glean/DevtoolsClientFrameworkMetrics.h"

mozilla::glean::devtools::cold_toolbox_open_delay
    .Get(toolbox_id)
    .AccumulateRawSamples(samples);
```

**JavaScript**

```js
Glean.devtools.coldToolboxOpenDelay[toolboxId].accumulateSamples(samples);
```

</div>

{{#include ../../../shared/tab_footer.md}}

#### Recorded errors

* [`invalid_value`](../../user/metrics/error-reporting.md): If recording a negative sample.
* [`invalid_overflow`](../../user/metrics/error-reporting.md): If recording a sample longer than the maximum for the given `time_unit`.
{{#include ../../_includes/label-errors.md}}

### `accumulateSingleSample`

Accumulates a single signed sample and appends it to the metric for the provided label.
Prefer `start()` and `stopAndAccumulate()` where possible,
but if you must record time externally please prefer this method for individual samples
(avoids having to allocate and pass collections).

A signed value is required so that the platform-specific code can provide
us with a 64 bit signed integer if no `u64` comparable type is available.
This will take care of filtering and reporting errors for a negative
sample.

Please note that this assumes that the provided sample is already in
the "unit" declared by the instance of the metric type (e.g. if the
instance this method was called on is using `TimeUnit::Second`, then
`sample` is assumed to be in that unit).

{{#include ../../../shared/tab_header.md}}

<div data-lang="Kotlin" class="tab"></div>
<div data-lang="Java" class="tab"></div>
<div data-lang="Swift" class="tab"></div>
<div data-lang="Python" class="tab"></div>
<div data-lang="Rust" class="tab">

```Rust
use glean_metrics::devtools;

devtools::cold_toolbox_open_delay
    .get(toolbox_id)
    .accumulate_single_sample(sample);
```

</div>
<div data-lang="JavaScript" class="tab"></div>
<div data-lang="Firefox Desktop" class="tab">

**C++**

```c++
#include "mozilla/glean/DevtoolsClientFrameworkMetrics.h"

mozilla::glean::devtools::cold_toolbox_open_delay
    .Get(toolboxId)
    .AccumulateRawDuration(aDuration);
```

**JavaScript**

```js
Glean.devtools.coldToolboxOpenDelay[toolboxId].accumulateSingleSample(sample);
```

</div>

{{#include ../../../shared/tab_footer.md}}

#### Recorded errors

* [`invalid_value`](../../user/metrics/error-reporting.md): If recording a negative sample.
* [`invalid_overflow`](../../user/metrics/error-reporting.md): If recording a sample longer than the maximum for the given `time_unit`.
{{#include ../../_includes/label-errors.md}}


### `measure`

For convenience one can measure the time of a function or block of code.

{{#include ../../../shared/tab_header.md}}

<div data-lang="Kotlin" class="tab"></div>
<div data-lang="Java" class="tab"></div>
<div data-lang="Swift" class="tab"></div>
<div data-lang="Python" class="tab"></div>
<div data-lang="Rust" class="tab"></div>
<div data-lang="JavaScript" class="tab"></div>
<div data-lang="Firefox Desktop" class="tab">

**C++**

```c++
#include "mozilla/glean/DevtoolsClientFrameworkMetrics.h"

{ // Scope for RAII
  auto timer = mozilla::glean::devtools::cold_toolbox_open_delay
      .Get(toolboxId)
      .Measure();
  // Open the toolbox. Cold.
}
```

**JavaScript**

*Not currently implemented.*

</div>

{{#include ../../../shared/tab_footer.md}}

## Testing API

### `testGetValue`

Gets the recorded value for a given label in a labeled timing distribution metric.
Returns a struct with counts per buckets and total sum if data is stored.
Returns a language-specific empty/null value if no data is stored.
Has an optional argument to specify the name of the ping you wish to retrieve data from, except
in Rust where it's required. `None` or no argument will default to the first value found for `send_in_pings`.

{{#include ../../../shared/tab_header.md}}

<div data-lang="Kotlin" class="tab"></div>
<div data-lang="Java" class="tab"></div>
<div data-lang="Swift" class="tab"></div>
<div data-lang="Python" class="tab"></div>
<div data-lang="Rust" class="tab">

```Rust
use glean_metrics::devtools;

// Get the current snapshot of stored values.
let snapshot = devtools::cold_toolbox_open_delay.get("webconsole").test_get_value(None).unwrap();

// Usually you don't know the exact timing values,
// but you do know how many samples there are:
assert_eq!(2, snapshot.count);
// ...and the lower bound of how long they all took:
assert_ge!(400, snapshot.sum);
```

</div>
<div data-lang="JavaScript" class="tab"></div>
<div data-lang="Firefox Desktop" class="tab">

**C++**

```c++
#include "mozilla/glean/DevtoolsClientFrameworkMetrics.h"

const data = mozilla::glean::devtools::cold_toolbox_open_delay
    .Get("webconsole"_ns)
    .TestGetValue().value().unwrap();
ASSERT_TRUE(data.sum > 0);
```

**JavaScript**

```js
Assert.ok(Glean.devtools.coldToolboxOpenDelay["webconsole"].testGetValue().sum > 0);
```

</div>

{{#include ../../../shared/tab_footer.md}}

### `testGetNumRecordedErrors`

Gets the number of errors recorded for a given labeled timing distribution metric in total.

{{#include ../../../shared/tab_header.md}}

<div data-lang="Kotlin" class="tab"></div>
<div data-lang="Java" class="tab"></div>
<div data-lang="Swift" class="tab"></div>
<div data-lang="Python" class="tab"></div>
<div data-lang="Rust" class="tab">

```Rust
use glean::ErrorType;
use glean_metrics::network;

// Assert there were no negative values instrumented.
assert_eq!(
    0,
    devtools::cold_toolbox_open_delay.test_get_num_recorded_errors(
        ErrorType::InvalidValue,
        None
    )
);
```

</div>
<div data-lang="JavaScript" class="tab"></div>
<div data-lang="Firefox Desktop" class="tab"></div>

{{#include ../../../shared/tab_footer.md}}

## Metric parameters

Example labeled timing distribution metric definition:

```YAML
devtools:
  cold_toolbox_open_delay:
    type: labeled_timing_distribution
    description: >
      Time taken to open the first DevTools toolbox, per tool being opened.
    time_unit: millisecond
    bugs:
      - https://bugzilla.mozilla.org/000000
    data_reviews:
      - https://bugzilla.mozilla.org/show_bug.cgi?id=000000#c3
    notification_emails:
      - me@mozilla.com
    expires: 175
    labels:
      - inspector
      - webconsole
      - jsdebugger
      ...
```

### Extra metric parameters

#### `time_unit`

Labeled timing distributions have an optional `time_unit`
parameter to specify the smallest unit of resolution that it will record.
The allowed values for `time_unit` are:

* `nanosecond` (default)
* `microsecond`
* `millisecond`
* `second`
* `minute`
* `hour`
* `day`

{{#include ../../_includes/labels-parameter.md}}

## Data questions

* What is the distribution of initial load times of devtools toolboxes, per tool?
* What is the distribution of how long it takes to load extensions' content scripts, by addon id?

## Limits

* Timings are recorded in nanoseconds
  * In Rust, [`time::precise_time_ns()`](https://docs.rs/time/0.1.42/time/fn.precise_time_ns.html) is used.
* The maximum timing value that will be recorded depends on the `time_unit` parameter:

  - `nanosecond`: 1ns <= x <= 10 minutes
  - `microsecond`: 1μs <= x <= ~6.94 days
  - `millisecond`: 1ms <= x <= ~19 years

  Longer times will be truncated to the maximum value and an error will be recorded.
{{#include ../../_includes/label-limits.md}}

## Reference

* Rust API docs: [`LabeledMetric`](../../../docs/glean/private/struct.LabeledMetric.html), [`TimingDistributionMetric`](../../../docs/glean/private/struct.TimingDistributionMetric.html)
