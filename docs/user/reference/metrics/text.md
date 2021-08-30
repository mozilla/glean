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

<div data-lang="Kotlin" class="tab"></div>

<div data-lang="Java" class="tab"></div>

<div data-lang="Swift" class="tab"></div>

<div data-lang="Python" class="tab"></div>

<div data-lang="JavaScript" class="tab">

```js
import * as article from "./path/to/generated/files/article.js";

article.content.set(extractedText);
```
</div>

<div data-lang="Rust" class="tab"></div>

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

## Testing API

### `testGetValue`

Gets the recorded value for a given text metric.

{{#include ../../../shared/tab_header.md}}

<div data-lang="Kotlin" class="tab"></div>

<div data-lang="Java" class="tab"></div>

<div data-lang="Swift" class="tab"></div>

<div data-lang="Python" class="tab"></div>

<div data-lang="JavaScript" class="tab">

```js
import * as article from "./path/to/generated/files/article.js";

assert.strictEqual("some content", await article.content.testGetValue());
```

</div>

<div data-lang="Rust" class="tab"></div>

<div data-lang="Firefox Desktop" class="tab"></div>

{{#include ../../../shared/tab_footer.md}}

### `testGetNumRecordedErrors`

Gets number of errors recorded for a given text metric.

{{#include ../../../shared/tab_header.md}}

<div data-lang="Kotlin" class="tab"></div>

<div data-lang="Java" class="tab"></div>

<div data-lang="Swift" class="tab"></div>

<div data-lang="Python" class="tab"></div>

<div data-lang="JavaScript" class="tab">

```js
import * as article from "./path/to/generated/files/article.js";
import { ErrorType } from "@mozilla/glean/<platform>";

assert.strictEqual(
  0,
  await article.content.testGetNumRecordedErrors(ErrorType.InvalidValue)
);
```

</div>

<div data-lang="Rust" class="tab"></div>

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
