# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.

import enum
from pathlib import Path
import time


import pytest


from glean import Configuration, Glean
from glean._dispatcher import Dispatcher
from glean import load_metrics
from glean import metrics
from glean.metrics import Lifetime
from glean import testing
from glean import __version__ as glean_version


ROOT = Path(__file__).parent


def test_the_api_records_to_its_storage_engine():
    class ClickKeys(enum.Enum):
        OBJECT_ID = 0
        OTHER = 1

    click = metrics.EventMetricType(
        disabled=False,
        category="ui",
        lifetime=Lifetime.PING,
        name="click",
        send_in_pings=["store1"],
        allowed_extra_keys=["object_id", "other"],
    )

    # Record two events of the same type, with a little delay
    click.record({ClickKeys.OBJECT_ID: "buttonA", ClickKeys.OTHER: "foo"})

    time.sleep(0.001)

    click.record({ClickKeys.OBJECT_ID: "buttonB", ClickKeys.OTHER: "bar"})
    click.record()

    # Check that data was properly recorded
    snapshot = click.test_get_value()
    assert click.test_has_value()
    assert 3 == len(snapshot)

    first_event = [x for x in snapshot if x.extra.get("object_id") == "buttonA"][0]
    assert "ui" == first_event.category
    assert "click" == first_event.name
    assert "ui.click" == first_event.identifier
    assert "foo" == first_event.extra["other"]

    second_event = [x for x in snapshot if x.extra.get("object_id") == "buttonB"][0]
    assert "ui" == second_event.category
    assert "click" == second_event.name
    assert "bar" == second_event.extra["other"]

    assert first_event.timestamp < second_event.timestamp


def test_the_api_records_to_its_storage_engine_when_category_is_empty():
    class ClickKeys(enum.Enum):
        OBJECT_ID = 0

    click = metrics.EventMetricType(
        disabled=False,
        category="",
        lifetime=Lifetime.PING,
        name="click",
        send_in_pings=["store1"],
        allowed_extra_keys=["object_id"],
    )

    # Record two events of the same type, with a little delay
    click.record(extra={ClickKeys.OBJECT_ID: "buttonA"})

    time.sleep(0.001)

    click.record(extra={ClickKeys.OBJECT_ID: "buttonB"})

    # Check that the data was properly recorded
    snapshot = click.test_get_value()
    assert click.test_has_value()
    assert 2 == len(snapshot)

    first_event = [x for x in snapshot if x.extra["object_id"] == "buttonA"][0]
    assert "click" == first_event.name

    second_event = [x for x in snapshot if x.extra["object_id"] == "buttonB"][0]
    assert "click" == second_event.name

    assert first_event.timestamp < second_event.timestamp


def test_disabled_events_must_not_record_data():
    class ClickKeys(enum.Enum):
        OBJECT_ID = 0
        OTHER = 1

    click = metrics.EventMetricType(
        disabled=True,
        category="ui",
        lifetime=Lifetime.PING,
        name="click",
        send_in_pings=["store1"],
        allowed_extra_keys=["object_id", "other"],
    )

    # Attempt to store the event
    click.record()

    # Check that nothing was recorded
    assert not click.test_has_value()


def test_test_get_value_throws_valueerror_if_nothing_is_stored():
    click = metrics.EventMetricType(
        disabled=False,
        category="ui",
        lifetime=Lifetime.PING,
        name="click",
        send_in_pings=["store1"],
        allowed_extra_keys=["object_id", "other"],
    )

    with pytest.raises(ValueError):
        click.test_get_value()


def test_the_api_records_to_secondary_pings():
    class ClickKeys(enum.Enum):
        OBJECT_ID = 0

    click = metrics.EventMetricType(
        disabled=False,
        category="ui",
        lifetime=Lifetime.PING,
        name="click",
        send_in_pings=["store1", "store2"],
        allowed_extra_keys=["object_id"],
    )

    # Record two events of the same type, with a little delay
    click.record(extra={ClickKeys.OBJECT_ID: "buttonA"})

    time.sleep(0.001)

    click.record(extra={ClickKeys.OBJECT_ID: "buttonB"})

    # Check that the data was properly recorded in the second ping
    snapshot = click.test_get_value("store2")
    assert click.test_has_value("store2")
    assert 2 == len(snapshot)

    first_event = [x for x in snapshot if x.extra["object_id"] == "buttonA"][0]
    assert "ui" == first_event.category
    assert "click" == first_event.name

    second_event = [x for x in snapshot if x.extra["object_id"] == "buttonB"][0]
    assert "ui" == first_event.category
    assert "click" == second_event.name

    assert first_event.timestamp < second_event.timestamp


