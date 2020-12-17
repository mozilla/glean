# Datetime

Datetimes are used to record an absolute date and time, for example the date and time that the application was first run.

The device's offset from UTC is recorded and sent with the Datetime value in the ping.

To record a single elapsed time, see [Timespan](timespan.md). To measure the distribution of multiple timespans, see [Timing Distributions](timing_distribution.md).

## Configuration

Datetimes have a required `time_unit` parameter to specify the smallest unit of resolution that the metric will record. The allowed values for `time_unit` are:

   - `nanosecond`
   - `microsecond`
   - `millisecond`
   - `second`
   - `minute`
   - `hour`
   - `day`

Carefully consider the required resolution for recording your metric, and choose the coarsest resolution possible.

You first need to add an entry for it to the `metrics.yaml` file:

```YAML
install:
  first_run:
    type: datetime
    time_unit: day
    description: >
      Records the date when the application was first run
    lifetime: user
    ...
```

## API

{{#include ../../tab_header.md}}

<div data-lang="Kotlin" class="tab">

```Kotlin
import org.mozilla.yourApplication.GleanMetrics.Install

Install.firstRun.set() // Records "now"
Install.firstRun.set(Calendar(2019, 3, 25)) // Records a custom datetime
```

There are test APIs available too.

```Kotlin
import org.mozilla.yourApplication.GleanMetrics.Install

// Was anything recorded?
assertTrue(Install.firstRun.testHasValue())
// Was it the expected value?
// NOTE: Datetimes always include a timezone offset from UTC, hence the
// "-05:00" suffix.
assertEquals("2019-03-25-05:00", Install.firstRun.testGetValueAsString())
// Was the value invalid?
assertEquals(0, Install.firstRun.testGetNumRecordedErrors(ErrorType.InvalidValue))
```

</div>

<div data-lang="Java" class="tab">

```Java
import org.mozilla.yourApplication.GleanMetrics.Install;

Install.INSTANCE.firstRun.set(); // Records "now"
Install.INSTANCE.firstRun.set(Calendar(2019, 3, 25)); // Records a custom datetime
```

There are test APIs available too:

```Java
import org.mozilla.yourApplication.GleanMetrics.Install;

// Was anything recorded?
assertTrue(Install.INSTANCE.firstRun.testHasValue());
// Was it the expected value?
// NOTE: Datetimes always include a timezone offset from UTC, hence the
// "-05:00" suffix.
assertEquals("2019-03-25-05:00", Install.INSTANCE.firstRun.testGetValueAsString());
// Was the value invalid?
assertEquals(0, Install.INSTANCE.firstRun.testGetNumRecordedErrors(ErrorType.InvalidValue));
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

There are test APIs available too:

```Swift
@testable import Glean

// Was anything recorded?
XCTAssert(Install.firstRun.testHasValue())
// Does the datetime have the expected value?
XCTAssertEqual(6, try Install.firstRun.testGetValue())
// Was the value invalid?
XCTAssertEqual(0, Install.firstRun.getNumRecordedErrors(.invalidValue))
```

</div>

<div data-lang="Python" class="tab">

```Python
import datetime

from glean import load_metrics
metrics = load_metrics("metrics.yaml")

# Records "now"
metrics.install.first_run.set()
# Records a custom datetime
metrics.install.first_run.set(datetime.datetime(2019, 3, 25))
```

There are test APIs available too.

```Python
# Was anything recorded?
assert metrics.install.first_run.test_has_value()

# Was it the expected value?
# NOTE: Datetimes always include a timezone offset from UTC, hence the
# "-05:00" suffix.
assert "2019-03-25-05:00" == metrics.install.first_run.test_get_value_as_str()
# Was the value invalid?
assert 0 == metrics.install.test_get_num_recorded_errors(
    ErrorType.INVALID_VALUE
)
```

</div>

<div data-lang="C#" class="tab">

```C#
using static Mozilla.YourApplication.GleanMetrics.Install;

// Records "now"
Install.firstRun.Set();
// Records a custom datetime
Install.firstRun.Set(new DateTimeOffset(2018, 2, 25, 11, 10, 0, TimeZone.CurrentTimeZone.BaseUtcOffset));
```

There are test APIs available too:

```C#
using static Mozilla.YourApplication.GleanMetrics.Install;

// Was anything recorded?
Assert.True(Install.firstRun.TestHasValue());
// Was it the expected value?
// NOTE: Datetimes always include a timezone offset from UTC, hence the
// "-05:00" suffix.
Assert.Equal("2019-03-25-05:00", Install.firstRun.TestGetValueAsString());
// Was the value invalid?
Assert.Equal(0, Install.firstRun.TestGetNumRecordedErrors(ErrorType.InvalidValue));
```

</div>

<div data-lang="Rust" class="tab">

```Rust
use glean::ErrorType;
use glean_metrics;

use chrono::{FixedOffset, TimeZone};

// Records "now"
install::first_run.set(None);
// Records a custom datetime
let custom_date = FixedOffset::east(0).ymd(2019, 3, 25).and_hms(0, 0, 0);
install::first_run.set(Some(custom_date));
```

There are test APIs available too.

```Rust
// Was anything recorded?
assert!(metrics.install.first_run.test_get_value(None).is_some());

// Was it the expected value?
let expected_date = FixedOffset::east(0).ymd(2019, 3, 25).and_hms(0, 0, 0);
assert_eq!(expected_date, metrics.install.first_run.test_get_value(None));
// Was the value invalid?
assert_eq!(0, install::first_run.test_get_num_recorded_errors(
    ErrorType::InvalidValue
));
```

</div>

<div data-lang="C++" class="tab">

> **Note**: C++ APIs are only available in Firefox Desktop.

```c++
#include "mozilla/glean/GleanMetrics.h"

PRExplodedTime date = {0, 35, 10, 12, 6, 10, 2020, 0, 0, {5 * 60 * 60, 0}};
mozilla::glean::install::first_run.Set(&date);
```

There are test APIs available too:

```c++
#include "mozilla/glean/GleanMetrics.h"

// Does it have the expected value?
ASSERT_STREQ(
  mozilla::glean::install::first_run.TestGetValue().value(),
  "2020-11-06T12:10:35+05:00"_ns
);
// Did it run across any errors?
// TODO: https://bugzilla.mozilla.org/show_bug.cgi?id=1683171
```

</div>

<div data-lang="JS" class="tab">

> **Note**: JS APIs are only available in Firefox Desktop.

```js
const value = new Date("2020-06-11T12:00:00");
Glean.install.firstRun.set(value.getTime() * 1000);
```

There are test APIs available too:

```js
Assert.ok(Glean.install.firstRun.testGetValue().startsWith("2020-06-11T12:00:00"));
// Did it run across any errors?
// TODO: https://bugzilla.mozilla.org/show_bug.cgi?id=1683171
```

</div>

{{#include ../../tab_footer.md}}

## Limits

* None.

## Examples

* When did the user first run the application?

## Recorded errors

* `invalid_value`: Setting the date time to an invalid value.

## Reference

* [Kotlin API docs](../../../javadoc/glean/mozilla.telemetry.glean.private/-datetime-metric-type/index.html)
* [Swift API docs](../../../swift/Classes/DatetimeMetricType.html)
* [Python API docs](../../../python/glean/metrics/datetime.html)
* [Rust API docs](../../../docs/glean/private/struct.DatetimeMetric.html)
