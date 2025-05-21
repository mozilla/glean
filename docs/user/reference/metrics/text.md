# Text

Records a single long Unicode text, used when the limits on `String` are too low.

{{#include ../../../shared/blockquote-warning.html}}

## Important

> This type should only be used in special cases when other metrics don't fit.
> See [limitations](#limits) below.  
> Reach out to the Glean team before using this.

## Recording API

### `set`

Sets a text metric to a specific value.

{{#include ../../../shared/tab_header.md}}

<div data-lang="Kotlin" class="tab">

```Kotlin
import org.mozilla.yourApplication.GleanMetrics.Article

Article.content.set(extractedText)
```

</div>

<div data-lang="Java" class="tab">

```Java
import org.mozilla.yourApplication.GleanMetrics.Article;

Article.INSTANCE.content().set(extractedText);
```

</div>

<div data-lang="Swift" class="tab">

```Swift
Article.content.set(extractedText)
```

</div>

<div data-lang="Python" class="tab">

```Python
from glean import load_metrics
metrics = load_metrics("metrics.yaml")

metrics.article.content.set(extracted_text)
```

</div>

<div data-lang="Rust" class="tab">

```Rust
use glean_metrics::article;

article::content.set(extracted_text);
```

</div>

<div data-lang="JavaScript" class="tab">

```js
import * as article from "./path/to/generated/files/article.js";

article.content.set(extractedText);
```
</div>

<div data-lang="Firefox Desktop" class="tab"></div>

{{#include ../../../shared/tab_footer.md}}

#### Limits

* Text metrics can only be sent in [custom pings](../../user/pings/custom.md).
* Text metrics are always of data collection category 3 (`web_activity`) or category 4 (`highly_sensitive`).
* Only `ping` and `application` lifetimes are allowed.
* Fixed maximum text length: 200 kilobytes.
  Longer text is truncated. This is measured in the number of bytes when the string is encoded in UTF-8.

#### Recorded errors

* [`invalid_overflow`](../../user/metrics/error-reporting.md): if the text is too long.
* [`invalid_type`](../../user/metrics/error-reporting.md): if a non-string value is given.

## Testing API

### `testGetValue`

Gets the recorded value for a given text metric.  
Returns the string if data is stored.  
Returns `null` if no data is stored.
Has an optional argument to specify the name of the ping you wish to retrieve data from, except
in Rust where it's required. `None` or no argument will default to the first value found for `send_in_pings`.

{{#include ../../../shared/tab_header.md}}

<div data-lang="Kotlin" class="tab">

```Kotlin
import org.mozilla.yourApplication.GleanMetrics.Article

assertEquals("some content", Article.content.testGetValue())
```

</div>

<div data-lang="Java" class="tab">

```Java
import org.mozilla.yourApplication.GleanMetrics.Article;

assertEquals("some content", Article.INSTANCE.content().testGetValue());
```

</div>

<div data-lang="Swift" class="tab">

```Swift
XCTAssertEqual("some content", Article.content.testGetValue())
```

</div>

<div data-lang="Python" class="tab">

```Python
from glean import load_metrics
metrics = load_metrics("metrics.yaml")

assert "some content" == metrics.article.content.test_get_value()
```

</div>

<div data-lang="Rust" class="tab">

```Rust
use glean_metrics::article;

assert_eq!("some content", article::content.test_get_value(None).unwrap());
```

</div>

<div data-lang="JavaScript" class="tab">

```js
import * as article from "./path/to/generated/files/article.js";

assert.strictEqual("some content", await article.content.testGetValue());
```

</div>

<div data-lang="Firefox Desktop" class="tab"></div>

{{#include ../../../shared/tab_footer.md}}

### `testGetNumRecordedErrors`

Gets the number of errors recorded for a given text metric.

{{#include ../../../shared/tab_header.md}}

<div data-lang="Kotlin" class="tab">

```Kotlin
import org.mozilla.yourApplication.GleanMetrics.Article

// Was the string truncated, and an error reported?
assertEquals(
    0,
    Article.content.testGetNumRecordedErrors(ErrorType.INVALID_OVERFLOW)
)
```

</div>

<div data-lang="Java" class="tab">

```Kotlin
import org.mozilla.yourApplication.GleanMetrics.Article;

// Was the string truncated, and an error reported?
assertEquals(
    0,
    Article.content.testGetNumRecordedErrors(ErrorType.INVALID_OVERFLOW)
);
```

</div>

<div data-lang="Swift" class="tab">

```Swift
// Was the string truncated, and an error reported?
XCTAssertEqual(0, Article.content.testGetNumRecordedErrors(.invalidOverflow))
```

</div>

<div data-lang="Python" class="tab">

```Python
from glean import load_metrics
metrics = load_metrics("metrics.yaml")

# Was the string truncated, and an error reported?
assert 0 == metrics.article.content.test_get_num_recorded_errors(
    ErrorType.INVALID_OVERFLOW
)
```

</div>

<div data-lang="Rust" class="tab">

```Rust
use glean::ErrorType;
use glean_metrics::article;

// Was the string truncated, and an error reported?
assert_eq!(
  0,
  article::content.test_get_num_recorded_errors(
    ErrorType::InvalidOverflow,
    None
  )
);
```

</div>

<div data-lang="JavaScript" class="tab">

```js
import * as article from "./path/to/generated/files/article.js";
import { ErrorType } from "@mozilla/glean/error";

assert.strictEqual(
  0,
  await article.content.testGetNumRecordedErrors(ErrorType.InvalidValue)
);
```

</div>

<div data-lang="Firefox Desktop" class="tab"></div>

{{#include ../../../shared/tab_footer.md}}

## Metric parameters

Example text metric definition:

```yaml
article:
  content:
    type: text
    lifetime: ping
    send_in_pings:
      - research
    data_sensitivity:
      - web_activity
    description: >
      The plaintext content of the displayed article.
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

## Data questions

* What article content was displayed to the user?

## Reference

* [Python API docs](../../../python/glean/metrics/index.html#glean.metrics.TextMetric)
* [Rust API docs](../../../docs/glean/private/struct.TextMetric.html)
* [Swift API docs](../../../swift/Classes/TextMetric.html)
