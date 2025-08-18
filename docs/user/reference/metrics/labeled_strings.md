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

Login.INSTANCE.errorsByStage()["server_auth"].set("Invalid password");
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

```Rust
use glean_metrics::login;

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
#include "mozilla/glean/DomWebauthnMetrics.h"

mozilla::glean::login::errors_by_stage.Get("server_auth"_ns).Set("Invalid password"_ns);
```

**JavaScript**
```js
Glean.login.errorsByStage["server_auth"].set("Invalid password");
```
</div>

{{#include ../../../shared/tab_footer.md}}

#### Recorded Errors

{{#include ../../_includes/string-errors.md}}
{{#include ../../_includes/label-errors.md}}

#### Limits

{{#include ../../_includes/string-limits.md}}
{{#include ../../_includes/label-limits.md}}

## Testing API

### `testGetValue`

Gets the recorded value for a given label in a labeled string metric.  
Returns the string if data is stored. The Glean SDK will return a map of each label with a
stored value to its value.   
Returns a language-specific empty/null value if no data is stored. The Glean SDK will always
return a map, but it will be empty if no data is stored. 
Has an optional argument to specify the name of the ping you wish to retrieve data from, except
in Rust where it's required. `None` or no argument will default to the first value found for `send_in_pings`.

{{#include ../../../shared/tab_header.md}}

<div data-lang="Kotlin" class="tab">

```Kotlin
import org.mozilla.yourApplication.GleanMetrics.Login

val values = Login.errorsByStage.testGetValue()
// Does the metric have the expected value?
assertTrue(values["server_auth"])
```
</div>

<div data-lang="Java" class="tab">

```Java
import org.mozilla.yourApplication.GleanMetrics.Login;

Map<String, ?> values = Login.INSTANCE.errorsByStage().testGetValue();
// Does the metric have the expected value?
assertTrue(values["server_auth"]);
```
</div>

<div data-lang="Swift" class="tab">

```Swift
let values = Login.errorsByStage.testGetValue()
// Does the metric have the expected value?
XCTAssert(values["server_auth"])
```

</div>

<div data-lang="Python" class="tab">

```Python
from glean import load_metrics
metrics = load_metrics("metrics.yaml")

values = metrics.login.errors_by_stage.testGetValue()
# Does the metric have the expected value?
assert "Invalid password" == values["server_auth"])
```
</div>

<div data-lang="Rust" class="tab">

```Rust
use glean_metrics::login;

let values = login::errors_by_stage.test_get_value(None).unwrap();
// Does the metric have the expected value?
assert!(values["server_auth"]);
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
#include "mozilla/glean/DomWebauthnMetrics.h"

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

### `testGetNumRecordedErrors`

Gets the number of errors recorded for a given labeled string metric in total.

{{#include ../../../shared/tab_header.md}}

<div data-lang="Kotlin" class="tab">

```Kotlin
import org.mozilla.yourApplication.GleanMetrics.Login

// Were there any invalid labels?
assertEquals(
    0,
    Login.errorsByStage.testGetNumRecordedErrors(ErrorType.INVALID_LABEL)
)
```
</div>

<div data-lang="Java" class="tab">

```Java
import org.mozilla.yourApplication.GleanMetrics.Login;

// Were there any invalid labels?
assertEquals(
    0,
    Login.INSTANCE.errorsByStage().testGetNumRecordedErrors(ErrorType.INVALID_LABEL)
);
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

```Rust
use glean::ErrorType;
use glean_metrics::login;

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
import { ErrorType } from "@mozilla/glean/error";

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

* Python API docs: [`LabeledStringMetricType`](../../../python/glean/metrics/labeled.html#glean.metrics.labeled.LabeledStringMetricType), [`StringMetricType`](../../../python/glean/metrics/index.html#glean.metrics.StringMetricType)
* Rust API docs: [`LabeledMetric`](../../../docs/glean/private/struct.LabeledMetric.html), [`StringMetric`](../../../docs/glean/private/struct.StringMetric.html)
* Swift API docs: [`LabeledMetricType`](../../../swift/Classes/LabeledMetricType.html), [`StringMetric`](../../../swift/Classes/StringMetric.html)
