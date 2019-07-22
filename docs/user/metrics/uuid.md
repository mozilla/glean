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

```Kotlin
import org.mozilla.yourApplication.GleanMetrics.User

User.clientId.generateAndSet() // Generate a new UUID and record it
User.clientId.set(UUID.randomUUID())  // Set a UUID explicitly
```

There are test APIs available too.

```Kotlin
import org.mozilla.yourApplication.GleanMetrics.User
Glean.enableTestingMode()

// Was anything recorded?
assertTrue(User.clientId.testHasValue())
// Was it the expected value? 
assertEquals(uuid, User.clientId.testGetValue())
```

## Limits

* None.

## Examples

* A unique identifier for the client.

## Recorded errors

* None.
 
## Reference

* See [Kotlin API docs](../../../javadoc/glean/mozilla.telemetry.glean.private/-uuid-metric-type/index.html).

