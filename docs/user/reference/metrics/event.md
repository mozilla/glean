# Events

Events allow recording of e.g. individual occurrences of user actions,
say every time a view was open and from where.

Each event contains the following data:

- A timestamp, in milliseconds. The first event in any ping always has a value of `0`, and subsequent event timestamps are relative to it.
  - If sending events in custom pings, see [note](../../user/pings/custom.md#the-gleanrestarted-event) on event timestamp calculation throughout restarts.
- The name of the event.
- A set of key-value pairs, where the keys are predefined in the `extra_keys` metric parameter, and the values are strings.

{{#include ../../../shared/blockquote-info.html}}

## Are you sure you need an event metric?

> Events are best-suited to measuring user behavior, where the frequency of events is relatively low and the order of the events matters. Therefore, events should be the default choice for most user-behavior telemetry.
>
> For other types of telemetry (e.g. performance or stability), events may be too expensive metric type to record, transmit, store and, most importantly, analyze. In those cases, consider lighter metrics, such as [counters](counter.md).
>  
> When in doubt, refer to the
> [metric type choosing guide](../../user/metrics/adding-new-metrics.mdl#choosing-a-metric-type).

## Recording API

### `record(object)`

Record a new event, with optional typed extra values.
See [Extra metrics parameters](#extra-metric-parameters).

{{#include ../../../shared/tab_header.md}}

<div data-lang="Kotlin" class="tab">

Note that an `enum` has been generated for handling the `extra_keys`: it has the same name as the event metric, with `Extra` added.

```Kotlin
import org.mozilla.yourApplication.GleanMetrics.Views

Views.loginOpened.record(Views.loginOpenedExtra(sourceOfLogin = "toolbar"))
```

</div>

<div data-lang="Java" class="tab"></div>

<div data-lang="Swift" class="tab">

Note that an `enum` has been generated for handling the `extra_keys`: it has the same name as the event metric, with `Extra` added.

```Swift
Views.loginOpened.record(LoginOpenedExtra(sourceOfLogin: "toolbar"))
```

</div>

<div data-lang="Python" class="tab">

Note that a `class` has been generated for handling the `extra_keys`: it has the same name as the event metric, with `Extra` added.

```Python
from glean import load_metrics
metrics = load_metrics("metrics.yaml")

metrics.views.login_opened.record(metrics.views.LoginOpenedExtra(sourceOfLogin="toolbar"))
```

</div>

<div data-lang="Rust" class="tab">

Note that an `enum` has been generated for handling the `extra_keys`: it has the same name as the event metric, with `Keys` added.

```Rust
use metrics::views::{self, LoginOpenedExtra};

let extra = LoginOpenedExtra { source_of_login: Some("toolbar".to_string()) };
views::login_opened.record(extra);
```

</div>

<div data-lang="JavaScript" class="tab">

```js
import * as views from "./path/to/generated/files/views.js";

views.loginOpened.record({ sourceOfLogin: "toolbar" });
```

</div>

<div data-lang="Firefox Desktop" class="tab">

**C++**

```c++
#include "mozilla/glean/GleanMetrics.h"

using mozilla::glean::views::LoginOpenedExtra;
LoginOpenedExtra extra = { .source_of_login = Some("value"_ns) };
mozilla::glean::views::login_opened.Record(std::move(extra))
```

**JavaScript**

```js
const extra = { sourceOfLogin: "toolbar" };
Glean.views.loginOpened.record(extra);
```

</div>

{{#include ../../../shared/tab_footer.md}}

#### Recorded errors

* [`invalid_overflow`](../../user/metrics/error-reporting.md):
  if any of the values in the `extras` object are greater than 500 bytes in length.
  (Prior to Glean 31.5.0, this recorded an `invalid_value`).
* [`invalid_value`](../../user/metrics/error-reporting.md): if there is an attempt to record to an extra key which is not allowed i.e. an extra key that has not been listed in the YAML registry file.
* [`invalid_type`](../../user/metrics/error-reporting.md): if the extra value given is not the expected type.

## Testing API

### `testGetValue`

Get the list of recorded events.
Returns a language-specific empty/null value if no data is stored.

{{#include ../../../shared/tab_header.md}}

<div data-lang="Kotlin" class="tab">

```Kotlin
import org.mozilla.yourApplication.GleanMetrics.Views

val snapshot = Views.loginOpened.testGetValue()
assertEquals(2, snapshot.size)
val first = snapshot.single()
assertEquals("login_opened", first.name)
```

</div>

<div data-lang="Java" class="tab">

```Java
import org.mozilla.yourApplication.GleanMetrics.Views

assertEquals(Views.INSTANCE.loginOpened().testGetValue().size)
```

</div>

<div data-lang="Swift" class="tab">

```swift
val snapshot = try! Views.loginOpened.testGetValue()
XCTAssertEqual(2, snapshot.size)
val first = snapshot[0]
XCTAssertEqual("login_opened", first.name)
```

</div>

<div data-lang="Python" class="tab">

```Python
from glean import load_metrics
metrics = load_metrics("metrics.yaml")

snapshot = metrics.views.login_opened.test_get_value()
assert 2 == len(snapshot)
first = snapshot[0]
assert "login_opened" == first.name
```

</div>

<div data-lang="Rust" class="tab">

```Rust
use metrics::views;

var snapshot = views::login_opened.test_get_value(None).unwrap();
assert_eq!(2, snapshot.len());
let first = &snapshot[0];
assert_eq!("login_opened", first.name);
```

</div>

<div data-lang="JavaScript" class="tab">

```js
import * as views from "./path/to/generated/files/views.js";

const snapshot = await views.loginOpened.testGetValue();
assert.strictEqual(2, snapshot.length);
const first = snapshot[0]
assert.strictEqual("login_opened", first.name)
```

</div>

<div data-lang="Firefox Desktop" class="tab">

**C++**

```c++
#include "mozilla/glean/GleanMetrics.h"

auto optEvents = mozilla::glean::views::login_opened.TestGetValue();
auto events = optEvents.extract();
ASSERT_EQ(2UL, events.Length());
ASSERT_STREQ("login_opened", events[0].mName.get());
```

**JavaScript**

```js
var events = Glean.views.loginOpened.testGetValue();
Assert.equal(2, events.length);
Assert.equal("login_opened", events[0].name);
```

</div>

{{#include ../../../shared/tab_footer.md}}

### `testGetNumRecordedErrors`

Get the number of errors recorded for a given event metric.

{{#include ../../../shared/tab_header.md}}

<div data-lang="Kotlin" class="tab">

```Kotlin
import mozilla.telemetry.glean.testing.ErrorType
import org.mozilla.yourApplication.GleanMetrics.Views

assertEquals(
    0,
    Views.loginOpened.testGetNumRecordedErrors(ErrorType.INVALID_OVERFLOW)
)
```

</div>

<div data-lang="Java" class="tab">

```Kotlin
import mozilla.telemetry.glean.testing.ErrorType
import org.mozilla.yourApplication.GleanMetrics.Views

assertEquals(
    0,
    Views.INSTANCE.loginOpened().testGetNumRecordedErrors(ErrorType.INVALID_OVERFLOW)
)
```

</div>

<div data-lang="Swift" class="tab">

```swift
XCTAssertEqual(0, Views.loginOpened.testGetNumRecordedErrors(.invalidOverflow))
```
</div>

<div data-lang="Python" class="tab">

```Python
from glean import load_metrics
from glean.testing import ErrorType
metrics = load_metrics("metrics.yaml")

assert 0 == metrics.views.login_opened.test_get_num_recorded_errors(
    ErrorType.INVALID_OVERFLOW
)
```

</div>

<div data-lang="Rust" class="tab">

```Rust
use glean::ErrorType;
use metrics::views;

assert_eq!(
    0,
    views::login_opened.test_get_num_recorded_errors(
        ErrorType::InvalidOverflow,
        None
    )
);
```

</div>

<div data-lang="JavaScript" class="tab">

```js
import * as views from "./path/to/generated/files/views.js";
import { ErrorType } from "@mozilla/glean/error";

assert.strictEqual(
  0,
  await views.loginOpened.testGetNumRecordedErrors(ErrorType.InvalidValue)
);
```
</div>

<div data-lang="Firefox Desktop" class="tab"></div>

{{#include ../../../shared/tab_footer.md}}

## Metric parameters

Example event metric definition:

```yaml
views:
  login_opened:
    type: event
    description: |
      Recorded when the login view is opened.
    bugs:
      - https://bugzilla.mozilla.org/000000
    data_reviews:
      - https://bugzilla.mozilla.org/show_bug.cgi?id=000000#c3
    notification_emails:
      - me@mozilla.com
    expires: 2020-10-01
    extra_keys:
      source_of_login:
        description: The source from which the login view was opened, e.g. "toolbar".
        type: string
```

For a full reference on metrics parameters common to all metric types,
refer to the [metrics YAML registry format](../yaml/metrics.md) reference page.

{{#include ../../../shared/blockquote-info.html}}

#### Events require `lifetime: ping`.

> Recorded events are always sent in their respective pings and then cleared.
> They cannot be persisted longer.
> The `glean_parser` will reject any other lifetime.

### Extra metric parameters

#### `extra_keys`

The acceptable keys on the "extra" object sent with events.
A maximum of 10 extra keys is allowed.

Each extra key contains additional metadata:

- `description`: **Required.** A description of the key.
* `type`: The type of value this extra key can hold. One of `string`, `boolean`, `quantity`. Defaults to `string`.
  **Note**: If not specified only the legacy API on `record` is available.

## Data questions

* When and from where was the login view opened?

## Limits

* When 500 events are queued on the client an "events" ping is immediately sent.
* The `extra_keys` allows for a maximum of 15 keys.
* The keys in the `extra_keys` list must be in dotted snake case, with a maximum length of 40 bytes, when encoded as UTF-8.
* The values in the `extras` object have a maximum length of 500 bytes when serialized and encoded as UTF-8.
  Longer values are truncated, and an `invalid_overflow` error is recorded.

## Reference

* [Swift API docs](../../../swift/Classes/EventMetricType.html)
* [Python API docs](../../../python/glean/metrics/event.html)
* [Rust API docs](../../../docs/glean/private/event/struct.EventMetric.html)
* [JavaScript API docs](https://mozilla.github.io/glean.js/classes/core_metrics_types_event.default.html)
