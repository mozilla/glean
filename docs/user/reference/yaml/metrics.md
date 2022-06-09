# Metrics YAML Registry Format

Metrics sent by an application or library are defined in YAML files which follow
the [`metrics.yaml` JSON schema](https://mozilla.github.io/glean_parser/metrics-yaml.html).

This files must be parsed by [`glean_parser`](https://pypi.org/project/glean-parser/) at build time
in order to generate code in the target language (e.g. Kotlin, Swift, ...). The generated code is
what becomes the public API to access the project's metrics.

For more information on how to introduce the `glean_parser` build step for a specific language /
environment, refer to the ["Adding Glean to your project"](../../user/adding-glean-to-your-project/index.md)
section of this book.

{{#include ../../../shared/blockquote-info.html}}

## Note on the naming of these files

> Although we refer to metrics definitions YAML files as `metrics.yaml` throughout Glean documentation
> this files may be named whatever makes the most sense for each project and may even be broken down
> into multiple files, if necessary.

## File structure

```yaml
---
# Schema
$schema: moz://mozilla.org/schemas/glean/metrics/2-0-0

$tags:
  - frontend

# Category
toolbar:
  # Name
  click:
    # Metric Parameters
    type: event
    description: |
      Event to record toolbar clicks.
    metadata:
      tags:
        - Interaction
    notification_emails:
      - CHANGE-ME@example.com
    bugs:
      - https://bugzilla.mozilla.org/123456789/
    data_reviews:
      - http://example.com/path/to/data-review
    expires: 2019-06-01

  double_click:
    ...
```

## Schema

Declaring the schema at the top of a metrics definitions file is required,
as it is what indicates that the current file is a metrics definitions file.

# `$tags`

You may optionally declare [tags](tags.md) at the file level that apply to all metrics in that file.

## Category

Categories are the top-level keys on metrics definition files. One single definition file
may contain multiple categories grouping multiple metrics. They serve the purpose of grouping related
metrics in a project.

Categories can contain alphanumeric lower case characters as well as the `.` and `_` characters
which can be used to provide extra structure, for example `category.subcategory` is a valid category.
Category lengths may not exceed 40 characters.

Categories may not start with the string `glean`. That prefix is reserved for Glean internal metrics.

See the ["Capitalization"](../../user/metrics/adding-new-metrics.md#capitalization)
note to understand how the category is formatted in generated code.

## Name

Metric names are the second-level keys on metrics definition files.

Names may contain alphanumeric lower case characters as well as the `_` character. Metric name
lengths may not exceed 30 characters.

["Capitalization"](../../user/metrics/adding-new-metrics.md#capitalization) rules also apply to
metric names on generated code.

## Metric parameters

Specific metric types may have special required parameters in their definition,
these parameters are documented in each ["Metric Type"](../metrics/index.md) reference page.

Following are the parameters common to all metric types.

### Required parameters

#### `type`

Specifies the type of a metric, like "counter" or "event".
This defines which operations are valid for the metric,
how it is stored and how data analysis tooling displays it.
See the list of [supported metric types](../metrics/index.md).

{{#include ../../../shared/blockquote-warning.html}}

##### Types must not be changed after release

> Once a metric is defined in a product, its `type` must not be changed.
> The ingestion pipeline will not be able to handle such a change.
> If a type change is required a new metric must be added with a new name and the new type.
> This will require an additional data review, in which you can also reference the old data review.

#### `description`

A textual description of the metric for humans.
It should describe what the metric does, what it means for analysts,
and its edge cases or any other helpful information.

The description field may contain [markdown syntax](https://www.markdownguide.org/basic-syntax/).

{{#include ../../../shared/blockquote-info.html}}

##### Imposed limits on line length

> The Glean linter uses a line length limit of 80 characters.
> If your description is longer, e.g. because it includes longer links,
> you can disable `yamllint` using the following annotations
> (and make sure to enable `yamllint` again as well):
>
> ```yaml
> # yamllint disable
> description: |
>   Your extra long description, that's longer than 80 characters by far.
> # yamllint enable
> ```
  
#### `notification_emails`

A list of email addresses to notify for important events with the metric
or when people with context or ownership for the metric need to be contacted.

For example when a metric's expiration is within in 14 days, emails will be sent
from `telemetry-alerts@mozilla.com` to the `notification_emails` addresses associated with the metric.

Consider adding both a group email address and an individual who is responsible for this metric.
  
#### `bugs`

A list of bugs (e.g. Bugzilla or GitHub) that are relevant to this metric.
For example, bugs that track its original implementation or later changes to it.

Each entry should be the full URL to the bug in an issue tracker.
The use of numbers alone is deprecated and will be an error in the future.

#### `data_reviews`

A list of URIs to any data collection review _responses_ relevant to the metric.

#### `expires`
  
When the metric is set to expire.

After a metric expires, an application will no longer collect or send data related to it.
May be one of the following values:

- `<build date>`: An ISO date `yyyy-mm-dd` in UTC on which the metric expires.
  For example, `2019-03-13`. This date is checked at build time. Except in special cases,
  this form should be used so that the metric automatically "sunsets" after a period of time.
  Emails will be sent to the `notification_emails` addresses when the metric is about to expire.
  Generally, when a metric is no longer needed, it should simply be removed.
  This does not affect the availability of data already collected by the pipeline.
- `never`: This metric never expires.
- `expired`: This metric is manually expired.

### Optional parameters

#### `tags`

_default: `[]`_

A list of tag names associated with this metric.
Must correspond to an entry specified in a [tags file](./tags.md).

#### `lifetime`

_default: `ping`_

Defines the lifetime of the metric. Different lifetimes affect when the metrics value is reset.

{{#include ../../_includes/lifetimes-parameters.md}}

#### `send_in_pings`

_default: `events`|`metrics`_

Defines which pings the metric should be sent on.
If not specified, the metric is sent on the default ping,
which is the `events` ping for events and the `metrics` ping for everything else.

Most metrics don't need to specify this unless they are sent on [custom pings](../../user/pings/custom.md).

The special value `default` may be used, in case it's required for a metric to be sent
on the default ping as well as in a custom ping.

{{#include ../../../shared/blockquote-info.html}}

##### Adding metrics to every ping

> For the small number of metrics that should be in every ping the Glean SDKs will eventually provide a solution.
> See [bug 1695236](https://bugzilla.mozilla.org/show_bug.cgi?id=1695236) for details.

```yaml
send_in_pings:
  - my-custom-ping
  - default
```

#### `disabled`

_default: `false`_

Data collection for this metric is disabled.

This is useful when you want to temporarily disable the collection for a specific metric
without removing references to it in your source code.

Generally, when a metric is no longer needed, it should simply be removed.
This does not affect the availability of data already collected by the pipeline.

#### `version`

_default: `0`_

The version of the metric. A monotonically increasing integer value.
This should be bumped if the metric changes in a backward-incompatible way.

#### `data_sensitivity`

_default: `[]`_

A list of data sensitivity categories that the metric falls under.
There are four data collection categories related to data sensitivity
[defined in Mozilla's data collection review process](https://wiki.mozilla.org/Firefox/Data_Collection):

##### Category 1: Technical Data (`technical`)

Information about the machine or Firefox itself. Examples include OS, available memory,
crashes and errors, outcome of automated processes like updates, safe browsing, activation,
versions, and build id. This also includes compatibility information about features and APIs
used by websites, add-ons, and other 3rd-party software that interact with Firefox during usage.

##### Category 2: Interaction Data (`interaction`)

Information about the user’s direct engagement with Firefox. Examples include how many tabs,
add-ons, or windows a user has open; uses of specific Firefox features; session length,
scrolls and clicks; and the status of discrete user preferences.

##### Category 3: Web activity data (`web_activity`)

Information about user web browsing that could be considered sensitive.
Examples include users’ specific web browsing history; general information
about their web browsing history (such as TLDs or categories of webpages visited over time);
and potentially certain types of interaction data about specific webpages visited.

##### Category 4: Highly sensitive data (`highly_sensitive`)

Information that directly identifies a person, or if combined with other data could identify a person.
Examples include e-mail, usernames, identifiers such as google ad id, apple id, Firefox account,
city or country (unless small ones are explicitly filtered out), or certain cookies.
It may be embedded within specific website content, such as memory contents, dumps,
captures of screen data, or DOM data.
