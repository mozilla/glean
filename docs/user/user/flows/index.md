# Flows

A *flow* is a multi-step task your product wants to measure as a unit: onboarding, checkout,
signing in. You declare it once, in a [`flows.yaml`](../../reference/yaml/flows.md) file, by naming
the events that open it, the events that close it, and the events that belong to it in between.
Glean then marks each of those events with the run of the task it belongs to, so that a funnel
becomes a simple grouping in analysis rather than something every analyst has to reconstruct.

Without flows, products typically build this plumbing themselves: a string metric holding a UUID,
set when the task starts and passed as an extra by every participating call site. Each product
does it differently, and the identifier is only as good as every call site remembering to pass it.

**Flows have no API.** A flow is driven entirely by events your product already records, so adopting
one requires no code change and no edit to any existing metric definition.

## How a flow works

```yaml
$schema: moz://mozilla.org/schemas/glean/flows/1-0-0

onboarding:
  description: First-run setup, from the first card through completion or dismissal.
  bugs: [...]
  data_reviews: [...]
  notification_emails: [...]
  starts_on: [onboarding.started]
  ends_on: [onboarding.finished, onboarding.dismissed]
  includes: [onboarding.card_shown, onboarding.permission_prompted]
  auto_close_timeout: 45m
```

Each run of the task is an **instance** of the flow.

- **Opening.** Recording an event in `starts_on` opens an instance, provided a session is active and
  the flow is enabled. If an instance of that flow is already open, nothing happens: the existing
  instance continues. A flow has at most one open instance at a time, though it may open and close
  any number of times in a session.
- **Membership.** While an instance is open, every event the flow names carries a `flows` entry
  identifying the instance. Events the flow doesn't name are unaffected, however close in time
  they are.
- **Closing.** Recording an event in `ends_on` closes the open instance. Both boundaries are
  inclusive: the entry event and the exit event are tagged with the instance they open and close.

Several different flows can be open at the same time, and one event can belong to several of them.

An instance never outlives its session. If the session ends while an instance is open, the instance
closes with it. If the app crashes, the instance is closed the next time it starts.

Whether a flow is open never affects whether an event is recorded. It only affects which `flows`
entries an event carries.

## What gets recorded

### The `flows` entry

Each tagged event carries a `flows` field next to its `session` metadata:

```json
"flows": [
  {
    "name": "onboarding",
    "seq": 0
  }
]
```

`seq` is a counter that starts at 0 for the first instance opened in a session, and goes up by one
for every instance of any flow opened after it. An event that belongs to no open instance has no
`flows` field. An event in several open instances has one entry each, in alphabetical order by
flow name.

**An instance is identified by its session and its `seq` together**, that is
`(session.id, flows[].seq)`. `seq` alone repeats in every session.

### `glean.flow_end`

Every instance records exactly one [`glean.flow_end`](../collected-metrics/metrics.md) event when it
closes, in the "events" ping. Its extras say which instance ended and why:

| `reason` | What happened | How to read it |
| -------- | ------------- | -------------- |
| `exit` | An event in `ends_on` was recorded. `terminating_event` names it. | Conclusive: the terminating event is the outcome. |
| `timeout` | `auto_close_timeout` elapsed. | Censored, not a failure. |
| `session_end` | The session ended while the instance was open. | Censored, not a failure. |
| `abnormal` | The app ended abnormally while the instance was open. Carries the instance's `session_id`. | Censored, not a failure. |

There is no start event. The entry event, tagged with the new instance, is what shows that an
instance began.

There are two narrow cases in which an instance records no `glean.flow_end`: when the user disables
telemetry while it is open, and when an app update removes the flow's definition while an instance
of it is saved across a restart.

For how to turn this into reliable numbers, see [Reading flow data](reading-flow-data.md).

## Registering flows

When you pass your `flows.yaml` files to `glean_parser` along with your `metrics.yaml` files, it
generates the flow definitions as `GleanFlows`, next to your generated metrics. Pass them to Glean
through the `flows` [configuration option](../../reference/general/initializing.md#configuration)
at initialization. A product that defines no flows passes nothing, and pays nothing.

{{#include ../../../shared/tab_header.md}}

<div data-lang="Kotlin" class="tab">

```Kotlin
import org.mozilla.yourApplication.GleanMetrics.GleanBuildInfo
import org.mozilla.yourApplication.GleanMetrics.GleanFlows

Glean.initialize(
    applicationContext = applicationContext,
    uploadEnabled = true,
    configuration = Configuration(flows = GleanFlows.definitions),
    buildInfo = GleanBuildInfo.buildInfo,
)
```

</div>

<div data-lang="Swift" class="tab">

```Swift
Glean.shared.initialize(
    uploadEnabled: true,
    configuration: Configuration(flows: GleanMetrics.GleanFlows.definitions),
    buildInfo: GleanMetrics.GleanBuild.info
)
```

</div>

<div data-lang="Python" class="tab">

The Python SDK loads metrics at runtime rather than generating code, and doesn't support flows yet.

</div>

<div data-lang="Rust" class="tab">

```Rust
let cfg = ConfigurationBuilder::new(true, data_path, "org.mozilla.yourApplication")
    .with_flows(glean_metrics::glean_flows::definitions())
    .build();
glean::initialize(cfg, client_info);
```

</div>

{{#include ../../../shared/tab_footer.md}}

## Which tasks to model as a flow

Flows suit **bounded, meaningful tasks**: something with a clear start, a small number of ways it
can end, and a reason to ask how often people finish it.

A few things make a flow measure well:

- **Declare every way out.** List the successful exit, and also dismissal, cancellation, and error
  paths. If some path is missing from `ends_on`, the instance stays open after the user has left,
  and a second attempt is absorbed into the first one instead of being counted as its own.
- **Set an `auto_close_timeout`.** It bounds how long an instance can stay open when an exit path
  is missed, which limits that conflation. It also limits how long a remotely disabled flow keeps
  recording its last open instance.
- **Use distinct exit events for distinct outcomes.** `terminating_event` names *where* a flow ended.
  If one exit event carries the outcome in an extra, it's still available, but only by joining on
  the instance.

### What flows cost

Each tagged event carries a few dozen extra bytes before compression, and noticeably fewer after.
The larger cost is that **every instance records one `glean.flow_end` event**, which counts towards
the number of events that triggers sending the "events" ping. Flows can therefore make "events"
pings more frequent, not just larger.

What decides the cost is **instances per session**. A product with a handful of long flows per
session won't notice. A flow opened on every page load, every tab, or every search adds one event
per occurrence, and at high rates that moves how often the "events" ping is sent. Prefer flows for
tasks that matter, not for routine interactions.

## Diagnostics

[`glean.flows_not_started`](../collected-metrics/metrics.md), in the "metrics" ping, counts by flow
the entry events recorded while no session was active, when no instance could open. It shows that
instances are going missing, and roughly how many. It isn't a loss rate: it travels in a different
ping from the events it would be compared with, on a different schedule.

## Turning a flow off

A flow whose instrumentation turns out to be wrong can be switched off remotely, without shipping a
build, using the [`flows_enabled` Server Knob](../server-knobs/other/flows-enabled.md). Switching a
flow off stops new instances from opening. It never cuts short one that is already open.

## More

- [Testing flows](testing-flows.md)
- [Reading flow data](reading-flow-data.md)
- [Flows YAML Registry Format](../../reference/yaml/flows.md)
