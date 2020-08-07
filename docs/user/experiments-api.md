# Using the experiments API

The Glean SDK supports tagging all its pings with experiments annotations. The annotations are useful to report that experiments were active at the time the measurement were collected. The annotations are reported in the optional `experiments` entry in the [`ping_info` section](pings/index.md) of all the Glean SDK pings.

> **Important**: the experiment annotations set through this API are not persisted by the Glean SDK.
> The application or consuming library is responsible for setting the relevant experiment annotations at each run.

## API

{{#include ../tab_header.md}}

<div data-lang="Kotlin" class="tab">

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

> **Important**: Experiment IDs and branches don't need to be pre-defined in the Glean SDK registry files.
Please also note that the `extra` map is a non-nested arbitrary `String` to `String` map. It also has limits on the size of the keys and values defined below.

There are test APIs available too:

```Kotlin
// Was the experiment annotated in Glean pings?
assertTrue(Glean.testIsExperimentActive("blue-button-effective"))
// Was the correct branch reported?
assertEquals(
  "branch-with-blue-button", Glean.testGetExperimentData("blue-button-effective")?.branch
)
```

</div>

<div data-lang="Swift" class="tab">

```Swift
// Annotate Glean pings with experiments data.
Glean.shared.setExperimentActive(
  experimentId: "blue-button-effective",
  branch: "branch-with-blue-button",
  extra: ["buttonLabel": "test"]
)
// After the experiment terminates, the annotation
// can be removed.
Glean.shared.setExperimentInactive(experimentId: "blue-button-effective")
```

> **Important**: Experiment IDs and branch don't need to be pre-defined in the Glean SDK registry files.
Please also note that the `extra` is a non-nested `Dictionary` of type `[String: String]`.

There are test APIs available too:

```Swift
// Was the experiment annotated in Glean pings?
XCTAssertTrue(Glean.shared.testIsExperimentActive(experimentId: "blue-button-effective"))
// Was the correct branch reported?
XCTAssertEqual(
  "branch-with-blue-button",
  Glean.testGetExperimentData(experimentId: "blue-button-effective")?.branch
)
```

</div>

<div data-lang="Python" class="tab">

```Python
from glean import Glean

# Annotate Glean pings with experiments data.
Glean.set_experiment_active(
  experiment_id="blue-button-effective",
  branch="branch-with-blue-button",
  extra={
    "buttonLabel": "test"
  }
)

# After the experiment terminates, the annotation
# can be removed.
Glean.set_experiment_inactive("blue-button-effective")
```

> **Important**: Experiment IDs and branch don't need to be pre-defined in the Glean SDK registry files.
Please also note that the `extra` dict is non-nested arbitrary `str` to `str` mapping.

There are test APIs available too:

```Python
from glean import Glean

# Was the experiment annotated in Glean pings?
assert Glean.test_is_experiment_active("blue-button-effective")
# Was the correct branch reported?
assert (
    "branch-with-blue-button" ==
    Glean.test_get_experiment_data("blue-button-effective").branch
)
```

</div>

<div data-lang="C#" class="tab">

```C#
// Annotate Glean pings with experiments data.
GleanInstance.SetExperimentActive(
  experimentId: "blue-button-effective",
  branch: "branch-with-blue-button",
  extra: new Dictionary<string, string>() {
    { "buttonLabel", "test"}
  }
);
// After the experiment terminates, the annotation
// can be removed.
GleanInstance.SetExperimentInactive("blue-button-effective");
```

> **Important**: Experiment IDs and branches don't need to be pre-defined in the Glean SDK registry files.
Please also note that the `extra` map is a non-nested arbitrary `string` to `string` dictionary. It also has limits on the size of the keys and values defined below.

There are test APIs available too:

```C#
// Was the experiment annotated in Glean pings?
Assert.True(GleanInstance.TestIsExperimentActive("blue-button-effective"));
// Was the correct branch reported?
Assert.Equal(
  "branch-with-blue-button", GleanInstance.TestGetExperimentData("blue-button-effective").Branch
);
```

</div>

{{#include ../tab_footer.md}}

## Limits

* `experimentId`, `branch`, and the keys and values of the 'extra' field are fixed at a maximum length of 100 bytes. Longer strings used as ids, keys, or values are truncated to their respective maximum length. (Specifically, this is measured in the number of bytes when the string is encoded in UTF-8.)
* `extra` map is limited to 20 entries. If passed a map which contains more elements than this, it is truncated to 20 elements.  **WARNING** Which items are truncated is nondeterministic due to the unordered nature of maps and what's left may not necessarily be the first elements added.

**NOTE:** Any truncation that occurs when recording to the Experiments API will result in an `invalid_value` error being recorded. See [Error Reporting](error-reporting.md) for more information about this type of error.

## Reference

* [Kotlin API docs](../../javadoc/glean/mozilla.telemetry.glean/-glean.html).
* [Python API docs](../../python/glean/glean.html)
