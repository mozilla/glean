# Labeled Strings

Labeled strings record multiple Unicode string values, each under a different label.

## Configuration

For example to record which kind of error occurred in different stages of a login process - `"RuntimeException"` in the `"server_auth"` stage or `"invalid_string"` in the `"enter_email"` stage:

```YAML
login:
  errors_by_stage:
    type: labeled_string
    description: Records the error type, if any, that occur in different stages of the login process.
    labels:
      - server_auth
      - enter_email
    ...
```

## API

Now you can use the labeled string from the application's code:

{{#include ../../tab_header.md}}

<div data-lang="Kotlin" class="tab">

```Kotlin
import org.mozilla.yourApplication.GleanMetrics.Login

Login.errorsByStage["server_auth"].set("Invalid password")
```

There are test APIs available too:

```Kotlin
import org.mozilla.yourApplication.GleanMetrics.Login

// Was anything recorded?
assertTrue(Login.errorsByStage["server_auth"].testHasValue())

// Were there any invalid labels?
assertEquals(0, Login.errorsByStage.testGetNumRecordedErrors(ErrorType.InvalidLabel))
```

</div>

<div data-lang="Swift" class="tab">

```Swift
Login.errorsByStage["server_auth"].set("Invalid password")
```

There are test APIs available too:

```Swift
@testable import Glean

// Was anything recorded?
XCTAssert(Login.errorsByStage["server_auth"].testHasValue())

// Were there any invalid labels?
XCTAssertEqual(0, Login.errorsByStage.testGetNumRecordedErrors(.invalidLabel))
```

</div>

<div data-lang="Python" class="tab">

```Python
from glean import load_metrics
metrics = load_metrics("metrics.yaml")

metrics.login.errors_by_stage["server_auth"].set("Invalid password")
```

There are test APIs available too:

```Python
# Was anything recorded?
assert metrics.login.errors_by_stage["server_auth"].test_has_value()

# Were there any invalid labels?
assert 0 == metrics.login.errors_by_stage.test_get_num_recorded_errors(
    ErrorType.INVALID_LABEL
)
```

</div>

<div data-lang="C#" class="tab">

```C#
using static Mozilla.YourApplication.GleanMetrics.Login;

Login.errorsByStage["server_auth"].Set("Invalid password");
```

There are test APIs available too:

```C#
using static Mozilla.YourApplication.GleanMetrics.Login;

// Was anything recorded?
Assert.True(Login.errorsByStage["server_auth"].TestHasValue());

// Were there any invalid labels?
Assert.Equal(0, Login.errorsByStage.TestGetNumRecordedErrors(ErrorType.InvalidLabel));
```

</div>

<div data-lang="Rust" class="tab">

```rust
use glean_metrics;

login::errors_by_stage.get("server_auth").set("Invalid password");
```

There are test APIs available too:

```rust
use glean::ErrorType;

use glean_metrics;

// Was anything recorded?
assert!(login::errors_by_stage.get("server_auth").test_get_value().is_sone());

// Were there any invalid labels?
assert_eq!(
  0,
  login::errors_by_stage.test_get_num_recorded_errors(
    ErrorType::InvalidLabel
  )
);
```

</div>

{{#include ../../tab_footer.md}}

## Limits


* Labels must conform to the [label formatting regular expression](index.md#label-format).

* Labels support lowercase alphanumeric characters; they additionally allow for dots (`.`), underscores (`_`) and/or hyphens (`-`).

* Each label must have a maximum of 60 bytes, when encoded as UTF-8.

* If the labels are specified in the `metrics.yaml`, using any label not listed in that file will be replaced with the special value `__other__`.

* If the labels aren't specified in the `metrics.yaml`, only 16 different dynamic labels may be used, after which the special value `__other__` will be used.

## Examples

* What kind of errors occurred at each step in the login process?

## Recorded Errors

* `invalid_label`: If the label contains invalid characters. Data is still recorded to the special label `__other__`.

* `invalid_label`: If the label exceeds the maximum number of allowed characters. Data is still recorded to the special label `__other__`.

## Reference

* Kotlin API docs: [`LabeledMetricType`](../../../javadoc/glean/mozilla.telemetry.glean.private/-labeled-metric-type/index.html), [`StringMetricType`](../../../javadoc/glean/mozilla.telemetry.glean.private/-string-metric-type/index.html)
* Swift API docs: [`LabeledMetricType`](../../../swift/Classes/LabeledMetricType.html), [`StringMetricType`](../../../swift/Classes/StringMetricType.html)
* Python API docs: [`LabeledMetricBase`](../../../python/glean/metrics/labeled.html), [`StringMetricType`](../../../python/glean/metrics/string.html)
