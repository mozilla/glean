# Dual Labeled Counters

Dual labeled counters allow two levels of labels for a counter. This metric type allows recording counts along two dimensions.
These labels are organized into a nested structure in the payload with a "key" label at the top level with "category" sub-labels that map to the underlying count.
Each counter always starts from `0`.
Each time you record to a dual labeled counter, its value is incremented.
Unless incremented by a positive value, a counter will not be reported in pings,
that means: the value `0` is never sent in a ping.

## Recording API

### `add`

Increases one of the key and category pairs in a dual labeled counter metric by a certain amount.
If no amount is passed it defaults to `1`.

{{#include ../../../shared/tab_header.md}}

<div data-lang="Kotlin" class="tab"></div>
<div data-lang="Java" class="tab"></div>
<div data-lang="Swift" class="tab"></div>
<div data-lang="Python" class="tab"></div>
<div data-lang="Rust" class="tab">

```Rust
use glean_metrics::glean_upload;

glean_upload::failures.get("metrics", "recoverable network error").add(1); // Adds 1 to the "metrics: recoverable network error" counter.
glean_upload::failures.get("baseline", "4xx").add(3); // Adds 3 to the "baseline: 4xx" counter.
```
</div>
<div data-lang="JavaScript" class="tab"></div>

<div data-lang="Firefox Desktop" class="tab"></div>

{{#include ../../../shared/tab_footer.md}}

#### Recorded Errors

* [`invalid_value`](../../user/metrics/error-reporting.md): if the counter is incremented by a negative value
  (or, in versions up to and including 54.0.0, `0`).
* [`invalid_type`](../../user/metrics/error-reporting.md): if a floating point or non-number value is given.
{{#include ../../_includes/label-errors.md}}

#### Limits

* Only increments
* Saturates at the largest value that can be represented as a 32-bit signed integer.
* Key and category labels must conform to the [label format](index.md#label-format).
* Each key or category label may have a maximum of 111 characters.
* The lists of key and category labels are limited to:
  * 16 different dynamic key labels and 16 category labels, if no static labels are defined.
    Additional key and category labels will all record to the special label `__other__`.
    These labels may contain any UTF-8 characters.
  * 4096 key labels and 4096 category labels if specified as static labels in `metrics.yaml`, see [key](#key) and [category](#category).
    Unknown key and category labels will be recorded under the special label `__other__`.
    These labels may only be made of printable ASCII characters.

## Testing API

### `testGetValue`

Gets the recorded value for a given key and category pair in a dual labeled counter metric.  
Returns the count if data is stored.  
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
use glean_metrics::glean_upload;

// Do the counters have the expected values?
assert_eq!(1, glean_upload::failures.get("metrics", "recoverable network error").test_get_value().unwrap());
assert_eq!(3, glean_upload::failures.get("baseline", "4xx").test_get_value().unwrap());
```
</div>
<div data-lang="JavaScript" class="tab"></div>

<div data-lang="Firefox Desktop" class="tab"></div>

{{#include ../../../shared/tab_footer.md}}

### `testGetNumRecordedErrors`

Gets the number of errors recorded for a given labeled counter metric in total.

{{#include ../../../shared/tab_header.md}}

<div data-lang="Kotlin" class="tab"></div>
<div data-lang="Java" class="tab"></div>
<div data-lang="Swift" class="tab"></div>
<div data-lang="Python" class="tab"></div>
<div data-lang="Rust" class="tab">

```Rust
use glean::ErrorType;

use glean_metrics::glean_upload;

// Were there any invalid labels?
assert_eq!(
  0,
  glean_uploade::failures.test_get_num_recorded_errors(
    ErrorType::InvalidLabel
  )
);
```
</div>
<div data-lang="JavaScript" class="tab"></div>

<div data-lang="Firefox Desktop" class="tab"></div>

{{#include ../../../shared/tab_footer.md}}

## Metric parameters

Example dual labeled counter metric definition:

```YAML
glean_upload:
  failures:
    type: dual_labeled_counter
    description: >
      Counts the number of upload failures by category per ping that occur in the application.
    bugs:
      - https://bugzilla.mozilla.org/000000
    data_reviews:
      - https://bugzilla.mozilla.org/show_bug.cgi?id=000000#c3
    notification_emails:
      - me@mozilla.com
    expires: 2020-10-01
    dual_labels:
      key:
        description: The ping associated with the upload failure
        labels:
          - baseline
          - events
          - metrics
      category:
        description: The category of failure that was encountered
        labels:
          - recoverable network error
          - 4xx
          - 5xx
          - unknown
      ...
```

### Extra metric parameters

#### `dual_labels:`

Dual labeled metrics have a required `dual_labels` parameter, which in turn has required `key` and `category` parameters.

##### `key`

The `key` parameter has a required `description` field to describe the dimension, along with an optional `labels` parameter which accepts a list of static labels.
The labels in this list must match the following requirements:

* Conform to the [label format](index.md#label-format).
* Each label must have a maximum of 111 characters.
* Each label must only contain printable ASCII characters.
* This list itself is limited to 4096 labels.

##### `category`

The `category` parameter has a required `description` field to describe the dimension, along with an optional `labels` parameter which accepts a list of static labels.
The labels in this list must match the following requirements:

* Conform to the [label format](index.md#label-format).
* Each label must have a maximum of 111 characters.
* Each label must only contain printable ASCII characters.
* This list itself is limited to 4096 labels.

{{#include ../../../shared/blockquote-warning.html}}

##### Important

> If the key and/or category labels are specified in the `metrics.yaml`, using any label not listed
> will be replaced with the special value `__other__`.
>
> If the key and/or category labels are **not** specified in the `metrics.yaml`, only 16 different dynamic labels
> may be used, after which the special value `__other__` will be used.

Removing or changing key and category labels, including their order in the registry file, is permitted.
Avoid reusing key and category labels that were removed in the past. It is best practice to add documentation
when they are removed to the description field so that analysts will know of their existence and meaning in
historical data. Special care must be taken when changing GeckoView metrics sent through the Glean SDK, as the
index of the labels is used to report Gecko data through the Glean SDK.

## Data questions

* How many upload errors in each category do we have for each ping type?

## Reference

* Rust API docs: [`DualLabeledMetric`](../../../docs/glean/private/struct.DualLabeledMetric.html), [`CounterMetric`](../../../docs/glean/private/struct.CounterMetric.html)
