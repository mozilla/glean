# Adding Glean to your Server Application

Glean enables the collection of behavioral metrics through events in server environments. This method does not rely on the Glean SDK but utilizes the Glean parser to generate native code for logging events in a standard format compatible with the ingestion pipeline.

## Differences from using the Glean SDK

This implementation of telemetry collection in server environments has some differences compared to using Glean SDK in client applications and Glean.js in the frontend of web applications. Primarily, in server environments the focus is exclusively on event-based metrics, diverging from the broader range of metric types supported by Glean. Additionally, there is no need to incorporate Glean SDK as a dependency in server applications. Instead, the Glean parser is used to generate native code for logging events.

## When to use server-side collection

This method is intended for collecting user-level behavioral events in server environments. It is not suitable for collecting system-level metrics or performance data, which should be collected using cloud monitoring tools.

## How to add Glean server side collection to your service

1. Integrate [`glean_parser`](https://github.com/mozilla/glean_parser#usage) into your build system. Follow instructions for other SDK-enabled platforms, e.g. [JavaScript](./javascript.md). Use a server outputter to generate the code that records your pings (see [Sending pings to the pipeline](#sending-pings-to-the-pipeline)). `glean_parser` currently supports [Go](https://github.com/mozilla/glean_parser/blob/main/glean_parser/go_server.py), [JavaScript/Typescript](https://github.com/mozilla/glean_parser/blob/main/glean_parser/javascript_server.py), [Python](https://github.com/mozilla/glean_parser/blob/main/glean_parser/python_server.py), and [Ruby](https://github.com/mozilla/glean_parser/blob/main/glean_parser/ruby_server.py).
2. Define your metrics in `metrics.yaml`
3. Request a [data review](https://wiki.mozilla.org/Firefox/Data_Collection) for the collected data
4. [Add your product to probe-scraper](./enable-data-ingestion.html#add-your-product-to-probe-scraper)
5. If you publish pings to Pub/Sub, include the service accounts your service runs as in stage and production in the bug from the previous step.

## Sending pings to the pipeline

Server outputters can write pings as JSON log lines to standard output (the default) or build Pub/Sub messages that your service publishes to the ingestion topic itself.

We recommend Pub/Sub for new services that send a lot of telemetry or need reliable delivery. Logging depends on the cluster's logging agent and gives your service no signal when a ping is lost. With Pub/Sub, your service learns whether each ping was accepted and can retry, and once a ping is accepted the pipeline keeps it until it is processed. Only the Go outputter supports Pub/Sub today. If your service uses another language, mention it in the bug from step 4 and the Data Engineering team will help you choose a transport and, if needed, add Pub/Sub support.

Logging needs no extra setup: on Mozilla's shared clusters a log sink forwards the log lines to the ingestion pipeline.

The Go outputter selects the transport with the `transport` option (`glean_parser` 20.0.0 or later):

```sh
glean_parser translate metrics.yaml -f go_server -s transport=pubsub -o glean/
```

- `logging` (default): log lines only.
- `pubsub`: Pub/Sub message builder only.
- `combined`: both, to move a service to Pub/Sub gradually or fall back to logging when a publish fails.

The generated code only builds messages. Your service owns the Pub/Sub client, retries and shutdown.

## How to add a new event to your server side collection

Follow the standard Glean SDK guide for adding metrics to `metrics.yaml` file.

## Requesting deletion of collected data

Server applications can ask the data pipeline to delete a user's data by sending the
`server-deletion-request` ping. This is the server-side counterpart to the SDK's
[`deletion-request` ping](../pings/deletion-request.md), but it works differently:
server applications have no Glean-managed `client_id`, so the ping carries whichever
identifier metrics your application already sends with its data.

The ping is defined in the `glean-server` library, so you do not need to declare it in a
`pings.yaml`. To use it, add `server-deletion-request` to the `send_in_pings` list of the
metric that identifies the user. `glean_parser` generates a logger for the ping alongside
the ones for your other pings, which you record when the user requests deletion.

That metric also needs to be sent in the pings that carry your telemetry, such as
`events`. The deletion request says whose data to remove, and the pipeline finds it by
matching the identifier in those pings.

{{#include ../../../shared/blockquote-warning.html}}

##### Sending the ping does not by itself delete anything

> The ping records which identifier should be deleted, but the pipeline also needs to know
> which tables and columns that identifier appears in. That mapping is configured
> separately, so coordinate with the Data Engineering team when you adopt this ping.
> Until it is configured, the ping is collected but no data is removed.

### Availability

The `server-deletion-request` ping is currently supported by the JavaScript and
TypeScript server outputters only.

## Technical details - ingestion

For more technical details on how ingestion works, see the [Confluence page](https://mozilla-hub.atlassian.net/wiki/spaces/IP/pages/741998604/Backend+telemetry+collection+with+Glean). The Pub/Sub message format is described in the [decoder specification](https://mozilla.github.io/gcp-ingestion/architecture/decoder_service_specification/#ingestion-sources).
