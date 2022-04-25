# UUID

UUIDs metrics are used to record values that uniquely identify some entity, such as a client id.

## Recording API

### `generateAndSet`

Sets a UUID metric to a randomly generated [UUID](https://datatracker.ietf.org/doc/html/rfc4122) value (UUID v4) .

{{#include ../../../shared/tab_header.md}}

<div data-lang="Kotlin" class="tab">

```Kotlin
import org.mozilla.yourApplication.GleanMetrics.User

// Generate a new UUID and record it
User.clientId.generateAndSet()
```

</div>

<div data-lang="Java" class="tab">

```Java
import org.mozilla.yourApplication.GleanMetrics.User;

// Generate a new UUID and record it
User.INSTANCE.clientId.generateAndSet();
```

</div>


<div data-lang="Swift" class="tab">

```Swift
// Generate a new UUID and record it
User.clientId.generateAndSet()
```

</div>

<div data-lang="Python" class="tab">

```Python
from glean import load_metrics
metrics = load_metrics("metrics.yaml")

# Generate a new UUID and record it
metrics.user.client_id.generate_and_set()
```

</div>

<div data-lang="Rust" class="tab">

```rust
use glean_metrics;
use uuid::Uuid;

// Generate a new UUID and record it
user::client_id.generate_and_set();
```
</div>

<div data-lang="JavaScript" class="tab">

```js
import * as user from "./path/to/generated/files/user.js";

user.clientId.generateAndSet();
```
</div>

<div data-lang="Firefox Desktop" class="tab">

**C++**

```c++
#include "mozilla/glean/GleanMetrics.h"

// Generate a new UUID and record it.
mozilla::glean::user::client_id.GenerateAndSet();
```

**JavaScript**

```js
// Generate a new UUID and record it.
Glean.user.clientId.generateAndSet();
```

</div>

{{#include ../../../shared/tab_footer.md}}


### `set`

Sets a UUID metric to a specific value.
Accepts any UUID version.

{{#include ../../../shared/tab_header.md}}

<div data-lang="Kotlin" class="tab">

```Kotlin
import org.mozilla.yourApplication.GleanMetrics.User

// Set a UUID explicitly
User.clientId.set(UUID.randomUUID())  // Set a UUID explicitly
```

</div>

<div data-lang="Java" class="tab">

```Java
import org.mozilla.yourApplication.GleanMetrics.User;

// Set a UUID explicitly
User.INSTANCE.clientId.set(UUID.randomUUID());
```
</div>


<div data-lang="Swift" class="tab">

```Swift
User.clientId.set(UUID())  // Set a UUID explicitly
```
</div>

<div data-lang="Python" class="tab">

```Python
import uuid

from glean import load_metrics
metrics = load_metrics("metrics.yaml")

# Set a UUID explicitly
metrics.user.client_id.set(uuid.uuid4())
```
</div>

<div data-lang="Rust" class="tab">

```rust
use glean_metrics;
use uuid::Uuid;

// Set a UUID explicitly
user::client_id.set(Uuid::new_v4());
```
</div>

<div data-lang="JavaScript" class="tab">

```js
import * as user from "./path/to/generated/files/user.js";

const uuid = "decafdec-afde-cafd-ecaf-decafdecafde";
user.clientId.set(uuid);
```
</div>

<div data-lang="Firefox Desktop" class="tab">

**C++**

```c++
#include "mozilla/glean/GleanMetrics.h"

// Set a specific value.
nsCString kUuid("decafdec-afde-cafd-ecaf-decafdecafde");
mozilla::glean::user::client_id.Set(kUuid);
```

**JavaScript**

```js
// Set a specific value.
const uuid = "decafdec-afde-cafd-ecaf-decafdecafde";
Glean.user.clientId.set(uuid);
```
</div>

{{#include ../../../shared/tab_footer.md}}

#### Recorded errors

* [`invalid_value`](../../user/metrics/error-reporting.md): if the value is set to a string that is not a UUID (only applies for dynamically-typed languages, such as Python).
* [`invalid_type`](../../user/metrics/error-reporting.md): if a non-string or non-UUID value is given.


## Testing API

### `testGetValue`

Gets the recorded value for a given UUID metric.

{{#include ../../../shared/tab_header.md}}

<div data-lang="Kotlin" class="tab">

```Kotlin
import org.mozilla.yourApplication.GleanMetrics.User

// Was it the expected value?
assertEquals(uuid, User.clientId.testGetValue())
```
</div>

<div data-lang="Java" class="tab">

```Java
import org.mozilla.yourApplication.GleanMetrics.User;

// Was it the expected value?
assertEquals(uuid, User.INSTANCE.clientId.testGetValue());
```
</div>


<div data-lang="Swift" class="tab">

```Swift
// Was it the expected value?
XCTAssertEqual(uuid, try User.clientId.testGetValue())
```

</div>

<div data-lang="Python" class="tab">

```Python
from glean import load_metrics
metrics = load_metrics("metrics.yaml")

# Was it the expected value?
assert uuid == metrics.user.client_id.test_get_value()
```

</div>

<div data-lang="Rust" class="tab">

```rust
use glean_metrics;
use uuid::Uuid;

let u = Uuid::new_v4();
// Does it have the expected value?
assert_eq!(u, user::client_id.test_get_value(None).unwrap());
```

</div>

<div data-lang="JavaScript" class="tab">

```js
import * as user from "./path/to/generated/files/user.js";

const uuid = "decafdec-afde-cafd-ecaf-decafdecafde";
assert(uuid, await user.clientId.testGetValue());
```
</div>

<div data-lang="Firefox Desktop" class="tab">

**C++**

```c++
#include "mozilla/glean/GleanMetrics.h"

// Is it clear of errors?
ASSERT_TRUE(mozilla::glean::user::client_id.TestGetValue().isOk());
// Does it have an expected values?
ASSERT_STREQ(kUuid.get(), mozilla::glean::user::client_id.TestGetValue().unwrap().value().get());
```

**JavaScript**

```js
const uuid = "decafdec-afde-cafd-ecaf-decafdecafde";
// testGetValue will throw NS_ERROR_LOSS_OF_SIGNIFICANT_DATA on error.
Assert.equal(Glean.user.clientId.testGetValue(), uuid);
```

</div>

{{#include ../../../shared/tab_footer.md}}

### `testHasValue`

Whether or not **any** value was recorded for a given UUID metric.

{{#include ../../../shared/tab_header.md}}

<div data-lang="Kotlin" class="tab">

```Kotlin
import org.mozilla.yourApplication.GleanMetrics.User

// Was anything recorded?
assertTrue(User.clientId.testHasValue())
```

</div>

<div data-lang="Java" class="tab">

```Java
import org.mozilla.yourApplication.GleanMetrics.User;

// Was anything recorded?
assertTrue(User.INSTANCE.clientId.testHasValue());
```

</div>


<div data-lang="Swift" class="tab">

```Swift
// Was anything recorded?
XCTAssert(User.clientId.testHasValue())
```

</div>

<div data-lang="Python" class="tab">

```Python
from glean import load_metrics
metrics = load_metrics("metrics.yaml")

# Was anything recorded?
assert metrics.user.client_id.test_has_value()
```

</div>

<div data-lang="Rust" class="tab"></div>

<div data-lang="JavaScript" class="tab"></div>

<div data-lang="Firefox Desktop" class="tab"></div>

{{#include ../../../shared/tab_footer.md}}

### `testGetNumRecordedErrors`

Gets number of errors recorded for a given UUID metric.

{{#include ../../../shared/tab_header.md}}

<div data-lang="Kotlin" class="tab">

```Kotlin
import org.mozilla.yourApplication.GleanMetrics.User

assertEquals(
    1, User.clientId.testGetNumRecordedErrors(ErrorType.InvalidValue)
)
```

</div>

<div data-lang="Java" class="tab">

```Java
import org.mozilla.yourApplication.GleanMetrics.User;

assertEquals(
    1, User.INSTANCE.clientId.testGetNumRecordedErrors(ErrorType.InvalidValue)
);
```

</div>


<div data-lang="Swift" class="tab">

```Swift
XCTAssertEqual(1, User.clientId.testGetNumRecordedErrors(.invalidValue))
```

</div>

<div data-lang="Python" class="tab">

```Python
from glean import load_metrics
metrics = load_metrics("metrics.yaml")

from glean.testing import ErrorType

assert 1 == metrics.user.client_id.test_get_num_recorded_errors(
    ErrorType.INVALID_VALUE
)
```
</div>

<div data-lang="Rust" class="tab">

```rust
use glean::ErrorType;

use glean_metrics;

assert_eq!(
  1,
  user::client_id.test_get_num_recorded_errors(
    ErrorType::InvalidValue
  )
);
```

</div>

<div data-lang="JavaScript" class="tab">

```js
import * as user from "./path/to/generated/files/user.js";
import { ErrorType } from "@mozilla/glean/<platform>";

// Was the string truncated, and an error reported?
assert.strictEqual(
  1,
  await user.clientId.testGetNumRecordedErrors(ErrorType.InvalidValue)
);
```
</div>

<div data-lang="Firefox Desktop" class="tab" data-info="Firefox Desktop uses testGetValue to communicate errors"></div>

{{#include ../../../shared/tab_footer.md}}

## Metric Parameters

You first need to add an entry for it to the `metrics.yaml` file:

```YAML
user:
  client_id:
    type: uuid
    description: >
      A unique identifier for the client's profile
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

* A unique identifier for the client.

## Reference

* [Swift API docs](../../../swift/Classes/UuidMetricType.html)
* [Python API docs](../../../python/glean/metrics/uuid.html)
* [Rust API docs](../../../docs/glean/private/uuid/struct.UuidMetric.html)
* [JavaScript API docs](https://mozilla.github.io/glean.js/classes/core_metrics_types_uuid.default.html#set)
