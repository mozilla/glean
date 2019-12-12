# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.


import io
import json
import re
import shutil
import sys


from glean_parser import validate_ping
import pytest


from glean import Configuration, Glean
from glean import __version__ as glean_version
from glean import _builtins
from glean import util
from glean._dispatcher import Dispatcher
from glean.metrics import CounterMetricType, Lifetime, PingType


GLEAN_APP_ID = "glean-python-test"


def test_setting_upload_enabled_before_initialization_should_not_crash():
    Glean.reset()
    Glean.set_upload_enabled(True)
    Glean.initialize(application_id=GLEAN_APP_ID, application_version=glean_version)


def test_getting_upload_enabled_before_initialization_should_not_crash():
    Glean.reset()

    Glean.set_upload_enabled(True)
    assert Glean.get_upload_enabled()

    Glean.initialize(application_id=GLEAN_APP_ID, application_version=glean_version)
    assert Glean.get_upload_enabled()


def test_submit_a_ping(safe_httpserver):
    safe_httpserver.serve_content(b"", code=200)

    Glean._configuration.server_endpoint = safe_httpserver.url
    Glean._configuration.log_pings = True

    counter_metric = CounterMetricType(
        disabled=False,
        category="telemetry",
        lifetime=Lifetime.APPLICATION,
        name="counter_metric",
        send_in_pings=["baseline"],
    )

    counter_metric.add()

    _builtins.pings.baseline.submit()

    assert 1 == len(safe_httpserver.requests)

    request = safe_httpserver.requests[0]
    assert "baseline" in request.url


def test_submiting_an_empty_ping_doesnt_queue_work(safe_httpserver):
    safe_httpserver.serve_content(b"", code=200)

    Glean._submit_pings_by_name(["metrics"])
    assert 0 == len(safe_httpserver.requests)


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


def test_experiments_recording():
    Glean.set_experiment_active("experiment_test", "branch_a")
    Glean.set_experiment_active("experiment_api", "branch_b", {"test_key": "value"})

    assert Glean.test_is_experiment_active("experiment_api")
    assert Glean.test_is_experiment_active("experiment_test")

    Glean.set_experiment_inactive("experiment_test")

    assert Glean.test_is_experiment_active("experiment_api")
    assert not Glean.test_is_experiment_active("experiment_test")

    stored_data = Glean.test_get_experiment_data("experiment_api")
    assert "branch_b" == stored_data.branch
    assert 1 == len(stored_data.extra)
    assert "value" == stored_data.extra["test_key"]


def test_experiments_recording_before_glean_inits():
    # This test relies on Glean not being initialized and task
    # queuing to be on.
    Glean.reset()

    Glean.set_experiment_active("experiment_set_preinit", "branch_a")
    Glean.set_experiment_active("experiment_preinit_disabled", "branch_a")

    Glean.set_experiment_inactive("experiment_preinit_disabled")

    # This will init Glean and flush the dispatcher's queue.
    Glean.initialize(application_id=GLEAN_APP_ID, application_version=glean_version)

    assert Glean.test_is_experiment_active("experiment_set_preinit")
    assert not Glean.test_is_experiment_active("experiment_preinit_disabled")


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
    Glean.initialize(
        application_id=GLEAN_APP_ID,
        application_version=glean_version,
        data_dir=filename,
    )

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

    Glean.initialize(application_id=GLEAN_APP_ID, application_version=glean_version)

    assert counter_metric.test_has_value()
    assert 2 == counter_metric.test_get_value()


def test_initializing_twice_is_a_no_op():
    before_config = Glean._configuration

    Glean.initialize(application_id=GLEAN_APP_ID, application_version=glean_version)

    assert before_config is Glean._configuration


@pytest.mark.skip
def test_dont_handle_events_when_uninitialized():
    pass


def test_dont_schedule_pings_if_metrics_disabled(safe_httpserver):
    safe_httpserver.serve_content(b"", code=200)

    counter_metric = CounterMetricType(
        disabled=False,
        category="telemetry",
        lifetime=Lifetime.APPLICATION,
        name="counter_metric",
        send_in_pings=["store1"],
    )

    custom_ping = PingType(name="store1", include_client_id=True, send_if_empty=False)

    counter_metric.add(10)

    Glean.set_upload_enabled(False)

    custom_ping.submit()

    assert 0 == len(safe_httpserver.requests)


