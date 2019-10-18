# Unit testing Glean metrics

In order to support unit testing inside of client applications using the Glean SDK, a set of testing API functions have been included.
The intent is to make the Glean SDK easier to test 'out of the box' in any client application it may be used in.
These functions expose a way to inspect and validate recorded metric values within the client application but are restricted to test code only through visibility annotations
(`@VisibleForTesting(otherwise = VisibleForTesting.NONE)` for Kotlin, `internal` methods for Swift).

## General test API method semantics

{{#include ../tab_header.md}}

<div data-lang="Kotlin" class="tab">

In order to prevent issues with async calls when unit testing Glean,
it is important to put the Glean SDK into testing mode by applying the JUnit `GleanTestRule` to your test class.
When the Glean SDK is in testing mode, it enables uploading and clears the recorded metrics at the beginning of each test run.
The rule can be used as shown below:

```kotlin
@RunWith(AndroidJUnit4::class)
class ActivityCollectingDataTest {
    // Apply the GleanTestRule to set up a disposable Glean instance.
    // Please note that this clears the Glean data across tests.
    @get:Rule
    val gleanRule = GleanTestRule(ApplicationProvider.getApplicationContext())

    @Test
    fun checkCollectedData() {
      // The Glean SDK testing API can be called here.
    }
}
```

This will ensure that metrics are done recording when the other test functions are used.

To check if a value exists (i.e. it has been recorded), there is a `testHasValue()` function on each of the metric instances:

```kotlin
assertTrue(GleanMetrics.Search.defaultSearchEngineUrl.testHasValue())
```

To check the actual values, there is a `testGetValue()` function on each of the metric instances.
It is important to check that the values are recorded as expected, since many of the metric types may truncate or error-correct the value.
This function will return a datatype appropriate to the specific type of the metric it is being used with:

```kotlin
assertEquals("https://example.com/search?", GleanMetrics.Search.defaultSearchEngineUrl.testGetValue())
```

Note that each of these functions has its visibility limited to the scope of unit tests by making use of the `@VisibleForTesting` annotation,
so the IDE should complain if you attempt to use them inside of client code.

</div>

<div data-lang="Swift" class="tab">

> **NOTE**: There's no automatic test rule for Glean tests implemented.

In order to prevent issues with async calls when unit testing Glean, it is important to put the Glean SDK into testing mode.
When the Glean SDK is in testing mode, it enables uploading and clears the recorded metrics at the beginning of each test run.

Activate it by resetting Glean in your test's setup:

```swift
@testable import Glean
import XCTest

class GleanUsageTests: XCTestCase {
    override func setUp() {
        Glean.shared.resetGlean(clearStores: true)
    }

    // ...
}
```

This will ensure that metrics are done recording when the other test functions are used.

To check if a value exists (i.e. it has been recorded), there is a `testHasValue()` function on each of the metric instances:

```Swift
XCTAssertTrue(GleanMetrics.Search.defaultSearchEngineUrl.testHasValue())
```

To check the actual values, there is a `testGetValue()` function on each of the metric instances.
It is important to check that the values are recorded as expected, since many of the metric types may truncate or error-correct the value.
This function will return a datatype appropriate to the specific type of the metric it is being used with:

```Swift
XCTAssertEqual("https://example.com/search?", try GleanMetrics.Search.defaultSearchEngineUrl.testGetValue())
```

Note that each of these functions is marked as `internal`, you need to import `Glean` explicitly in test mode:

```Swift
@testable import Glean
```

</div>

{{#include ../tab_footer.md}}

## Testing metrics for custom pings

In order to test metrics where the metric is included in more than one ping, the test functions take an optional `pingName` argument.
This is the name of the ping that the metric is being sent in, such as `"events"` for the [`events` ping](pings/events.md),
or `"metrics"` for the [`metrics` ping](pings/metrics.md).
This could also be a custom ping name that the metric is being sent in.
In most cases you should not have to supply the ping name to the test function and can just use the default which is the "default" ping that this metric is sent in.
You should only need to provide a `pingName` if the metric is being sent in more than one ping in order to identify the correct metric store.

You can call the `testHasValue()` and `testGetValue()` functions with `pingName` like this:

```kotlin
GleanMetrics.Foo.uriCount.testHasValue("customPing")
GleanMetrics.Foo.uriCount.testGetValue("customPing")
```

## Example of using the test API

{{#include ../tab_header.md}}

<div data-lang="Kotlin" class="tab">

Here is a longer example to better illustrate the intended use of the test API:

```kotlin
// Record a metric value with extra to validate against
GleanMetrics.BrowserEngagement.click.record(
    mapOf(
        BrowserEngagement.clickKeys.font to "Courier"
    )
)

// Record more events without extras attached
BrowserEngagement.click.record()
BrowserEngagement.click.record()

// Check if we collected any events into the 'click' metric
assertTrue(BrowserEngagement.click.testHasValue())

// Retrieve a snapshot of the recorded events
val events = BrowserEngagement.click.testGetValue()

// Check if we collected all 3 events in the snapshot
assertEquals(3, events.size)

// Check extra key/value for first event in the list
assertEquals("Courier", events.elementAt(0).extra["font"])
```

</div>

<div data-lang="Swift" class="tab">

Here is a longer example to better illustrate the intended use of the test API:

```Swift
// Record a metric value with extra to validate against
GleanMetrics.BrowserEngagement.click.record([.font: "Courier"])

// Record more events without extras attached
BrowserEngagement.click.record()
BrowserEngagement.click.record()

// Check if we collected any events into the 'click' metric
XCTAssertTrue(BrowserEngagement.click.testHasValue())

// Retrieve a snapshot of the recorded events
let events = try! BrowserEngagement.click.testGetValue()

// Check if we collected all 3 events in the snapshot
XCTAssertEqual(3, events.count)

// Check extra key/value for first event in the list
XCTAssertEqual("Courier", events[0].extra?["font"])
```

</div>

{{#include ../tab_footer.md}}
