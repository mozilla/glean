# Data Control Plane (a.k.a. Server Knobs)

Glean provides a Data [Control Plane] through which metrics can be enabled, disabled or throttled through a [Nimbus] rollout or experiment.

Products can use this capability to control "data-traffic", similar to how a network control plane controls "network-traffic".

This provides the ability to do the following:

- Allow runtime changes to data collection without needing to land code and ride release trains.
- Eliminate the need for manual creation and maintenance of feature flags specific to data collection.
- Sampling of measurements from a subset of the population so that we do not collect or ingest more data than is necessary from high traffic areas of an application instrumented with Glean metrics.
- Operational safety through being able to react to high-volume or unwanted data.
- Visibility into sampling and sampling rates for remotely configured metrics.

## Contents
- [Example Scenarios](./example-scenarios.md)
- [Product Integration](./product-integration.md)
- [Experimenter Configuration](./experimenter-configuration.md)
- [Advanced Topics](./advanced-topics.md)
- [Frequently Asked Questions](./faq.md)

[Control Plane]: https://en.wikipedia.org/wiki/Control_plane
[Nimbus]: https://experimenter.info