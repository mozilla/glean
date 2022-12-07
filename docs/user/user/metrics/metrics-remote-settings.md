# Remote Configuration of Metrics

## Overview

> **Important:** This functionality is experimental and is not ready for production use without coordination with the Glean Team.

Glean metrics have the ability to be disabled and enabled at runtime, effectively overriding the `disabled` property of the metric defined for it in the `metrics.yaml` file. This functionality is currently able to be controlled through [Nimbus experiments and rollouts](https://experimenter.info).

Having metrics which can be remotely turned on and off via remote settings allows us to precisely control the sample of the population sending us telemetry.
Through this, event metrics can be instrumented in areas of high activity and only a subset of the population can be sampled to reduce the amount of traffic and data storage needed while still maintaining enough of a signal from the telemetry to make data-informed decisions based on it.


## How to Enable/Disable Metrics

The instructions and requirements for running Nimbus experiments and rollouts can be found at https://experimenter.info. The purpose of the instructions found here in the Glean Book are meant to supplement the Nimbus information and aid in running experiments that interact with Glean metrics.

When creating an experiment definition in the Experimenter UI, during the Branches Configuration stage:

- Be sure to select `Glean` as the feature from the dropdown. If you do not see `Glean` as an option, then it has not been enabled for your application yet.
- Ensure that the feature is enabled is toggled to "On" in the Branch Configuration page for each branch.
- To disable remote configuration of metrics for a branch, enter empty braces into the "value" field: `{}`.
- To enable a remote configuration for a branch, enter JSON into the "Value" field in the following format:
  - ```JSON
    {
      "metricsDisabled": {
        "category.name": true,
        "category.different_name": false,
        ...
      }
    }
    ```
  - Do not change `"metricsDisabled"`, this is the required object key for Nimbus to recognize the Glean feature.
  - Do change `"category.name"` to match the category and name of the metric to disable/enable.
  - This is a list you can add as many entries in the format as needed.
  - Since this controls the `disabled` property of the metrics, `true` tells Glean to *_disable_* the metric, while `false` would tell Glean to *_enable_* the metric.
