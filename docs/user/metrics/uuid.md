# UUID

UUIDs are used to record values that uniquely identify some entity, such as a client id.

## Configuration

You first need to add an entry for it to the `metrics.yaml` file:

```YAML
user:
  client_id:
    type: uuid
    description: >
      A unique identifier for the client's profile
    lifetime: user
    ...
```

## API

Now that the UUID is defined in `metrics.yaml`, you can use the metric to record values in the application's code.

{{#include ../../tab_header.md}}

<div data-lang="Kotlin" class="tab">

```Kotlin
import org.mozilla.yourApplication.GleanMetrics.User

User.clientId.generateAndSet() // Generate a new UUID and record it
User.clientId.set(UUID.randomUUID())  // Set a UUID explicitly
```

There are test APIs available too.

```Kotlin
import org.mozilla.yourApplication.GleanMetrics.User

// Was anything recorded?
assertTrue(User.clientId.testHasValue())
// Was it the expected value?
assertEquals(uuid, User.clientId.testGetValue())
```

</div>

<div data-lang="Java" class="tab">

```Java
import org.mozilla.yourApplication.GleanMetrics.User;

User.INSTANCE.clientId.generateAndSet(); // Generate a new UUID and record it
User.INSTANCE.clientId.set(UUID.randomUUID());  // Set a UUID explicitly
```

There are test APIs available too:

```Java
import org.mozilla.yourApplication.GleanMetrics.User;

// Was anything recorded?
assertTrue(User.INSTANCE.clientId.testHasValue());
// Was it the expected value?
assertEquals(uuid, User.INSTANCE.clientId.testGetValue());
```

</div>


<div data-lang="Swift" class="tab">

```Swift
User.clientId.generateAndSet() // Generate a new UUID and record it
User.clientId.set(UUID())  // Set a UUID explicitly
```

There are test APIs available too.

```Swift
@testable import Glean

// Was anything recorded?
XCTAssert(User.clientId.testHasValue())
// Was it the expected value?
XCTAssertEqual(uuid, try User.clientId.testGetValue())
```

</div>

<div data-lang="Python" class="tab">

```Python
import uuid

from glean import load_metrics
metrics = load_metrics("metrics.yaml")

# Generate a new UUID and record it
metrics.user.client_id.generate_and_set()
# Set a UUID explicitly
metrics.user.client_id.set(uuid.uuid4())
```

There are test APIs available too.

```Python
# Was anything recorded?
assert metrics.user.client_id.test_has_value()
# Was it the expected value?
assert uuid == metrics.user.client_id.test_get_value()
```

</div>

<div data-lang="C#" class="tab">

```C#
using static Mozilla.YourApplication.GleanMetrics.User;

User.clientId.GenerateAndSet(); // Generate a new UUID and record it
User.clientId.Set(System.Guid.NewGuid()); // Set a UUID explicitly
```

There are test APIs available too:

```C#
using static Mozilla.YourApplication.GleanMetrics.User;

// Was anything recorded?
Assert.True(User.clientId.TestHasValue());
// Was it the expected value?
Assert.Equal(uuid, User.clientId.TestGetValue());
```

</div>

<div data-lang="Rust" class="tab">

```rust
use glean_metrics;
use uuid::Uuid;

user::client_id.generate_and_set(); // Generate a new UUID and record it
user::client_id.set(Uuid::new_v4()); // Set a UUID explicitly
```

There are test APIs available too:

```rust
use glean_metrics;

let u = Uuid::new_v4();
user::client_id.set(u);
// Was anything recorded?
assert!(user::client_id.test_get_value(None).is_some());
// Does it have the expected value?
assert_eq!(u, user::client_id.test_get_value(None).unwrap());
```

</div>

<div data-lang="C++" class="tab">

> **Note**: C++ APIs are only available in Firefox Desktop.

```c++
#include "mozilla/glean/GleanMetrics.h"

// Generate a new UUID and record it.
mozilla::glean::user::client_id.GenerateAndSet();
// Set a specific value.
nsCString kUuid("decafdec-afde-cafd-ecaf-decafdecafde");
mozilla::glean::user::client_id.Set(kUuid);
```

There are test APIs available too:

```c++
#include "mozilla/glean/GleanMetrics.h"

// Does it have an expected values?
ASSERT_STREQ(kUuid.get(), mozilla::glean::user::client_id.TestGetValue().value().get());
// Did it run across any errors?
// TODO: https://bugzilla.mozilla.org/show_bug.cgi?id=1683171
```

</div>

<div data-lang="JS" class="tab">

> **Note**: JS APIs are currently only available in Firefox Desktop.
> General JavaScript support is coming soon via [the Glean.js project](https://github.com/mozilla/glean.js/).

```js
// Generate a new UUID and record it.
Glean.user.clientId.generateAndSet();
// Set a specific value.
const uuid = "decafdec-afde-cafd-ecaf-decafdecafde";
Glean.user.clientId.set(uuid);
```

There are test APIs available too:

```js
Assert.equal(Glean.user.clientId.testGetValue(), uuid);
// Did it run across any errors?
// TODO: https://bugzilla.mozilla.org/show_bug.cgi?id=1683171
```

</div>

{{#include ../../tab_footer.md}}

## Limits

* None.

## Examples

* A unique identifier for the client.

## Recorded errors

* `invalid_value`: if the value is set to a string that is not a UUID (only applies for dynamically-typed languages, such as Python).

## Reference

* [Kotlin API docs](../../../javadoc/glean/mozilla.telemetry.glean.private/-uuid-metric-type/index.html).
* [Swift API docs](../../../swift/Classes/UuidMetricType.html)
* [Python API docs](../../../python/glean/metrics/uuid.html)
* [Rust API docs](../../../docs/glean/private/uuid/struct.UuidMetric.html)
