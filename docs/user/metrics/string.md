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

### Kotlin

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
```

### Swift

```Swift
// Record a value into the metric.
SearchDefault.name.set("duck duck go")
// If it changed later, you can record the new value:
SearchDefault.name.set("wikipedia")
```

There are test APIs available too:

```Kotlin
@testable import Glean

// Was anything recorded?
XCTAssert(SearchDefault.name.testHasValue())
// Does the string metric have the expected value?
// IMPORTANT: It may have been truncated -- see "Limits" below
XCTAssertEqual("wikipedia", try SearchDefault.name.testGetValue())
```

## Limits

* Fixed maximum string length: 50. Longer strings are truncated. For the original Kotlin implementation of the Glean SDK, this is measured in Unicode characters. For the Rust implementation, this is measured in the number of bytes when the string is encoded in UTF-8.

## Examples

* Record the operating system name with a value of "android".

* Recording the device model with a value of "SAMSUNG-SGH-I997".

## Recorded errors

* `invalid_value`: if the string is too long

## Reference

* [Kotlin API docs](../../../javadoc/glean/mozilla.telemetry.glean.private/-string-metric-type/index.html).
