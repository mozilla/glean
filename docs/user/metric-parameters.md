# Metric parameters

## Required metric parameters

- `type`: **Required.**  Specifies the type of a metric, like "counter" or "event". This defines which operations are valid for the metric, how it is stored and how data analysis tooling displays it. See the list of [supported metric types](metrics/index.md).

> **Important**: Once a metric is released in a product, its `type` should not be changed. If any data was collected locally with the older `type`, and hasn't yet been sent in a ping, recording data with the new `type` may cause any old persisted data to be lost for that metric. See [this comment](https://bugzilla.mozilla.org/show_bug.cgi?id=1621757#c1) for an extended explanation of the different scenarios.

- `description`: **Required.** A textual description of the metric for humans. It should describe what the metric does, what it means for analysts, and its edge cases or any other helpful information.

  The description field may contain [markdown syntax](https://www.markdownguide.org/basic-syntax/).
  
- `notification_emails`: **Required.** A list of email addresses to notify for important events with the metric or when people with context or ownership for the metric need to be contacted.
  For example when a metric's expiration is within in 14 days, emails will be sent from `telemetry-alerts@mozilla.com` to the `notification_emails` addresses associated with the metric.
  Consider adding both a group email address and an individual who is responsible for this metric.
  
- `bugs`: **Required.** A list of bugs (e.g. Bugzilla or GitHub) that are relevant to this metric. For example, bugs that track its original implementation or later changes to it.

  Each entry should be the full URL to the bug in an issue tracker. The use of numbers alone is deprecated and will be an error in the future.
  
- `data_reviews`: **Required.** A list of URIs to any data collection reviews _responses_ relevant to the metric.
  
- `expires`: **Required.** When the metric is set to expire. After a metric expires, an application will no longer collect or send data related to it. May be one of the following values:
  - `<build date>`: An ISO date `yyyy-mm-dd` in UTC on which the metric expires. For example, `2019-03-13`. This date is checked at build time. Except in special cases, this form should be used so that the metric automatically "sunsets" after a period of time.
    Emails will be sent to the `notification_emails` addresses when the metric is about to expire.
    Generally, when a metric is no longer needed, it should simply be removed. This does not affect the availability of data already collected by the pipeline.
  - `never`: This metric never expires.
  - `expired`: This metric is manually expired.
  
## Optional metric parameters

- `lifetime`: Defines the lifetime of the metric. Different lifetimes affect when the metrics value is reset.

{{#include lifetimes-parameters.md}}
    
- `send_in_pings`: Defines which pings the metric should be sent on. If not specified, the metric is sent on the "default ping", which is the `events` ping for events and the `metrics` ping for everything else. Most metrics don't need to specify this unless they are sent on [custom pings](pings/custom.md).

- `disabled`: (default: `false`) Data collection for this metric is disabled.
  This is useful when you want to temporarily disable the collection for a specific metric without removing references to it in your source code.
  Generally, when a metric is no longer needed, it should simply be removed. This does not affect the availability of data already collected by the pipeline.

- `version`: (default: 0) The version of the metric. A monotonically increasing integer value. This should be bumped if the metric changes in a backward-incompatible way.

- `data_sensitivity`: (default: []) A list of data sensitivity categories that the metric falls under. There are four data collection categories related to data sensitivity [defined in Mozilla's data collection review process](https://wiki.mozilla.org/Firefox/Data_Collection):
   
    - **Category 1: Technical Data:** (`technical`) Information about the machine or Firefox itself. Examples include OS, available memory, crashes and errors, outcome of automated processes like updates, safe browsing, activation, version \#s, and build id. This also includes compatibility information about features and APIs used by websites, add-ons, and other 3rd-party software that interact with Firefox during usage.

    - **Category 2: Interaction Data:** (`interaction`) Information about the user’s direct engagement with Firefox. Examples include how many tabs, add-ons, or windows a user has open; uses of specific Firefox features; session length, scrolls and clicks; and the status of discrete user preferences.

    - **Category 3: Web activity data:** (`web_activity`) Information about user web browsing that could be considered sensitive. Examples include users’ specific web browsing history; general information about their web browsing history (such as TLDs or categories of webpages visited over time); and potentially certain types of interaction data about specific webpages visited.

    - **Category 4: Highly sensitive data:** (`highly_sensitive`) Information that directly identifies a person, or if combined with other data could identify a person. Examples include e-mail, usernames, identifiers such as google ad id, apple id, Firefox account, city or country (unless small ones are explicitly filtered out), or certain cookies. It may be embedded within specific website content, such as memory contents, dumps, captures of screen data, or DOM data.


