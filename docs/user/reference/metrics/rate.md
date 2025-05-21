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

Numerators and denominators need to be counted individually.

{{#include ../../../shared/tab_header.md}}
<div data-lang="Kotlin" class="tab">

```Kotlin
import org.mozilla.yourApplication.GleanMetrics.Network

if (connectionHadError) {
    Network.httpConnectionError.addToNumerator(1)
}

Network.httpConnectionError.addToDenominator(1)
```

#### External Denominators

If the rate uses an external denominator,
adding to the denominator must be done through the denominator's
[`counter` API](./counter.md#recording-api):

```Kotlin
import org.mozilla.yourApplication.GleanMetrics.Network

if (connectionHadError) {
    Network.httpConnectionError.addToNumerator(1)
}

Network.httpConnections.add(1)
```

</div>
<div data-lang="Java" class="tab">

```Java
import org.mozilla.yourApplication.GleanMetrics.Network;

if (connectionHadError) {
    Network.INSTANCE.httpConnectionError().addToNumerator(1);
}

Network.IMPORTANT.httpConnectionError().addToDenominator(1);
```

#### External Denominators

If the rate uses an external denominator,
adding to the denominator must be done through the denominator's
[`counter` API](./counter.md#recording-api):

```Java
import org.mozilla.yourApplication.GleanMetrics.Network;

if (connectionHadError) {
    Network.INSTANCE.httpConnectionError().addToNumerator(1);
}

Network.INSTANCE.httpConnections().add(1)
```

</div>
<div data-lang="Swift" class="tab">

```Swift
if (connectionHadError) {
    Network.httpConnectionError.addToNumerator(1)
}

Network.httpConnectionError.addToDenominator(1)
```

#### External Denominators

If the rate uses an external denominator,
adding to the denominator must be done through the denominator's
[`counter` API](./counter.md#recording-api):

```Swift
if (connectionHadError) {
    Network.httpConnectionError.addToNumerator(1)
}

Network.httpConnections.add(1)
```

</div>
<div data-lang="Python" class="tab">

```Python
from glean import load_metrics
metrics = load_metrics("metrics.yaml")

if connection_had_error:
    metrics.network.http_connection_error.add_to_numerator(1)

metrics.network.http_connection_error.add_to_denominator(1)
```

#### External Denominators

If the rate uses an external denominator,
adding to the denominator must be done through the denominator's
[`counter` API](./counter.md#recording-api):

```Python
from glean import load_metrics
metrics = load_metrics("metrics.yaml")

if connection_had_error:
    metrics.network.http_connection_error.add_to_numerator(1)

metrics.network.http_connections.add(1)
```

</div>
<div data-lang="Rust" class="tab">

```Rust
use glean_metrics::network;

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
use glean_metrics::network;

if connection_had_error {
    network::http_connection_error.add_to_numerator(1);
}

network::http_connections.add(1);
```

</div>
<div data-lang="JavaScript" class="tab">

```js
import * as network from "./path/to/generated/files/network.js";

if (connectionHadError) {
  network.httpConnectionError.addToNumerator(1);
}

network.httpConnectionError.addToDenominator(1);
```

</div>
<div data-lang="Firefox Desktop" class="tab">

**C++**

```cpp
#include "mozilla/glean/NetwerkProtocolHttpMetrics.h"

if (aHadError) {
mozilla::glean::network::http_connection_error.AddToNumerator(1);
}
mozilla::glean::network::http_connection_error.AddToDenominator(1);
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

* [`invalid_value`](../../user/metrics/error-reporting.md): If either numerator or denominator is incremented by a negative value.
* [`invalid_type`](../../user/metrics/error-reporting.md): If a floating point or non-number value is given.

#### Limits

* Numerator and Denominator only increment.
* Numerator and Denominator saturate at the largest value that can be represented as a 32-bit signed integer (`2147483647`).

## Testing API

### `testGetValue`

Gets the recorded value for a given rate metric.  
Returns the numerator/denominator pair if data is stored.  
Returns a language-specific empty/null value if no data is stored.
Has an optional argument to specify the name of the ping you wish to retrieve data from, except
in Rust where it's required. `None` or no argument will default to the first value found for `send_in_pings`.

{{#include ../../../shared/tab_header.md}}
<div data-lang="Kotlin" class="tab">

```Kotlin
import org.mozilla.yourApplication.GleanMetrics.Network

assertEquals(Rate(1, 1), Network.httpConnectionError.testGetValue())
```

</div>
<div data-lang="Java" class="tab">

```Java
import org.mozilla.yourApplication.GleanMetrics.Network;

assertEquals(Rate(1, 1), Network.INSTANCE.httpConnectionError().testGetValue());
```

</div>
<div data-lang="Swift" class="tab">

```Swift
XCTAssertEqual(
    Rate(numerator: 1, denominator: 1),
    Network.httpConnectionError.testGetValue()
)
```

</div>
<div data-lang="Python" class="tab">

```Python
from glean import load_metrics
metrics = load_metrics("metrics.yaml")

assert Rate(1, 1) == metrics.network.http_connection_error.test_get_value()
```

</div>
<div data-lang="Rust" class="tab">

```Rust
use glean_metrics::network;

let rate = network::http_connection_error.test_get_value(None).unwrap();
assert_eq!(1, rate.numerator);
assert_eq!(1, rate.denominator);
```

</div>
<div data-lang="JavaScript" class="tab">

```js
import * as network from "./path/to/generated/files/network.js";

const { numerator, denominator } = await network.httpConnectionError.testGetValue();
assert.strictEqual(numerator, 1);
assert.strictEqual(denominator, 1);
```

</div>
<div data-lang="Firefox Desktop" class="tab">

**C++**

```cpp
#include "mozilla/glean/NetwerkProtocolHttpMetrics.h"

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

### `testGetNumRecordedErrors`

{{#include ../../../shared/tab_header.md}}
<div data-lang="Kotlin" class="tab">

```Kotlin
import org.mozilla.yourApplication.GleanMetrics.Network

assertEquals(
    0,
    Network.httpConnectionError.testGetNumRecordedErrors(ErrorType.INVALID_VALUE)
)
```

</div>
<div data-lang="Java" class="tab">

```Java
import org.mozilla.yourApplication.GleanMetrics.Network;

assertEquals(
    0,
    Network.INSTANCE.httpConnectionError().testGetNumRecordedErrors(
        ErrorType.INVALID_VALUE
    )
);
```

</div>
<div data-lang="Swift" class="tab">

```Swift
XCTAssertEqual(
    0,
    Network.httpConnectionError.testGetNumRecordedErrors(.invalidValue)
)
```

</div>
<div data-lang="Python" class="tab">

```Python
from glean import load_metrics
metrics = load_metrics("metrics.yaml")

from glean.testing import ErrorType

assert 0 == metrics.network.http_connection_error.test_get_num_recorded_errors(
    ErrorType.INVALID_VALUE
)
```

</div>
<div data-lang="Rust" class="tab">

```Rust
use glean::ErrorType;
use glean_metrics::network;

assert_eq!(
    0,
    network::http_connection_error.test_get_num_recorded_errors(
        ErrorType::InvalidValue,
        None
    )
);
```

</div>
<div data-lang="JavaScript" class="tab">


```js
import * as network from "./path/to/generated/files/network.js";
import { ErrorType } from "@mozilla/glean/error";

assert.strictEqual(
  1,
  await network.httpConnectionError.testGetNumRecordedErrors(ErrorType.InvalidValue)
);
```

</div>
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

## External Denominators

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

{{#include ../../../shared/blockquote-info.html}}

> The Glean JavaScript SDK does not support external denominators for Rate metrics, yet.
> Follow [Bug 1745753](https://bugzilla.mozilla.org/show_bug.cgi?id=1745753) for updates
> on that features development.

## Data Questions

* How often did an HTTP connection error?
* How many documents used a given CSS Property?

## Reference

* [Python API docs](../../../python/glean/metrics/index.html#glean.metrics.RateMetric)
* [Rust API docs](../../../docs/glean/private/struct.RateMetric.html)
* [Swift API docs](../../../swift/Classes/RateMetric.html)
