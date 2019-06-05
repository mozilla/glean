# Datetime

**STATUS: Not implemented.**

Datetimes are used to record an absolute date and time, for example the date and time that the application was first run.

The device's offset from UTC is recorded and sent with the datetime value in the ping.

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
    ...
```

## API

```Kotlin
import org.mozilla.yourApplication.GleanMetrics.Install

Install.firstRun.set() // Records "now"
Install.firstRun.set(Calendar(2019, 3, 25)) // Records a custom datetime
```

There are test APIs available too.

```Kotlin
import org.mozilla.yourApplication.GleanMetrics.Install
Glean.enableTestingMode()

// Was anything recorded?
assertTrue(Install.firstRun.testHasValue())
// Was it the expected value?
// NOTE: Datetimes always include a timezone offset from UTC, hence the 
// "-05:00" suffix.
assertEquals("2019-03-25-05:00", Install.firstRun.testGetValueAsString())
```

## Limits

* None.

## Examples

* When did the user first run the application?

## Recorded errors

* None.

## Reference

* [Kotlin API docs](../../../javadoc/glean/mozilla.telemetry.glean.private/-datetime-metric-type/index.html)
