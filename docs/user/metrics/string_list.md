# String List

Strings lists are used for recording a list of Unicode string values, such as the names of the enabled search engines.

---

_Note:_ Be careful using arbitrary strings and make sure they can't accidentally contain identifying data (like directory paths or user input).

---

## Configuration

First you need to add an entry for the counter to the `metrics.yaml` file:

```YAML
search:
  engines:
    type: string_list
    description: >
      Records the name of the enabled search engines.
    lifetime: application
    ...
```

## API

{{#include ../../tab_header.md}}

<div data-lang="Kotlin" class="tab">

```Kotlin
import org.mozilla.yourApplication.GleanMetrics.Search

// Add them one at a time
engines.forEach {
  Search.engines.add(it)
}

// Set them in one go
Search.engines.set(engines)
```

There are test APIs available too:

```Kotlin
import org.mozilla.yourApplication.GleanMetrics.Search

// Was anything recorded?
assertTrue(Search.engines.testHasValue())
// Does it have the expected value?
// IMPORTANT: It may have been truncated -- see "Limits" below
assertEquals(listOf("Google", "DuckDuckGo"), Search.engines.testGetValue())
// Were any of the values too long, and thus an error was recorded?
assertEquals(1, Search.engines.testGetNumRecordedErrors(ErrorType.InvalidValue))
```

</div>

<div data-lang="Swift" class="tab">

```Swift
// Add them one at a time
for engine in engines {
    Search.engines.add(engine)
}

// Set them in one go
Search.engines.set(engines)
```

There are test APIs available too:

```Swift
@testable import Glean

// Was anything recorded?
XCTAssert(Search.engines.testHasValue())
// Does it have the expected value?
// IMPORTANT: It may have been truncated -- see "Limits" below
XCTAssertEqual(["Google", "DuckDuckGo"], try Search.engines.testGetValue())
// Were any of the values too long, and thus an error was recorded?
XCTAssertEqual(1, Search.engines.testGetNumRecordedErrors(.invalidValue))
```

</div>

<div data-lang="Python" class="tab">

```Python
from glean import load_metrics

metrics = load_metrics("metrics.yaml")

# Add them one at a time
for engine in engines:
    metrics.search.engines.add(engine)

# Set them in one go
metrics.search.engines.set(engines)
```

There are test APIs available too:

```Python
# Was anything recorded?
assert metrics.search.engines.test_has_value()
# Does it have the expected value?
# IMPORTANT: It may have been truncated -- see "Limits" below
assert ["Google", "DuckDuckGo"] == metrics.search.engines.test_get_value()
# Were any of the values too long, and thus an error was recorded?
assert 1 == metrics.search.engines.test_get_num_recorded_errors(
    ErrorType.INVALID_VALUE
)
```

</div>

<div data-lang="C#" class="tab">

```C#
using static Mozilla.YourApplication.GleanMetrics.Search;

// Set a string array into the metric.
Search.engines.Set(new string[] { "Google", "DuckDuckGo" });
// Add another string into the metric.
Search.engines.Add("Baidu");
```

There are test APIs available too:

```C#
using static Mozilla.YourApplication.GleanMetrics.Search;

// Was anything recorded?
Assert.True(Search.engines.TestHasValue());
// Does the string list metric have the expected value?
// IMPORTANT: It may have been truncated -- see "Limits" below
var snapshot = Search.engines.TestGetValue();
Assert.Equal(3, Search.engines.Length);
Assert.Equal("Google", Search.engines[0]);
Assert.Equal("DuckDuckGo", Search.engines[1]);
Assert.Equal("Baidu", Search.engines[2]);
// Was the string truncated, and an error reported?
Assert.Equal(
    1,
    Search.engines.TestGetNumRecordedErrors(
        ErrorType.InvalidValue
    )
);
```
</div>

<div data-lang="Rust" class="tab">

```Rust
use glean_metrics;

// Add them one at a time
engines.iter().for_each(|x|
  search::engines.add(x)
);

// Set them in one go
search::engines.set(engines)
```

There are test APIs available too:

```Rust
use glean::ErrorType;
use glean_metrics;

// Was anything recorded?
assert!(search::engines.test_get_value(None).is_some());
// Does it have the expected value?
// IMPORTANT: It may have been truncated -- see "Limits" below
assert_eq!(
    vec!["Google".to_string(), "DuckDuckGo".to_string()], 
    search::engines.test_get_value(None)
);
// Were any of the values too long, and thus an error was recorded?
assert_eq!(
    0, 
    search::engines.test_get_num_recorded_errors(ErrorType::InvalidValue)
);
```

</div>

<div data-lang="C++" class="tab">

> **Note**: C++ APIs are only available in Firefox Desktop.

```c++
#include "mozilla/glean/GleanMetrics.h"

mozilla::glean::search::engines.Add("wikipedia"_ns);
mozilla::glean::search::engines.Add("duck duck go"_ns);
```

There are test APIs available too:

```c++
#include "mozilla/glean/GleanMetrics.h"

// Does it have the expected values?
nsTArray<nsCString> list = mozilla::glean::search::engines.TestGetValue();
ASSERT_TRUE(list.Contains("wikipedia"_ns));
ASSERT_TRUE(list.Constains("duck duck go"_ns));
// Did it run across any errors?
// TODO: https://bugzilla.mozilla.org/show_bug.cgi?id=1683171
```

</div>

<div data-lang="JS" class="tab">

> **Note**: JS APIs are only available in Firefox Desktop.

```js
Glean.search.engines.add("wikipedia");
Glean.search.engines.add("duck duck go");
```

There are test APIs available too:

```js
const engines = Glean.search.engines.testGetValue();
Assert.ok(engines.includes("wikipedia"));
Assert.ok(engines.includes("duck duck go"));
// Did it run across any errors?
// TODO: https://bugzilla.mozilla.org/show_bug.cgi?id=1683171
```

</div>

{{#include ../../tab_footer.md}}

## Limits

* Fixed maximum string length: 50. Longer strings are truncated. This is measured in the number of bytes when the string is encoded in UTF-8.

* Fixed maximum list length: 20 items. Additional strings are dropped.

## Examples

* The names of the enabled search engines.

## Recorded errors

* `invalid_overflow`: if the string is too long. (Prior to Glean 31.5.0, this recorded an `invalid_value`).

* `invalid_value`: if the list is too long

## Reference

* [Kotlin API docs](../../../javadoc/glean/mozilla.telemetry.glean.private/-string-list-metric-type/index.html)
* [Python API docs](../../../python/glean/metrics/string_list.html)
* [Rust API docs](../../../docs/glean/private/struct.StringListMetric.html)
