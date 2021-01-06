# Events

Events allow recording of e.g. individual occurrences of user actions, say every time a view was open and from where.

Each event contains the following data:

- A timestamp, in milliseconds. The first event in any ping always has a value of `0`, and subsequent event timestamps are relative to it.
- The name of the event.
- A set of key-value pairs, where the keys are predefined in the `extra_keys` metric parameter, and the values are strings.

## Configuration

Say you're adding a new event for when a view is shown. First you need to add an entry for the event to the `metrics.yaml` file:

```YAML
views:
  login_opened:
    type: event
    description: >
      Recorded when the login view is opened.
    ...
    extra_keys:
      source_of_login:
        description: The source from which the login view was opened, e.g. "toolbar".
```

## API

{{#include ../../tab_header.md}}

<div data-lang="Kotlin" class="tab">

Note that an `enum` has been generated for handling the `extra_keys`: it has the same name as the event metric, with `Keys` added.

```Kotlin
import org.mozilla.yourApplication.GleanMetrics.Views

Views.loginOpened.record(mapOf(Views.loginOpenedKeys.sourceOfLogin to "toolbar"))
```

There are test APIs available too, for example:

```Kotlin
import org.mozilla.yourApplication.GleanMetrics.Views

// Was any event recorded?
assertTrue(Views.loginOpened.testHasValue())
// Get a List of the recorded events.
val snapshot = Views.loginOpened.testGetValue()
// Check that two events were recorded.
assertEquals(2, snapshot.size)
val first = snapshot.single()
assertEquals("login_opened", first.name)
// Check that no errors were recorded
assertEquals(0, Views.loginOpened.testGetNumRecordedErrors(ErrorType.InvalidOverflow))
```

</div>

<div data-lang="Swift" class="tab">

Note that an `enum` has been generated for handling the `extra_keys`: it has the same name as the event metric, with `Keys` added.

```Swift
Views.loginOpened.record(extra: [.sourceOfLogin: "toolbar"])
```

There are test APIs available too, for example:

```Kotlin
@testable import Glean

// Was any event recorded?
XCTAssert(Views.loginOpened.testHasValue())
// Get a List of the recorded events.
val snapshot = try! Views.loginOpened.testGetValue()
// Check that two events were recorded.
XCTAssertEqual(2, snapshot.size)
val first = snapshot[0]
XCTAssertEqual("login_opened", first.name)
// Check that no errors were recorded
XCTAssertEqual(0, Views.loginOpened.testGetNumRecordedErrors(.invalidOverflow))
```

</div>

<div data-lang="Python" class="tab">

Note that an `enum` has been generated for handling the `extra_keys`: it has the same name as the event metric, with `_keys` added.

```Python
from glean import load_metrics
metrics = load_metrics("metrics.yaml")

metrics.views.login_opened.record(
    {
        metrics.views.login_opened_keys.SOURCE_OF_LOGIN: "toolbar"
    }
)
```

There are test APIs available too, for example:

```Python
# Was any event recorded?
assert metrics.views.login_opened.test_has_value()
# Get a List of the recorded events.
snapshot = metrics.views.login_opened.test_get_value()
# Check that two events were recorded.
assert 2 == len(snapshot)
first = snapshot[0]
assert "login_opened" == first.name
# Check that no errors were recorded
assert 0 == metrics.views.login_opened.test_get_num_recorded_errors(
    ErrorType.INVALID_OVERFLOW
)
```

</div>

<div data-lang="C#" class="tab">

Note that an `enum` has been generated for handling the `extra_keys`: it has the same name as the event metric, with `Keys` added.

```C#
using static Mozilla.YourApplication.GleanMetrics.Views;

Views.loginOpened.Record(new Dictionary<clickKeys, string> {
    { Views.loginOpenedKeys.sourceOfLogin, "toolbar" }
});
```

There are test APIs available too, for example:

```C#
using static Mozilla.YourApplication.GleanMetrics.Views;

// Was any event recorded?
Assert.True(Views.loginOpened.TestHasValue());
// Get a List of the recorded events.
var snapshot = Views.loginOpened.TestGetValue();
// Check that two events were recorded.
Assert.Equal(2, snapshot.Length);
var first = snapshot.First();
Assert.Equal("login_opened", first.Name);
// Check that no errors were recorded
Assert.Equal(0, Views.loginOpened.TestGetNumRecordedErrors(ErrorType.InvalidOverflow));
```

</div>

<div data-lang="Rust" class="tab">

Note that an `enum` has been generated for handling the `extra_keys`: it has the same name as the event metric, with `Keys` added.

```rust
use metrics::views;

let mut extra = HashMap::new();
extra.insert(views::LoginOpenedKeys::SourceOfLogin, "toolbar".into());
views::login_opened.record(extra);
```

There are test APIs available too, for example:

```rust
use metrics::views;

// Was any event recorded?
assert!(views::login_opened.test_get_value(None).is_some());
// Get a List of the recorded events.
var snapshot = views::login_opened.test_get_value(None).unwrap();
// Check that two events were recorded.
assert_eq!(2, snapshot.len());
let first = &snapshot[0];
assert_eq!("login_opened", first.name);
// Check that no errors were recorded
assert_eq!(0, views::login_opened.test_get_num_recorded_errors(ErrorType::InvalidOverflow, None));
```

</div>

<div data-lang="C++" class="tab">

> **Note**: C++ APIs are only available in Firefox Desktop.

```c++
#include "mozilla/glean/GleanMetrics.h"

using mozilla::glean::views::LoginOpenedKeys;
nsTArray<Tuple<LoginOpenedKeys, nsCString>> extra;
nsCString source = "toolbar"_ns;
extra.AppendElement(MakeTuple(LoginOpenedKeys::SourceOfLogin, source));

mozilla::glean::views::login_opened.Record(std::move(extra))
```

There are test APIs available too:

```c++
#include "mozilla/glean/GleanMetrics.h"

// Does it have a value?
ASSERT_TRUE(mozilla::glean::views::login_opened.TestGetValue().isSome());
// Does it have the expected value?
// TODO: https://bugzilla.mozilla.org/show_bug.cgi?id=1678567
// Did it run across any errors?
// TODO: https://bugzilla.mozilla.org/show_bug.cgi?id=1683171
```

</div>

<div data-lang="JS" class="tab">

> **Note**: JS APIs are only available in Firefox Desktop.

```js
let extra = { sourceOfLogin: "toolbar" };
Glean.views.loginOpened.record(extra);
```

There are test APIs available too:

```js
// Does it have the expected value?
// TODO: https://bugzilla.mozilla.org/show_bug.cgi?id=1678567
// Did it run across any errors?
// TODO: https://bugzilla.mozilla.org/show_bug.cgi?id=1683171
```

</div>

{{#include ../../tab_footer.md}}

## Limits

* When 500 events are queued on the client an events ping is immediately sent.

* The `extra_keys` allows for a maximum of 10 keys.

* The keys in the `extra_keys` list must be in dotted snake case, with a maximum length of 40 bytes in UTF-8.

* The values in the `extras` object have a maximum length of 50 in UTF-8.
  
## Examples

* Every time a new tab is opened.

## Recorded errors

* `invalid_overflow`: if any of the values in the `extras` object are greater than 50 bytes in length.  (Prior to Glean 31.5.0, this recorded an `invalid_value`).
 
## Reference

* [Kotlin API docs](../../../javadoc/glean/mozilla.telemetry.glean.private/-event-metric-type/index.html)
* [Swift API docs](../../../swift/Classes/EventMetricType.html)
* [Python API docs](../../../python/glean/metrics/event.html)
* [Rust API docs](../../../docs/glean/private/event/struct.EventMetric.html)
