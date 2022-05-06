# Validating the collected data

Once instrumentation is added to a product, it is worth investing time early to understand if the data looks as expected and reasonable, while clients start sending it, and act if it does not.

The following checklist could help guide such validation effort.

1. Before releasing the product with the new data collection, make sure the data looks as expected by generating sample data on a local machine and inspecting it on the [Glean Debug View](../../reference/debug/debugViewTag.md):

    a. is the data showing up in the correct ping(s)?

    b. does the metric report the expected data?

    c. if exercising the same path again, it is expected for the data to be submitted again? And does it?

2. As users start adopting the version of the product with the new data collection, the initial data coming in should be checked, to understand how the measurements are behaving in the wild:

    a. does the shape of the collected data fit our prior expectations?

    b. is there any new [error](../../user/metrics/error-reporting.md) being reported for the new data points? If so, does this point to an edge case that should be documented and/or fixed in the code?

    c. in case of oddities in the data, how much of the product population is affected? Does this require changing the instrumentation or documenting?

3. After enough data is collected from the product population, are the expectations from the previous points still met?

{{#include ../../../shared/blockquote-info.html}}

### Does the product support multiple release channels?

> In case of multiple distinct product populations, the above checklist should be ideally run against all of them. For example, in case of _Firefox_, the checklist should be run for the Nightly population first, then on the other channels as the collection moves across the release trains.

