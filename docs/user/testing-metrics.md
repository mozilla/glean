# Unit testing Glean metrics

In order to support unit testing inside of client applications using the Glean SDK, a set of testing API functions have been included.
The intent is to make the Glean SDK easier to test 'out of the box' in any client application it may be used in.
These functions expose a way to inspect and validate recorded metric values within the client application but are restricted to test code only through visibility annotations
(`@VisibleForTesting(otherwise = VisibleForTesting.NONE)` for Kotlin, `internal` methods for Swift).
(Outside of a testing context, Glean APIs are otherwise write-only so that it can enforce semantics and constraints about data).

## General test API method semantics

{{#include ../tab_header.md}}

<div data-lang="Kotlin" class="tab">

Using the Glean SDK's unit testing API requires adding [Robolectric 4.0 or later](http://robolectric.org/) as a testing dependency. In Gradle, this can be done by declaring a `testImplementation` dependency:

```groovy
dependencies {
    testImplementation "org.robolectric:robolectric:4.3.1" 
}
```

In order to prevent issues with async calls when unit testing the Glean SDK,
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

In order to prevent issues with async calls when unit testing the Glean SDK, it is important to put the Glean SDK into testing mode.
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

<div data-lang="Python" class="tab">

It is generally a good practice to "reset" the Glean SDK prior to every unit test that uses the Glean SDK, to prevent side effects of one unit test impacting others.
The Glean SDK contains a helper function `glean.testing.reset_glean()` for this purpose.
It has two required arguments: the application ID, and the application version.
Each reset of the Glean SDK will create a new temporary directory for Glean to store its data in.
This temporary directory is automatically cleaned up the next time the Glean SDK is reset or when the testing framework finishes.

The instructions below assume you are using [pytest](https://pypi.org/project/pytest/) as the test runner.
Other test-running libraries have similar features, but are different in the details.

Create a file `conftest.py` at the root of your test directory, and add the following to reset Glean at the start of every test in your suite:

```python
import pytest
from glean import testing

@pytest.fixture(name="reset_glean", scope="function", autouse=True)
def fixture_reset_glean():
    testing.reset_glean(application_id="my-app-id", application_version="0.1.0")
```

To check if a value exists (i.e. it has been recorded), there is a `test_has_value()` function on each of the metric instances:

```python
from glean import load_metrics
metrics = load_metrics("metrics.yaml")

# ...

assert metrics.search.search_engine_url.test_has_value()
```

To check the actual values, there is a `test_get_value()` function on each of the metric instances.
It is important to check that the values are recorded as expected, since many of the metric types may truncate or error-correct the value.
This function will return a datatype appropriate to the specific type of the metric it is being used with:

```python
assert (
    "https://example.com/search?" ==
    metrics.search.default_search_engine_url.test_get_value()
)
```

</div>

<div data-lang="C#" class="tab">

TODO. To be implemented in [bug 1648448](https://bugzilla.mozilla.org/show_bug.cgi?id=1648448).

</div>

{{#include ../tab_footer.md}}

## Testing metrics for custom pings

In order to test metrics where the metric is included in more than one ping, the test functions take an optional `pingName` argument (`ping_name` in Python).
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

<div data-lang="Python" class="tab">

Here is a longer example to better illustrate the intended use of the test API:

```python
from glean import load_metrics
metrics = load_metrics("metrics.yaml")

# Record a metric value with extra to validate against
metrics.url.visit.add(1)

# Check if we collected any events into the 'click' metric
assert metrics.url.visit.test_has_value()

# Retrieve a snapshot of the recorded events
assert 1 == metrics.url.visit.test_get_value()
```

</div>

<div data-lang="C#" class="tab">

TODO. To be implemented in [bug 1648448](https://bugzilla.mozilla.org/show_bug.cgi?id=1648448).

</div>

{{#include ../tab_footer.md}}
