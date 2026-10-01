# Flows YAML Registry Format

[Flows](../../user/flows/index.md) are declared in YAML files that follow the `flows.yaml` schema
(`moz://mozilla.org/schemas/glean/flows/1-0-0`). Like `metrics.yaml` and `pings.yaml`, these files are parsed by
[`glean_parser`](https://pypi.org/project/glean-parser/) at build time.

Unlike those files, `flows.yaml` produces no public API. A flow is driven entirely by events your
product already records, so there is nothing to call. What `glean_parser` generates instead is the
flow registration data that the Glean SDK loads at initialization, passed in through the `flows`
[configuration option](../general/initializing.md#configuration).

{{#include ../../../shared/blockquote-info.html}}

## Note on the naming of these files

> As with the other registry files, `flows.yaml` may be named whatever makes the most sense for
> your project, and may be split across several files. A library may ship a `flows.yaml` of its
> own. The application's `glean_parser` invocation receives every one of them and emits the
> registration data for all of them together.

## File structure

```yaml
---
# Schema
$schema: moz://mozilla.org/schemas/glean/flows/1-0-0

# Name
onboarding:
  description: |
    First-run setup, from the first onboarding card shown through to
    either completion or dismissal.
  bugs:
    - https://bugzilla.mozilla.org/show_bug.cgi?id=0000000
  data_reviews:
    - https://bugzilla.mozilla.org/show_bug.cgi?id=0000000
  notification_emails:
    - CHANGE-ME@example.com
  starts_on:
    - onboarding.started
  ends_on:
    - onboarding.finished
    - onboarding.dismissed
    - onboarding.error_shown
  includes:
    - onboarding.card_shown
    - onboarding.permission_prompted
  auto_close_timeout: 45m
```

## Schema

Declaring the schema at the top of the file is required, as it is what identifies the file as a
flows definitions file.

## Name

Flow names are the top-level keys. One file may declare several flows.

Flow names are limited to lowercase letters, digits, and underscores, start with a letter, and are
at most 40 characters long. They appear in event payloads and as metric labels.

Flow names must be unique across every `flows.yaml` passed to `glean_parser`. Two definitions of
the same name are a build-time error rather than a merge, so a library can never redefine an
application's flow by declaring one with the same name.

## Fields

| Field | Required | Description |
| ----- | -------- | ----------- |
| `description` | yes | What task the flow measures, from where to where. |
| `bugs` | yes | Bugs related to the flow, as for metrics. |
| `data_reviews` | yes | Data reviews covering the flow, as for metrics. |
| `notification_emails` | yes | Who to contact about the flow, as for metrics. |
| `starts_on` | yes | Events that open an instance of the flow. At least one is required. |
| `ends_on` | no | Events that close the active instance. |
| `includes` | no | Other events that belong to the flow without opening or closing it. |
| `auto_close_timeout` | no | The longest an instance may stay open, as `<integer><unit>` where the unit is one of `s`, `m`, `h`, or `d` (for example `45m` or `7d`). Measured from when the instance opened. Without it, an instance stays open until it exits or its session ends. |
| `disabled` | no | Ship the flow switched off. Defaults to `false`. Can be overridden remotely with the [`flows_enabled` Server Knob](../../user/server-knobs/other/flows-enabled.md). |

Events are named by their identifier, `category.name`, exactly as they appear in `metrics.yaml`.

**The flow's members are everything in `starts_on`, `ends_on`, and `includes`.** Entry and exit
events are members by definition, so they are never repeated in `includes`.

**The three lists are sets.** Their order means nothing, and a flow does not declare the order its
steps are expected to happen in. The order events actually occur in is what's being measured.

## Validation

`glean_parser` checks every event a flow names that is defined in the files it was given:

- The event must exist and be an `event` metric.
- The event must be `in_session`. Flows live inside sessions, so an event recorded outside any
  session can't belong to one.

It also checks the flow itself:

- An event may appear in at most one of a flow's three lists. Naming the same event in both
  `starts_on` and `ends_on`, or in `includes` alongside either, is an error.
- `starts_on` must not be empty.
- A project that passes `flows.yaml` files can't also have a metrics category named `flows`.

### Warnings

- **`ends_on` is empty** (`EMPTY_ENDS_ON`). The flow is legal, but it can only close by timing out or when its
  session ends, so it can never record a conclusive outcome. Without an exit, a second genuine
  attempt is also absorbed into the instance that is still open.
- **An event doesn't resolve** (`UNRESOLVED_FLOW_EVENT`). If a flow names an event that isn't defined in any file passed to
  `glean_parser`, it is treated as an external reference to an event defined in a library built
  separately, and reported as a warning rather than an error. The warning names the flow, the list,
  and the event, and notes that the event's existence, type, and `in_session` setting were not
  checked. The rule that an event appears in at most one list still applies.

Either warning can be turned off for a flow with `no_lint`, or for a whole file with a top-level
`no_lint`, as in `metrics.yaml`.

`glean_parser` can't tell an external reference from a typo: both are a name that resolves to
nothing, and at runtime both simply never match. If your flow names an event from another library,
write the test described in
[Testing a reference to another library's event](../../user/flows/testing-flows.md#testing-a-reference-to-another-librarys-event).
It's the check that catches the typo.
