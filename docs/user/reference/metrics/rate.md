# Rate

Used to count how often something happens relative to how often something else happens.
Like how many documents use a particular CSS Property,
or how many HTTP connections had an error.
You can think of it like a fraction, with a numerator and a denominator.

All rates start without a value.
A rate with a numerator of 0 is valid and will be sent to ensure we capture the
"no errors happened" or "no use counted" cases.

{{#include ../../../shared/blockquote-warning.html}}

## Let the Glean metric do the counting

> When using a rate metric, it is important to let the Glean metric do the counting.
> Using your own variable for counting and setting the metric yourself could be problematic:
> ping scheduling will make it difficult to ensure the metric is at the correct value at the correct time.
> Instead, count to the numerator and denominator as you go.

## Recording API

### `addToNumerator` / `addToDenominator`

{{#include ../../../shared/tab_header.md}}
<div data-lang="Kotlin" class="tab"></div>
<div data-lang="Java" class="tab"></div>
<div data-lang="Swift" class="tab"></div>
<div data-lang="Python" class="tab"></div>
<div data-lang="Rust" class="tab">

Count numerator and denominator individually:

```Rust
use glean_metrics::*;

if connection_had_error {
    network::http_connection_error.add_to_numerator(1);
}

network::http_connection_error.add_to_denominator(1);
```

#### External Denominators

If the rate uses an external denominator,
adding to the denominator must be done through the denominator's
[`counter` API](./counter.md#recording-api):

```Rust
use glean_metrics;

if connection_had_error {
    network::http_connection_error.add_to_numerator(1);
}

network::http_connections.add(1);
```

</div>
<div data-lang="Javascript" class="tab"></div>
<div data-lang="Firefox Desktop" class="tab">

  **C++**

  ```cpp
  #include "mozilla/glean/GleanMetrics.h"

  if (aHadError) {
    mozilla::glean::network::http_connection_error.add_to_numerator(1);
  }
  mozilla::glean::network::http_connection_error.add_to_denominator(1);
  ```

  **JavaScript**

  ```js
  if (aHadError) {
    Glean.network.httpConnectionError.addToNumerator(1);
  }
  Glean.network.httpConnectionError.addToDenominator(1);
  ```
</div>
{{#include ../../../shared/tab_footer.md}}

#### Recorded errors

* `invalid_value`: If either numerator or denominator is incremented by a negative value.

#### Limits

* Numerator and Denominator only increment.
* Numerator and Denominator saturate at the largest value that can be represented as a 32-bit signed integer (`2147483647`).

## Testing API

### `testGetValue`

{{#include ../../../shared/tab_header.md}}
<div data-lang="Kotlin" class="tab"></div>
<div data-lang="Java" class="tab"></div>
<div data-lang="Swift" class="tab"></div>
<div data-lang="Python" class="tab"></div>
<div data-lang="Rust" class="tab">

```Rust
use glean_metrics::*;

assert_eq!((1, 1), network::http_connection_error.test_get_value(None).unwrap());
```

</div>
<div data-lang="Javascript" class="tab"></div>
<div data-lang="Firefox Desktop" class="tab">

  **C++**

  ```cpp
  #include "mozilla/glean/GleanMetrics.h"

  auto pair = mozilla::glean::network::http_connection_error.TestGetValue().unwrap();
  ASSERT_EQ(1, pair.first);
  ASSERT_EQ(1, pair.second);
  ```

  **JavaScript**

  ```js
  // testGetValue will throw NS_ERROR_LOSS_OF_SIGNIFICANT_DATA on error.
  Assert.deepEqual(
    { numerator: 1, denominator: 1 },
    Glean.network.httpConnectionError.testGetValue()
  );
  ```
</div>
{{#include ../../../shared/tab_footer.md}}

### `testHasValue`

{{#include ../../../shared/tab_header.md}}
<div data-lang="Kotlin" class="tab"></div>
<div data-lang="Java" class="tab"></div>
<div data-lang="Swift" class="tab"></div>
<div data-lang="Python" class="tab"></div>
<div data-lang="Rust" class="tab"></div>
<div data-lang="Javascript" class="tab"></div>
<div data-lang="Firefox Desktop" class="tab"></div>
{{#include ../../../shared/tab_footer.md}}

### `testGetNumRecordedErrors`

{{#include ../../../shared/tab_header.md}}
<div data-lang="Kotlin" class="tab"></div>
<div data-lang="Java" class="tab"></div>
<div data-lang="Swift" class="tab"></div>
<div data-lang="Python" class="tab"></div>
<div data-lang="Rust" class="tab">

```Rust
use glean_metrics::*;
use glean::ErrorType;

assert_eq!(
    0,
    network::http_connection_error.test_get_num_recorded_errors(
        ErrorType::InvalidValue
    )
);
```

</div>
<div data-lang="Javascript" class="tab"></div>
<div data-lang="Firefox Desktop" class="tab" data-info="Firefox Desktop uses testGetValue to communicate errors"></div>
{{#include ../../../shared/tab_footer.md}}

## Metric parameters

Example rate metric definition:

```YAML
network:
  http_connection_error:
    type: rate
    description: >
      How many HTTP connections error out out of the total connections made.
    bugs:
      - https://bugzilla.mozilla.org/000000
    data_reviews:
      - https://bugzilla.mozilla.org/show_bug.cgi?id=000000#c3
    notification_emails:
      - me@mozilla.com
    expires: 2020-10-01
```

For a full reference on metrics parameters common to all metric types,
refer to the [metrics YAML registry format](../yaml/metrics.md) reference page.

### External Denominators

If several rates share the same denominator
then the denominator should be defined as a `counter` and shared between
`rates` using the `denominator_metric` property:

```YAML
network:
  http_connections:
    type: counter
    description: >
      Total number of http connections made.
    ...

  http_connection_error:
    type: rate
    description: >
      How many HTTP connections error out out of the total connections made.
    denominator_metric: network.http_connections
    ...

  http_connection_slow:
    type: rate
    description: >
      How many HTTP connections were slow, out of the total connections made.
    denominator_metric: network.http_connections
    ...
```

## Data Questions

* How often did an HTTP connection error?
* How many documents used a given CSS Property?

## Reference

* [Rust API docs](../../../docs/glean/private/rate/struct.RateMetric.html)
