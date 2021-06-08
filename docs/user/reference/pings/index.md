# Submitting a custom ping

To collect and queue a custom ping for eventual uploading, call the `submit` method on the `PingType` object that the Glean SDK generated for your ping.

By default, if the ping doesn't currently have any events or metrics set, `submit` will do nothing.  However, if the `send_if_empty` flag is set to true in the ping definition, it will always be submitted.

For example, to submit the custom ping defined in [Adding new custom pings](../../user/pings/custom.md#defining-a-custom-ping):

{{#include ../../../shared/tab_header.md}}

<div data-lang="Kotlin" class="tab">

```kotlin
import org.mozilla.yourApplication.GleanMetrics.Pings
Pings.search.submit(
    GleanMetrics.Pings.searchReasonCodes.performed
)
```

</div>

<div data-lang="Java" class="tab"></div>

<div data-lang="Swift" class="tab">

```swift
import Glean

GleanMetrics.Pings.shared.search.submit(
    reason: .performed
)
```

</div>

<div data-lang="Python" class="tab">

```Python
from glean import load_pings

pings = load_pings("pings.yaml")

pings.search.submit(pings.search_reason_codes.PERFORMED)
```

</div>

<div data-lang="Rust" class="tab">

```Rust
use glean::Pings;

pings::search.submit(pings::SearchReasonCodes::Performed);
```

</div>

<div data-lang="Javascript" class="tab">

```js
import * as pings from "./path/to/generated/files/pings.js";

pings.search.submit("performed");
```
</div>

<div data-lang="Firefox Desktop" class="tab">

**C++**

```cpp
mozilla::glean_pings::Search.Submit("performed"_ns);
```

**JavaScript**

```js
GleanPings.search.submit("performed");
```

</div>

{{#include ../../../shared/tab_footer.md}}

If none of the metrics for the ping contain data the ping is not sent (unless `send_if_empty` is set to true in the definition file)
