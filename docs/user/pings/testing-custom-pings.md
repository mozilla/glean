# Unit testing Glean custom pings

Applications defining [custom pings](custom.md) can use use the strategy defined in this document to test these pings in unit tests.

## General testing strategy

The schedule of custom pings depends on the specific application implementation, since it is up to the SDK user to define the ping semantics. This makes writing unit tests for custom pings a bit more involved.

One possible strategy could be to wrap the Glean SDK API call to send the ping in a function that can be mocked in the unit test. This would allow for checking the status and the values of the metrics contained in the ping at the time in which the application would have sent it.

## Example testing of a custom ping

Let us start by defining a custom ping with a sample metric in it. Here is the `pings.yaml` file:

```yaml
# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.

$schema: moz://mozilla.org/schemas/glean/pings/1-0-0

my_custom_ping:
  description: >
    This ping is intended to showcase the recommended testing strategy for
    custom pings.
  include_client_id: false
  bugs:
    - 1556985
  data_reviews:
    - https://bugzilla.mozilla.org/show_bug.cgi?id=1556985
  notification_emails:
    - custom-ping-owner@example.com

```

And here is the `metrics.yaml`

```yaml
# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.

$schema: moz://mozilla.org/schemas/glean/metrics/1-0-0

my_custom_ping:
  sample_string:
    type: string
    lifetime: ping
    description: >
      An hashed and salted version of the Google Advertising ID from the device.
      This will never be sent in a ping that also contains the client_id.
    send_in_pings:
      - my_custom_ping
    bugs:
      - 1556985
    data_reviews:
      - https://bugzilla.mozilla.org/show_bug.cgi?id=1556985
    notification_emails:
      - custom-ping-owner@example.com
    expires: "2019-10-01"
```

A potential usage of the Glean SDK generated API could be the following:

```kotlin
import my.component.GleanMetrics.Pings
import my.component.GleanMetrics.MyCustomPing

class MyCustomPingScheduler {
  /**
   * HERE ONLY TO KEEP THE EXAMPLE SIMPLE.
   *
   * A function that consumes the Glean SDK generated metrics API to
   * record some data. It doesn't really need to be in a function, nor
   * in this class. The Glean SDK API can be called when the data is
   * generated.
   */
  fun addSomeData() {
    // Record some sample data.
    MyCustomPing.sampleString.set("test-data")
  }

  /**
   * Called to implement the ping scheduling logic for 'my_custom_ping'.
   */
  fun schedulePing() {
    // ... some scheduling logic that will end up calling the function below.
    sendPing()
  }

  /**
   * Internal function to only be overriden in tests. This
   * calls the Glean SDK API to send custom pings.
   */
  @VisibleForTesting(otherwise = VisibleForTesting.NONE)
  internal fun sendPing() {
    MyCustomPing.send()
  }
}
```

Finally, here is a simple unit test that intercepts the `MyCustomPingScheduler.schedulePing()` call in order to perform the validation on the data. This specific example uses Mockito, but any other framework would work.

```kotlin
@Test
fun `verify custom ping metrics`() {
  // Enable testing mode
  // (Perhaps called from a @Before method so it precedes every test in the suite.)
  Glean.enableTestingMode()

  val scheduler = spy(MyCustomPingScheduler())
  `when`(scheduler.sendPing()).then {
    // Here we validate the content of the ping.
    assertTrue(MyCustomPing.sampleString.testHasValue())
    assertEquals("test-data", MyCustomPing.sampleString.testGetValue())
  }

  scheduler.addSomeData()
  scheduler.schedulePing()
}
```
