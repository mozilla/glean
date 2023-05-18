# Example Scenarios

## Scenario 1

> *Landing a metric that is disabled by default and then enabling it for some segment of the population*

This scenario can be expected in cases such as when instrumenting high-traffic areas of the browser. These are instrumentations that would normally generate a lot of data because they are recorded frequently for every user.

In this case, the telemetry which has the potential to be high-volume would land with the “disabled” property of the metric set to “true”. This will ensure that it does not record data by default.

An example metric definition with this property set would look something like this:

```yaml
urlbar:
  impression:
    disabled: true
    type: event
    description: Recorded when urlbar results are shown to the user.
  ...
```

Once the instrumentation is landed, it can now be enabled for a subset of the population through a Nimbus rollout or experiment without further code changes.

Through [Nimbus], we have the ability to sample the population by setting the audience size to a certain percentage of the eligible population. Nimbus also provides the ability to target clients based on the available targeting parameters for a particular application (for instance, Firefox Desktop’s available targeting parameters).

This can be used to slowly roll out instrumentations to the population in order to validate the data we are collecting before measuring the entire population and potentially avoiding costs and  overhead by collecting data that isn’t useful.

## Scenario 2

> *Landing a metric that is enabled by default and then disabling it for a segment of the population*

This is effectively the inverse of [Scenario 1](#scenario-1), instead of landing the metrics disabled by default, they are landed as enabled so that they are normally collecting data from the entire population.

Similar to the first scenario, a [Nimbus] rollout or experiment can then be launched to configure the metrics as disabled for a subset of the population.

This provides a mechanism by which we can disable the sending of telemetry from an audience that we do not wish to collect telemetry data from.

For instance, this could be useful in tuning out telemetry data coming from automation sources or bad actors. In addition, it provides a way to disable broken, incorrect, or unexpectedly noisy instrumentations as an operational safety mechanism to directly control the volume of the data we collect and ingest.

[Nimbus]: https://experimenter.info
