# Labeled Memory Distributions

Labeled memory distributions are used to record different related distributions of memory sizes.

See [the Memory Distribution reference](memory_distribution.md) for details on bucket distribution,
and a histogram simulator.

## Recording API

### `accumulate`

Accumulate the provided sample in the metric.

{{#include ../../../shared/tab_header.md}}

<div data-lang="Kotlin" class="tab"></div>
<div data-lang="Java" class="tab"></div>
<div data-lang="Swift" class="tab"></div>
<div data-lang="Python" class="tab"></div>
<div data-lang="Rust" class="tab">

```Rust
use glean_metrics::network;

network::http_upload_bandwidth
    .get(http_version)
    .accumulate(self.request_size * 8.0 / 1048576.0 / send_time.as_secs());
```

</div>
<div data-lang="JavaScript" class="tab"></div>
<div data-lang="Firefox Desktop" class="tab">

**C++**

```c++
#include "mozilla/glean/NetwerkMetrics.h"

mozilla::glean::network::http_upload_bandwidth
    .Get(httpVersion)
    .Accumulate(this.mRequestSize * 8.0 / 1048576.0 / sendTime.AsSeconds());

// If the labels are defined statically in metrics.yaml, you can use enum values instead of strings:
mozilla::glean::network::http_upload_bandwidth
    .EnumGet(mozilla::glean::network::HttpUploadBandwidthLabel::eH2)
    .Accumulate(this.mRequestSize * 8.0 / 1048576.0 / sendTime.AsSeconds());

// If you would like to use the process type name as a label, you can use ProcessGet():
mozilla::glean::network::http_upload_bandwidth_by_process
    .ProcessGet()
    .Accumulate(this.mRequestSize * 8.0 / 1048576.0 / sendTime.AsSeconds());
```

**JavaScript**

```js
Glean.network.httpUploadBandwidth[httpVersion]
  .accumulate(requestSize * 8.0 / 1048576.0 / sendTime.asSeconds())
```

</div>

{{#include ../../../shared/tab_footer.md}}

#### Recorded Errors

* [`invalid_value`](../../user/metrics/error-reporting.md): if recording a memory size that is negative or over 1 TB.
{{#include ../../_includes/label-errors.md}}

## Testing API

### `testGetValue`

Gets the recorded value for a given label in a labeled memory distribution metric.
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

// Assert the sum of all HTTP2 samples is 42MBps.
assert_eq!(42, network::http_upload_bandwidth.get("h2").test_get_value(None).unwrap().sum);

// Assert there's only the one sample
assert_eq!(1, network::http_upload_badwidth.get("h2").test_get_value(None).unwrap().count);

// Buckets are indexed by their lower bound.
assert_eq!(1, network::http_upload_bandwidth.get("h2").test_get_value(None).unwrap().values[41]);
```

</div>
<div data-lang="JavaScript" class="tab"></div>
<div data-lang="Firefox Desktop" class="tab">

**C++**

```c++
#include "mozilla/glean/NetwerkMetrics.h"

const data = mozilla::glean::network::http_upload_bandwidth
    .Get("h2")
    .TestGetValue().value().unwrap()
ASSERT_EQ(42UL, data.sum);
```

**JavaScript**

```js
const data = Glean.network.httpUploadBandwidth["h2"].testGetValue();
Assert.equal(42, data.sum);
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

// Assert there were no negative or overlarge values instrumented.
assert_eq!(
    0,
    network::http_upload_bandwidth.test_get_num_recorded_errors(
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

Example labeled memory distribution metric definition:

```YAML
network:
  http_upload_bandwidth:
    type: labeled_memory_distribution
    description: >
      The upload bandwidth for requests larger than 10MB,
      per HTTP protocol version.
    memory_unit: megabyte
    bugs:
      - https://bugzilla.mozilla.org/000000
    data_reviews:
      - https://bugzilla.mozilla.org/show_bug.cgi?id=000000#c3
    notification_emails:
      - me@mozilla.com
    expires: 175
    labels:
      - h3
      - h2
      - http/1.0
      - http/1.1
```

### Extra metric parameters

#### `memory_unit`

Memory distributions have an optional `memory_unit` parameter,
which specifies the unit the incoming memory size values are recorded in.

The allowed values for `memory_unit` are:

* `byte` (default)
* `kilobyte` (`= 2^10 = 1,024 bytes`)
* `megabyte` (`= 2^20 = 1,048,576 bytes`)
* `gigabyte` (`= 2^30 = 1,073,741,824 bytes`)

{{#include ../../_includes/labels-parameter.md}}

## Data questions

* What is the distribution of upload bandwidth rates per HTTP protocol version?
* What is the distribution of bytes received per DOM network API?

## Limits

* The maximum memory size that can be recorded is 1 Terabyte (2<sup>40</sup> bytes).
  Larger sizes will be truncated to 1 Terabyte.
{{#include ../../_includes/label-limits.md}}

## Reference

* Rust API docs: [`LabeledMetric`](../../../docs/glean/private/struct.LabeledMetric.html), [`MemoryDistributionMetric`](../../../docs/glean/private/struct.MemoryDistributionMetric.html)
