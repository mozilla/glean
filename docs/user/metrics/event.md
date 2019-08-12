# Events

Events allow recording of e.g. individual occurences of user actions, say every time a view was open and from where. Each time you record an event, it records a
timestamp, the event's name and a set of custom values.

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
```

## Limits

* When 500 events are queued on the client an events ping is immediately sent.

* The keys in the `extra_keys` list must be in dotted snake case, with a maximum length of 40.  For the original Kotlin implementation of the Glean SDK, this is measured in Unicode characters. For the Rust implementation, this is measured in the number of bytes when the string is encoded in UTF-8.

* The values in the `extras` object have a maximum length of 50. For the original Kotlin implementation of the Glean SDK, this is measured in Unicode characters. For the Rust implementation, this is measured in the number of bytes when the string is encoded in UTF-8.
  
## Examples

* Every time a new tab is opened.

## Recorded errors

* `invalid_value`: if any of the values in the `extras` object are greater than 50 bytes in length.
 
## Reference

* See [Kotlin API docs](../../../javadoc/glean/mozilla.telemetry.glean.private/-event-metric-type/index.html).

