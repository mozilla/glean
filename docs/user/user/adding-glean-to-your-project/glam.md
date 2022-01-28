[GLAM](https://glam.telemetry.mozilla.org/)

GLAM is an interactive dashboard that is Mozilla’s primary self-service tool for examining the distributions of values of specific individual telemetry metrics, over time and across different user populations.
To evaluate the impact of a specific change, engineers can instrument their code using telemetry to capture information back to Mozilla’s data platform. These data points are then collected and aggregated for data visualization.

[Glean](https://docs.telemetry.mozilla.org/concepts/glean/glean.html)

Glean is a product analytics & telemetry solution that provides a consistent experience and behavior across all of our products.


## What does GLAM expect from GLEAN?

### To support GLAM Glean products should ideally

1. Associate dates with build-id

Glam currently relies on the fact that the build id has encoded some sort of date that we can use to build plots over time. 
As an example, the [glam explore chart](https://glam.telemetry.mozilla.org/firefox/probe/http_response_version/explore?activeBuckets=%5B%2220%22%2C%2211%22%2C%2230%22%2C%2210%22%2C%220%22%2C%221%22%2C%222%22%2C%223%22%2C%224%22%2C%225%22%5D&process=parent) helps us to evaluate the impact of a specific change to the probe over builds (mapped to dates): http_response_version
(HTTP: Protocol Version Used on Response from nsHTTP.h)

2. map version release markers to dates

Similar to the above, in the [explore chart](https://glam.telemetry.mozilla.org/firefox/probe/http_response_version/explore?activeBuckets=%5B%2220%22%2C%2211%22%2C%2230%22%2C%2210%22%2C%220%22%2C%221%22%2C%222%22%2C%223%22%2C%224%22%2C%225%22%5D&process=parent&timeHorizon=QUARTER) GLAM aggregates the data at major version level and relies on the fact that major release versions are mapped to dates. 

Previously, we did not have a source of truth for glean apps such as iOS, Kotlin, and others like buildhub or geckoview version, so it was tricky to map build id to a date format suitable for GLAM. We currently use only the major version for calculating the aggregated values. 

The most straightforward path was to add the build date as an optional measurement to the Glean "client_info" that is sent in all pings. Here is the [proposal](https://docs.google.com/document/d/1_7kTePQHHRhsAqOYPiw8ptoN9ytRnsWMcN-tddnV0Cg/edit) detailing the next steps and the [bugzilla ticket](https://bugzilla.mozilla.org/show_bug.cgi?id=1742448). The implementation is currently available for Fenix Nightly, Glean Kotlin and Glean iOS. 

Coming to the major version mapping, apart from Rally all Glean apps support major version releases. 

## What if either of the mapping does not exist:

We still have Glean SDKs that don't have a build date yet and might take longer to implement. We need a decision on whether we can get them on GLAM without it or having a build date is a requirement right away. 


