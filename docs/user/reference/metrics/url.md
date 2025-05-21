# URL

URL metrics allow recording URL-like[^1] strings.

This metric type does not support recording [data URLs](https://developer.mozilla.org/en-US/docs/Web/HTTP/Basics_of_HTTP/Data_URIs) - please get in contact with the Glean team if you're missing a type.

{{#include ../../../shared/blockquote-warning.html}}

## Important

> Be careful using arbitrary URLs and make sure they can't accidentally contain identifying data
> (like directory paths, user input or credentials).

[^1]: The Glean SDKs specifically do not validate if a URL is fully spec compliant,
all the validations performed are the ones listed in the
["Recorded errors"](#recorded-errors) section of this page.

## Recording API

### `set`

Set a URL metric to a specific string value.

{{#include ../../../shared/tab_header.md}}
<div data-lang="Kotlin" class="tab">

```Kotlin
import org.mozilla.yourApplication.GleanMetrics.Search

Search.template.set("https://mysearchengine.com/")
```

</div>

<div data-lang="Java" class="tab">

```Java
import org.mozilla.yourApplication.GleanMetrics.Search;

Search.INSTANCE.template().set("https://mysearchengine.com/");
```

</div>

<div data-lang="Swift" class="tab">

```Swift
Search.template.set("https://mysearchengine.com")

// Swift's URL type is supported
let url = URL(string: "https://mysearchengine.com")!
Search.template.set(url: url)
```

</div>

<div data-lang="Python" class="tab">

```Python
from glean import load_metrics
metrics = load_metrics("metrics.yaml")

metrics.search.template.set("https://mysearchengine.com/")
```

</div>

<div data-lang="Rust" class="tab">

```Rust
use glean_metrics::search;

search::template.set("https://mysearchengine.com/");
```

</div>
<div data-lang="JavaScript" class="tab">

```js
import * as search from "./path/to/generated/files/search.js";

search.template.set("https://mysearchengine.com/");
```
</div>
<div data-lang="Firefox Desktop" class="tab">

**C++**

```c++
#include "mozilla/glean/SearchMetrics.h"

mozilla::glean::search::template.Set("https://mysearchengine.com/"_ns);
```

**JavaScript**

```js
Glean.search.template.set("https://mysearchengine.com/");
```

</div>
{{#include ../../../shared/tab_footer.md}}

### `setUrl`

Set a URL metric to a specific URL value.

{{#include ../../../shared/tab_header.md}}
<div data-lang="Kotlin" class="tab"></div>

<div data-lang="Java" class="tab"></div>

<div data-lang="Swift" class="tab"></div>

<div data-lang="Python" class="tab"></div>

<div data-lang="Rust" class="tab"></div>

<div data-lang="JavaScript" class="tab">

```js
import * as search from "./path/to/generated/files/search.js";

search.template.setUrl(new URL("https://mysearchengine.com/"));
```
</div>
<div data-lang="Firefox Desktop" class="tab"></div>
{{#include ../../../shared/tab_footer.md}}

#### Recorded errors

* [`invalid_value`](../../user/metrics/error-reporting.md):
  * If the URL passed does not start with a [scheme](https://url.spec.whatwg.org/#url-representation) followed by a `:` character.
  * If the URL passed uses the `data:` protocol.
* [`invalid_overflow`](../../user/metrics/error-reporting.md): if the URL passed is longer than 8192 characters (before encoding).
* [`invalid_type`](../../user/metrics/error-reporting.md): if a non-string value is given.

#### Limits

* Fixed maximum URL length: 8192. Longer URLs are truncated and recorded along with an `invalid_overflow` error.

## Testing API

### `testGetValue`

Gets the recorded value for a given URL metric as a (unencoded) string.  
Returns a URL if data is stored.  
Returns a language-specific empty/null value if no data is stored.
Has an optional argument to specify the name of the ping you wish to retrieve data from, except
in Rust where it's required. `None` or no argument will default to the first value found for `send_in_pings`.

{{#include ../../../shared/tab_header.md}}

<div data-lang="Kotlin" class="tab">

```Kotlin
import org.mozilla.yourApplication.GleanMetrics.Search

assertEquals("https://mysearchengine.com/", Search.template.testGetValue())
```

</div>

<div data-lang="Java" class="tab">

```Java
import org.mozilla.yourApplication.GleanMetrics.Search

assertEquals("https://mysearchengine.com/", Search.INSTANCE.template().testGetValue());
```

</div>

<div data-lang="Swift" class="tab">

```Swift
XCTAssertEqual("https://mysearchengine.com/", try Search.template.testGetValue())
```

</div>

<div data-lang="Python" class="tab">

```Python
from glean import load_metrics
metrics = load_metrics("metrics.yaml")

assert "https://mysearchengine.com/" == metrics.search.template.test_get_value()
```

</div>

<div data-lang="Rust" class="tab">

```Rust
use glean_metrics::search;

assert_eq!("https://mysearchengine.com/", search::template.test_get_value(None).unwrap());
```

</div>

<div data-lang="JavaScript" class="tab">

```js
import * as search from "./path/to/generated/files/search.js";

assert.strictEqual("https://mysearchengine.com/", await search.template.testGetValue());
```
</div>
<div data-lang="Firefox Desktop" class="tab"></div>

{{#include ../../../shared/tab_footer.md}}

### `testGetNumRecordedErrors`

Gets the number of errors recorded for a given counter metric.

{{#include ../../../shared/tab_header.md}}

<div data-lang="Kotlin" class="tab">

```Kotlin
import org.mozilla.yourApplication.GleanMetrics.Search

assertEquals(0, Search.template.testGetNumRecordedErrors(ErrorType.INVALID_VALUE))
assertEquals(0, Search.template.testGetNumRecordedErrors(ErrorType.INVALID_OVERFLOW))
```

</div>

<div data-lang="Java" class="tab">

```Java
import org.mozilla.yourApplication.GleanMetrics.Search;

assertEquals(
    0,
    Search.INSTANCE.template().testGetNumRecordedErrors(ErrorType.INVALID_VALUE)
);

assertEquals(
    0,
    Search.INSTANCE.template().testGetNumRecordedErrors(ErrorType.INVALID_OVERFLOW)
);
```

</div>

<div data-lang="Swift" class="tab">

```Swift
XCTAssertEqual(0, Search.template.testGetNumRecordedErrors(.invalidValue))
XCTAssertEqual(0, Search.template.testGetNumRecordedErrors(.invalidOverflow))
```

</div>

<div data-lang="Python" class="tab">

```Python
from glean import load_metrics
metrics = load_metrics("metrics.yaml")

assert 0 == metrics.search.template.test_get_num_recorded_errors(
    ErrorType.INVALID_VALUE
)

assert 0 == metrics.search.template.test_get_num_recorded_errors(
    ErrorType.INVALID_OVERFLOW
)
```

</div>

<div data-lang="Rust" class="tab">

```Rust
use glean::ErrorType;
use glean_metrics::search;

assert_eq!(
  0,
  search::template.test_get_num_recorded_errors(
    ErrorType::InvalidValue
  )
);

assert_eq!(
  0,
  search::template.test_get_num_recorded_errors(
    ErrorType::InvalidOverflow
  )
);
```

</div>

<div data-lang="JavaScript" class="tab">

```js
import * as search from "./path/to/generated/files/search.js";
import { ErrorType } from "@mozilla/glean/error";

assert.strictEqual(
  0,
  await search.template.testGetNumRecordedErrors(ErrorType.InvalidValue)
);
assert.strictEqual(
  0,
  await search.template.testGetNumRecordedErrors(ErrorType.InvalidOverflow)
);
```
</div>
<div data-lang="Firefox Desktop" class="tab" data-info="Firefox Desktop uses testGetValue to communicate errors"></div>

{{#include ../../../shared/tab_footer.md}}

## Metric parameters

Example URL metric definition:

```yaml
search:
  template:
    type: url
    description: >
      The base URL used to build the search query for the search engine.
    bugs:
      - https://bugzilla.mozilla.org/000000
    data_reviews:
      - https://bugzilla.mozilla.org/show_bug.cgi?id=000000#c3
    notification_emails:
      - me@mozilla.com
    expires: 2020-10-01
    data_sensitivity:
      - web_activity
```

For a full reference on metrics parameters common to all metric types,
refer to the metrics [YAML format](../yaml/index.md) reference page.

{{#include ../../../shared/blockquote-warning.html}}

## Note on `data_sensitivity` of URL metrics

> URL metrics can only either be on categories 3 or 4, namely
> ["Stored Content & Communications"](../yaml/metrics.md#category-3-stored-content--communications-stored_content) or
> ["Highly sensitive data"](../yaml/metrics.md#category-4-highly-sensitive-data-or-clearly-identifiable-personal-data-highly_sensitive).

### Extra metric parameters

N/A

## Data questions

* What is the base URL used to build the search query for the search engine?

## Reference

* [Python API docs](../../../python/glean/metrics/index.html#glean.metrics.UrlMetricType)
* [Rust API docs](../../../docs/glean/private/struct.UrlMetric.html)
* [Swift API docs](../../../swift/Classes/UrlMetric.html)
