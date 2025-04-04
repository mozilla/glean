# Adding Glean to your project

This page describes the steps for adding Glean to a project. This does not include the
steps for adding a new metrics or pings to an existing Glean integration. If that is what
your are looking for, refer to the [Adding new metrics](../metrics/adding-new-metrics.md) or the [Adding new pings](../pings/custom.md) guide.

## Glean integration checklist

The Glean integration checklist can help to ensure your Glean SDK-using product is meeting all of the recommended guidelines.

Products (applications or libraries) using a Glean SDK to collect telemetry **must**:

1. [Integrate the Glean SDK into the build system](#looking-for-an-integration-guide). Since the Glean SDK does some code generation for your metrics at build time, this requires a few more steps than just adding a library.

2. Go through [data review process](https://wiki.mozilla.org/Firefox/Data_Collection) for all newly collected data.

3. Ensure that telemetry coming from automated testing or continuous integration is either not sent to the telemetry server or [tagged with the `automation` tag using the `sourceTag` feature](../../reference/debug/sourceTags.md).

4. At least one week before releasing your product, [enable your product's application id and metrics](./enable-data-ingestion.md) to be ingested by the data platform (and, as a consequence, indexed by the [Glean Dictionary]).

> **Important consideration for libraries:** For libraries that are adding Glean, you will need to indicate which _applications_ use the library as a dependency so that the library metrics get correctly indexed and added to the products that consume the library. If the library is added to a new product later, then it is necessary to file a new [bug][dataeng-bug] to add it as a dependency to that product in order for the library metrics to be collected along with the data from the new product.

Additionally, applications (but not libraries) **must**:

5. Request a [data review](https://wiki.mozilla.org/Firefox/Data_Collection) to add Glean to your application (since it _can_ send data out of the box).

6. [Initialize Glean](../../reference/general/initializing.md) as early as possible at application startup.

7. Provide a way for users to turn data collection off (e.g. providing settings to control `Glean.setCollectionEnabled()`). The exact method used is application-specific.

{{#include ../../../shared/blockquote-info.html}}

##### Looking for an integration guide?

> Step-by-step tutorials for each supported language/platform,
> can be found on the specific integration guides:
>
> - [JavaScript](./javascript.md)
> - [Kotlin](./kotlin.md)
> - [Python](./python.md)
> - [Rust](./rust.md)
> - [Swift](./swift.md)
> - [Qt/QML](./qt.md)
> - [Server](./server.md)

[Glean Dictionary]: https://dictionary.telemetry.mozilla.org
