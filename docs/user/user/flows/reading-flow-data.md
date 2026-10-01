# Reading flow data

Flow data makes funnel analysis cheap to express: an instance is a grouping on `session.id` together
with `flows[].seq`, and its outcome is a read of `glean.flow_end`. Several properties of the data are
correct but easy to misread, though, and each of them fails quietly rather than loudly. This page
collects them.

## Where flow data lives

- **Raw `events` tables.** A row is a ping, not an event, and the events arrive as a nested repeated
  field on it. `flows` is a repeated field inside that.
- **`events_stream`.** One row per event. This is where most analysis starts. `flows` is kept as an
  array, so an event in two flows is still one row, and scalar `flow_name` and `flow_seq` columns
  hold the single entry for the common case of an event in exactly one flow (they're null
  otherwise).

A repeated field is never null. **An untagged event has `ARRAY_LENGTH(flows) = 0`**, not
`flows IS NULL`. A null `flow_name` on its own doesn't mean the event belongs to no flow, since it is
also null for an event in more than one.

`flows` only exists from the point a product adopted a flow, and can't be reconstructed for earlier
data. Treat adoption as a break in the series, not as a period with no instances.

## An instance is `(session.id, flows[].seq)`, never `seq` alone

`seq` is a counter that restarts in every session. Grouping on it without `session.id` merges
unrelated instances across sessions and across clients.

To order instances within a session, use `seq`. To order them across a client's sessions, use
`session.session_seq` and then `seq`.

## Terminated is not the same as abandoned

Glean observes *that* an instance ended. Only the terminating event says anything about why.

- `reason: exit` is conclusive, and `terminating_event` names the outcome.
- `timeout`, `session_end`, and `abnormal` are **censored observations**, not failures. The user may
  simply have put the app in the background mid-task and come back later, at which point the rest of
  the task is a new instance, since an instance never outlives its session.

Treating `session_end` as abandonment inflates drop-off, worst of all for the long-running flows
that are most interesting to measure. Compute drop-off from where in the funnel the last tagged event
landed, treating `timeout`, `session_end`, and `abnormal` as right-censoring.

A flow's inclusion probability is exactly its session's sample rate, which is already in the session
metadata. Flows add no sampling of their own, so there's nothing further to correct for.

## Duration comes from tagged events, never from `glean.flow_end`

`glean.flow_end` is timestamped when Glean *noticed* the instance had ended. That's when it ended
only for `reason: exit`. For every other reason, the gap can be arbitrarily long:

- **`timeout`** is checked lazily, the next time Glean handles the flow, not at the moment it
  expired. The real expiry is the instance's first tagged event's time plus `auto_close_timeout`.
- **`session_end`** happens when the session ends, and an `Auto` mode session that timed out is
  ended at the next activation, which may be hours later.
- **`abnormal`** is recorded when the app next starts. A crash on Monday followed by a relaunch on
  Friday produces a `glean.flow_end` with Friday's timestamp for an instance whose events are all
  from Monday.

**An instance's duration is the span from its first tagged event to its last.** Use the terminating
event's timestamp for the end even when `reason` is `exit`.

## An instance with two entry events has been conflated

If a flow's `ends_on` misses a path the user can take, the instance stays open after they've left,
and a second genuine attempt is absorbed into it. Because entry events are tagged, the re-recorded
entry event carries the *existing* instance, so **an instance containing more than one event from its
flow's `starts_on` is direct evidence that two attempts were merged.**

This is a free data-quality check worth running routinely. A conflated flow under-reports its
attempts and over-reports the duration and step count of the merged instances. The fix is in the
instrumentation: add the missing exit path to `ends_on`, or shorten `auto_close_timeout`.

## An instance whose first event isn't an entry event is missing its beginning

There's no start event, so "the instance began here" means "this is the earliest tagged event we
received". If the ping carrying the real entry event was lost, the earliest surviving event is from
the middle of the funnel.

**If an instance's first tagged event isn't in its flow's `starts_on`, its beginning is missing.**
Leave such instances out of duration and first-step conversion. They're still valid for later-stage
analysis.

## "How many instances?" has two answers

- **Distinct `(session.id, flows[].seq)` over tagged events** counts every instance that produced any
  event, including those still open at the edge of the data.
- **Counting `glean.flow_end` events** counts only instances that closed.

Neither is wrong. Use the first for how many were attempted, and the second for how many completed
their lifecycle. Reporting one as the other is the mistake to avoid.

Joining `glean.flow_end` to its instance's events needs a cast: `flow_seq` is a **string** in its
extras, while `seq` in a `flows` entry is an **integer**. A comparison without a cast returns no rows rather
than an error.

## Unnesting multiplies rows

Once `flows` is unnested, an event in two flows produces two rows, so `COUNT(*)` counts it once per
flow. Count distinct events, or filter to one flow before aggregating.

## A flow's definition can change while its name stays the same

`flows.yaml` belongs to the product and changes over time. Adding an event to `includes`, a path to
`ends_on`, or changing `auto_close_timeout` changes what the funnel measures while the series looks
continuous. **Before comparing across app versions, check that the flow's definition didn't change
between them.** `flows.yaml` is version-controlled in the product's repository, and
`client_info.app_display_version` is in every ping.

## When a flow is switched off remotely

Every instance in the data is complete and correct, whether or not the
[`flows_enabled` Server Knob](../server-knobs/other/flows-enabled.md) switched its flow off. Instances
already open when the switch is thrown finish normally. What changes is the population: new instances
of that flow fall to zero as clients pick up the configuration. Treat that as a break in the series,
not a change in behavior. Because Server Knobs can target audiences, the drop is gradual across the
population even though it's immediate for each client.

## `glean.flows_not_started` is a magnitude, not a rate

It counts entry events recorded while no session was active, so it shows that a flow is losing
instances to timing, and roughly how many. It travels in the "metrics" ping, while instance counts
come from the "events" ping on an unrelated schedule, so a ratio of the two over a window is only
approximate. "This flow loses a meaningful number of instances" is a supportable conclusion. "This
flow loses 3.2% of its instances" is not.

Sampled-out sessions don't increment it: no events are recorded in them, so no entry event is lost
to timing.
