# Testing flows

A flow has no API, so there's no flow object to assert against. You test one through the events it
is made of, using the [metrics testing API](../metrics/testing-metrics.md) you already use for
events:

- **That an instance opened, and that an event belongs to it:** `testGetValue` on a member event
  returns events whose `flows` field names the instance.
- **That it closed, and how:** `testGetValue` on `glean.flow_end`, whose extras give the `reason`
  and, for an exit, the `terminating_event`.
- **That something is *not* part of a flow:** an event whose `flows` field is empty.

As with any metric test, Glean must first be reset and put in testing mode. See
[Initializing - Testing API](../../reference/general/initializing.md#testing-api).

## Getting hold of `glean.flow_end`

`glean.flow_end` is recorded by Glean itself, so your generated metrics don't include it. To read it
in a test, construct an event metric with the same identity: category `glean`, name `flow_end`,
sent in the `events` ping. Reading it this way records nothing.

## Example

The examples below use the `onboarding` flow from the
[Flows YAML Registry Format](../../reference/yaml/flows.md) page. They walk through one instance:
it opens, an unrelated event is recorded alongside it, and it closes when the user dismisses it.

{{#include ../../../shared/tab_header.md}}

<div data-lang="Kotlin" class="tab">

```Kotlin
val flowEnd = EventMetricType<NoExtras>(
    CommonMetricData(
        category = "glean",
        name = "flow_end",
        sendInPings = listOf("events"),
        lifetime = Lifetime.PING,
        disabled = false,
    ),
    allowedExtraKeys = emptyList(),
)

Onboarding.started.record()
Onboarding.cardShown.record()
Settings.opened.record()
Onboarding.dismissed.record()

// The flow opened, and the card belongs to its first instance.
val shown = Onboarding.cardShown.testGetValue()!!
assertEquals("onboarding", shown[0].flows[0].name)
assertEquals(0UL, shown[0].flows[0].seq)

// An event the flow doesn't name is never part of it.
assertTrue(Settings.opened.testGetValue()!![0].flows.isEmpty())

// The instance closed, through the dismissal.
val ended = flowEnd.testGetValue()!!
assertEquals("onboarding", ended[0].extra!!["flow_name"])
assertEquals("exit", ended[0].extra!!["reason"])
assertEquals("onboarding.dismissed", ended[0].extra!!["terminating_event"])
```

</div>

<div data-lang="Swift" class="tab">

```Swift
let flowEnd = EventMetricType<NoExtras>(CommonMetricData(
    category: "glean",
    name: "flow_end",
    sendInPings: ["events"],
    lifetime: .ping,
    disabled: false
))

GleanMetrics.Onboarding.started.record()
GleanMetrics.Onboarding.cardShown.record()
GleanMetrics.Settings.opened.record()
GleanMetrics.Onboarding.dismissed.record()

// The flow opened, and the card belongs to its first instance.
let shown = GleanMetrics.Onboarding.cardShown.testGetValue()!
XCTAssertEqual("onboarding", shown[0].flows[0].name)
XCTAssertEqual(0, shown[0].flows[0].seq)

// An event the flow doesn't name is never part of it.
XCTAssertTrue(GleanMetrics.Settings.opened.testGetValue()![0].flows.isEmpty)

// The instance closed, through the dismissal.
let ended = flowEnd.testGetValue()!
XCTAssertEqual("onboarding", ended[0].extra?["flow_name"])
XCTAssertEqual("exit", ended[0].extra?["reason"])
XCTAssertEqual("onboarding.dismissed", ended[0].extra?["terminating_event"])
```

</div>

<div data-lang="Python" class="tab">

```python
from glean import load_metrics
from glean.metrics import CommonMetricData, EventMetricType, Lifetime

metrics = load_metrics("metrics.yaml")

flow_end = EventMetricType(
    CommonMetricData(
        category="glean",
        name="flow_end",
        send_in_pings=["events"],
        lifetime=Lifetime.PING,
        disabled=False,
        label=None,
    ),
    allowed_extra_keys=[],
)

metrics.onboarding.started.record()
metrics.onboarding.card_shown.record()
metrics.settings.opened.record()
metrics.onboarding.dismissed.record()

# The flow opened, and the card belongs to its first instance.
shown = metrics.onboarding.card_shown.test_get_value()
assert "onboarding" == shown[0].flows[0].name
assert 0 == shown[0].flows[0].seq

# An event the flow doesn't name is never part of it.
assert [] == metrics.settings.opened.test_get_value()[0].flows

# The instance closed, through the dismissal.
ended = flow_end.test_get_value()
assert "onboarding" == ended[0].extra["flow_name"]
assert "exit" == ended[0].extra["reason"]
assert "onboarding.dismissed" == ended[0].extra["terminating_event"]
```

</div>

<div data-lang="Rust" class="tab">

```Rust
use glean::private::EventMetric;
use glean::traits::NoExtraKeys;
use glean::{CommonMetricData, Lifetime, TestGetValue};

let flow_end = EventMetric::<NoExtraKeys>::new(CommonMetricData {
    category: "glean".into(),
    name: "flow_end".into(),
    send_in_pings: vec!["events".into()],
    lifetime: Lifetime::Ping,
    ..Default::default()
});

onboarding::started.record(None);
onboarding::card_shown.record(None);
settings::opened.record(None);
onboarding::dismissed.record(None);

// The flow opened, and the card belongs to its first instance.
let shown = onboarding::card_shown.test_get_value(None).unwrap();
assert_eq!("onboarding", shown[0].flows[0].name);
assert_eq!(0, shown[0].flows[0].seq);

// An event the flow doesn't name is never part of it.
assert!(settings::opened.test_get_value(None).unwrap()[0].flows.is_empty());

// The instance closed, through the dismissal.
let ended = flow_end.test_get_value(None).unwrap();
let extra = ended[0].extra.as_ref().unwrap();
assert_eq!("onboarding", extra["flow_name"]);
assert_eq!("exit", extra["reason"]);
assert_eq!("onboarding.dismissed", extra["terminating_event"]);
```

</div>

{{#include ../../../shared/tab_footer.md}}

A session must be active for an instance to open. In the default `Auto` session mode, a session
starts when Glean handles the application coming to the foreground. If your test setup doesn't
simulate that, initialize Glean in `Manual` session mode and start a session explicitly before
recording, with `Glean.sessionStart()` (`session_start()` in Python and Rust).

## What else is worth testing

- **Every exit path.** Record each event in `ends_on` after an entry event, and check that each one
  produces a `glean.flow_end` with that `terminating_event`. A missed exit path is the most common
  way a flow goes wrong, and it fails quietly.
- **A second attempt.** Open, close, and open again, and check that the second instance's events
  carry `seq` 1. If they carry 0, the first instance never closed.
- **Events that shouldn't be members.** Check that an unrelated event recorded mid-flow has no
  `flows` entry.

## Testing a reference to another library's event

When a flow names an event that `glean_parser` couldn't see, because it's defined in a library built
separately, `glean_parser` can only warn. A typo in that name looks exactly the same: a reference
that resolves to nothing, and at runtime simply never matches.

The test that tells them apart is the membership check above, applied to the library's event: open
the flow, record the library event, and assert that it carries a `flows` entry for your flow. It
fails loudly on a typo, before the instrumentation ships. Write one for every event a flow names
across a build boundary.
