# Integrating Glean for product managers

This chapter provides guidance for planning the work involved in integrating Glean into your product, for internal Mozilla customers.
For a technical coding perspective, see [adding Glean to your project](adding-glean-to-your-project/index.html).

Glean is the standard telemetry platform required for all new Mozilla products.
While there are some upfront costs to integrating Glean in your product, this pays off in easier long-term maintenance and a rich set of self-serve analysis tools.
The Glean team is happy to support your telemetry integration and make it successful.
Find us in [#glean](https://chat.mozilla.org/#/room/#glean:mozilla.org) or email [glean-team@mozilla.com](mailto:glean-team@mozilla.com).

## Building a telemetry plan

The Glean SDKs provide support for answering basic product questions out-of-the-box, such as daily active users, product version and platform information.  
However, it is also a good idea to have a sense of any additional product-specific questions you are trying to answer with telemetry, and, when possible, in collaboration with a data scientist.  
This of course helps for your own planning, but is also invaluable for the Glean team to support you, since we will understand the ultimate goals of your product's telemetry and ensure the design will meet those goals and we can identify any new features that may be required.
It is best to frame this document in the form of questions and use cases rather than as specific data points and schemas.

## Integrating Glean into your product

The [technical steps for integrating Glean in your product are documented in its own chapter](adding-glean-to-your-project/index.html) for supported platforms.
We recommend having a member of the Glean team review this integration to catch any potential pitfalls.

## (Optional) Adapting Glean to your platform

The Glean SDKs are a collection of cross platform libraries and tools that facilitate collection of Glean conforming telemetry from applications.  
Consult the list of the currently supported [platforms and languages](../index.html).
If your product's tech stack isn't currently supported, please reach out to the Glean team: significant work will be required to create a new integration.  
In previous efforts, this has ranged from 1 to 3 months FTE of work, so it is important to plan for this work well in advance.
While the first phase of this work generally requires the specialized expertise of the Glean team, the second half can benefit from outside developers to move faster.

## (Optional) Designing ping submission

The Glean SDKs periodically send telemetry to our servers in a bundle known as a "[ping](../appendix/glossary.html#ping)".  
For mobile applications with common interaction models, such as web browsers, the Glean SDKs provide [basic pings out-of-the-box](pings/index.html).
For other kinds of products, it may be necessary to carefully design what triggers the submission of a ping.
It is important to have a solid telemetry plan (see above) so we can make sure the ping submission will be able to answer the telemetry questions required of the product.

## (Optional) New metric types

The Glean SDKs have a number of [different metric types](https://mozilla.github.io/glean/book/user/metrics/index.html) that it can collect.  
Metric types provide "guardrails" to make sure that telemetry is being collected correctly, and to present the data at analysis time more automatically.
Occasionally, products need to collect data that doesn't fit neatly into one of the available metric types.
Glean [has a process](https://wiki.mozilla.org/Glean/Adding_or_changing_Glean_metric_types) to request and introduce more metric types and we will work with you to design something appropriate.
This design and implementation work is at least 4 weeks, though we are working on the foundation to accelerate that.
Having a telemetry plan (see above) will help to identify this work early.

## Integrating Glean into GLAM

To use GLAM for analysis of your application's data [file a ticket in the GLAM repository](https://github.com/mozilla/glam/issues/new?assignees=&labels=&template=add_to_glam.md&title=).

A data engineer from the GLAM team will reach out to you if further information is required.
