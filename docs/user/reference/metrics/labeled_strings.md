# Labeled Strings

Labeled strings record multiple Unicode string values, each under a different label.

## Recording API

### `set`

Sets one of the labels in a labeled string metric to a specific value.

{{#include ../../../shared/tab_header.md}}

<div data-lang="Kotlin" class="tab">

```Kotlin
import org.mozilla.yourApplication.GleanMetrics.Login

Login.errorsByStage["server_auth"].set("Invalid password")
```
</div>

<div data-lang="Java" class="tab">

```Java
import org.mozilla.yourApplication.GleanMetrics.Login;

Login.INSTANCE.errorsByStage["server_auth"].set("Invalid password");
```
</div>

<div data-lang="Swift" class="tab">

```Swift
Login.errorsByStage["server_auth"].set("Invalid password")
```
</div>

<div data-lang="Python" class="tab">

```Python
from glean import load_metrics
metrics = load_metrics("metrics.yaml")

metrics.login.errors_by_stage["server_auth"].set("Invalid password")
```
</div>

<div data-lang="Rust" class="tab">

```rust
use glean_metrics;

login::errors_by_stage.get("server_auth").set("Invalid password");
```
</div>

<div data-lang="JavaScript" class="tab">

```js
import * as login from "./path/to/generated/files/login.js";

login.errorsByStage["server_auth"].set("Invalid password");
```
</div>

<div data-lang="Firefox Desktop" class="tab">

**C++**
```cpp
#include "mozilla/glean/GleanMetrics.h"

mozilla::glean::login::errors_by_stage.Get("server_auth"_ns).Set("Invalid password"_ns);
```

**JavaScript**
```js
Glean.login.errorsByStage["server_auth"].set("Invalid password");
```
</div>

{{#include ../../../shared/tab_footer.md}}

#### Limits

* Fixed maximum string length: 100. Longer strings are truncated. This is measured in the number of bytes when the string is encoded in UTF-8.

#### Recorded Errors

* [`invalid_overflow`](../../user/metrics/error-reporting.md): if the string is too long. (Prior to Glean 31.5.0, this recorded an `invalid_value`).
* [`invalid_label`](../../user/metrics/error-reporting.md):
  * If the label contains invalid characters. Data is still recorded to the special label `__other__`.
  * If the label exceeds the maximum number of allowed characters. Data is still recorded to the special label `__other__`.
* [`invalid_type`](../../user/metrics/error-reporting.md): if a non-string value is given.

## Testing API

### `testGetValue`

Gets the recorded value for a given label in a labeled string metric.

{{#include ../../../shared/tab_header.md}}

<div data-lang="Kotlin" class="tab">

```Kotlin
import org.mozilla.yourApplication.GleanMetrics.Login

// Does the metric have the expected value?
assertTrue(Login.errorsByStage["server_auth"].testGetValue())
```
</div>

<div data-lang="Java" class="tab">

```Java
import org.mozilla.yourApplication.GleanMetrics.Login;

// Does the metric have the expected value?
assertTrue(Login.INSTANCE.errorsByStage["server_auth"].testGetValue());
```
</div>

<div data-lang="Swift" class="tab">

```Swift
// Does the metric have the expected value?
XCTAssert(Login.errorsByStage["server_auth"].testGetValue())
```

</div>

<div data-lang="Python" class="tab">

```Python
from glean import load_metrics
metrics = load_metrics("metrics.yaml")

# Does the metric have the expected value?
assert "Invalid password" == metrics.login.errors_by_stage["server_auth"].testGetValue())
```
</div>

<div data-lang="Rust" class="tab">

```rust
use glean_metrics;

// Does the metric have the expected value?
assert!(login::errors_by_stage.get("server_auth").test_get_value());
```
</div>

<div data-lang="JavaScript" class="tab">

```js
import * as login from "./path/to/generated/files/login.js";

// Does the metric have the expected value?
assert.strictEqual("Invalid password", await metrics.login.errorsByStage["server_auth"].testGetValue())
```
</div>

<div data-lang="Firefox Desktop" class="tab">

**C++**
```cpp
#include "mozilla/glean/GleanMetrics.h"

ASSERT_STREQ("Invalid password",
             mozilla::glean::login::errors_by_stage.Get("server_auth"_ns)
                .TestGetValue()
                .unwrap()
                .ref()
                .get());
```

**JavaScript**
```js
Assert.equal("Invalid password", Glean.login.errorsByStage["server_auth"].testGetValue());
```
</div>
{{#include ../../../shared/tab_footer.md}}

### `testHasValue`

Whether or not **any** value was recorded for a given label in a labeled string metric.

{{#include ../../../shared/tab_header.md}}

<div data-lang="Kotlin" class="tab">

```Kotlin
import org.mozilla.yourApplication.GleanMetrics.Login

// Was anything recorded?
assertTrue(Login.errorsByStage["server_auth"].testHasValue())
```
</div>

<div data-lang="Java" class="tab">

```Java
import org.mozilla.yourApplication.GleanMetrics.Login;

// Was anything recorded?
assertTrue(Login.INSTANCE.errorsByStage["server_auth"].testHasValue());
```
</div>

<div data-lang="Swift" class="tab">

```Swift
// Was anything recorded?
XCTAssert(Login.errorsByStage["server_auth"].testHasValue())
```

</div>

<div data-lang="Python" class="tab">

```Python
# Was anything recorded?
assert metrics.login.errors_by_stage["server_auth"].test_has_value()
```
</div>

<div data-lang="Rust" class="tab"></div>

<div data-lang="JavaScript" class="tab"></div>

<div data-lang="Firefox Desktop" class="tab"></div>

{{#include ../../../shared/tab_footer.md}}

### `testGetNumRecordedErrors`

Gets the number of errors recorded for a given labeled string metric in total.

{{#include ../../../shared/tab_header.md}}

<div data-lang="Kotlin" class="tab">

```Kotlin
import org.mozilla.yourApplication.GleanMetrics.Login

// Were there any invalid labels?
assertEquals(0, Login.errorsByStage.testGetNumRecordedErrors(ErrorType.InvalidLabel))
```
</div>

<div data-lang="Java" class="tab">

```Java
import org.mozilla.yourApplication.GleanMetrics.Login;

// Were there any invalid labels?
assertEquals(0, Login.INSTANCE.errorsByStage.testGetNumRecordedErrors(ErrorType.InvalidLabel));
```
</div>

<div data-lang="Swift" class="tab">

```Swift
// Were there any invalid labels?
XCTAssertEqual(0, Login.errorsByStage.testGetNumRecordedErrors(.invalidLabel))
```

</div>

<div data-lang="Python" class="tab">

```Python
from glean import load_metrics
metrics = load_metrics("metrics.yaml")

# Were there any invalid labels?
assert 0 == metrics.login.errors_by_stage.test_get_num_recorded_errors(
    ErrorType.INVALID_LABEL
)
```
</div>

<div data-lang="Rust" class="tab">

```rust
use glean::ErrorType;

use glean_metrics;

// Were there any invalid labels?
assert_eq!(
  0,
  login::errors_by_stage.test_get_num_recorded_errors(
    ErrorType::InvalidLabel
  )
);
```
</div>

<div data-lang="JavaScript" class="tab">

```js
import * as login from "./path/to/generated/files/login.js";
import { ErrorType } from "@mozilla/glean/<platform>";

// Were there any invalid labels?
assert(
  0,
  await login.errorsByStage.testGetNumRecordedErrors(ErrorType.InvalidLabel)
);
```
</div>

<div data-lang="Firefox Desktop" calss="tab"></div>

{{#include ../../../shared/tab_footer.md}}

## Metric parameters

Example labeled boolean metric definition:

```YAML
login:
  errors_by_stage:
    type: labeled_string
    description: Records the error type, if any, that occur in different stages of the login process.
    bugs:
      - https://bugzilla.mozilla.org/000000
    data_reviews:
      - https://bugzilla.mozilla.org/show_bug.cgi?id=000000#c3
    notification_emails:
      - me@mozilla.com
    expires: 2020-10-01
    labels:
      - server_auth
      - enter_email
      ...
```

### Extra metric parameters

{{#include ../../_includes/labels-parameter.md}}

## Data questions

* What kinds of errors occurred at each step in the login process?

## Reference

* Swift API docs: [`LabeledMetricType`](../../../swift/Classes/LabeledMetricType.html), [`StringMetricType`](../../../swift/Classes/StringMetricType.html)
* Python API docs: [`LabeledMetricBase`](../../../python/glean/metrics/labeled.html), [`StringMetricType`](../../../python/glean/metrics/string.html)
* Rust API docs: [`LabeledMetric`](../../../docs/glean/private/struct.LabeledMetric.html), [`StringMetricType`](../../../docs/glean/private/struct.StringMetric.html)
* JavaScript API docs: [`LabeledMetricType`](https://mozilla.github.io/glean.js/classes/core_metrics_types_labeled.default.html), [`StringMetricType`](https://mozilla.github.io/glean.js/classes/core_metrics_types_string.default.html)
