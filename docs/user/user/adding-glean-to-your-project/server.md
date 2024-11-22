# Adding Glean to your Server Application

Glean enables the collection of behavioral metrics through events in server environments. This method does not rely on the Glean SDK but utilizes the Glean parser to generate native code for logging events in a standard format compatible with the ingestion pipeline.

## Differences from using the Glean SDK

This implementation of telemetry collection in server environments has some differences compared to using Glean SDK in client applications and Glean.js in the frontend of web applications. Primarily, in server environments the focus is exclusively on event-based metrics, diverging from the broader range of metric types supported by Glean. Additionally, there is no need to incorporate Glean SDK as a dependency in server applications. Instead, the Glean parser is used to generate native code for logging events.

## When to use server-side collection

This method is intended for collecting user-level behavioral events in server environments. It is not suitable for collecting system-level metrics or performance data, which should be collected using cloud monitoring tools.

## How to add Glean server side collection to your service

1. Integrate [`glean_parser`](https://github.com/mozilla/glean_parser#usage) into your build system. Follow instructions for other SDK-enabled platforms, e.g. [JavaScript](./javascript.md). Use a server outputter to generate logging code. `glean_parser` currently supports [Go](https://github.com/mozilla/glean_parser/blob/main/glean_parser/go_server.py), [JavaScript/Typescript](https://github.com/mozilla/glean_parser/blob/main/glean_parser/javascript_server.py), [Python](https://github.com/mozilla/glean_parser/blob/main/glean_parser/python_server.py), and [Ruby](https://github.com/mozilla/glean_parser/blob/main/glean_parser/ruby_server.py).
2. Define your metrics in `metrics.yaml`
3. Request a [data review](https://wiki.mozilla.org/Firefox/Data_Collection) for the collected data
4. [Add your product to probe-scraper](./enable-data-ingestion.html#add-your-product-to-probe-scraper)

## How to add a new event to your server side collection

Follow the standard Glean SDK guide for adding metrics to `metrics.yaml` file.

## Technical details - ingestion

For more technical details on how ingestion works, see the [Confluence page](https://mozilla-hub.atlassian.net/wiki/spaces/IP/pages/1139181778/Backend+telemetry+collection+with+Glean).
