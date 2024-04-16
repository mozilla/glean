# Data Control Plane (a.k.a. Server Knobs)

Glean provides a Data [Control Plane] through which pings can be enabled or disabled through a [Nimbus] rollout or experiment.

Products can use this capability to control "data-traffic", similar to how a network control plane controls "network-traffic".

This provides the ability to do the following:

- Allow runtime changes to data collection without needing to land code and ride release trains.
- Eliminate the need for manual creation and maintenance of feature flags specific to data collection.
- Sampling of measurements from a subset of the population so that we do not collect or ingest more data than is necessary from high traffic areas of an application instrumented with Glean metrics.
- Operational safety through being able to react to high-volume or unwanted data.
- Visibility into sampling and sampling rates for remotely configured metrics.

For information on controlling metrics with Server Knobs, see the metrics documentation for [Server Knobs - Metrics].

## Contents
- [Product Integration](./product-integration.md)
- [Experimenter Configuration](./experimenter-configuration.md)

[Control Plane]: https://en.wikipedia.org/wiki/Control_plane
[Nimbus]: https://experimenter.info
[Server Knobs - Metrics]: ../../metrics/data-control-plane/index.md