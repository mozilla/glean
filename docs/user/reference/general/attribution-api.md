# Recording product attribution and distribution

The Glean SDKs support the reporting of product attribution
(the source and context that may have led a person to choose the product)
and product distribution
(the name and context of partners distributing the product on our behalf)
in all pings that contain a `client_info`.
The Glean SDK has no means to determine this itself,
so it relies on products setting and keeping these values up-to-date via general APIs.

## Recording API

### `updateAttribution`

Updates the values sent in the `client_info.attribution` section.

{{#include ../../../shared/tab_header.md}}

<div data-lang="Kotlin" class="tab">

```Kotlin
Glean.updateAttribution(AttributionMetrics(
  source = "google-play",
  medium = "organic",
  campaign = "mozilla-org",
  term = "browser with developer tools for android",
  content = "firefoxview",
))
```
</div>

<div data-lang="Java" class="tab"></div>

<div data-lang="Swift" class="tab">

```Swift
Glean.shared.updateAttribution(AttributionMetrics(
  source: "google-play",
  medium: "organic",
  campaign: "mozilla-org",
  term: "browser with developer tools for android",
  content: "firefoxview",
))
```
</div>

<div data-lang="Python" class="tab">

```Python
from glean import Glean
from glean.metrics import AttributionMetrics

Glean.update_attribution(AttributionMetrics(
  source="google-play",
  medium="organic",
  campaign="mozilla-org",
  term="browser with developer tools for android",
  content="firefoxview",
))
```
</div>

<div data-lang="JavaScript" class="tab"></div>

<div data-lang="Rust" class="tab">

```Rust
glean::update_attribution(AttributionMetrics {
  source: Some("google-play".into()),
  medium: Some("organic".into()),
  campaign: Some("mozilla-org".into()),
  term: Some("browser with developer tools for android".into()),
  content: Some("firefoxview".into()),
});
```
</div>

<div data-lang="Firefox Desktop" class="tab" data-bug="1955429"></div>

{{#include ../../../shared/tab_footer.md}}

#### Limits

* Each `AttributionMetrics` field inherits the limits of a `string` metric.
{{#include ../../_includes/string-limits.md}}

#### Recorded errors

* Each `AttributionMetrics` field may report errors like `string` metrics do.
{{#include ../../_includes/string-errors.md}}

### `updateDistribution`

Updates the value sent in the `client_info.distribution` section.

{{#include ../../../shared/tab_header.md}}

<div data-lang="Kotlin" class="tab">

```Kotlin
Glean.updateDistribution(DistributionMetrics(
  name = "MozillaOnline",
))
```
</div>

<div data-lang="Java" class="tab"></div>

<div data-lang="Swift" class="tab">

```Swift
Glean.shared.updateDistribution(DistributionMetrics(
  name: "MozillaOnline",
))
```
</div>

<div data-lang="Python" class="tab">

```Python
from glean import Glean
from glean.metrics import DistributionMetrics

Glean.update_attribution(DistributionMetrics(
  name="MozillaOnline",
))
```
</div>

<div data-lang="JavaScript" class="tab"></div>

<div data-lang="Rust" class="tab">

```Rust
glean::update_attribution(DistributionMetrics {
  name: Some("MozillaOnline".into()),
});
```
</div>

<div data-lang="Firefox Desktop" class="tab" data-bug="1955429"></div>

{{#include ../../../shared/tab_footer.md}}

#### Limits

* Each `DistributionMetrics` field inherits the limits of a `string` metric.
{{#include ../../_includes/string-limits.md}}

#### Recorded errors

* Each `DistributionMetrics` field may report errors like `string` metrics do.
{{#include ../../_includes/string-errors.md}}

## Testing API

### `testGetAttribution`

Returns the current values of Attribution fields.

{{#include ../../../shared/tab_header.md}}

<div data-lang="Kotlin" class="tab">

```Kotlin
import org.junit.Assert.assertEquals

assertEquals(AttributionMetrics(
  source: "google-play",
  medium: "organic",
  campaign: "mozilla-org",
  term: "browser with developer tools for android",
  content: "firefoxview",
), Glean.testGetAttribution())
```
</div>

<div data-lang="Java" class="tab"></div>

<div data-lang="Swift" class="tab">

```Swift
XCTAssertEqual(AttributionMetrics(
  source: "google-play",
  medium: "organic",
  campaign: "mozilla-org",
  term: "browser with developer tools for android",
  content: "firefoxview",
), Glean.shared.testGetAttribution())
```
</div>

<div data-lang="Python" class="tab">

```Python
from glean import Glean
from glean.metrics import AttributionMetrics

assert AttributionMetrics(
  source="google-play",
  medium="organic",
  campaign="mozilla-org",
  term="browser with developer tools for android",
  content="firefoxview") == Glean.test_get_attribution()
```
</div>

<div data-lang="JavaScript" class="tab" data-bug="1741583"></div>

<div data-lang="Rust" class="tab">

```Rust
assert_eq!(AttributionMetrics {
  source: Some("google-play".into()),
  medium: Some("organic".into()),
  campaign: Some("mozilla-org".into()),
  term: Some("browser with developer tools for android".into()),
  content: Some("firefoxview".into()),
}, glean::test_get_attribution());
```
</div>

<div data-lang="Firefox Desktop" class="tab" data-bug="1955429"></div>

{{#include ../../../shared/tab_footer.md}}

### `testGetDistribution`

Returns the current values of Distribution fields.

{{#include ../../../shared/tab_header.md}}

<div data-lang="Kotlin" class="tab">

```Kotlin
import org.junit.Assert.assertEquals

assertEquals(DistributionMetrics(
  name: "MozillaOnline",
), Glean.testGetDistribution())
```
</div>

<div data-lang="Java" class="tab"></div>

<div data-lang="Swift" class="tab">

```Swift
XCTAssertEqual(DistributionMetrics(
  name: "MozillaOnline",
), Glean.shared.testGetDistribution())
```
</div>

<div data-lang="Python" class="tab">

```Python
from glean import Glean
from glean.metrics import DistributionMetrics

assert DistributionMetrics(name="MozillaOnline") == Glean.test_get_distribution()
```
</div>

<div data-lang="JavaScript" class="tab" data-bug="1741583"></div>

<div data-lang="Rust" class="tab">

```Rust
assert_eq!(DistributionMetrics {
  name: Some("MozillaOnline".into()),
}, glean::test_get_distribution());
```
</div>

<div data-lang="Firefox Desktop" class="tab" data-bug="1955429"></div>

{{#include ../../../shared/tab_footer.md}}

## Extending product attribution and distribution

In addition to the fields in these APIs,
products may also define and record extended product attribution and
distribution information in two `object` metrics:
* `glean.attribution.ext`, and
* `glean.distribution.ext`.

These metrics are different from normal `object` metrics:
* They are defined by products, even though they use the reserved `glean` metric category
* Columns for their data in datasets are in `client_info` instead of in `metrics.object`
  * Specifically, `client_info.attribution.ext` and `client_info.distribution.ext`

These metrics are different from other product attribution and distribution fields:
* Their structure and contents will differ from product to product
* They will only be present in pings defined in their metric definitions
* Their lifetime might not be `user`,
  so they may not have values in all pings they are defined to be sent in

Otherwise, they are defined, recorded to, tested, and consumed like
[typical `object` metrics](../metrics/object.md).

## Reference

* [Swift API docs](../../../swift/Classes/Glean.html)
* [Python API docs](../../../python/glean/index.html#glean.Glean.update_attribution)
* [Rust API docs](../../../docs/glean/fn.update_attribution.html)
