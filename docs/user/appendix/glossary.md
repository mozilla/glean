# Glossary

A glossary with explanations and background for wording used in the Glean project.

## Glean

According to the [dictionary](https://www.merriam-webster.com/dictionary/glean) the word “glean” means:

> to gather information or material bit by bit

Glean is the combination of the Glean SDK, the Glean pipeline & Glean tools.

See also: [Glean - product analytics & telemetry](https://docs.telemetry.mozilla.org/concepts/glean/glean.html).

## Glean Pipeline

The general data pipeline is the infrastructure that collects, stores, and analyzes telemetry data from our products and logs from various services.
See [An overview of Mozilla’s Data Pipeline](https://docs.telemetry.mozilla.org/concepts/pipeline/gcp_data_pipeline.html).

The Glean pipeline additionally consists of

* the [Probe Info Service](https://github.com/mozilla/probe-scraper#glean-metrics-data-files),
* the [schema generator](https://github.com/mozilla/mozilla-schema-generator/),
* the [JSON Schema transpiler](https://github.com/mozilla/jsonschema-transpiler),
* the [ping schemas](https://github.com/mozilla-services/mozilla-pipeline-schemas).

## Glean SDK

The Glean SDK is the bundle of libraries with support for different platforms.
The source code is available at <https://github.com/mozilla/glean>.

## Glean SDK book

This documentation.

## Glean tools

Glean provides additional tools for its usage:

* [Glean parser](https://mozilla.github.io/glean_parser/) (Source code: <https://github.com/mozilla/glean_parser/>)


## Metric

Metrics are the individual things being measured using Glean.
They are defined in [metrics.yaml](https://mozilla.github.io/glean_parser/metrics-yaml.html) files, also known as _registry files_.

Glean itself provides [some metrics out of the box](../user/collected-metrics/metrics.md).

## Ping

A ping is a bundle of related metrics, gathered in a payload to be transmitted.
The Glean SDK provides default pings and allows for custom ping, see [Glean Pings](../user/pings/index.md).

## Submission

"To submit" means to collect & to enqueue a ping for uploading.

The Glean SDK stores locally all the metrics set by it or by its clients.
Each ping has its own schedule to gather all its locally saved metrics and create a JSON payload with them. This is called "collection".

Upon successful collection, the payload is queued for upload, which may not happen immediately or at all (in case network connectivity is not available).

Unless the user has defined their own custom pings, they don’t need to worry too much about submitting pings.

All the default pings have their scheduling and submission handled by the SDK.

## Measurement window

The measurement window of a ping is the time frame in which metrics are being actively gathered for it.

The measurement window start time is the moment the previous ping is submitted. In the absence of a previous ping, this time will be the time the application process started.

The measurement window end time is the moment the current ping gets submitted. Any new metric recorded after submission will be part of the next ping, so this pings measurement window is over.


## This Week in Glean (TWiG)

[This Week in Glean](twig.md) is a series of blog posts that the Glean Team at Mozilla is using to try to communicate better about our work.
