# String

String metrics allow recording a Unicode string value with arbitrary content.

This metric type does not support recording JSON blobs
- please get in contact with the Glean team if you're missing a type.

{{#include ../../../shared/blockquote-warning.html}}

## Important

> Be careful using arbitrary strings and make sure they can't accidentally contain identifying data (like directory paths or user input).

## Recording API

### `set`

Set a string metric to a specific value.

{{#include ../../../shared/tab_header.md}}

<div data-lang="Kotlin" class="tab">

```Kotlin
import org.mozilla.yourApplication.GleanMetrics.SearchDefault

// Record a value into the metric.
SearchDefault.name.set("duck duck go")
// If it changed later, you can record the new value:
SearchDefault.name.set("wikipedia")
```

</div>

<div data-lang="Java" class="tab">

```Java
import org.mozilla.yourApplication.GleanMetrics.SearchDefault;

// Record a value into the metric.
SearchDefault.INSTANCE.name().set("duck duck go");
// If it changed later, you can record the new value:
SearchDefault.INSTANCE.name().set("wikipedia");
```

</div>

<div data-lang="Swift" class="tab">

```Swift
// Record a value into the metric.
SearchDefault.name.set("duck duck go")
// If it changed later, you can record the new value:
SearchDefault.name.set("wikipedia")
```

</div>

<div data-lang="Python" class="tab">

```Python
from glean import load_metrics
metrics = load_metrics("metrics.yaml")

# Record a value into the metric.
metrics.search_default.name.set("duck duck go")
# If it changed later, you can record the new value:
metrics.search_default.name.set("wikipedia")
```

</div>

<div data-lang="Rust" class="tab">

```Rust
use glean_metrics::search_default;

// Record a value into the metric.
search_default::name.set("duck duck go");
// If it changed later, you can record the new value:
search_default::name.set("wikipedia");
```
</div>

<div data-lang="JavaScript" class="tab">

```js
import * as searchDefault from "./path/to/generated/files/searchDefault.js";

// Record a value into the metric.
searchDefault.name.set("duck duck go");
// If it changed later, you can record the new value:
searchDefault.name.set("wikipedia");
```
</div>

<div data-lang="Firefox Desktop" class="tab">

**C++**

```c++
#include "mozilla/glean/SearchMetrics.h"

mozilla::glean::search_default::name.Set("wikipedia"_ns);
```

**JavaScript**

```js
Glean.searchDefault.name.set("wikipedia");
```

</div>

{{#include ../../../shared/tab_footer.md}}

#### Recorded errors

{{#include ../../_includes/string-errors.md}}

#### Limits

{{#include ../../_includes/string-limits.md}}

## Testing API

### `testGetValue`

Get the recorded value for a given string metric.  
Returns the string if data is stored.  
Returns a language-specific empty/null value if no data is stored.
Has an optional argument to specify the name of the ping you wish to retrieve data from, except
in Rust where it's required. `None` or no argument will default to the first value found for `send_in_pings`.

The recorded value may have been truncated. See ["Limits"](#limits) section above.

{{#include ../../../shared/tab_header.md}}

<div data-lang="Kotlin" class="tab">

```Kotlin
import org.mozilla.yourApplication.GleanMetrics.SearchDefault

// Does the string metric have the expected value?
assertEquals("wikipedia", SearchDefault.name.testGetValue())
```

</div>

<div data-lang="Java" class="tab">

```Java
import org.mozilla.yourApplication.GleanMetrics.SearchDefault;

// Does the string metric have the expected value?
assertEquals("wikipedia", SearchDefault.INSTANCE.name().testGetValue());
```

</div>

<div data-lang="Swift" class="tab">

```Swift
// Does the string metric have the expected value?
XCTAssertEqual("wikipedia", SearchDefault.name.testGetValue())
```

</div>

<div data-lang="Python" class="tab">

```Python
from glean import load_metrics
metrics = load_metrics("metrics.yaml")

# Does the string metric have the expected value?
assert "wikipedia" == metrics.search_default.name.test_get_value()
```

</div>

<div data-lang="Rust" class="tab">

```Rust
use glean_metrics::search_default;

// Does the string metric have the expected value?
assert_eq!(6, search_default::name.test_get_value(None).unwrap());
```

</div>

<div data-lang="JavaScript" class="tab">

```js
import * as searchDefault from "./path/to/generated/files/searchDefault.js";

assert.strictEqual("wikipedia", await searchDefault.name.testGetValue());
```
</div>

<div data-lang="Firefox Desktop" class="tab">

**C++**

```c++
#include "mozilla/glean/SearchMetrics.h"

// Is it clear of errors?
ASSERT_TRUE(mozilla::glean::search_default::name.TestGetValue().isOk());
// Does it have the expected value?
ASSERT_STREQ(
  "wikipedia",
  mozilla::glean::search_default::name.TestGetValue().unwrap().value().get()
);
```

**JavaScript**

```js
// Does it have the expected value?
// testGetValue will throw NS_ERROR_LOSS_OF_SIGNIFICANT_DATA on error.
Assert.equal("wikipedia", Glean.searchDefault.name.testGetValue());
```

</div>

{{#include ../../../shared/tab_footer.md}}

### `testGetNumRecordedErrors`

Gets the number of errors recorded for a given string metric.

{{#include ../../../shared/tab_header.md}}

<div data-lang="Kotlin" class="tab">

```Kotlin
import org.mozilla.yourApplication.GleanMetrics.SearchDefault

// Was the string truncated, and an error reported?
assertEquals(
    0,
    SearchDefault.name.testGetNumRecordedErrors(ErrorType.INVALID_OVERFLOW)
)
```

</div>

<div data-lang="Java" class="tab">

```Java
import org.mozilla.yourApplication.GleanMetrics.SearchDefault;

// Was the string truncated, and an error reported?
assertEquals(
    0,
    SearchDefault.INSTANCE.name().testGetNumRecordedErrors(ErrorType.INVALID_OVERFLOW)
);
```

</div>

<div data-lang="Swift" class="tab">

```Swift
// Was the string truncated, and an error reported?
XCTAssertEqual(0, SearchDefault.name.testGetNumRecordedErrors(.invalidOverflow))
```

</div>

<div data-lang="Python" class="tab">

```Python
from glean import load_metrics
metrics = load_metrics("metrics.yaml")

# Was the string truncated, and an error reported?
assert 0 == metrics.search_default.name.test_get_num_recorded_errors(
    ErrorType.INVALID_OVERFLOW
)
```

</div>

<div data-lang="Rust" class="tab">

```Rust
use glean::ErrorType;
use glean_metrics::search_default;

// Was the string truncated, and an error reported?
assert_eq!(
  0,
  search_default::name.test_get_num_recorded_errors(
    ErrorType::InvalidOverflow
  )
);
```
</div>

<div data-lang="JavaScript" class="tab">

```js
import * as searchDefault from "./path/to/generated/files/searchDefault.js";
import { ErrorType } from "@mozilla/glean/error";

// Was the string truncated, and an error reported?
assert.strictEqual(
  0,
  await searchDefault.name.testGetNumRecordedErrors(ErrorType.InvalidOverflow)
);
```
</div>

<div data-lang="Firefox Desktop" class="tab" data-info="Firefox Desktop uses testGetValue to communicate errors"></div>

{{#include ../../../shared/tab_footer.md}}

## Metric parameters

Example string metric definition:

```yaml
controls:
  refresh_pressed:
    type: string
    description: >
      The name of the default search engine.
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

### Extra metric parameters

N/A

## Data questions

* Record the operating system name with a value of `"android"`.

* Recording the device model with a value of `"SAMSUNG-SGH-I997"`.

## Reference

* [Python API docs](../../../python/glean/metrics/string.html)
* [Rust API docs](../../../docs/glean/private/struct.StringMetric.html)
