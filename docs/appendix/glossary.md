# Glossary

A glossary with explanations and background for wording used in the Glean project.

## Glean

According to the [dictionary](https://www.merriam-webster.com/dictionary/glean) the word “glean” means:

> to gather information or material bit by bit

Glean is the name of the whole product, see [Glean - product analytics & telemetry](https://docs.telemetry.mozilla.org/concepts/glean/glean.html).

## Glean SDK

The Glean SDK is the bundle of libraries with support for different platforms.
The source code is available at <https://github.com/mozilla/glean>.

## Glean SDK book

This documentation.

## Metric

Metrics are the individual things being measured using Glean.
They are defined in [metrics.yaml](https://mozilla.github.io/glean_parser/metrics-yaml.html) files.

Glean itself provides some metrics out of the box.

## Ping

A ping is an entity used to bundle related metrics.
The Glean SDK provides default pings and allows for custom ping, see [Glean Pings](../user/pings/index.md).

## Pipeline

The data pipeline is the infrastructure that collects, stores, and analyzes Telemetry data from our products and logs from various services.
See [An overview of Mozilla’s Data Pipeline](https://docs.telemetry.mozilla.org/concepts/pipeline/gcp_data_pipeline.html).

## Submission

"To submit" means to collect & to upload a ping.

The Glean SDK stores locally all the metrics set by it or by its clients.
Each ping has its own schedule to gather all its locally saved metrics and create a JSON payload with them. This is called "collection".

Upon successful collection, the payload is queued for upload, which may not happen immediately or at all (in case network connectivity is not available).

Unless the user has defined their own custom pings, they don’t need to worry too much about submitting pings.
All the default pings have their scheduling and submission handled by the SDK.

## This Week in Glean (TWiG)

[This Week in Glean](twig.md) is a series of blog posts that the Glean Team at Mozilla is using to try to communicate better about our work.
