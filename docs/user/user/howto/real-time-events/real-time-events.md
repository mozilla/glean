# "Real-Time" Events

## Defining "Real-Time" Glean Events

"Real-Time" in the context of [Glean events](../../../reference/metrics/event.md) directly relates to configuring the Glean SDK to send each event in an ["events" ping](../../pings/events.md) as soon as it is recorded.
This minimizes the time between instrumentation and reporting, but doesn't describe how quickly received data is made available for querying.

## Configuring Glean For "Real-Time" Events

Glean "events" ping submission can either be configured at [initialization](../../../reference/general/initializing.md) or through [Server Knobs](../../../user/server-knobs/other/max-events.md).

Setting the maximum event threshold to a value of `1` will configure the Glean SDK to submit an "events" ping for every event recorded.

## Considerations

### What "Real-Time" Glean Events Are _Not_

Configuring the Glean SDK to submit events as soon as they are recorded does not mean to imply that the event data is available for analysis in
real time. There are networks to traverse, ingestion pipelines, etl, etc. that are all factors to keep in mind when considering how soon the data
is available for analysis purposes. This documentation only purports to cover configuring the Glean SDK to send the data in a real-time fashion and
does not make any assumptions about the analysis of data in real-time.

### More Network Requests

For every event recorded, a network request will be generated when the event is submitted for ingestion. By default, the Glean SDK batches up to
500 events per "events" ping, so this has the potential to generate up to 500 times as many network requests than the current default.

### More Ingestion Endpoint Traffic

As a result of the increased network requests, the ingestion endpoint will need to handle this additional traffic. This increases the load
of all the processing steps that are involved with ingesting event data from an application.

### Storage Space

Typically the raw dataset for Glean events contains 1-500 events in a single row of the database. This row also includes metadata such as
information about the client application and the ping itself. With only a single event per "events" ping, the replication of this metadata
across the database will use additional space to house this repeated information that should rarely if ever change between events
