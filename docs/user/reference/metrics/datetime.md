# Datetime

Datetimes are used to record an absolute date and time,
for example the date and time that the application was first run.

The device's offset from UTC is recorded and sent with the Datetime value in the ping.

To record a single elapsed time, see [Timespan](timespan.md).
To measure the distribution of multiple timespans, see [Timing Distributions](timing_distribution.md).

## Recording API

### `set`

Sets a datetime metric to a specific date value. Defaults to now.

{{#include ../../../shared/tab_header.md}}

<div data-lang="Kotlin" class="tab">

```Kotlin
import org.mozilla.yourApplication.GleanMetrics.Install

Install.firstRun.set() // Records "now"
Install.firstRun.set(Calendar(2019, 3, 25)) // Records a custom datetime
```

</div>

<div data-lang="Java" class="tab">

```Java
import org.mozilla.yourApplication.GleanMetrics.Install;

Install.INSTANCE.firstRun.set(); // Records "now"
Install.INSTANCE.firstRun.set(Calendar(2019, 3, 25)); // Records a custom datetime
```

</div>

<div data-lang="Swift" class="tab">

```Swift
Install.firstRun.set() // Records "now"
let dateComponents = DateComponents(
                        calendar: Calendar.current,
                        year: 2004, month: 12, day: 9, hour: 8, minute: 3, second: 29
                     )
Install.firstRun.set(dateComponents.date!) // Records a custom datetime
```
</div>

<div data-lang="Python" class="tab">

```Python
import datetime

from glean import load_metrics
metrics = load_metrics("metrics.yaml")

metrics.install.first_run.set() # Records "now"
metrics.install.first_run.set(datetime.datetime(2019, 3, 25)) # Records a custom datetime
```

</div>

<div data-lang="Rust" class="tab">

```Rust
use glean_metrics;

use chrono::{FixedOffset, TimeZone};

install::first_run.set(None); // Records "now"
let custom_date = FixedOffset::east(0).ymd(2019, 3, 25).and_hms(0, 0, 0);
install::first_run.set(Some(custom_date)); // Records a custom datetime
```

</div>

<div data-lang="JavaScript" class="tab">

```js
import * as install from "./path/to/generated/files/install.js";

install.firstRun.set(); // Records "now"
install.firstRun.set(new Date("March 25, 2019 00:00:00")); // Records a custom datetime
```
</div>

<div data-lang="Firefox Desktop" class="tab">

**C++**

```c++
#include "mozilla/glean/GleanMetrics.h"

PRExplodedTime date = {0, 35, 10, 12, 6, 10, 2020, 0, 0, {5 * 60 * 60, 0}};
mozilla::glean::install::first_run.Set(&date);
```

**JavaScript**

```js
const value = new Date("2020-06-11T12:00:00");
Glean.install.firstRun.set(value.getTime() * 1000);
```

</div>

{{#include ../../../shared/tab_footer.md}}

#### Recorded errors

* [`invalid_value`](../../user/metrics/error-reporting.md): setting the date time to an invalid value.
* [`invalid_type`](../../user/metrics/error-reporting.md): if a non-Date object is given.

## Testing API

### `testGetValue`

Get the recorded value for a given datetime metric as a language-specific Date object.

{{#include ../../../shared/tab_header.md}}

<div data-lang="Kotlin" class="tab">

```Kotlin
import org.mozilla.yourApplication.GleanMetrics.Install

assertEquals(Install.firstRun.testGetValue(), Date(2019, 3, 25))
```
</div>

<div data-lang="Java" class="tab">

```Java
import org.mozilla.yourApplication.GleanMetrics.Install;

assertEquals(Install.INSTANCE.firstRun.testGetValue(), Date(2019, 3, 25));
```
</div>

<div data-lang="Swift" class="tab">

```Swift
let expectedDate = DateComponents(
                      calendar: Calendar.current,
                      year: 2004, month: 12, day: 9, hour: 8, minute: 3, second: 29
                   )
XCTAssertEqual(expectedDate.date!, try Install.firstRun.testGetValue())
```
</div>

<div data-lang="Python" class="tab">

```Python
import datetime

from glean import load_metrics
metrics = load_metrics("metrics.yaml")

value = datetime.datetime(1993, 2, 23, 5, 43, tzinfo=datetime.timezone.utc)
assert value == metrics.install.first_run.test_get_value()
```
</div>

<div data-lang="Rust" class="tab">

```Rust
use glean_metrics;

use chrono::{FixedOffset, TimeZone};

let expected_date = FixedOffset::east(0).ymd(2019, 3, 25).and_hms(0, 0, 0);
assert_eq!(expected_date, metrics.install.first_run.test_get_value(None));
```

</div>

<div data-lang="JavaScript" class="tab">

```js
import * as install from "./path/to/generated/files/install.js";

const expectedDate = new Date("March 25, 2019 00:00:00");
assert.deepStrictEqual(expectedDate, await install.firstRun.testGetValue());
```
</div>

<div data-lang="Firefox Desktop" class="tab">

**C++**

```c++
#include "mozilla/glean/GleanMetrics.h"

PRExplodedTime date{0, 35, 10, 12, 6, 10, 2020, 0, 0, {5 * 60 * 60, 0}};
ASSERT_TRUE(mozilla::glean::install::first_run.TestGetValue().isOk());
ASSERT_EQ(
    0,
    std::memcmp(
        &date,
        mozilla::glean::install::first_run.TestGetValue().unwrap().ptr(),
        sizeof(date)));
```

**JavaScript**

```js
const value = new Date("2020-06-11T12:00:00");
// testGetValue will throw NS_ERROR_LOSS_OF_SIGNIFICANT_DATA on error.
Assert.equal(Glean.install.firstRun.testGetValue().getTime(), value.getTime());
```

</div>
{{#include ../../../shared/tab_footer.md}}

### `testGetValueAsString`

Get the recorded value for a given datetime metric as an [ISO Date String](https://en.wikipedia.org/wiki/ISO_8601#Dates).

The returned string will be truncated to the metric's [time unit](#time_unit)
and will include the timezone offset from UTC, e.g. `2019-03-25-05:00`
(in this example, `time_unit` is `day`).

{{#include ../../../shared/tab_header.md}}

<div data-lang="Kotlin" class="tab">

```Kotlin
import org.mozilla.yourApplication.GleanMetrics.Install

assertEquals("2019-03-25-05:00", Install.firstRun.testGetValueAsString())
```

</div>

<div data-lang="Java" class="tab">

```Java
import org.mozilla.yourApplication.GleanMetrics.Install;

assertEquals("2019-03-25-05:00", Install.INSTANCE.firstRun.testGetValueAsString());
```

</div>

<div data-lang="Swift" class="tab">

```Swift
assertEquals("2019-03-25-05:00", try Install.firstRun.testGetValueAsString())
```
</div>

<div data-lang="Python" class="tab">

```Python
from glean import load_metrics
metrics = load_metrics("metrics.yaml")

assert "2019-03-25-05:00" == metrics.install.first_run.test_get_value_as_str()
```

</div>

<div data-lang="Rust" class="tab"></div>

<div data-lang="JavaScript" class="tab">

```js
import * as install from "./path/to/generated/files/install.js";

assert.strictEqual("2019-03-25-05:00", await install.firstRun.testGetValueAsString());
```
</div>

<div data-lang="Firefox Desktop" class="tab"></div>

{{#include ../../../shared/tab_footer.md}}

### `testHasValue`

Whether or not **any** value was recorded for a given datetime metric.

{{#include ../../../shared/tab_header.md}}

<div data-lang="Kotlin" class="tab">

```Kotlin
import org.mozilla.yourApplication.GleanMetrics.Install

assertTrue(Install.firstRun.testHasValue())
```

</div>

<div data-lang="Java" class="tab">

```Java
import org.mozilla.yourApplication.GleanMetrics.Install;

assertTrue(Install.INSTANCE.firstRun.testHasValue());
```

</div>

<div data-lang="Swift" class="tab">

```Swift
XCTAssert(Install.firstRun.testHasValue())
```
</div>

<div data-lang="Python" class="tab">

```Python
from glean import load_metrics
metrics = load_metrics("metrics.yaml")

assert metrics.install.first_run.test_has_value()
```

</div>

<div data-lang="Rust" class="tab"></div>

<div data-lang="JavaScript" class="tab"></div>

<div data-lang="Firefox Desktop" class="tab"></div>

{{#include ../../../shared/tab_footer.md}}

### `testGetNumRecordedErrors`

Get number of errors recorded for a given datetime metric.

{{#include ../../../shared/tab_header.md}}

<div data-lang="Kotlin" class="tab">

```Kotlin
import mozilla.telemetry.glean.testing.ErrorType
import org.mozilla.yourApplication.GleanMetrics.Install

assertEquals(0, Install.firstRun.testGetNumRecordedErrors(ErrorType.InvalidValue))
```

</div>

<div data-lang="Java" class="tab">

```Java
import mozilla.telemetry.glean.testing.ErrorType
import org.mozilla.yourApplication.GleanMetrics.Install;

assertEquals(0, Install.INSTANCE.firstRun.testGetNumRecordedErrors(ErrorType.InvalidValue));
```

</div>

<div data-lang="Swift" class="tab">

```Swift
XCTAssertEqual(0, Install.firstRun.getNumRecordedErrors(.invalidValue))
```
</div>

<div data-lang="Python" class="tab">

```Python
from glean import load_metrics
metrics = load_metrics("metrics.yaml")

assert 0 == metrics.install.test_get_num_recorded_errors(
    ErrorType.INVALID_VALUE
)
```

</div>

<div data-lang="Rust" class="tab">

```rust
use glean::ErrorType;
use glean_metrics;

assert_eq!(0, install::first_run.test_get_num_recorded_errors(
    ErrorType::InvalidValue
));
```

</div>

<div data-lang="JavaScript" class="tab">

```js
import * as install from "./path/to/generated/files/install.js";
import { ErrorType } from "@mozilla/glean/<platform>";

assert.strictEqual(
  1,
  await install.firstRun.testGetNumRecordedErrors(ErrorType.InvalidValue)
);
```
</div>

<div data-lang="Firefox Desktop" class="tab" data-info="Firefox Desktop uses testGetValue to communicate errors"></div>

{{#include ../../../shared/tab_footer.md}}

## Metric parameters

Example datetime metric definition:

```yaml
install:
  first_run:
    type: datetime
    time_unit: day
    description: >
      Records the date when the application was first run
    bugs:
      - https://bugzilla.mozilla.org/000000
    data_reviews:
      - https://bugzilla.mozilla.org/show_bug.cgi?id=000000#c3
    notification_emails:
      - me@mozilla.com
    expires: 2020-10-01
```

For a full reference on metrics parameters common to all metric types,
refer to the [metrics YAML registry format](../yaml/metrics.md) reference page.

### Extra metric parameters

#### `time_unit`

Datetimes have a required `time_unit` parameter to specify the smallest unit of resolution that the metric will record. The allowed values for `time_unit` are:

* `nanosecond`
* `microsecond`
* `millisecond`
* `second`
* `minute`
* `hour`
* `day`

Carefully consider the required resolution for recording your metric, and choose the coarsest resolution possible.

## Data questions

* When did the user first run the application?

## Reference

* [Swift API docs](../../../swift/Classes/DatetimeMetricType.html)
* [Python API docs](../../../python/glean/metrics/datetime.html)
* [Rust API docs](../../../docs/glean/private/struct.DatetimeMetric.html)
* [Datetime API docs](https://mozilla.github.io/glean.js/classes/core_metrics_types_datetime.default.html)
