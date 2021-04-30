# Rate

Used to count how often something happens relative to how often something else happens.
Like how many documents use a particular CSS Property,
or how many HTTP connections had an error.
You can think of it like a fraction, with a numerator and a denominator.

All rates start without a value.
A rate with a numerator of 0 is valid and will be sent to ensure we capture the
"no errors happened" or "no use counted" cases.

> **IMPORTANT:** When using a rate metric, it is important to let the Glean metric do the counting.
  Using your own variable for counting and setting the metric yourself could be problematic:
  ping scheduling will make it difficult to ensure the metric is at the correct value at the correct time.
  Instead, count to the numerator and denominator as you go.

## Configuration

Say you're adding a new rate for how often HTTP connections have errors.
First you need to add an entry for the rate to the `metrics.yaml` file:

```YAML
network:
  http_connection_error:
    type: rate
    description: >
      How many HTTP connections error out out of the total connections made.
    ...
```

### External Denominators

If several rates share the same denominator
(from our example above, maybe there are multiple rates per total connections made)
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

## API

{{#include ../../../shared/tab_header.md}}

<div data-lang="Rust" class="tab">

Since a rate is two numbers, you add to each one individually:

```rust
use glean_metrics::*;

if connection_had_error {
    network::http_connection_error.add_to_numerator(1);
}

network::http_connection_error.add_to_denominator(1);
```

If the rate uses an external denominator,
adding to the denominator must be done through the denominator's
`counter` API:

```rust
use glean_metrics;

if connection_had_error {
    network::http_connection_error.add_to_numerator(1);
}
if connection_was_slow {
    network::http_connection_slow.add_to_numerator(1);
}

// network::http_connection_error has no `add_to_denominator` method.
network::http_connections.add(1);
```

There are test APIs available too.
Whether the rate has an external denominator or not,
you can use this API to get the current value:

```rust
use glean::ErrorType;

use glean_metrics;

// Was anything recorded?
assert!(network::http_connection_error.test_get_value(None).is_some());
// Does it contain counter have the expected values?
assert_eq!((1, 1), network::http_connection_error.test_get_value(None).unwrap());
// Did the numerator or denominator ever have a negative value added?
assert_eq!(
  0,
  network::http_connection_error.test_get_num_recorded_errors(
    ErrorType::InvalidValue
  )
);
```

</div>

{{#include ../../../shared/tab_footer.md}}

## Limits

* Numerator and Denominator only increment.
* Numerator and Denominator saturate at the largest value that can be represented as a 32-bit signed integer (`2147483647`).

## Examples

* How often did an HTTP connection error?
* How many documents used a given CSS Property?

## Recorded errors

* `invalid_value`: If either numerator or denominator is incremented by a negative value.

## Reference

* [Rust API docs](../../../docs/glean/private/rate/struct.RateMetric.html)
