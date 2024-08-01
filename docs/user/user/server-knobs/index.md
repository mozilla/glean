# Server Knobs: Glean Data Control Plane

Glean provides Server Knobs, a Data [Control Plane] through which Glean runtime settings can be changed remotely including the ability to enable, disable or throttle metrics and pings through a [Nimbus] rollout or experiment.

Products can use this capability to control "data traffic", similar to how a network control plane controls "network traffic".

Server Knobs provides the ability to do the following:

- Allow runtime changes to data collection without needing to land code and ride release trains.
- Eliminate the need for manual creation and maintenance of feature flags specific to data collection.
- Sampling of measurements from a subset of the population so that we do not collect or ingest more data than is necessary from high traffic areas of an application instrumented with Glean metrics.
- Operational safety through being able to react to high-volume or unwanted data.
- Visibility into sampling and sampling rates for remotely configured metrics.

## Contents

- [Controlling Metrics with Server Knobs]
- [Controlling Pings with Server Knobs]
- [Other Server Knobs]

[Control Plane]: https://en.wikipedia.org/wiki/Control_plane
[Nimbus]: https://experimenter.info
[Controlling Metrics with Server Knobs]: ./metrics/index.md
[Controlling Pings with Server Knobs]: ./pings/index.md
[Other Server Knobs]: ./other/index.md