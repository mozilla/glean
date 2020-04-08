# The `metrics` ping

## Description
The `metrics` ping is intended for all of the metrics that are explicitly set by the application or are included in the application's `metrics.yaml` file (except events). 
The reported data is tied to the ping's *measurement window*, which is the time between the collection of two `metrics` pings. 
Ideally, this window is expected to be about 24 hours, given that the collection is scheduled daily at 04:00. 
However, the metrics ping is only submitted while the application is actually running, so in practice, it may not meet the 04:00 target very frequently.
Data in the [`ping_info`](index.md#the-ping_info-section) section of the ping can be used to infer the length of this window and the reason that triggered the ping to be submitted.
If the application crashes, unsent recorded metrics are sent along with the next `metrics` ping.

Additionally, it is undesirable to mix metric recording from different versions of the application. Therefore, if a version upgrade is detected, the `metrics` ping is collected immediately before further metrics from the new version are recorded.

> **Note:** As the `metrics` ping was specifically designed for mobile operating systems, it is not sent when using the Glean Python bindings.

## Scheduling
The desired behavior is to collect the ping at the first available opportunity after 04:00 local time on a new calendar day, but given constraints of the platform, it can only be submitted while the application is running. 
This breaks down into three scenarios:

1. the application was just installed;
2. the application was just upgraded (the version of the app is different from the last time the app was run);
3. the application was just started (after a crash or a long inactivity period);
4. the application was running at 04:00.

In the first case, since the application was just installed, if the due time for the current calendar day has passed, a `metrics` ping is immediately generated and scheduled for sending (reason code `overdue`). Otherwise, if the due time for the current calendar day has not passed, a ping collection is scheduled for that time (reason code `today`). 

In the second case, if a version change is detected at startup, the metrics ping is immediately submitted so that metrics from one version are not aggregated with metrics from another version (reason code `upgrade`).

In the third case, if the `metrics` ping was not already collected on the current calendar day, and it is before 04:00, a collection is scheduled for 04:00 on the current calendar day (reason code `today`).
If it is after 04:00, a new collection is scheduled immediately (reason code `overdue`).
Lastly, if a ping was already collected on the current calendar day, the next one is scheduled for collecting at 04:00 on the next calendar day (reason code `tomorrow`).

In the fourth and last case, the application is running during a scheduled ping collection time.
The next ping is scheduled for 04:00 the next calendar day (reason code `reschedule`).

More [scheduling examples](#scheduling-examples) are included below.

See also the [ping schedules and timing overview](ping-schedules-and-timings.html).

## Contents
The `metrics` ping contains all of the metrics defined in `metrics.yaml` (except events) that don't specify a ping or where `default` is specified in their [`send in pings`](https://mozilla.github.io/glean_parser/metrics-yaml.html#send-in-pings) property.

Additionally, error metrics in the `glean.error` category are included in the `metrics` ping.

The `metrics` ping shall also include the common [`ping_info`](index.md#the-ping_info-section) and ['client_info'](index.md#the-client_info-section) sections.

### Querying ping contents

Information about query ping contents is available in [Accessing Glean data](https://docs.telemetry.mozilla.org/concepts/glean/accessing_glean_data.html) in the Firefox data docs.

## Scheduling Examples

### Crossing due time with the application closed
1. The application is opened on Feb 7 on 15:00, closed on 15:05.

    * Glean records one metric A (say startup time in ms) during this measurement window MW1.

2. The application is opened again on Feb 8 on 17:00.

  * Glean notes that we passed local 04:00 since MW1.
  * Glean closes MW1, with:

      * `start_time=Feb7/15:00`;
      * `end_time=Feb8/17:00`.

  * Glean records metric A again, into MW2, which has a start_time of Feb8/17:00.

### Crossing due time and changing timezones
1. The application is opened on Feb 7 on 15:00 in timezone UTC, closed on 15:05.

    * Glean records one metric A (say startup time in ms) during this measurement window MW1.

2. The application is opened again on Feb 8 on 17:00 in timezone UTC+1.
    * Glean notes that we passed local 04:00 UTC+1 since MW1.
    * Glean closes MW1, with:

        * `start_time=Feb7/15:00/UTC`;
        * `end_time=Feb8/17:00/UTC+1`.

    * Glean records metric A again, into MW2.

### The application doesn’t run in a week
1. The application is opened on Feb 7 on 15:00 in timezone UTC, closed on 15:05.

    * Glean records one metric A (say startup time in ms) during this measurement window MW1.

2. The application is opened again on Feb 16 on 17:00 in timezone UTC.

    * Glean notes that we passed local 04:00 UTC since MW1.
    * Glean closes MW1, with:

        * `start_time=Feb7/15:00/UTC`;
        * `end_time=Feb16/17:00/UTC`.

    * Glean records metric A again, into MW2.

### The application doesn’t run for a week, and when it’s finally re-opened the timezone has changed
1. The application is opened on Feb 7 on 15:00 in timezone UTC, closed on 15:05.

    * Glean records one metric A (say startup time in ms) during this measurement window MW1.

2. The application is opened again on Feb 16 on 17:00 in timezone UTC+1.

    * Glean notes that we passed local 04:00 UTC+1 since MW1.
    * Glean closes MW1, with:

        * `start_time=Feb7/15:00/UTC`
        * `end_time=Feb16/17:00/UTC+1`.

    * Glean records metric A again, into MW2.

### The user changes timezone in an extreme enough fashion that they cross 04:00 twice on the same date
1. The application is opened on Feb 7 at 15:00 in timezone UTC+11, closed at 15:05.

    * Glean records one metric A (say startup time in ms) during this measurement window MW1.

2. The application is opened again on Feb 8 at 04:30 in timezone UTC+11.

    * Glean notes that we passed local 04:00 UTC+11.
    * Glean closes MW1, with:

        * `start_time=Feb7/15:00/UTC+11`;
        * `end_time=Feb8/04:30/UTC+11`.

    * Glean records metric A again, into MW2.

3. The user changes to timezone UTC-10 and opens the application at Feb 7 at 22:00 in timezone UTC-10

    * Glean records metric A again, into MW2 (not MW1, which was already sent).

4. The user opens the application at Feb 8 05:00 in timezone UTC-10

    * Glean notes that we have not yet passed local 04:00 on Feb 9
    * Measurement window MW2 remains the current measurement window

5. The user opens the application at Feb 9 07:00 in timezone UTC-10

    * Glean notes that we have passed local 04:00 on Feb 9
    * Glean closes MW2 with:

        * `start_time=Feb8/04:30/UTC+11`;
        * `end_time=Feb9/19:00/UTC-10`.

    * Glean records metric A again, into MW3.