def test_dont_schedule_pings_if_there_is_no_ping_content(safe_httpserver):
    safe_httpserver.serve_content(b"", code=200)

    custom_ping = PingType(name="store1", include_client_id=True, send_if_empty=False)

    custom_ping.submit()

    assert 0 == len(safe_httpserver.requests)


def test_the_app_channel_must_be_correctly_set():
    Glean.reset()
    Glean.initialize(
        application_id=GLEAN_APP_ID,
        application_version=glean_version,
        configuration=Configuration(channel="my-test-channel"),
    )
    assert (
        "my-test-channel"
        == _builtins.metrics.glean.internal.metrics.app_channel.test_get_value()
    )


def test_get_language_tag_reports_the_tag_for_the_default_locale():
    tag = util.get_locale_tag()
    assert re.match("[a-z][a-z]-[A-Z][A-Z]", tag)


@pytest.mark.skip
def test_get_language_tag_reports_the_correct_tag_for_a_non_default_language():
    """
    Not relevant for non-Java platforms.
    """
    pass


@pytest.mark.skip
def test_get_language_reports_the_modern_translation_for_some_languages():
    """
    Not relevant for non-Java platforms.
    """
    pass


@pytest.mark.skip
def test_ping_collection_must_happen_after_currently_scheduled_metrics_recordings():
    pass


def test_basic_metrics_should_be_cleared_when_disabling_uploading():
    counter_metric = CounterMetricType(
        disabled=False,
        category="telemetry",
        lifetime=Lifetime.APPLICATION,
        name="counter_metric",
        send_in_pings=["store1"],
    )

    counter_metric.add(10)
    assert counter_metric.test_has_value()

    Glean.set_upload_enabled(False)
    assert not counter_metric.test_has_value()
    counter_metric.add(10)
    assert not counter_metric.test_has_value()

    Glean.set_upload_enabled(True)
    assert not counter_metric.test_has_value()
    counter_metric.add(10)
    assert counter_metric.test_has_value()


def test_core_metrics_should_be_cleared_with_disabling_and_enabling_uploading():
    assert _builtins.metrics.glean.internal.metrics.os.test_has_value()
    Glean.set_upload_enabled(False)
    assert not _builtins.metrics.glean.internal.metrics.os.test_has_value()
    Glean.set_upload_enabled(True)
    assert _builtins.metrics.glean.internal.metrics.os.test_has_value()


def test_collect():
    counter_metric = CounterMetricType(
        disabled=False,
        category="telemetry",
        lifetime=Lifetime.APPLICATION,
        name="counter_metric",
        send_in_pings=["store1"],
    )

    custom_ping = PingType(name="store1", include_client_id=True, send_if_empty=False)

    counter_metric.add(10)

    json_content = Glean.test_collect(custom_ping)

    assert isinstance(json_content, str)

    json_tree = json.loads(json_content)

    assert 10 == json_tree["metrics"]["counter"]["telemetry.counter_metric"]

    assert 0 == validate_ping.validate_ping(io.StringIO(json_content), sys.stdout)


def test_tempdir_is_cleared():
    tempdir = Glean._data_dir

    Glean.reset()

    assert not tempdir.exists()


def test_set_application_id_and_version():
    Glean.reset()

    Glean.initialize(application_id="my-id", application_version="my-version")

    assert (
        "my-id" == _builtins.metrics.glean.internal.metrics.app_build.test_get_value()
    )
    assert (
        "my-version"
        == _builtins.metrics.glean.internal.metrics.app_display_version.test_get_value()
    )


def test_disabling_upload_sends_deletion_request(safe_httpserver):
    safe_httpserver.serve_content(b"", code=200)
    Glean._configuration.server_endpoint = safe_httpserver.url

    # Ensure nothing was received yet
    assert 0 == len(safe_httpserver.requests)

    # Disabling upload will trigger a deletion-request ping
    Glean.set_upload_enabled(False)
    assert 1 == len(safe_httpserver.requests)
