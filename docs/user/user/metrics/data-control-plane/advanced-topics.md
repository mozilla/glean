# Advanced Topics

## Merging of Configurations from Multiple Features

Since each feature defined as a Nimbus Feature can independently provide a Glean configuration, these must be merged together into a cohesive configuration for the entire set of metrics collected by Glean.

Configurations will be merged together along with the default values in the metrics.yaml file and applied to the appropriate metrics. Only the latest configuration provided for a given metric will be applied and any previously configured metrics that are omitted from the new configuration will not be changed.

### Example

Imagine a situation where we have 3 features (`A`, `B`, `C`). Each of these features has an event (`A.event`, `B.event`, `C.event`) and these events all default to disabled from their definition in the metrics.yaml file.

Let’s walk through an example of changing configurations for these features that illustrates how the merging will work:

#### _Initial State_

This is what the initial state of the events looks like with no configurations applied. All of the events are falling back to the defaults from the metrics.yaml file. This is the starting point for Scenario 1 in the [Example Scenarios].

- Feature A
  - No config, default used
  - A.event is disabled
- Feature B
  - No config, default used
  - B.event is disabled
- Feature C
  - No config, default used
  - C.event is disabled

#### _Second State_

In this state, let’s create two rollouts which will provide configurations for features A and B that will enable the events associated with each. The first rollout selects Feature A in experimenter and provides the indicated configuration in the Branch setup page. The second rollout does the same thing, only for Feature B.

- Feature A
  - Configuration:
    - ```json
        {
            // Other variable configs

            // Glean metric config
            "gleanMetricConfiguration": {
                "A.event": true
            }
        }
        ```
  - A.event is enabled
- Feature B
  - Configuration:
    - ```json
        {
            // Other variable configs

            // Glean metric config
            "gleanMetricConfiguration": {
                "B.event": true
            }
        }
        ```
  - B.event is enabled
- Feature C
  - No config, default used
  - C.event is disabled

As you can see, the A.event and B.event are enabled by the configurations while C.event remains disabled because there is no rollout for it.

#### _Third State_

In this state, let’s end the rollout for Feature B, start a rollout for Feature C, and launch an experiment for Feature A. Because experiments take precedence over rollouts, this should supersede our configuration from the rollout for Feature A.

- Feature A
  - Configuration:
    - ```json
        {
            // Other variable configs

            // Glean metric config
            "gleanMetricConfiguration": {
                "A.event": false
            }
        }
        ```
  - A.event is disabled
- Feature B
  - No config, default used
  - B.event is disabled
- Feature C
  - Configuration:
    - ```json
        {
            // Other variable configs

            // Glean metric config
            "gleanMetricConfiguration": {
                "C.event": true
            }
        }
        ```
  - C.event is enabled

After the new changes to the currently running rollouts and experiments, this client is now enrolled in the experiment for Feature A and the configuration is suppressing the A.event. Feature B is no longer sending B.event because it is reverting back to the defaults from the metrics.yaml file. And finally, Feature C is sending C.event with the rollout configuration applied.

#### _Fourth State_

Finally, in this state, let’s end the rollout for Feature C along with the experiment for Feature A. This should stop the sending of the B.event and C.event and resume sending of the A.event as the rollout configuration will again be applied since the experiment configuration is no longer available.

- Feature A
  - Configuration
    -  ```json
        {
            // Other variable configs

            // Glean metric config
            "gleanMetricConfiguration": {
                "A.event": true
            }
        }
        ```
  - A.event is enabled
- Feature B
  - No config, default used
  - B.event is disabled
- Feature C
  - No config, default used
  - C.event is disabled

After the new changes to the currently running rollouts and experiments, this client is now enrolled in the experiment for Feature A and the configuration is suppressing the A.event. Feature B is no longer sending B.event because it is reverting back to the defaults from the metrics.yaml file. And finally, Feature C is sending C.event with the rollout configuration applied.

In each case, Glean only updates the configuration associated with the feature that provided it. Nimbus’ feature exclusion would prevent a client from being enrolled in multiple rollouts or experiments for a given feature, so no more than one configuration would be applied per feature for a given client.

### Merging Caveats

Because there is currently nothing that ties a particular Nimbus Feature to a set of metrics, care must be taken to avoid feature overlap over a particular metric. If two different features supply conflicting configurations for the same metric, then whether or not the metric is enabled will likely come down to a race condition of whoever set the configuration last.

[Example Scenarios]: example-scenarios.md
