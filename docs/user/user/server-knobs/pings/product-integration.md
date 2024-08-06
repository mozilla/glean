# Product Integration

Glean provides a general Nimbus feature named `glean` that can be used for configuration of pings. Simply select the `glean` feature along with your Nimbus feature from the Experimenter UI when configuring the experiment or rollout (see [Nimbus] documentation for more information on multi-feature experiments).

The `glean` Nimbus feature requires the `gleanMetricConfiguration` variable to be used to provide the required metric configuration. The format of the configuration is defined in the [Experimenter Configuration] section.
If a ping is not included, it will default to the value found in the pings.yaml.
Note that this can also serve as an override for Glean builtin pings disabled using the Configuration property `enable_internal_pings=false` during initialization.  

[Experimenter Configuration]: ./experimenter-configuration.md
[Nimbus]: https://experimenter.info
[Nimbus Desktop Feature API]: https://experimenter.info/desktop-feature-api
