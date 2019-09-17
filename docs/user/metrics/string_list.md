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
```

## Limits

* Empty string lists are not accepted by the `set()` method. Attempting to record an empty string list will result in an `invalid_value` error and nothing being recorded.

* Fixed maximum string length: 50. Longer strings are truncated. For the original Kotlin implementation of the Glean SDK, this is measured in Unicode characters. For the Rust implementation, this is measured in the number of bytes when the string is encoded in UTF-8.

* Fixed maximum list length: 20 items. Additional strings are dropped.

## Examples

* The names of the enabled search engines.

## Recorded errors

* `invalid_value`: if an empty list is passed to `set()`

* `invalid_value`: if the string is too long

* `invalid_value`: if the list is too long

## Reference

* [Kotlin API docs](../../../javadoc/glean/mozilla.telemetry.glean.private/-string-list-metric-type/index.html)

