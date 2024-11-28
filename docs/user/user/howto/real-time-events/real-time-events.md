# "Real-Time" Events

## Defining "real-time" events within the Glean SDK

For the purposes of the Glean SDK and its capabilities, "real-time" is limited to: minimizing the time between instrumentation and reporting.
It does not imply or describe how quickly received data is made available for querying.

## Methods to achieve this with Glean

### Option 1: Configuring Glean to send all events as soon as they are recorded

Glean ["events" ping](../../pings/events.md) submission can be configured either during [initialization](../../../reference/general/initializing.md) or through [Server Knobs](../../../user/server-knobs/other/max-events.md).

Setting the maximum event threshold to a value of `1` will configure the Glean SDK to submit an "events" ping for each and every [event](../../../reference/metrics/event.md) as they
are recorded. By default, the Glean SDK will batch 500 events per "events" ping.

#### As of November 2024, Desktop Release:

##### Median user per day:

- 67 events / 3 pings
- The impact of turning on one event per ping based on the median user would result in an increase of approximately 21 times more event ping volume.

##### 85th percentile user per day:

- 305 events / 11 pings
- The impact of turning on one event per ping based on the 85th percentile user would result in an increase of approximately 26 times more event ping volume.

##### 95th percentile user per day:

- 706 events / 19 pings
- The impact of turning on one event per ping based on the 95th percentile user would result in an increase of approximately 36 times more event ping volume.

The current release population of Desktop as a whole sends us over 10 billion events per day in over 340 million event pings. Sending each of those events as a ping would increase the ping volume by 32 times the current rate.

Based on this it is safe to assume that sending 1 event per event ping would increase the ingestion traffic and downstream overhead between 20-40x the current levels with Glean batching of events in the client. This is a significant increase that should be taken into consideration before configuring Glean to disable event batching.


### Option 2: Using a custom ping and submitting it immediately ("Pings-as-Events")

If it isn't necessary to receive all Glean SDK events that are instrumented in an application in "real-time", it may be preferable to create a
[custom ping](../../pings/custom.md) which contains the relevant information to capture the context around the event and submit it as soon as
the application event occurs.

This has some additional advantages over using just an event in that custom pings are less restrictive than the extras attached to the event
in what data and Glean SDK metric types can be used.

If it is important to see the event that is being represented as a custom ping in context with other application events, then you only need to
define an event metric and use the `send_in_pings` parameter to send it in both the custom ping and the Glean built-in "events" ping. It can
then be seen in sequence and within context of all of the application events, and still be sent in "real-time" as needed.

## Considerations

### What "real-time" Glean events/pings are _not_

Configuring the Glean SDK to submit events as soon as they are recorded or using custom pings to submit data immediately does not mean that the
data is available for analysis in real time. There are networks to traverse, ingestion pipelines, etl, etc. that are all factors to keep in
mind when considering how soon the data is available for analysis purposes. This documentation only purports to cover configuring the Glean SDK
to send the data in a real-time fashion and does not make any assumptions about the analysis of data in real-time.

### More network requests

For every event recorded or custom ping submitted, a network request will be generated as the ping is submitted for ingestion. By default, the
Glean SDK batches up to 500 events per "events" ping, so this has the potential to generate up to 500 times as many network requests than the
current defaults for the Glean SDK "events" ping.

### More ingestion endpoint traffic

As a result of the increased network requests, the ingestion endpoint will need to handle this additional traffic. This increases the load
of all the processing steps that are involved with ingesting event data from an application.

### Storage space requirements

Typically the raw dataset for Glean events contains 1-500 events in a single row of the database. This row also includes metadata such as
information about the client application and the ping itself. With only a single event per "events" ping, the replication of this metadata
across the database will use additional space to house this repeated information that should rarely if ever change between events
