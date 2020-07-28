# Unit testing Glean custom pings for Android

Applications defining [custom pings](custom.md) can use use the strategy defined in this document to test these pings in unit tests.

## General testing strategy

The schedule of custom pings depends on the specific application implementation, since it is up to the SDK user to define the ping semantics. This makes writing unit tests for custom pings a bit more involved.

One possible strategy could be to wrap the Glean SDK API call to send the ping in a function that can be mocked in the unit test. This would allow for checking the status and the values of the metrics contained in the ping at the time in which the application would have sent it.

## Example testing of a custom ping

Let us start by defining a custom ping with a sample metric in it. Here is the `pings.yaml` file:

```yaml
$schema: moz://mozilla.org/schemas/glean/pings/1-0-0

my-custom-ping:
  description: >
    This ping is intended to showcase the recommended testing strategy for
    custom pings.
  include_client_id: false
  bugs:
    - https://bugzilla.mozilla.org/1556985/
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
      - my-custom-ping
    bugs:
      - https://bugzilla.mozilla.org/1556985/
    data_reviews:
      - https://bugzilla.mozilla.org/show_bug.cgi?id=1556985
    notification_emails:
      - custom-ping-owner@example.com
    expires: "2019-10-01"
```

A potential usage of the Glean SDK generated API could be the following:

{{#include ../../tab_header.md}}

<div data-lang="Kotlin" class="tab">

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
    submitPing()
  }

  /**
   * Internal function to only be overridden in tests. This
   * calls the Glean SDK API to send custom pings.
   */
  @VisibleForTesting(otherwise = VisibleForTesting.NONE)
  internal fun submitPing() {
    Pings.MyCustomPing.submit()
  }
}
```

The following unit test intercepts the `MyCustomPingScheduler.submitPing()` call in order to perform the validation on the data.
This specific example uses Mockito, but any other framework would work.

```kotlin
// Metrics and pings definitions.
import my.component.GleanMetrics.Pings
import my.component.GleanMetrics.CustomPingData

// Mockito imports for using spies.
import org.mockito.Mockito.spy
import org.mockito.Mockito.`when`

@RunWith(AndroidJUnit4::class)
class MyCustomPingSchedulerTest {
    // Apply the GleanTestRule to set up a disposable Glean instance.
    // Please note that this clears the Glean data across tests.
    @get:Rule
    val gleanRule = GleanTestRule(ApplicationProvider.getApplicationContext())

    @Test
    fun `verify custom ping metrics`() {
      val scheduler = spy(MyCustomPingScheduler())
      doAnswer {
        // Here we validate the content that goes into the ping.
        assertTrue(CustomPingData.sampleString.testHasValue())
        assertEquals("test-data", CustomPingData.sampleString.testGetValue())

        // We want to intercept this call, but we also want to make sure the
        // real Glean API is called in order to clear the ping store and to provide
        // consistent behaviour with respect to the application.
        it.callRealMethod()
      }.`when`(scheduler).submitPing()

      scheduler.addSomeData()
      scheduler.schedulePing()
    }
}
```

</div>

<div data-lang="Swift" class="tab">

```swift
import Foundation
import Glean

// Use typealiases to simplify usage.
// This can be placed anywhere in your code to be available in all files.
typealias CustomPingData = GleanMetrics.CustomPingData
typealias Pings = GleanMetrics.Pings

class MyCustomPingScheduler {
    /**
     * HERE ONLY TO KEEP THE EXAMPLE SIMPLE.
     *
     * A function that consumes the Glean SDK generated metrics API to
     * record some data. It doesn't really need to be in a function, nor
     * in this class. The Glean SDK API can be called when the data is
     * generated.
     */
    func addSomeData() {
       // Record some sample data.
       CustomPingData.sampleString.set("test-data")
    }

    /**
     * Called to implement the ping scheduling logic for 'my_custom_ping'.
     */
    func schedulePing() {
        // ... some scheduling logic that will end up calling the function below.
        submitPing()
    }

    /**
     * Internal function to only be overridden in tests. This
     * calls the Glean SDK API to send custom pings.
     */
    internal func submitPing() {
        Pings.shared.myCustomPing.submit()
    }
}
```

The following unit test intercepts the `MyCustomPingScheduler.submitPing()` call in order to perform the validation on the data.
This example uses a manual mock implementation, but you could use a framework for that.

```swift
@testable import YourApplication
import Glean
import XCTest

class MyCustomPingSchedulerMock: MyCustomPingScheduler {
    var submitWasCalled = false

    deinit {
        XCTAssertTrue(submitWasCalled, "submitPing should have been called once")
    }

    override func submitPing() {
        submitWasCalled = true

        XCTAssertTrue(CustomPingData.os.testHasValue())
        XCTAssertEqual("test-data", try! CustomPingData.os.testGetValue())

        super.submitPing()
    }
}

class MyCustomPingSchedulerTests: XCTestCase {
    override func setUp() {
        Glean.shared.resetGlean(clearStores: true)
    }

    func testCustomPingMetrics() {
        let scheduler = MyCustomPingSchedulerMock()
        scheduler.addSomeData()
        scheduler.schedulePing()
    }
}
```

</div>

<div data-lang="Python" class="tab">

```python
import glean

metrics = glean.load_metrics("metrics.yaml")
pings = glean.load_pings("pings.yaml")


class MyCustomPingScheduler:
    def add_some_data(self):
        """
        HERE ONLY TO KEEP THE EXAMPLE SIMPLE.

        A function that consumes the Glean SDK generated metrics API to
        record some data. It doesn't really need to be in a function, nor
        in this class. The Glean SDK API can be called when the data is
        generated.
        """
        # Record some sample data.
        metrics.custom_ping_data.sample_string.set("test-data")

    def schedule_ping(self):
        """
        Called to implement the ping scheduling logic for 'my_custom_ping'.
        """
        # ... some scheduling logic that will end up calling the function below.
        self._submit_ping()

    def _submit_ping(self):
        """
        Internal function to only be overridden in tests.
        """
        pings.my_custom_ping.submit()
```

The following unit test intercepts the `MyCustomPingScheduler._submit_ping()` call in order to perform the validation on the data.

```python
from unittest.mock import MagicMock

from glean import testing

import custom_ping_scheduler


# This will be run before every test in the entire test suite
def pytest_runtest_setup(item):
    testing.reset_glean(application_id="my-app", application_version="0.0.1")


def test_verify_custom_ping_metrics():
    scheduler = custom_ping_scheduler.MyCustomPingScheduler()

    original_submit_ping = scheduler._submit_ping

    def _submit_ping(self):
        # Here we validate the content that goes into the ping.
        assert (
            custom_ping_scheduler.metrics.custom_ping_data.sample_string.test_has_value()
        )
        assert (
            "test-data"
            == custom_ping_scheduler.metrics.custom_ping_data.sample_string.test_get_value()
        )

        # We want to intercept this call, but we also want to make sure the
        # real Glean API is called in order to clear the ping strong and to
        # provide consistent behavior with respect to the application.
        original_submit_ping(self)

    scheduler._submit_ping = MagicMock(_submit_ping)

    scheduler.add_some_data()
    scheduler.schedule_ping()
```

</div>

<div data-lang="C#" class="tab">

TODO. To be implemented in [bug 1648446](https://bugzilla.mozilla.org/show_bug.cgi?id=1648446).

</div>

{{#include ../../tab_footer.md}}
