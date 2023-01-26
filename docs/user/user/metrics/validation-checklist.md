# Validating the collected data

It is worth investing time when instrumentation is added to the product to understand if the data looks reasonable and expected, and to take action if it does not.
It is important to highlight that an automated rigorous test suite for [testing metrics](testing-metrics.md) is an important precondition for building confidence in newly collected data (especially business-critical ones).

The following checklist could help guide this validation effort.

1. Before releasing the product with the new data collection, make sure the data looks as expected by generating sample data on a local machine and inspecting it on the Glean Debug View[(see the debugging facilities)](../../reference/debug/index.md):

    a. Is the data showing up in the correct ping(s)?

    b. Does the metric report the expected data?

    c. If exercising the same path again, is it expected for the data to be submitted again? And does it?

2. As users start adopting the version of the product with the new data collection (usually within a few days of release), the initial data coming in should be checked, to understand how the measurements are behaving in the wild:

    a. Does this organically-sent data satisfy the same quality expectations the manually-sent data did in Step 1?

    b. Is the metric showing up correctly in the [Glean Dictionary](https://dictionary.telemetry.mozilla.org/)?

    c. Is there any new [error](../../user/metrics/error-reporting.md) being reported for the new data points? If so, does this point to an edge case that should be documented and/or fixed in the code?

    d. As the first three or four days pass, distributions will converge towards their final shapes. Consider extreme values; are there a very high number of zero/minimum values when there shouldn't be, or values near what you would realistically expect to be the maximum (e.g. a timespan for a single day that is reporting close to 86,400 seconds)? In case of oddities in the data, how much of the product population is affected? Does this require changing the instrumentation or documenting?

{{#include ../../../shared/blockquote-info.html}}

### How to annotate metrics without changing the source code?

> Data practitioners that lack familiarity with YAML or product-specific development workflows can still document any discovered edge-cases and anomalies by identifying the metric in the [Glean Dictionary](https://dictionary.telemetry.mozilla.org/) and initiate adding commentary from the metric page.

3. After enough data is collected from the product population, are the expectations from the previous points still met?

{{#include ../../../shared/blockquote-info.html}}

### Does the product support multiple release channels?

> In case of multiple distinct product populations, the above checklist should be ideally run against all of them.
> For example, in case of _Firefox_, the checklist should be run for the Nightly population first, then on the other channels as the collection moves across the release trains.
