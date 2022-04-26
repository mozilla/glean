# Pings YAML Registry Format

Custom pings sent by an application or library are defined in YAML files which follow
the [`pings.yaml` JSON schema](https://mozilla.github.io/glean_parser/pings-yaml.html).

This files must be parsed by [`glean_parser`](https://pypi.org/project/glean-parser/) at build time
in order to generate code in the target language (e.g. Kotlin, Swift, ...). The generated code is
what becomes the public API to access the project's custom pings.

For more information on how to introduce the `glean_parser` build step for a specific language /
environment, refer to the ["Adding Glean to your project"](../../user/adding-glean-to-your-project/index.md)
section of this book.

{{#include ../../../shared/blockquote-info.html}}

## Note on the naming of these files

> Although we refer to pings definitions YAML files as `pings.yaml` throughout Glean documentation
> this files may be named whatever makes the most sense for each project and may even be broken down
> into multiple files, if necessary.

## File structure

```yaml
---
# Schema
$schema: moz://mozilla.org/schemas/glean/pings/2-0-0

# Name
search:
  # Ping parameters
  description: >
    A ping to record search data.
  include_client_id: false
  notification_emails:
    - CHANGE-ME@example.com
  bugs:
    - http://bugzilla.mozilla.org/123456789/
  data_reviews:
    - http://example.com/path/to/data-review
```

## Schema

Declaring the schema at the top of a pings definitions file is required,
as it is what indicates that the current file is a pings definitions file.

## Name

Ping names are the top-level keys on pings definitions files.
One single definition file may contain multiple ping declarations.

Ping names are limited to lowercase letters from the [ISO basic Latin alphabet](https://en.wikipedia.org/wiki/ISO_basic_Latin_alphabet)
and hyphens and a maximum of 30 characters.

Pings may not contain the words `custom` or `ping` in their names. These are considered redundant
words and will trigger a `REDUNDANT_PING` lint failure on `glean_parser`.

["Capitalization"](../../user/metrics/adding-new-metrics.md#capitalization) rules apply to
ping names on generated code.

{{#include ../../../shared/blockquote-info.html}}

### Reserved ping names

> The names `baseline`, `metrics`, `events`, `deletion-request`, `default` and `all-pings` are reserved
> and may not be used as the name of a custom ping.

## Ping parameters

### Required parameters

#### `description`

A textual description of the purpose of the ping.
It may contain [markdown syntax](https://www.markdownguide.org/basic-syntax/).

#### `metadata`

_default: `{}`_

A dictionary of extra metadata associated with this ping.
Currently the only allowed key is `tags` (see below).

##### `tags`

_default: `[]`_

A list of tag names associated with this ping.
Must correspond to an entry specified in a [tags file](./tags.md).


#### `include_client_id`

A boolean indicating whether to include the client_id in
[the client_info section](../../user/pings/index.md#the-client_info-section) of the ping.

#### `notification_emails`

A list of email addresses to notify for important events with the ping
or when people with context or ownership for the ping need to be contacted.

Consider adding both a group email address and an individual who is responsible for this ping.

#### `bugs`

A list of bugs (e.g. Bugzilla or GitHub) that are relevant to this ping.
For example, bugs that track its original implementation or later changes to it.

Each entry should be the full URL to the bug in an issue tracker.
The use of numbers alone is deprecated and will be an error in the future.

#### `data_reviews`

A list of URIs to any data collection review _responses_ relevant to the metric.

### Optional parameters

#### `send_if_empty`

_default: `false`_

A boolean indicating if the ping is sent if it contains no metric data.

#### `reasons`

_default: `{}`_

The reasons that this ping may be sent. The keys are the reason codes,
and the values are a textual description of each reason.
The ping payload will (optionally) contain one of these reasons in the `ping_info.reason` field.
