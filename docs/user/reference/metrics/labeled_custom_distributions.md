# Labeled Custom Distributions

Labeled custom distributions are used to record different related distributions of arbitrary values.

If your data is timing or memory based and you don't need direct control over histogram buckets,
consider instead:

* [Labeled Timing Distributions](labeled_timing_distributions.md)
* [Labeled Memory Distributions](labeled_memory_distributions.md)

## Recording API

### `accumulateSamples`

Accumulate the provided samples in the metric.

{{#include ../../../shared/tab_header.md}}

<div data-lang="Kotlin" class="tab"></div>
<div data-lang="Java" class="tab"></div>
<div data-lang="Swift" class="tab"></div>
<div data-lang="Python" class="tab"></div>
<div data-lang="Rust" class="tab">

```Rust
use glean_metrics::network;

network::http3_late_ack_ratio
    .get("ack")
    .accumulateSamples(vec![(stats.late_ack * 10000) / stats.packets_tx]);
network::http3_late_ack_ratio
    .get("pto")
    .accumulateSamples(vec![(stats.pto_ack * 10000) / stats.packets_tx]);
```

</div>
<div data-lang="JavaScript" class="tab"></div>
<div data-lang="Firefox Desktop" class="tab">

**C++**
```cpp
#include "mozilla/glean/NetwerkMetrics.h"

mozilla::glean::network::http3_late_ack_ratio
    .Get("ack")
    .AccumulateSamples({(stats.late_ack * 10000) / stats.packets_tx});
mozilla::glean::network::http3_late_ack_ratio
    .Get("pto")
    .AccumulateSamples({(stats.pto_ack * 10000) / stats.packets_tx});
```

**JavaScript**
```js
Glean.network.http3LateAckRatio["ack"]
  .accumulateSamples([(stats.late_ack * 10000) / stats.packets_tx]);
Glean.network.http3LateAckRatio["pto"]
  .accumulateSamples([(stats.pto_ack * 10000) / stats.packets_tx]);
```
</div>

{{#include ../../../shared/tab_footer.md}}

#### Recorded Errors

* [`invalid_value`](../../user/metrics/error-reporting.md): if recording any negative samples
{{#include ../../_includes/label-errors.md}}

### `accumulateSingleSample`

Accumulates one sample and appends it to the metric.

{{#include ../../../shared/tab_header.md}}

<div data-lang="Kotlin" class="tab"></div>
<div data-lang="Java" class="tab"></div>
<div data-lang="Swift" class="tab"></div>
<div data-lang="Python" class="tab"></div>
<div data-lang="Rust" class="tab">

```Rust
use glean_metrics::network;

network::http3_late_ack_ratio
    .get("ack")
    .accumulateSingleSample((stats.late_ack * 10000) / stats.packets_tx);
network::http3_late_ack_ratio
    .get("pto")
    .accumulateSingleSample((stats.pto_ack * 10000) / stats.packets_tx);
```

</div>
<div data-lang="JavaScript" class="tab"></div>
<div data-lang="Firefox Desktop" class="tab">

**C++**
```cpp
#include "mozilla/glean/NetwerkMetrics.h"

mozilla::glean::network::http3_late_ack_ratio
    .Get("ack")
    .AccumulateSingleSample((stats.late_ack * 10000) / stats.packets_tx);
mozilla::glean::network::http3_late_ack_ratio
    .Get("pto")
    .AccumulateSingleSample((stats.pto_ack * 10000) / stats.packets_tx);
```

**JavaScript**
```js
Glean.network.http3LateAckRatio["ack"]
  .accumulateSingleSample((stats.late_ack * 10000) / stats.packets_tx);
Glean.network.http3LateAckRatio["pto"]
  .accumulateSingleSample((stats.pto_ack * 10000) / stats.packets_tx);
```
</div>

{{#include ../../../shared/tab_footer.md}}

#### Recorded Errors

* [`invalid_value`](../../user/metrics/error-reporting.md): if recording a negative sample
{{#include ../../_includes/label-errors.md}}

## Testing API

### `testGetValue`

Gets the recorded value for a given label in a labeled custom distribution metric.
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
use glean_metrics::network;

// Assert the sum of all samples is 42.
assert_eq!(42, network::http3_late_ack_ratio.get("ack").test_get_value(None).unwrap().sum);

// Assert there's only the one sample
assert_eq!(1, network::http3_late_ack_ratio.get("ack").test_get_value(None).unwrap().count);

// Buckets are indexed by their lower bound.
assert_eq!(1, network::http3_late_ack_ratio.get("ack").test_get_value(None).unwrap().values[41]);
```

</div>
<div data-lang="JavaScript" class="tab"></div>
<div data-lang="Firefox Desktop" class="tab">

**C++**
```cpp
#include "mozilla/glean/NetwerkMetrics.h"

auto data = mozilla::glean::network::http3_late_ack_ratio.Get("ack").TestGetValue().value();
ASSERT_EQ(42UL, data.sum);
ASSERT_EQ(1, data.count);
ASSERT_EQ(1, data.values[41]);
```

**JavaScript**
```js
let data = Glean.network.http3LateAckRatio["ack"].testGetValue();
Assert.equal(42, data.sum);
Assert.equal(1, data.count);
Assert.equal(1, data.values[41]);
```
</div>

{{#include ../../../shared/tab_footer.md}}

### `testGetNumRecordedErrors`

Gets the number of errors recorded for a given labeled custom distribution metric in total.

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
    network::http3_late_ack_ratio.test_get_num_recorded_errors(
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

Example labeled custom distribution metric definition:

```YAML
network:
  http3_late_ack_ratio:
    type: labeled_custom_distribution
    description: >
      HTTP3: The ratio of spurious retransmissions per packets sent,
      represented as an integer permdecimille:
      `(spurious_retransmission / packet sent * 10000)`
    range_min: 1
    range_max: 2000
    bucket_count 100
    histogram_type: exponential
    bugs:
      - https://bugzilla.mozilla.org/000000
    data_reviews:
      - https://bugzilla.mozilla.org/show_bug.cgi?id=000000#c3
    notification_emails:
      - me@mozilla.com
    expires: 175
    labels:
      - ack
      - pto
```

### Extra metric parameters

#### `range_min`, `range_max`, `bucket_count`, and `histogram_type` (Required)

Labeled custom distributions have the following required parameters:

- `range_min`: (Integer) The minimum value of the first bucket
- `range_max`: (Integer) The minimum value of the last bucket
- `bucket_count`: (Integer) The number of buckets
- `histogram_type`:
  - `linear`: The buckets are evenly spaced
  - `exponential`: The buckets follow a natural logarithmic distribution

{{#include ../../_includes/labels-parameter.md}}

## Data questions

* What is the distribution of retransmission ratios per connection?
* What is the distribution of how long it takes to load extensions' content scripts, by addon id?

## Limits

* The maximum value of `bucket_count` is 100.
* Only non-negative integer values may be recorded (`>=0`).
{{#include ../../_includes/label-limits.md}}

## Reference

* Rust API docs: [`LabeledMetric`](../../../docs/glean/private/struct.LabeledMetric.html), [`CustomDistributionMetric`](../../../docs/glean/private/struct.CustomDistributionMetric.html)
