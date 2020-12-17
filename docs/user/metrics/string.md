# Strings

This allows recording a Unicode string value with arbitrary content.

> **Note**: Be careful using arbitrary strings and make sure they can't accidentally contain identifying data (like directory paths or user input).

> **Note**: This is does not support recording JSON blobs - please get in contact with the Telemetry team if you're missing a type.

## Configuration

Say you're adding a metric to find out what the default search in a browser is. First you need to add an entry for the metric to the `metrics.yaml` file:

```YAML
search.default:
  name:
    type: string
    description: >
      The name of the default search engine.
    lifetime: application
    ...
```

## API

{{#include ../../tab_header.md}}

<div data-lang="Kotlin" class="tab">

```Kotlin
import org.mozilla.yourApplication.GleanMetrics.SearchDefault

// Record a value into the metric.
SearchDefault.name.set("duck duck go")
// If it changed later, you can record the new value:
SearchDefault.name.set("wikipedia")
```

There are test APIs available too:

```Kotlin
import org.mozilla.yourApplication.GleanMetrics.SearchDefault

// Was anything recorded?
assertTrue(SearchDefault.name.testHasValue())
// Does the string metric have the expected value?
// IMPORTANT: It may have been truncated -- see "Limits" below
assertEquals("wikipedia", SearchDefault.name.testGetValue())
// Was the string truncated, and an error reported?
assertEquals(1, SearchDefault.name.testGetNumRecordedErrors(ErrorType.InvalidValue))
```

</div>

<div data-lang="Java" class="tab">

```Java
import org.mozilla.yourApplication.GleanMetrics.SearchDefault;

// Record a value into the metric.
SearchDefault.INSTANCE.name.set("duck duck go");
// If it changed later, you can record the new value:
SearchDefault.INSTANCE.name.set("wikipedia");
```

There are test APIs available too:

```Java
import org.mozilla.yourApplication.GleanMetrics.SearchDefault

// Was anything recorded?
assertTrue(SearchDefault.INSTANCE.name.testHasValue());
// Does the string metric have the expected value?
// IMPORTANT: It may have been truncated -- see "Limits" below
assertEquals("wikipedia", SearchDefault.INSTANCE.name.testGetValue());
// Was the string truncated, and an error reported?
assertEquals(
    1,
    SearchDefault.INSTANCE.name.testGetNumRecordedErrors(
        ErrorType.InvalidValue
    )
);
```

</div>

<div data-lang="Swift" class="tab">

```Swift
// Record a value into the metric.
SearchDefault.name.set("duck duck go")
// If it changed later, you can record the new value:
SearchDefault.name.set("wikipedia")
```

There are test APIs available too:

```Swift
@testable import Glean

// Was anything recorded?
XCTAssert(SearchDefault.name.testHasValue())
// Does the string metric have the expected value?
// IMPORTANT: It may have been truncated -- see "Limits" below
XCTAssertEqual("wikipedia", try SearchDefault.name.testGetValue())
// Was the string truncated, and an error reported?
XCTAssertEqual(1, SearchDefault.name.testGetNumRecordedErrors(.invalidValue))
```

</div>

<div data-lang="Python" class="tab">

```Python
from glean import load_metrics
metrics = load_metrics("metrics.yaml")

# Record a value into the metric.
metrics.search_default.name.set("duck duck go")
# If it changed later, you can record the new value:
metrics.search_default.name.set("wikipedia")
```

There are test APIs available too:

```Python
# Was anything recorded?
assert metrics.search_default.name.test_has_value()
# Does the string metric have the expected value?
# IMPORTANT: It may have been truncated -- see "Limits" below
assert "wikipedia" == metrics.search_default.name.test_get_value()
# Was the string truncated, and an error reported?
assert 1 == metrics.search_default.name.test_get_num_recorded_errors(
    ErrorType.INVALID_VALUE
)
```

</div>

<div data-lang="C#" class="tab">

```C#
using static Mozilla.YourApplication.GleanMetrics.SearchDefault;

// Record a value into the metric.
SearchDefault.name.Set("duck duck go");
// If it changed later, you can record the new value:
SearchDefault.name.Set("wikipedia");
```

There are test APIs available too:

```C#
using static Mozilla.YourApplication.GleanMetrics.SearchDefault;

// Was anything recorded?
Assert.True(SearchDefault.name.TestHasValue());
// Does the string metric have the expected value?
// IMPORTANT: It may have been truncated -- see "Limits" below
Assert.Equal("wikipedia", SearchDefault.name.TestGetValue());
// Was the string truncated, and an error reported?
Assert.Equal(
    1,
    SearchDefault.name.TestGetNumRecordedErrors(
        ErrorType.InvalidValue
    )
);
```


</div>

<div data-lang="C++" class="tab">

> **Note**: C++ APIs are only available in Firefox Desktop.

```c++
#include "mozilla/glean/GleanMetrics.h"

mozilla::glean::search_default::name.Set("wikipedia"_ns);
```

There are test APIs available too:

```c++
#include "mozilla/glean/GleanMetrics.h"

// Does it have the expected value?
ASSERT_STREQ(
  "wikipedia",
  mozilla::glean::search_default::name.TestGetValue().value().get()
);
// Did it run across any errors?
// TODO: https://bugzilla.mozilla.org/show_bug.cgi?id=1683171
```

</div>

<div data-lang="JS" class="tab">

> **Note**: JS APIs are currently only available in Firefox Desktop.
> General JavaScript support is coming soon via [the Glean.js project](https://github.com/mozilla/glean.js/).

```js
Glean.searchDefault.name.set("wikipedia");
```

There are test APIs available too:

```js
Assert.equal("wikipedia", Glean.searchDefault.name.testGetValue());
// Did it run across any errors?
// TODO: https://bugzilla.mozilla.org/show_bug.cgi?id=1683171
```

</div>

{{#include ../../tab_footer.md}}

## Limits

* Fixed maximum string length: 100. Longer strings are truncated. This is measured in the number of bytes when the string is encoded in UTF-8.

## Examples

* Record the operating system name with a value of `"android"`.

* Recording the device model with a value of `"SAMSUNG-SGH-I997"`.

## Recorded errors

* `invalid_overflow`: if the string is too long. (Prior to Glean 31.5.0, this recorded an `invalid_value`).

## Reference

* [Kotlin API docs](../../../javadoc/glean/mozilla.telemetry.glean.private/-string-metric-type/index.html).
* [Swift API docs](../../../swift/Classes/StringMetricType.html)
* [Python API docs](../../../python/glean/metrics/string.html)
