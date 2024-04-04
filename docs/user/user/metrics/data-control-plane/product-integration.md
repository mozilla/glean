# Product Integration

In order to enable sharing of this functionality between multiple Nimbus Features, the implementation is not defined as part of the stand-alone Glean feature defined in the Nimbus Feature Manifest, but instead is intended to be added as a feature variable to other Nimbus Feature definitions for them to make use of.

## Desktop Feature Integration

In order to make use of the remote metric configuration in a Firefox Desktop component, there are two available options.

### Integration Option 1:

Glean provides a general Nimbus feature that can be used for configuration of metrics named `glean`. Simply select the `glean` feature along with your Nimbus feature from the Experimenter UI when configuring the experiment or rollout (see https://experimenter.info for more information on multi-feature experiments).

The `glean` Nimbus feature requires the `gleanMetricConfiguration` variable to be used to provide the require metric configuration. The format of the configuration is a map of the fully qualified metric name (category.name) of the metric to a boolean value representing whether the metric is enabled. If a metric is omitted from this map, it will default to the value found in the metrics.yaml. An example configuration for the `glean` feature can be found on the [Experimenter Configuration](./experimenter-configuration.md) page.


### Integration Option 2 (Advanced use):

 A second option that can give you more control over the metric configuration, especially if there are more than one experiments or rollouts that are currently using the `glean` feature from Option 1 above, is to add a Feature Variable to represent the Glean metric configuration in your own feature. This can be accomplished by modifying the FeatureManifest.yaml file, adding a variable through which to pass metric configurations. Glean will handle merging this configuration with other metrics configurations for you (See [Advanced Topics](./advanced-topics.md) for more info on this). An example feature manifest entry would look like the following:

```yaml
variables:
  ... // Definitions of other feature variables
  gleanMetricConfiguration:
    type: json
    description: >-
    "Glean metric configuration"
```

This definition allows for configuration to be set in a Nimbus rollout or experiment and fetched by the client to be applied based on the enrollment. Once the Feature Variable has been defined, the final step is to fetch the configuration from Nimbus and supply it to the Glean API. This can be done during initialization and again any time afterwards, such as in response to receiving an updated configuration from Nimbus. Glean will merge this configuration with any other active configurations and enable or disable the metrics accordingly. An example call to set a configuration through your Nimbus Feature could look like this:

```JavaScript
// Fetch the Glean metric configuration from your feature's Nimbus variable
let cfg = lazy.NimbusFeatures.yourNimbusFeatureName.getVariable(
  "gleanMetricConfiguration"
);
// Apply the configuration through the Glean API
Services.fog.setMetricsFeatureConfig(JSON.stringify(cfg));
```

It is also recommended to register to listen for updates for the Nimbus Feature and apply new configurations as soon as possible. The following example illustrates how a Nimbus Feature might register and update the metric configuration whenever there is a change to the Nimbus configuration:

```JavaScript
// Register to listen for the `onUpdate` event from Nimbus
lazy.NimbusFeatures.yourNimbusFeatureName.onUpdate(() => {
  // Fetch the Glean metric configuration from your feature's Nimbus variable
  let cfg = lazy.NimbusFeatures.yourNimbusFeatureName.getVariable(
    "gleanMetricConfiguration"
  );
  // Apply the configuration through the Glean API
  Services.fog.setMetricsFeatureConfig(JSON.stringify(cfg));
});
```

## Mobile Feature Integration

### Integration Option 1:

Glean provides a general Nimbus feature that can be used for configuration of metrics named `glean`. Simply select the `glean` feature along with your Nimbus feature from the Experimenter UI when configuring the experiment or rollout (see https://experimenter.info for more information on multi-feature experiments).

The `glean` Nimbus feature requires the `gleanMetricConfiguration` variable to be used to provide the require metric configuration. The format of the configuration is a map of the fully qualified metric name (category.name) of the metric to a boolean value representing whether the metric is enabled. If a metric is omitted from this map, it will default to the value found in the metrics.yaml. An example configuration for the `glean` feature can be found on the [Experimenter Configuration](./experimenter-configuration.md) page.

### Integration Option 2 (Advanced use):

A second option that can give you more control over the metric configuration, especially if there are more than one experiments or rollouts that are currently using the `glean` feature from Option 1 above, is to add a Feature Variable to represent the Glean metric configuration in your own feature. This can be accomplished by modifying the Nimbus Feature Manifest file, adding a variable through which to pass metric configurations. Glean will handle merging this configuration with other metrics configurations for you (See [Advanced Topics](./advanced-topics.md) for more info on this). An example feature manifest entry would look like the following:

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
Glean.setRemoteSettingsConfig(FxNimbus.features.homescreen.value().metricsEnabled)
```

Since mobile experiments only update on initialization of the application, it isn't necessary to register to listen for notifications for experiment updates.

[Nimbus]: https://experimenter.info
[Nimbus Desktop Feature API]: https://experimenter.info/desktop-feature-api