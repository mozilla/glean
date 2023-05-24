# Product Integration

In order to enable sharing of this functionality between multiple Nimbus Features, the implementation is not defined as part of the stand-alone Glean feature defined in the Nimbus Feature Manifest, but instead is intended to be added as a feature variable to other Nimbus Feature definitions for them to make use of.

## Desktop Feature Integration

In order to make use of the remote metric configuration in a Firefox Desktop component, the component must first be defined as a Nimbus Feature. The instructions for defining your component as a feature (or multiple features) can be found in the Nimbus documentation specific to the [Nimbus Desktop Feature API]. Once you have defined a feature, you simply need to add a Feature Variable to represent the piece of the Glean configuration that will be provided by this Nimbus Feature. For example:

```yaml
variables:
  ... // Definitions of other feature variables
  gleanMetricConfiguration:
    type: json
    description: >-
    "Glean metric configuration"
```

This definition allows for configuration to be set in a Nimbus rollout or experiment and fetched by the client to be applied based on the enrollment. Once the Feature Variable has been defined, the final step is to fetch the configuration from Nimbus and supply it to the Glean API. This can be done during initialization and again any time afterwards, such as in response to receiving an updated configuration from Nimbus. Only the latest configuration provided will be applied and any previously configured metrics that are omitted from the new configuration will not be changed. An example call to set a configuration from the “urlbar” Nimbus Feature could look like this:

```JavaScript
let cfg = lazy.NimbusFeatures.urlbar.getVariable(
  "gleanMetricConfiguration"
);
Services.fog.setMetricsFeatureConfig(JSON.stringify(cfg));
```

It is also recommended to register to listen for updates for the Nimbus Feature and apply new configurations as soon as possible. The following example illustrates how the “urlbar” Nimbus Feature might register and update the metric configuration:

```JavaScript
lazy.NimbusFeatures.urlbar.onUpdate(() => {
  let cfg = lazy.NimbusFeatures.urlbar.getVariable(
    "gleanMetricConfiguration"
  );
  Services.fog.setMetricsFeatureConfig(JSON.stringify(cfg));
});
```

## Mobile Feature Integration

In order to make use of the remote metric configuration in a Firefox Mobile application, you must first define a Nimbus Feature or add a variable to an existing Nimbus Feature. The instructions for defining your component as a feature (or multiple features) can be found in the Nimbus documentation specific to the Nimbus Mobile Feature API. Once you have defined a feature, you simply need to add a Feature Variable to represent the piece of the Glean configuration that will be provided by this Nimbus Feature. For example:

```yaml
features:
  homescreen:
    description: |
      The homescreen that the user goes to when they press home or new    
      tab.
    variables:
      ... // Other homescreen variables
      gleanMetricConfiguration:
        description: Glean metric configuration
        type: String
        default: "{}"
```

Once the Feature Variable has been defined, the final step is to fetch the configuration from Nimbus and supply it to the Glean API. This can be done during initialization and again any time afterwards, such as in response to receiving an updated configuration from Nimbus. Only the latest configuration provided will be applied and any previously configured metrics that are omitted from the new configuration will not be changed. An example call to set a configuration from the “homescreen” Nimbus Feature could look like this:

```Swift
Glean.setMetricsEnabledConfig(FxNimbus.features.homescreen.value().metricsEnabled)
```

Since mobile experiments only update on initialization of the application, it isn't necessary to register to listen for notifications for experiment updates.

[Nimbus]: https://experimenter.info
[Nimbus Desktop Feature API]: https://experimenter.info/desktop-feature-api