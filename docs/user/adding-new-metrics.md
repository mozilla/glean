# Adding new metrics

All metrics that your project collects must be defined in a `metrics.yaml` file. 
This file should be at the root of the application or library module (the same directory as the `build.gradle` file you updated). 
The format of that file is documented [here](https://mozilla.github.io/glean_parser/metrics-yaml.html).

When adding a new metric, the workflow is:

* Decide on which [metric type](metrics/index.md) you want to use.
* Add a new entry to [`metrics.yaml`](#The-metricsyaml-file).
* Add code to your project to record into the metric by calling the Glean SDK.

> **Important**: Any new data collection requires documentation and data-review.
This is also required for any new metric automatically collected by the Glean SDK.

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

  double_click:
    ...

category2.subcategory:  # Categories can contain subcategories
  metric:
    ...

```

The details of the metric parameters are described in [metric parameters](metric-parameters.md).

The `metrics.yaml` file is used to generate `Kotlin` code that becomes the public API to access your application's metrics.

## Recommendations for defining new metrics

“There are only two hard things in Computer Science: cache invalidation and naming things” -- attributed to Phil Karlton.

### Lifetimes

The `lifetime` parameter of a metric defines when it will be reset. There are three options available:

- `ping` (default): The metric is reset each time it is sent in the ping.
  This is the most common case, and should be used for metrics that are highly dynamic, such as things computed in response to the user's interaction with the application.
- `application`: The metric is related to an application run, and is reset only when the application restarts.
  This should be used for things that are constant during the run of an application, such as the operating system version.
  In practice, these metrics are generally set during application startup.
  A common mistake---using the `ping` lifetime for these type of metrics---means that they will only be included in the first ping sent during a particular run of the application.
- `user`: The metric is part of the user's profile.
  This should be used for things that change only when the user's profile is created.
  It is rare to use this lifetime outside of some metrics that are built in to Glean, such as `client_id`.

### Naming things

Choose the category and names of metrics to be as specific as possible.
It is not necessary to put the type of the metric in the category or name, since this information is retained in other ways through the entire end-to-end system.

For example, if defining a set of events related to search, put them in a category called `search`, rather than just `events` or `search_events`.

## A note about case inflection

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
