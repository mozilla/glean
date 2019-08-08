# Using the experiments API

The Glean SDK supports tagging all its pings with experiments annotations. The annotations are useful to report that experiments were active at the time the measurement were collected. The annotations are reported in the optional `experiments` entry in the [`ping_info` section](pings/index.md) of all the Glean SDK pings.

> **Important**: the data set through this API is not persisted by the Glean SDK.
The application or consuming library is responsible for setting the relevant experiment annotations at each run.

## API

```Kotlin
// Annotate Glean pings with experiments data.
Glean.setExperimentActive(
  experimentId = "blue-button-effective",
  branch = "branch-with-blue-button",
  extra: mapOf(
    "buttonLabel" to "test"
  )
)
// After the experiment terminates, the annotation
// can be removed.
Glean.setExperimentInactive("blue-button-effective")
```

> **Important**: Experiment IDs and branch don't need to be pre-defined in the Glean SDK registry files.
Please also note that the `extra` map is non-nested arbitrary `String` to `String` map.

There are test APIs available too:

```Kotlin
import org.mozilla.yourApplication.GleanMetrics.SearchDefault

// Was the experiment annotated in Glean pings?
assertTrue(Glean.testIsExperimentActive("blue-button-effective"))
// Was the correct branch reported?
assertEquals(
  "branch-with-blue-button", Glean.testGetExperimentData("blue-button-effective")?.branch
)
```

## Limits

* Fixed maximum experiment and branch id lengths: 30. Longer strings are truncated. For the original Kotlin implementation of the Glean SDK, this is measured in Unicode characters. For the Rust implementation, this is measured in the number of bytes when the string is encoded in UTF-8.

## Reference

* [Kotlin API docs](../../javadoc/glean/mozilla.telemetry.glean/-glean.html).
