# Unit testing Glean metrics

In order to support unit testing inside of client applications using the Glean SDK,
a set of testing API functions have been included.
The intent is to make the Glean SDKs easier to test 'out of the box'
in any client application it may be used in.
These functions expose a way to inspect and validate recorded metric values within the client application. but are restricted to test code only.
(Outside of a testing context, Glean APIs are otherwise write-only so that it can enforce semantics and constraints about data).

## Example of using the test API

In order to enable metrics testing APIs in each SDK, Glean must be reset and put in testing mode.
For documentation on how to do that, refer to [Initializing - Testing API](../../reference/general/initializing.md#testing-api).

Check out full examples of using the metric testing API on each Glean SDK.
All examples omit the step of resetting Glean for tests to focus solely on metrics unit testing.

{{#include ../../../shared/tab_header.md}}

<div data-lang="Kotlin" class="tab">

```Kotlin
// Record a metric value with extra to validate against
GleanMetrics.BrowserEngagement.click.record(
    BrowserEngagementExtras(font = "Courier")
)

// Record more events without extras attached
BrowserEngagement.click.record()
BrowserEngagement.click.record()

// Retrieve a snapshot of the recorded events
val events = BrowserEngagement.click.testGetValue()!!

// Check if we collected all 3 events in the snapshot
assertEquals(3, events.size)

// Check extra key/value for first event in the list
assertEquals("Courier", events.elementAt(0).extra["font"])
```

</div>

<div data-lang="Swift" class="tab">

```Swift
// Record a metric value with extra to validate against
GleanMetrics.BrowserEngagement.click.record([.font: "Courier"])

// Record more events without extras attached
BrowserEngagement.click.record()
BrowserEngagement.click.record()

// Retrieve a snapshot of the recorded events
let events = BrowserEngagement.click.testGetValue()!

// Check if we collected all 3 events in the snapshot
XCTAssertEqual(3, events.count)

// Check extra key/value for first event in the list
XCTAssertEqual("Courier", events[0].extra?["font"])
```

</div>

<div data-lang="Python" class="tab">

```python
from glean import load_metrics
metrics = load_metrics("metrics.yaml")

# Record a metric value with extra to validate against
metrics.url.visit.add(1)

# Check if we collected any events into the 'click' metric
assert metrics.url.visit.test_get_value() is not Null

# Retrieve a snapshot of the recorded events
assert 1 == metrics.url.visit.test_get_value()
```

</div>

{{#include ../../../shared/tab_footer.md}}
