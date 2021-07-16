# FIXME(title): Avoiding the Environment

The Glean SDK collects a limited amount of metrics that are generally useful across products.
These metrics are sent in the [`client_info`][client_info] section of every ping.
The data is provided by the embedding application or automatically fetched by the Glean SDK.
It is collected at initialization time and sent in every ping afterwards.
For historical reasons it contains metrics that are only useful on a certain platform.

[client_info]: ../pings/index.md#the-client_info-section

## Adding new metrics to `client_info`

Adding new metrics maintained by the Glean SDK team will require a full proposal
and details on why that value is useful across multiple platforms and products and needs SDK team ownership.

The Glean SDK is not taking ownership of new metrics that are platform- or product-specific.

## Adding new application-specific metrics

New metrics can be added to products or libraries at any time.
See [Adding new metrics](adding-new-metrics.md).
If the recorded data should not be cleared once set the `application`-lifetime might be suitable.
See [When should the Glean SDK automatically clear the measurement?](adding-new-metrics.md#when-should-the-glean-sdk-automatically-clear-the-measurement)
for details.

Note that due to scheduling of Glean-owned pings at early startup the data might still be missing from these pings.

## Adding metrics to every ping

For the majority of metrics it will probably not be necessary to be in every ping that is sent from the application,
regardless of who controls the ping.
Instead list out the exact pings the data should be in in the `send_in_pings` attribute for each metric.

For the small number of metrics that should be in every ping the Glean SDK will provide a solution.
See [bug FILL-IN](https://bugzilla.mozilla.org/show_bug.cgi?id=FILL-IN) for details.

If you identified a bundle of related metrics that you consider useful for a wider variety of products,
it might be a good idea to ship these as a library the products can consume.
Please contact the Glean SDK team for any questions or help with this approach.