def test_events_should_not_record_when_upload_is_disabled():
    class EventKeys(enum.Enum):
        TEST_NAME = 0

    event_metric = metrics.EventMetricType(
        disabled=False,
        category="ui",
        lifetime=Lifetime.PING,
        name="click",
        send_in_pings=["store1"],
        allowed_extra_keys=["test_name"],
    )

    Glean.set_upload_enabled(True)
    event_metric.record({EventKeys.TEST_NAME: "event1"})
    snapshot1 = event_metric.test_get_value()
    assert 1 == len(snapshot1)
    Glean.set_upload_enabled(False)
    event_metric.record({EventKeys.TEST_NAME: "event2"})
    with pytest.raises(ValueError):
        event_metric.test_get_value()
    Glean.set_upload_enabled(True)
    event_metric.record({EventKeys.TEST_NAME: "event3"})
    snapshot3 = event_metric.test_get_value()
    assert 1 == len(snapshot3)


def test_flush_queued_events_on_startup(safe_httpserver):
    safe_httpserver.serve_content(b"", code=200)

    Glean._configuration.server_endpoint = safe_httpserver.url

    class EventKeys(enum.Enum):
        SOME_EXTRA = 0

    event = metrics.EventMetricType(
        disabled=False,
        category="telemetry",
        lifetime=Lifetime.PING,
        name="test_event",
        send_in_pings=["events"],
        allowed_extra_keys=["some_extra"],
    )

    event.record(extra={EventKeys.SOME_EXTRA: "bar"})
    assert 1 == len(event.test_get_value())

    testing.reset_glean(
        application_id="glean-python-test",
        application_version=glean_version,
        clear_stores=False,
        configuration=Configuration(server_endpoint=safe_httpserver.url),
    )

    assert 1 == len(safe_httpserver.requests)

    request = safe_httpserver.requests[0]
    assert "events" in request.url


def test_flush_queued_events_on_startup_and_correctly_handle_preinit_events(
    safe_httpserver,
):
    safe_httpserver.serve_content(b"", code=200)

    Glean._configuration.server_endpoint = safe_httpserver.url

    class EventKeys(enum.Enum):
        SOME_EXTRA = 0

    event = metrics.EventMetricType(
        disabled=False,
        category="telemetry",
        lifetime=Lifetime.PING,
        name="test_event",
        send_in_pings=["events"],
        allowed_extra_keys=["some_extra"],
    )

    event.record(extra={EventKeys.SOME_EXTRA: "run1"})
    assert 1 == len(event.test_get_value())

    Dispatcher.set_task_queueing(True)
    event.record(extra={EventKeys.SOME_EXTRA: "pre-init"})

    testing.reset_glean(
        application_id="glean-python-test",
        application_version=glean_version,
        clear_stores=False,
        configuration=Configuration(server_endpoint=safe_httpserver.url),
    )

    event.record(extra={EventKeys.SOME_EXTRA: "post-init"})

    assert 1 == len(safe_httpserver.requests)
    request = safe_httpserver.requests[0]
    assert "events" in request.url

    assert 1 == len(event.test_get_value())

    Glean._submit_ping_by_name("events")

    assert 2 == len(safe_httpserver.requests)
    request = safe_httpserver.requests[1]
    assert "events" in request.url


def test_long_extra_values_record_an_error():
    class ClickKeys(enum.Enum):
        OBJECT_ID = 0
        OTHER = 1

    click = metrics.EventMetricType(
        disabled=False,
        category="ui",
        lifetime=Lifetime.PING,
        name="click",
        send_in_pings=["store1"],
        allowed_extra_keys=["object_id", "other"],
    )

    long_string = "0123456789" * 11

    click.record(extra={ClickKeys.OBJECT_ID: long_string})

    assert 1 == click.test_get_num_recorded_errors(testing.ErrorType.INVALID_OVERFLOW)


def test_event_enum_is_generated_correctly():
    metrics = load_metrics(
        ROOT.parent / "data" / "core.yaml", config={"allow_reserved": True}
    )

    print(dir(metrics.environment))
    metrics.environment.event_example.record(
        {
            metrics.environment.event_example_keys.KEY1: "value1",
            metrics.environment.event_example_keys.KEY2: "value2",
        }
    )

    assert {
        "key1": "value1",
        "key2": "value2",
    } == metrics.environment.event_example.test_get_value()[0].extra
