# Datetime

Datetimes are used to record an absolute date and time, for example the date and time that the application was first run.

The device's offset from UTC is recorded and sent with the Datetime value in the ping.

To measure the distribution of multiple timespans, see [Timing Distributions](timing_distribution.md). To record absolute times, see [Datetimes](datetime.md).

## Configuration

Datetimes have a required `time_unit` parameter to specify the smallest unit of resolution that the timespan will record. The allowed values for `time_unit` are:

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
assertEquals(1, Install.firstRun.testGetNumRecordedErrors(ErrorType.InvalidValue))
```

</div>

<div data-lang="Java" class="tab">

```Java
import org.mozilla.yourApplication.GleanMetrics.Install

Install.INSTANCE.getFirstRun.set() // Records "now"
Install.INSTANCE.getFirstRun.set(Calendar(2019, 3, 25)) // Records a custom datetime
```

There are test APIs available too:

```Java
import org.mozilla.yourApplication.GleanMetrics.Install

// Was anything recorded?
assertTrue(Install.INSTANCE.getFirstRun.testHasValue())
// Was it the expected value?
// NOTE: Datetimes always include a timezone offset from UTC, hence the
// "-05:00" suffix.
assertEquals("2019-03-25-05:00", Install.INSTANCE.getFirstRun.testGetValueAsString())
// Was the value invalid?
assertEquals(1, Install.INSTANCE.getFirstRun.testGetNumRecordedErrors(ErrorType.InvalidValue))
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
XCTAssertEqual(1, Install.firstRun.getNumRecordedErrors(.invalidValue))
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
