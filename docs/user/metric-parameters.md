# Metric parameters

## Required metric parameters

- `type`: **Required.**  Specifies the type of a metric, like "counter" or "event". This defines which operations are valid for the metric, how it is stored and how data analysis tooling displays it. See the list of [supported metric types](metrics/index.md).

- `description`: **Required.** A textual description of the metric for humans. It should describe what the metric does, what it means for analysts, and its edge cases or any other helpful information.
  
- `notification_emails`: **Required.** A list of email addresses to notify for important events with the metric or when people with context or ownership for the metric need to be contacted.
  
- `bugs`: **Required.** A list of bugs (e.g. Bugzilla or Github) that are relevant to this metric. For example, bugs that track its original implementation or later changes to it. 

  Each entry should be the full URL to the bug in an issue tracker. The use of numbers alone is deprecated and will be an error in the future.
  
- `data_reviews`: **Required.** A list of URIs to any data collection reviews relevant to the metric.
  
- `expires`: **Required.** May be one of the following values:
  - `<build date>`: An ISO date `yyyy-mm-dd` in UTC on which the metric expires. For example, `2019-03-13`. This date is checked at build time. Except in special cases, this form should be used so that the metric automatically "sunsets" after a period of time.
    Emails will be sent to the `notification_emails` addresses when the metric is about to expire.
    Generally, when a metric is no longer needed, it should simply be removed. This does not affect the availability of data already collected by the pipeline.
  - `never`: This metric never expires.
  - `expired`: This metric is manually expired.
  
## Optional metric parameters

- `lifetime`: Defines the lifetime of the metric. Different lifetimes affect when the metrics value is reset.
  - `ping` (default): The metric is reset each time it is sent in a ping.
  - `application`: The metric is related to an application run, and is reset when the application restarts.
  - `user`: The metric is part of the user's profile.
    
- `send_in_pings`: Defines which pings the metric should be sent on. If not specified, the metric is sent on the "default ping", which is the `events` ping for events and the `metrics` ping for everything else. Most metrics don't need to specify this unless they are sent on [custom pings](pings/custom.md).

- `disabled`: (default: `false`) Data collection for this metric is disabled.
  This is useful when you want to temporarily disable the collection for a specific metric without removing references to it in your source code.
  Generally, when a metric is no longer needed, it should simply be removed. This does not affect the availability of data already collected by the pipeline.

- `version`: (default: 0) The version of the metric. A monotonically increasing integer value. This should be bumped if the metric changes in a backward-incompatible way.


