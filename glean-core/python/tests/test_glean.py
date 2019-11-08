# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.


import json
import shutil


import pytest


from glean import Glean
from glean import _builtins
from glean import testing
from glean._dispatcher import Dispatcher
from glean.metrics import CounterMetricType, Lifetime, PingType


def setup_function():
    testing.reset_glean()


def test_setting_upload_enabled_before_initialization_should_not_crash():
    Glean.reset()
    Glean.set_upload_enabled(True)
    Glean.initialize()


def test_getting_upload_enabled_before_initialization_should_not_crash():
    Glean.reset()

    Glean.set_upload_enabled(True)
    assert Glean.get_upload_enabled()

    Glean.initialize()
    assert Glean.get_upload_enabled()


def test_send_a_ping(httpserver):
    httpserver.serve_content(b"", code=200)

    Glean._configuration.server_endpoint = httpserver.url
    Glean._configuration.log_pings = True

    counter_metric = CounterMetricType(
        disabled=False,
        category="telemetry",
        lifetime=Lifetime.APPLICATION,
        name="counter_metric",
        send_in_pings=["baseline"],
    )

    counter_metric.add()

    _builtins.pings.baseline.send()

    assert 1 == len(httpserver.requests)

    request = httpserver.requests[0]
    assert "baseline" in request.url


def test_sending_an_empty_ping_doesnt_queue_work(httpserver):
    httpserver.serve_content(b"", code=200)

    Glean._send_pings_by_name(["metrics"])
    assert 0 == len(httpserver.requests)


def test_disabling_upload_should_disable_metrics_recording():
    counter_metric = CounterMetricType(
        disabled=False,
        category="telemetry",
        lifetime=Lifetime.APPLICATION,
        name="counter_metric",
        send_in_pings=["store1"],
    )

    Glean.set_upload_enabled(False)
    assert False is Glean.get_upload_enabled()
    counter_metric.add(1)
    assert False is counter_metric.test_has_value()


@pytest.mark.skip
def test_experiments_recording():
    pass


@pytest.mark.skip
def test_sending_of_background_pings():
    pass


def test_initialize_must_not_crash_if_data_dir_is_messed_up(tmpdir):
    filename = tmpdir / "dummy_file"

    # Create a file in a temporary directory
    with open(filename, "w") as fd:
        fd.write("Contents\n")

    Glean.reset()
    assert False is Glean.is_initialized()

    # Pass in the filename as the data_dir
    Glean.initialize(data_dir=filename)

    # This should cause initialization to fail
    assert False is Glean.is_initialized()

    shutil.rmtree(tmpdir)


def test_queued_recorded_metrics_correctly_during_init():
    Glean.reset()

    # Enable queueing
    Dispatcher.set_task_queueing(True)

    counter_metric = CounterMetricType(
        disabled=False,
        category="telemetry",
        lifetime=Lifetime.APPLICATION,
        name="counter_metric",
        send_in_pings=["store1"],
    )

    for i in range(2):
        counter_metric.add()

    Glean.initialize()

    assert counter_metric.test_has_value()
    assert 2 == counter_metric.test_get_value()


def test_initializing_twice_is_a_no_op():
    before_config = Glean._configuration

    Glean.initialize()

    assert before_config is Glean._configuration


@pytest.mark.skip
def test_dont_handle_events_when_uninitialized():
    pass


def test_dont_schedule_pings_if_metrics_disabled(httpserver):
    httpserver.serve_content(b"", code=200)

    counter_metric = CounterMetricType(
        disabled=False,
        category="telemetry",
        lifetime=Lifetime.APPLICATION,
        name="counter_metric",
        send_in_pings=["store1"],
    )

    custom_ping = PingType(name="store1", include_client_id=True)

    counter_metric.add(10)

    Glean.set_upload_enabled(False)

    custom_ping.send()

    assert 0 == len(httpserver.requests)


def test_dont_schedule_pings_if_there_is_no_ping_content(httpserver):
    httpserver.serve_content(b"", code=200)

    custom_ping = PingType(name="store1", include_client_id=True)

    custom_ping.send()

    assert 0 == len(httpserver.requests)


@pytest.mark.skip
def test_the_app_channel_must_be_correctly_set():
    pass


@pytest.mark.skip
def test_get_language_tag_reports_the_tag_for_the_default_locale():
    pass


@pytest.mark.skip
def test_get_language_tag_reports_the_correct_tag_for_a_non_default_language():
    pass


@pytest.mark.skip
def test_get_language_reports_the_modern_translation_for_some_languages():
    pass


@pytest.mark.skip
def test_ping_collection_must_happen_after_currently_scheduled_metrics_recordings():
    pass


@pytest.mark.skip
def test_basic_metrics_should_be_cleared_when_disabling_uploading():
    pass


@pytest.mark.skip
def test_core_metrics_should_be_cleared_with_disabling_and_enabling_uploading():
    pass


def test_collect():
    counter_metric = CounterMetricType(
        disabled=False,
        category="telemetry",
        lifetime=Lifetime.APPLICATION,
        name="counter_metric",
        send_in_pings=["store1"],
    )

    custom_ping = PingType(name="store1", include_client_id=True)

    counter_metric.add(10)

    json_content = Glean.test_collect(custom_ping)

    assert isinstance(json_content, str)

    json_tree = json.loads(json_content)

    assert 10 == json_tree["metrics"]["counter"]["telemetry.counter_metric"]


def test_tempdir_is_cleared():
    tempdir = Glean._data_dir

    Glean.reset()

    assert not tempdir.exists()
