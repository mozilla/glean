# Pings

{{#include ../../../shared/blockquote-info.html}}

#### Glean-owned pings are submitted automatically

> Products do not need to submit Glean built-in pings,
> as their scheduling is managed internally. The APIs
> on this page are only relevant for products defining
> [custom pings](../../user/pings/custom.md#defining-a-custom-ping).

## Submission API

### `submit`

Collect and queue a custom ping for eventual uploading.

By default, if the ping doesn't currently have any events or metrics set, `submit` will do nothing. However, if the `send_if_empty` flag is set to true in the ping definition, it will always be submitted.

It is not necessary for the caller to check if Glean is disabled before calling `submit`.
If Glean is disabled `submit` is a no-op.

For example, to submit the custom ping defined in [Adding new custom pings](../../user/pings/custom.md#defining-a-custom-ping):

{{#include ../../../shared/tab_header.md}}

<div data-lang="Kotlin" class="tab">

```kotlin
import org.mozilla.yourApplication.GleanMetrics.Pings
Pings.search.submit(
    Pings.searchReasonCodes.performed
)
```

</div>

<div data-lang="Java" class="tab">

```java
import org.mozilla.yourApplication.GleanMetrics.Pings

Pings.INSTANCE.search.submit(
    Pings.INSTANCE.searchReasonCodes.performed
);
```

</div>

<div data-lang="Swift" class="tab">

```swift
import Glean

GleanMetrics.Pings.shared.search.submit(
    reason: .performed
)
```

</div>

<div data-lang="Python" class="tab">

```Python
from glean import load_pings

pings = load_pings("pings.yaml")

pings.search.submit(pings.search_reason_codes.PERFORMED)
```

</div>

<div data-lang="Rust" class="tab">

```Rust
use glean::Pings;

pings::search.submit(pings::SearchReasonCodes::Performed);
```

</div>

<div data-lang="JavaScript" class="tab">

```js
import * as pings from "./path/to/generated/files/pings.js";

pings.search.submit(pings.searchReasonCodes.Performed);
```
</div>

<div data-lang="Firefox Desktop" class="tab">

**C++**

```cpp
mozilla::glean_pings::Search.Submit("performed"_ns);
```

**JavaScript**

```js
GleanPings.search.submit("performed");
```

</div>

{{#include ../../../shared/tab_footer.md}}

#### Unrecorded errors

* If called with a ping reason not matching one present in the ping's `reasons` definition field,
  the ping is submitted with an empty reason and an error is logged.
    * See [bug 2000701](https://bugzilla.mozilla.org/show_bug.cgi?id=2000701) for future developments.

### `setEnabled`

Called with `true`: enables the ping to store data and be able to be submitted.
Called with `false`: disables the ping, deletes stored data and refuses new data,
and doesn't submit when `submit()` is called.

You shouldn't need to call this unless your ping has `follows_collection_enabled: false`,
as Glean will take care of it for you.

{{#include ../../../shared/tab_header.md}}

<div data-lang="Kotlin" class="tab">

```kotlin
import org.mozilla.yourApplication.GleanMetrics.Pings

Pings.search.setEnabled(false)
```

</div>

<div data-lang="Java" class="tab">

```java
import org.mozilla.yourApplication.GleanMetrics.Pings

Pings.INSTANCE.search.setEnabled(false);
```

</div>

<div data-lang="Swift" class="tab">

```swift
import Glean

GleanMetrics.Pings.shared.search.setEnabled(false)
```

</div>

<div data-lang="Python" class="tab">

```Python
from glean import load_pings

pings = load_pings("pings.yaml")

pings.search.set_enabled(False)
```

</div>

<div data-lang="Rust" class="tab">

```Rust
use glean::Pings;

pings::search.set_enabled(false);
```

</div>

<div data-lang="JavaScript" class="tab"></div>

<div data-lang="Firefox Desktop" class="tab">

**C++**

```cpp
mozilla::glean_pings::Search.SetEnabled(false);
```

**JavaScript**

```js
GleanPings.search.setEnabled(false);
```

</div>

{{#include ../../../shared/tab_footer.md}}

## Testing API

### `getRegisteredPingNames`

Gets a set of the currently registered ping names.

Useful when debugging to know which pings are able to be sent.

{{#include ../../../shared/tab_header.md}}

<div data-lang="Kotlin" class="tab">

```kotlin
val knownPings = Glean.getRegisteredPingNames()
```

</div>

<div data-lang="Swift" class="tab">

```Swift
let knownPings = Glean.shared.getRegisteredPingNames()
```

</div>

<div data-lang="Python" class="tab"></div>

<div data-lang="Rust" class="tab">

```Rust
let known_pings = glean.get_registered_ping_names();
```

</div>

{{#include ../../../shared/tab_footer.md}}

### `testBeforeNextSubmit`

Runs a validation function before the ping is collected.

{{#include ../../../shared/tab_header.md}}

<div data-lang="Kotlin" class="tab">

```kotlin
import org.mozilla.yourApplication.GleanMetrics.Search
import org.mozilla.yourApplication.GleanMetrics.Pings

// Record some data.
Search.defaultEngine.add(5);

// Instruct the ping API to validate the ping data.
var validatorRun = false
Pings.search.testBeforeNextSubmit { reason ->
    assertEquals(Pings.searchReasonCodes.performed, reason)
    assertEquals(5, Search.defaultEngine.testGetValue())
    validatorRun = true
}

// Submit the ping.
Pings.search.submit(
    Pings.searchReasonCodes.performed
)

// Verify that the validator run.
assertTrue(validatorRun)
```

</div>

<div data-lang="Java" class="tab">

```java
import org.mozilla.yourApplication.GleanMetrics.Search
import org.mozilla.yourApplication.GleanMetrics.Pings

// Record some data.
Search.INSTANCE.defaultEngine.add(5);

// Instruct the ping API to validate the ping data.
boolean validatorRun = false;
Pings.INSTANCE.search.testBeforeNextSubmit((reason) -> {
    assertEquals(Pings.searchReasonCodes.performed, reason);
    assertEquals(5, Search.INSTANCE.defaultEngine.testGetValue());
    validatorRun = true;
});

// Submit the ping.
Pings.INSTANCE.search.submit(
    Pings.INSTANCE.searchReasonCodes.performed
);

// Verify that the validator run.
assertTrue(validatorRun);
```

</div>

<div data-lang="Swift" class="tab">

```swift
// Record some data.
Search.defaultEngine.add(5)

// Instruct the ping API to validate the ping data.
var validatorRun = false
GleanMetrics.pings.shared.search.testBeforeNextSubmit { reason in
    XCTAssertEqual(.performed, reason, "Unexpected reason for search ping submitted")
    XCTAssertEqual(5, try Search.defaultEngine.testGetValue(), "Unexpected value for default engine in search ping")
    validatorRun = true
}

// Submit the ping.
GleanMetrics.Pings.shared.search.submit(
    reason: .performed
)

// Verify that the validator run.
XCTAssert(validatorRun, "Expected validator to be called by now.")
```

</div>

<div data-lang="Python" class="tab">

```Python
from glean import load_metrics, load_pings

pings = load_pings("pings.yaml")
metrics = load_metrics("metrics.yaml")

# Record some data.
metrics.search.default_engine.add(5)

# Need a mutable object and plain booleans are not.
callback_was_called = [False]

def check_custom_ping(reason):
    assert reason == pings.search_reason_codes.PERFORMED
    assert 5 == metrics.search.default_engine.test_get_value()
    callback_was_called[0] = True

# Instruct the ping API to validate the ping data.
pings.search.test_before_next_submit(check_custom_ping)

# Submit the ping.
pings.search.submit(pings.search_reason_codes.PERFORMED)

# Verify that the validator run.
assert callback_was_called[0]
```

</div>

<div data-lang="Rust" class="tab">

```Rust
use glean_metrics::{search, pings};

// Record some data.
search::default_engine.add(5);

// Instruct the ping API to validate the ping data.
pings::search.test_before_next_submit(move |reason| {
    assert_eq!(pings::SearchReasonCodes::Performed, reason);
    assert_eq!(5, search::default_engine.test_get_value(None).unwrap());
});

// When the `submit` API is not directly called by the
// test code, it may be worth checking that the validator
// function run by using a canary boolean in the closure
// used in `test_before_next_submit` and asserting on its
// value after submission.

// Submit the ping.
pings::search.submit(pings::SearchReasonCodes::Performed);
```

</div>

<div data-lang="JavaScript" class="tab">

```js
import * as search from "./path/to/generated/files/search.js";
import * as pings from "./path/to/generated/files/pings.js";

// Record some data.
search.defaultEngine.add(5);

// Instruct the ping API to validate the ping data.
let validatorRun = false;
const p = pings.search.testBeforeNextSubmit(async reason => {
  assert.strictEqual(reason, "performed");
  assert.strictEqual(await search.defaultEngine.testGetValue(), 5);
  validatorRun = true;
});

// Submit the ping.
pings.search.submit("performed");
// Wait for the validation to finish.
assert.doesNotThrow(async () => await p);

// Verify that the validator run.
assert.ok(validatorRun);
```

</div>

<div data-lang="Firefox Desktop" class="tab">

**JavaScript:**
```js
Glean.search.defaultEngine.add(5);
let submitted = false;
GleanPings.search.testBeforeNextSubmit(reason => {
  submitted = true;
  Assert.equal(5, Glean.search.defaultEngine.testGetValue());
});
GleanPings.search.submit();
Assert.ok(submitted);
```

**C++:**
```cpp
mozilla::glean::search::default_engine.Add(5);
bool submitted = false;
mozilla::glean_pings::Search.TestBeforeNextSubmit([&submitted](const nsACString& aReason) {
  submitted = true;
  ASSERT_EQ(false, mozilla::glean::search::default_engine.TestGetValue().unwrap().ref());
});
mozilla::glean_pings::Search.Submit();
ASSERT_TRUE(submitted);
```
</div>

{{#include ../../../shared/tab_footer.md}}
