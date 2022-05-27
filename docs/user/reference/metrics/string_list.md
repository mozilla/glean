# String List

Strings lists are used for recording a list of Unicode string values, such as the names of the enabled search engines.

{{#include ../../../shared/blockquote-warning.html}}

##### Important

> Be careful using arbitrary strings and make sure they can't accidentally contain identifying data (like directory paths or user input).

## Recording API

### `add`

Add a new string to the list.

{{#include ../../../shared/tab_header.md}}
<div data-lang="Kotlin" class="tab">

```Kotlin
import org.mozilla.yourApplication.GleanMetrics.Search

Search.engines.add("wikipedia")
Search.engines.add("duck duck go")
```

</div>
<div data-lang="Java" class="tab">

```Java
import org.mozilla.yourApplication.GleanMetrics.Search;

Search.INSTANCE.engines().add("wikipedia");
Search.INSTANCE.engines().add("duck duck go");
```

</div>
<div data-lang="Swift" class="tab">

```Swift
Search.engines.add("wikipedia")
Search.engines.add("duck duck go")
```

</div>
<div data-lang="Python" class="tab">

```Python
from glean import load_metrics
metrics = load_metrics("metrics.yaml")

metrics.search.engines.add("wikipedia")
metrics.search.engines.add("duck duck go")
```

</div>
<div data-lang="Rust" class="tab">

```Rust
use glean_metrics::search;

search::engines.add("wikipedia".to_string());
search::engines.add("duck duck go".to_string());
```

</div>
<div data-lang="JavaScript" class="tab">

```js
Glean.search.engines.add("wikipedia");
Glean.search.engines.add("duck duck go");
```

</div>
<div data-lang="Firefox Desktop" class="tab">

**C++**

```cpp
#include "mozilla/glean/GleanMetrics.h"

mozilla::glean::search::engines.Add("wikipedia"_ns);
mozilla::glean::search::engines.Add("duck duck go"_ns);
```

**JavaScript**

```js
Glean.search.engines.add("wikipedia");
Glean.search.engines.add("duck duck go");
```

</div>
{{#include ../../../shared/tab_footer.md}}

#### Recorded errors

* [`invalid_overflow`](../../user/metrics/error-reporting.md): if the string is too long. (Prior to Glean 31.5.0, this recorded an `invalid_value`).
* [`invalid_value`](../../user/metrics/error-reporting.md): if the list is too long.
* [`invalid_type`](../../user/metrics/error-reporting.md): if a non-string value is given.

#### Limits

* Fixed maximum string length: 50. Longer strings are truncated. This is measured in the number of bytes when the string is encoded in UTF-8.
* Fixed maximum list length: 20 items. Additional strings are dropped.

### `set`

Set the metric to a specific list of strings.

{{#include ../../../shared/tab_header.md}}
<div data-lang="Kotlin" class="tab">

```Kotlin
import org.mozilla.yourApplication.GleanMetrics.Search

Search.engines.set(listOf("wikipedia", "duck duck go"))
```

</div>
<div data-lang="Java" class="tab">

```Java
import org.mozilla.yourApplication.GleanMetrics.Search;

Search.INSTANCE.engines().set(listOf("wikipedia", "duck duck go"));
```

</div>
<div data-lang="Swift" class="tab">

```Swift
Search.engines.set(["wikipedia", "duck duck go"])
```

</div>
<div data-lang="Python" class="tab">

```Python
from glean import load_metrics
metrics = load_metrics("metrics.yaml")

metrics.search.engines.set(["wikipedia", "duck duck go"])
```

</div>
<div data-lang="Rust" class="tab">

```Rust
use glean_metrics::search;

search::engines.set(vec!["wikipedia".to_string(), "duck duck go".to_string()])
```

</div>
<div data-lang="JavaScript" class="tab">

```js
Glean.search.engines.set(["wikipedia", "duck duck go"]);
```

</div>
<div data-lang="Firefox Desktop" class="tab">

**C++**

```cpp
#include "mozilla/glean/GleanMetrics.h"

mozilla::glean::search::engines.Set({"wikipedia"_ns, "duck duck go"_ns});
```

**JavaScript**

```js
Glean.search.engines.set(["wikipedia", "duck duck go"]);
```

</div>
{{#include ../../../shared/tab_footer.md}}

#### Recorded errors

* `invalid_overflow`: if the string is too long. (Prior to Glean 31.5.0, this recorded an `invalid_value`).
* `invalid_value`: if the list is too long.
* [`invalid_type`](../../user/metrics/error-reporting.md): if a non-string array is given.

#### Limits

* Fixed maximum string length: 50. Longer strings are truncated. This is measured in the number of bytes when the string is encoded in UTF-8.
* Fixed maximum list length: 20 items. Additional strings are dropped.

## Testing API

### `testGetValue`

Gets the recorded value for a given string list metric.  
Returns the list of strings if data is stored.  
Returns a language-specific empty/null value if no data is stored.

{{#include ../../../shared/tab_header.md}}
<div data-lang="Kotlin" class="tab">

```Kotlin
import org.mozilla.yourApplication.GleanMetrics.Search

assertEquals(listOf("Google", "DuckDuckGo"), Search.engines.testGetValue())
```

</div>
<div data-lang="Java" class="tab">

```Java
import org.mozilla.yourApplication.GleanMetrics.Search;

assertEquals(
    Arrays.asList("Google", "DuckDuckGo"),
    Search.INSTANCE.engines().testGetValue()
);
```

</div>
<div data-lang="Swift" class="tab">

```Swift
XCTAssertEqual(["Google", "DuckDuckGo"], Search.engines.testGetValue())
```

</div>
<div data-lang="Python" class="tab">

```python
from glean import load_metrics
metrics = load_metrics("metrics.yaml")

assert ["Google", "DuckDuckGo"] == metrics.search.engines.test_get_value()
```

</div>
<div data-lang="Rust" class="tab">

```Rust
use glean_metrics::search;

assert_eq!(
    vec!["Google".to_string(), "DuckDuckGo".to_string()],
    search::engines.test_get_value(None).unwrap()
);
```

</div>
<div data-lang="JavaScript" class="tab">

```js
// testGetValue will throw NS_ERROR_LOSS_OF_SIGNIFICANT_DATA on error.
const engines = Glean.search.engines.testGetValue();
Assert.ok(engines.includes("wikipedia"));
Assert.ok(engines.includes("duck duck go"));
```

</div>
<div data-lang="Firefox Desktop" class="tab">

**C++**

```cpp
#include "mozilla/glean/GleanMetrics.h"

ASSERT_EQUAL(mozilla::glean::search::engines.TestGetValue().isOk());
nsTArray<nsCString> list = mozilla::glean::search::engines.TestGetValue().unwrap();
ASSERT_TRUE(list.Contains("wikipedia"_ns));
ASSERT_TRUE(list.Constains("duck duck go"_ns));
```

**JavaScript**

```js
// testGetValue will throw NS_ERROR_LOSS_OF_SIGNIFICANT_DATA on error.
const engines = Glean.search.engines.testGetValue();
Assert.ok(engines.includes("wikipedia"));
Assert.ok(engines.includes("duck duck go"));
```

</div>
{{#include ../../../shared/tab_footer.md}}

### `testGetNumRecordedErrors`

Gets the number of errors recorded for a given string list metric.

{{#include ../../../shared/tab_header.md}}
<div data-lang="Kotlin" class="tab">

```Kotlin
import org.mozilla.yourApplication.GleanMetrics.Search

assertEquals(
    0,
    Search.engines.testGetNumRecordedErrors(ErrorType.INVALID_VALUE)
)
```

</div>
<div data-lang="Java" class="tab">

```Java
import org.mozilla.yourApplication.GleanMetrics.Search;

assertEquals(
    0,
    Search.INSTANCE.engines().testGetNumRecordedErrors(ErrorType.INVALID_VALUE)
);
```

</div>
<div data-lang="Swift" class="tab">

```Swift
// Were any of the values too long, and thus an error was recorded?
XCTAssertEqual(0, Search.engines.testGetNumRecordedErrors(.invalidValue))
```

</div>
<div data-lang="Python" class="tab">

```Python
from glean import load_metrics
metrics = load_metrics("metrics.yaml")

assert 0 == metrics.search.engines.test_get_num_recorded_errors(
    ErrorType.INVALID_VALUE
)
```

</div>
<div data-lang="Rust" class="tab">

```Rust
use glean::ErrorType;
use glean_metrics::search;

assert_eq!(
    0,
    search::engines.test_get_num_recorded_errors(ErrorType::InvalidValue)
);
```

</div>
<div data-lang="JavaScript" class="tab"></div>
<div data-lang="Firefox Desktop" class="tab" data-bug="Firefox Desktop uses testGetValue to communicate errors"></div>
{{#include ../../../shared/tab_footer.md}}

## Metric parameters

Example string list metric definition:

```YAML
search:
  engines:
    type: string_list
    description: >
      Records the name of the enabled search engines.
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

* Which search engines are enabled?

## Reference

* [Swift API docs](../../../swift/Classes/StringListMetricType.html)
* [Python API docs](../../../python/glean/metrics/string_list.html)
* [Rust API docs](../../../docs/glean/private/struct.StringListMetric.html)
