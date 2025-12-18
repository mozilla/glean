# Using the experiments API

The Glean SDKs support tagging all their pings with experiments annotations. The annotations are useful to report that experiments were active at the time the measurement were collected. The annotations are reported in the optional `experiments` entry in the [`ping_info` section](../../user/pings/index.md) of all pings.

{{#include ../../../shared/blockquote-warning.html}}

##### Experiment annotations are not persisted

> The experiment annotations set through this API are not persisted by the Glean SDKs.
> The application or consuming library is responsible for setting the relevant experiment annotations at each run.

{{#include ../../../shared/blockquote-info.html}}

##### It's not required to define experiment IDs and branches

> Experiment IDs and branches don't need to be pre-defined in the Glean SDK registry files.
> Please also note that the `extra` map is a non-nested arbitrary `String` to `String` map. It also has limits on the size of the keys and values defined below.

## Recording API

### `setExperimentActive`

Annotates Glean pings with experiment data.

{{#include ../../../shared/tab_header.md}}

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
```
</div>

<div data-lang="Java" class="tab"></div>

<div data-lang="Swift" class="tab">

```Swift
// Annotate Glean pings with experiments data.
Glean.shared.setExperimentActive(
  experimentId: "blue-button-effective",
  branch: "branch-with-blue-button",
  extra: ["buttonLabel": "test"]
)
```
</div>

<div data-lang="Python" class="tab">

```Python
from glean import Glean

Glean.set_experiment_active(
  experiment_id="blue-button-effective",
  branch="branch-with-blue-button",
  extra={
    "buttonLabel": "test"
  }
)
```
</div>

<div data-lang="JavaScript" class="tab" data-bug="1741583"></div>

<div data-lang="Rust" class="tab">

```Rust
let mut extra = HashMap::new();
extra.insert("buttonLabel".to_string(), "test".to_string());
glean::set_experiment_active(
    "blue-button-effective".to_string(),
    "branch-with-blue-button".to_string(),
    Some(extra),
);
```
</div>

<div data-lang="Firefox Desktop" class="tab">

**C++**

{{#include ../../../shared/blockquote-info.html}}

##### At present there is no dedicated C++ Experiments API for Firefox Desktop

> If you require one, please [file a bug](https://bugzilla.mozilla.org/enter_bug.cgi?product=Toolkit&component=Telemetry).

**JavaScript**

```js
let FOG = Cc["@mozilla.org/toolkit/glean;1"].createInstance(Ci.nsIFOG);
FOG.setExperimentActive(
  "blue-button-effective",
  "branch-with-blue-button",
  {"buttonLabel": "test"}
);
```
</div>

{{#include ../../../shared/tab_footer.md}}

#### Limits

* `experimentId`, `branch`, and the keys and values of the `extra`
  field are fixed at a maximum length of 100 bytes.
  Longer strings are truncated.
  (Specifically, length is measured in the number of bytes when the string is encoded in UTF-8.)
* `extra` map is limited to 20 entries.
  If passed a map which contains more elements than this, it is truncated to 20 elements.
  **WARNING** Which items are truncated is nondeterministic due to the unordered nature of maps.
  What's left may not necessarily be the first elements added.

#### Recorded errors

* [`invalid_value`](../../user/metrics/error-reporting.md):
  If the values of `experimentId` or `branch` are truncated for length,
  if the keys or values in the `extra` map are truncated for length,
  or if the `extra` map is truncated for the number of elements.

### `setExperimentInactive`

Removes the experiment annotation.
Should be called when the experiment ends.

{{#include ../../../shared/tab_header.md}}

<div data-lang="Kotlin" class="tab">

```Kotlin
Glean.setExperimentInactive("blue-button-effective")
```
</div>

<div data-lang="Java" class="tab"></div>

<div data-lang="Swift" class="tab">

```Swift
Glean.shared.setExperimentInactive(experimentId: "blue-button-effective")
```
</div>

<div data-lang="Python" class="tab">

```Python
from glean import Glean

Glean.set_experiment_inactive("blue-button-effective")
```
</div>

<div data-lang="JavaScript" class="tab" data-bug="1741583"></div>

<div data-lang="Rust" class="tab">

```Rust
glean::set_experiment_inactive("blue-button-effective".to_string());
```
</div>

<div data-lang="Firefox Desktop" class="tab">

**C++**

{{#include ../../../shared/blockquote-info.html}}

##### At present there is no dedicated C++ Experiments API for Firefox Desktop

> If you require one, please [file a bug](https://bugzilla.mozilla.org/enter_bug.cgi?product=Toolkit&component=Telemetry).

**JavaScript**

```js
let FOG = Cc["@mozilla.org/toolkit/glean;1"].createInstance(Ci.nsIFOG);
FOG.setExperimentInactive("blue-button-effective");
```
</div>

{{#include ../../../shared/tab_footer.md}}

### Set an experimentation identifier

An experimentation enrollment identifier that is derived and provided by the application can be set through the configuration object passed into the `initialize` function. See the section on [Initializing Glean](initializing.md) for more information on how to set this within the `Configuration` object.

This identifier will be set during initialization and sent along with all pings sent by Glean, unless that ping is has opted out of sending the client_id. This identifier is not persisted by Glean and must be persisted by the application if necessary for it to remain consistent between runs.

#### Limits

The experimentation ID is subject to the same limitations as a [string metric type](../metrics/string.md#limits).

#### Recorded errors

The experimentation ID will produce the same errors as a [string metric type](../metrics/string.md#recorded-errors).

## Testing API

### `testIsExperimentActive`

Reveals if the experiment is annotated in Glean pings.

{{#include ../../../shared/tab_header.md}}

<div data-lang="Kotlin" class="tab">

```Kotlin
assertTrue(Glean.testIsExperimentActive("blue-button-effective"))
```
</div>

<div data-lang="Java" class="tab"></div>

<div data-lang="Swift" class="tab">

```Swift
XCTAssertTrue(Glean.shared.testIsExperimentActive(experimentId: "blue-button-effective"))
```
</div>

<div data-lang="Python" class="tab">

```Python
from glean import Glean

assert Glean.test_is_experiment_active("blue-button-effective")
```
</div>

<div data-lang="JavaScript" class="tab" data-bug="1741583"></div>

<div data-lang="Rust" class="tab">

```Rust
assert!(glean::test_is_experiment_active("blue-button-effective".to_string());
```
</div>

<div data-lang="Firefox Desktop" class="tab"></div>

{{#include ../../../shared/tab_footer.md}}

### `testGetExperimentData`

Returns the recorded experiment data including branch and extras.

{{#include ../../../shared/tab_header.md}}

<div data-lang="Kotlin" class="tab">

```Kotlin
assertEquals(
  "branch-with-blue-button", Glean.testGetExperimentData("blue-button-effective")?.branch
)
```
</div>

<div data-lang="Java" class="tab"></div>

<div data-lang="Swift" class="tab">

```Swift
XCTAssertEqual(
  "branch-with-blue-button",
  Glean.testGetExperimentData(experimentId: "blue-button-effective")?.branch
)
```
</div>

<div data-lang="Python" class="tab">

```Python
from glean import Glean

assert (
    "branch-with-blue-button" ==
    Glean.test_get_experiment_data("blue-button-effective").branch
)
```
</div>

<div data-lang="JavaScript" class="tab" data-bug="1741583"></div>

<div data-lang="Rust" class="tab">

```Rust
assert_eq!(
    "branch-with-blue-button",
    glean::test_get_experiment_data("blue-button-effective".to_string()).branch,
);
```
</div>

<div data-lang="Firefox Desktop" class="tab">

**C++**

{{#include ../../../shared/blockquote-info.html}}

##### At present there is no dedicated C++ Experiments API for Firefox Desktop

> If you require one, please [file a bug](https://bugzilla.mozilla.org/enter_bug.cgi?product=Toolkit&component=Telemetry).

**JavaScript**

```js
let FOG = Cc["@mozilla.org/toolkit/glean;1"].createInstance(Ci.nsIFOG);
Assert.equals(
  "branch-with-blue-button",
  FOG.testGetExperimentData("blue-button-effective").branch
);
```
</div>

{{#include ../../../shared/tab_footer.md}}

### `testGetExperimentationId`

Returns the current Experimentation ID, if any.

{{#include ../../../shared/tab_header.md}}

<div data-lang="Kotlin" class="tab">

```Kotlin
assertEquals("alpha-beta-gamma-delta", Glean.testGetExperimentationId())
```
</div>

<div data-lang="Java" class="tab"></div>

<div data-lang="Swift" class="tab">

```Swift
XCTAssertEqual(
  "alpha-beta-gamma-delta",
  Glean.shared.testGetExperimentationId()!,
  "Experimenatation ids must match"
)
```
</div>

<div data-lang="Python" class="tab">

```Python
from glean import Glean

assert "alpha-beta-gamma-delta" == Glean.test_get_experimentation_id()
```
</div>

<div data-lang="JavaScript" class="tab" data-bug="1850323"></div>

<div data-lang="Rust" class="tab">

```Rust
assert_eq!(
  "alpha-beta-gamma-delta".to_string(),
  glean_test_get_experimentation_id(),
  "Experimentation id must match"
);
```
</div>

<div data-lang="Firefox Desktop" class="tab" data-bug="1850479"></div>

{{#include ../../../shared/tab_footer.md}}

## Reference

* [Python API docs](../../../python/glean/index.html#glean.Glean.set_experiment_active)
* [Rust API docs](../../../docs/glean/fn.set_experiment_active.html)
