# URL

URL metrics allow recording URL-like[^1] strings.

This metric type does not support recording [data URLs](https://developer.mozilla.org/en-US/docs/Web/HTTP/Basics_of_HTTP/Data_URIs) - please get in contact with the Glean SDK team if you're missing a type.

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
<div data-lang="Kotlin" class="tab"></div>
<div data-lang="Java" class="tab"></div>
<div data-lang="Swift" class="tab"></div>
<div data-lang="Python" class="tab"></div>
<div data-lang="Rust" class="tab"></div>
<div data-lang="JavaScript" class="tab">

```js
import * as search from "./path/to/generated/files/search.js";

search.template.set("https://mysearchengine.com/");
```
</div>
<div data-lang="Firefox Desktop" class="tab"></div>
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
* [`invalid_overflow`](../../user/metrics/error-reporting.md): If the URL passed is longer than 2048 characters (before encoding).

#### Limits

* Fixed maximum URL length: 2048. Longer URLs are dropped.

## Testing API

### `testGetValue`

Gets the recorded value for a given URL metric as a (unencoded) string.

{{#include ../../../shared/tab_header.md}}

<div data-lang="Kotlin" class="tab"></div>
<div data-lang="Java" class="tab"></div>
<div data-lang="Swift" class="tab"></div>
<div data-lang="Python" class="tab"></div>
<div data-lang="Rust" class="tab"></div>
<div data-lang="JavaScript" class="tab">

```js
import * as search from "./path/to/generated/files/search.js";

assert.strictEqual(, await search.template.testGetValue());
```
</div>
<div data-lang="Firefox Desktop" class="tab"></div>

{{#include ../../../shared/tab_footer.md}}

### `testGetNumRecordedErrors`

Gets number of errors recorded for a given counter metric.

{{#include ../../../shared/tab_header.md}}

<div data-lang="Kotlin" class="tab"></div>
<div data-lang="Java" class="tab"></div>
<div data-lang="Swift" class="tab"></div>
<div data-lang="Python" class="tab"></div>
<div data-lang="Rust" class="tab"></div>
<div data-lang="JavaScript" class="tab">

```js
import * as search from "./path/to/generated/files/search.js";
import { ErrorType } from "@mozilla/glean/<platform>";

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
> ["Web activity data"](../yaml/metrics.md#category-3-web-activity-data-web_activity) or
> ["Highly sensitive data"](../yaml/metrics.md#category-4-highly-sensitive-data-highly_sensitive).

### Extra metric parameters

N/A

## Data questions

* What is the base URL used to build the search query for the search engine?

## Reference

* [JavaScript API docs](https://mozilla.github.io/glean.js/classes/core_metrics_types_url.default.html)
