# Adding new metrics

All metrics that your project collects must be defined in a `metrics.yaml` file. 
This file should be at the root of the application or library module (the same directory as the `build.gradle` file you updated). 
The format of that file is documented [here](https://mozilla.github.io/glean_parser/metrics-yaml.html).

When adding a new metric, the workflow is:

* Decide on which [metric type](metrics/index.md) you want to use.
* Add a new entry to [`metrics.yaml`](#The-metricsyaml-file).
* Add code to your project to record into the metric by calling Glean.

**Important**: Any new data collection requires documentation and data-review.
This is also required for any new metric automatically collected by Glean.

## The `metrics.yaml` file

The `metrics.yaml` file defines the metrics your application or library will send. 
They are organized into categories.
The overall organization is:

```YAML
# Required to indicate this is a `metrics.yaml` file
$schema: moz://mozilla.org/schemas/glean/metrics/1-0-0

toolbar:
  click:
    type: event
    description: |
      Event to record toolbar clicks.
    notification_emails:
      - CHANGE-ME@example.com
    bugs:
      - 123456789
    data_reviews:
      - http://example.com/path/to/data-review
    expires:
      - 2019-06-01  # <-- Update to a date in the future
    
  metric2:
    ...
    
category2.subcategory:  # Categories can contain subcategories
  metric3:
    ...

```

The details of the metric parameters are described in [metric parameters](metric-parameters.md).

The `metrics.yaml` file is used to generate `Kotlin` code that becomes the public API to access your application's metrics.

## Metric naming

Category and metric names in the `metrics.yaml` are in `snake_case`, but given the Kotlin coding standards defined by [ktlint](https://github.com/pinterest/ktlint), these identifiers must be `camelCase` in Kotlin. For example, the metric defined in the `metrics.yaml` as:


```YAML
views:
  login_opened:
    ...
```

is accessible in Kotlin as:

```Kotlin
import org.mozilla.yourApplication.GleanMetrics.Views
Views.loginOpened...
```
