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

{{#include ../../tab_footer.md}}

## Limits

* None.

## Examples

* A unique identifier for the client.

## Recorded errors

* None.

## Reference

* [Kotlin API docs](../../../javadoc/glean/mozilla.telemetry.glean.private/-uuid-metric-type/index.html).
* [Swift API docs](../../../swift/Classes/UuidMetricType.html)
