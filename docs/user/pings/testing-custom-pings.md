# Unit testing Glean custom pings

Applications defining [custom pings](custom.md) can use use the strategy defined in this document to test these pings in unit tests.

## General testing strategy

The schedule of custom pings depends on the specific application implementation, since it is up to the SDK user to define the ping semantics. This makes writing unit tests for custom pings a bit more involved.

One possible strategy could be to wrap the Glean SDK API call to send the ping in a function that can be mocked in the unit test. This would allow for checking the status and the values of the metrics contained in the ping at the time in which the application would have sent it.

## Example testing of a custom ping

Let us start by defining a custom ping with a sample metric in it. Here is the `pings.yaml` file:

```yaml
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
$schema: moz://mozilla.org/schemas/glean/metrics/1-0-0

custom_ping_data:
  sample_string:
    type: string
    lifetime: ping
    description: >
      A sample string metric for demonstrating unit tests for custom pings.
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
import my.component.GleanMetrics.CustomPingData

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
    CustomPingData.sampleString.set("test-data")
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
    Pings.MyCustomPing.send()
  }
}
```

Finally, here is a simple unit test that intercepts the `MyCustomPingScheduler.schedulePing()` call in order to perform the validation on the data. This specific example uses Mockito, but any other framework would work.

```kotlin
// Metrics and pings definitions.
import my.component.GleanMetrics.Pings
import my.component.GleanMetrics.CustomPingData

// Mockito imports for using spies.
import org.mockito.Mockito.spy
import org.mockito.Mockito.`when`

/**
 * This is an helper function used to enable testing mode for Glean.
 * Should only be called once before the tests, but nothing breaks if it's
 * called more than once!
 */
fun setupGleanOnce() {
  // Enable testing mode
  // (Perhaps called from a @Before method so it precedes every test in the suite.)
  Glean.enableTestingMode()

  // We're using the WorkManager in a bunch of places, and Glean will crash
  // in tests without this line. Let's simply put it here.
  WorkManagerTestInitHelper.initializeTestWorkManager(context)

  Glean.initialize(context)
}

@Test
fun `verify custom ping metrics`() {
  setupGleanOnce()

  val scheduler = spy(MyCustomPingScheduler())
  doAnswer {
    // Here we validate the content that goes into the ping.
    assertTrue(CustomPingData.sampleString.testHasValue())
    assertEquals("test-data", CustomPingData.sampleString.testGetValue())

    // We want to intercept this call, but we also want to make sure the
    // real Glean API is called in order to clear the ping store and to provide
    // consistent behaviour with respect to the application.
    it.callRealMethod()
  }.`when`(scheduler).sendPing()

  scheduler.addSomeData()
  scheduler.schedulePing()
}
```
