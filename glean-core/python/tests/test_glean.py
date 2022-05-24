# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.


import io
import os
import json
from pathlib import Path
import re
import shutil
import subprocess
import sys
import time
import uuid


from glean_parser import validate_ping
import pytest


from glean import Configuration, Glean, load_metrics
from glean import __version__ as glean_version
from glean import _builtins
from glean import _util
from glean.metrics import (
    CounterMetricType,
    CommonMetricData,
    Lifetime,
    PingType,
    StringMetricType,
)
from glean.net import PingUploadWorker
from glean.testing import _RecordingUploader
from glean._uniffi import glean_set_test_mode

GLEAN_APP_ID = "glean-python-test"


ROOT = Path(__file__).parent


def test_setting_upload_enabled_before_initialization_should_not_crash():
    Glean._reset()
    Glean.set_upload_enabled(True)
    Glean._initialize_with_tempdir_for_testing(
        application_id=GLEAN_APP_ID,
        application_version=glean_version,
        upload_enabled=True,
    )


def test_submit_a_ping(safe_httpserver):
    safe_httpserver.serve_content(b"", code=200)

    Glean._configuration.server_endpoint = safe_httpserver.url

    counter_metric = CounterMetricType(
        CommonMetricData(
            disabled=False,
            category="telemetry",
            lifetime=Lifetime.APPLICATION,
            name="counter_metric",
            send_in_pings=["baseline"],
            dynamic_label=None,
        )
    )

    counter_metric.add()

    _builtins.pings.baseline.submit()

    assert 1 == len(safe_httpserver.requests)

    request = safe_httpserver.requests[0]
    assert "baseline" in request.url


def test_submiting_an_empty_ping_doesnt_queue_work(safe_httpserver):
    safe_httpserver.serve_content(b"", code=200)

    _builtins.pings.metrics.submit()
    assert 0 == len(safe_httpserver.requests)


def test_disabling_upload_should_disable_metrics_recording():
    counter_metric = CounterMetricType(
        CommonMetricData(
            disabled=False,
            category="telemetry",
            lifetime=Lifetime.APPLICATION,
            name="counter_metric",
            send_in_pings=["store1"],
            dynamic_label=None,
        )
    )

    Glean.set_upload_enabled(False)
    counter_metric.add(1)
    assert None is counter_metric.test_get_value()


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
    Glean._reset()

    Glean.set_experiment_active("experiment_set_preinit", "branch_a")
    Glean.set_experiment_active("experiment_preinit_disabled", "branch_a")

    Glean.set_experiment_inactive("experiment_preinit_disabled")

    # This will init Glean and flush the dispatcher's queue.
    Glean._initialize_with_tempdir_for_testing(
        application_id=GLEAN_APP_ID,
        application_version=glean_version,
        upload_enabled=True,
    )

    assert Glean.test_is_experiment_active("experiment_set_preinit")
    assert not Glean.test_is_experiment_active("experiment_preinit_disabled")


@pytest.mark.skip
def test_sending_of_background_pings():
    pass


def test_initialize_must_not_crash_if_data_dir_is_messed_up(tmpdir):
    filename = tmpdir / "dummy_file"

    # Create a file in a temporary directory
    with filename.open("w") as fd:
        fd.write("Contents\n")

    Glean._reset()
    assert False is Glean.is_initialized()

    # Pass in the filename as the data_dir
    Glean.initialize(
        application_id=GLEAN_APP_ID,
        application_version=glean_version,
        upload_enabled=True,
        data_dir=filename,
    )

    # This should cause initialization to fail,
    # but we don't have a way to check.

    shutil.rmtree(str(tmpdir), ignore_errors=True)


def test_queued_recorded_metrics_correctly_during_init():
    Glean._reset()

    counter_metric = CounterMetricType(
        CommonMetricData(
            disabled=False,
            category="telemetry",
            lifetime=Lifetime.APPLICATION,
            name="counter_metric",
            send_in_pings=["store1"],
            dynamic_label=None,
        )
    )

    for _ in range(2):
        counter_metric.add()

    Glean._initialize_with_tempdir_for_testing(
        application_id=GLEAN_APP_ID,
        application_version=glean_version,
        upload_enabled=True,
    )

    assert 2 == counter_metric.test_get_value()


def test_initializing_twice_is_a_no_op():
    before_config = Glean._configuration

    Glean._initialize_with_tempdir_for_testing(
        application_id=GLEAN_APP_ID,
        application_version=glean_version,
        upload_enabled=True,
    )

    assert before_config is Glean._configuration


@pytest.mark.skip
def test_dont_handle_events_when_uninitialized():
    pass


def test_dont_schedule_pings_if_metrics_disabled(safe_httpserver):
    safe_httpserver.serve_content(b"", code=200)

    counter_metric = CounterMetricType(
        CommonMetricData(
            disabled=False,
            category="telemetry",
            lifetime=Lifetime.APPLICATION,
            name="counter_metric",
            send_in_pings=["store1"],
            dynamic_label=None,
        )
    )

    custom_ping = PingType(
        name="store1", include_client_id=True, send_if_empty=False, reason_codes=[]
    )

    counter_metric.add(10)

    Glean.set_upload_enabled(False)

    custom_ping.submit()

    assert 0 == len(safe_httpserver.requests)


def test_dont_schedule_pings_if_there_is_no_ping_content(safe_httpserver):
    safe_httpserver.serve_content(b"", code=200)

    custom_ping = PingType(
        name="store1", include_client_id=True, send_if_empty=False, reason_codes=[]
    )

    custom_ping.submit()

    assert 0 == len(safe_httpserver.requests)


def test_the_app_channel_must_be_correctly_set():
    Glean._reset()
    Glean._initialize_with_tempdir_for_testing(
        application_id=GLEAN_APP_ID,
        application_version=glean_version,
        upload_enabled=True,
        configuration=Configuration(channel="my-test-channel"),
    )
    assert (
        "my-test-channel"
        == _builtins.metrics.glean.internal.metrics.app_channel.test_get_value()
    )


def test_get_language_tag_reports_the_tag_for_the_default_locale():
    tag = _util.get_locale_tag()
    assert re.match("(und)|([a-z][a-z]-[A-Z][A-Z])", tag)


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


def test_ping_collection_must_happen_after_currently_scheduled_metrics_recordings(
    tmpdir, ping_schema_url, monkeypatch
):
    # Given the following block of code:
    #
    # metrics.metric.a.set("SomeTestValue")
    # Glean.submit_pings(["custom-ping-1"])
    #
    # This test ensures that "custom-ping-1" contains "metric.a" with a value of "SomeTestValue"
    # when the ping is collected.

    info_path = Path(str(tmpdir)) / "info.txt"

    monkeypatch.setattr(
        Glean._configuration, "ping_uploader", _RecordingUploader(info_path)
    )

    ping_name = "custom_ping_1"
    ping = PingType(
        name=ping_name, include_client_id=True, send_if_empty=False, reason_codes=[]
    )
    string_metric = StringMetricType(
        CommonMetricData(
            disabled=False,
            category="category",
            lifetime=Lifetime.PING,
            name="string_metric",
            send_in_pings=[ping_name],
            dynamic_label=None,
        )
    )

    # This test relies on testing mode to be disabled, since we need to prove the
    # real-world async behaviour of this.
    Glean._testing_mode = False
    glean_set_test_mode(False)

    # This is the important part of the test. Even though both the metrics API and
    # sendPings are async and off the main thread, "SomeTestValue" should be recorded,
    # the order of the calls must be preserved.
    test_value = "SomeTestValue"
    string_metric.set(test_value)
    ping.submit()

    while not info_path.exists():
        time.sleep(0.1)

    with info_path.open("r") as fd:
        url_path = fd.readline()
        serialized_ping = fd.readline()

    assert ping_name == url_path.split("/")[3]

    json_content = json.loads(serialized_ping)

    assert 0 == validate_ping.validate_ping(
        io.StringIO(serialized_ping),
        sys.stdout,
        schema_url=ping_schema_url,
    )

    assert {"category.string_metric": test_value} == json_content["metrics"]["string"]


def test_basic_metrics_should_be_cleared_when_disabling_uploading():
    counter_metric = CounterMetricType(
        CommonMetricData(
            disabled=False,
            category="telemetry",
            lifetime=Lifetime.APPLICATION,
            name="counter_metric",
            send_in_pings=["store1"],
            dynamic_label=None,
        )
    )

    counter_metric.add(10)
    assert 10 == counter_metric.test_get_value()

    Glean.set_upload_enabled(False)
    assert not counter_metric.test_get_value()
    counter_metric.add(10)
    assert not counter_metric.test_get_value()

    Glean.set_upload_enabled(True)
    assert not counter_metric.test_get_value()
    counter_metric.add(10)
    assert 10 == counter_metric.test_get_value()


def test_core_metrics_should_be_cleared_with_disabling_and_enabling_uploading():
    assert _builtins.metrics.glean.internal.metrics.os.test_get_value()
    Glean.set_upload_enabled(False)
    assert not _builtins.metrics.glean.internal.metrics.os.test_get_value()
    Glean.set_upload_enabled(True)
    assert _builtins.metrics.glean.internal.metrics.os.test_get_value()


def test_tempdir_is_cleared():
    tempdir = Glean._data_dir

    Glean._reset()

    assert not tempdir.exists()


def test_tempdir_is_cleared_multiprocess(safe_httpserver):
    safe_httpserver.serve_content(b"", code=200)
    Glean._configuration.server_endpoint = safe_httpserver.url

    # This test requires us to write a few files in the pending pings
    # directory, to which language bindings have theoretically no access.
    # Manually create the path to that directory, at the risk of breaking
    # the test in the future, if that changes in the Rust code.
    pings_dir = Glean._data_dir / "pending_pings"
    pings_dir.mkdir()

    for _ in range(10):
        with (pings_dir / str(uuid.uuid4())).open("wb") as fd:
            fd.write(b"/data/path/\n")
            fd.write(b"{}\n")

    # Make sure that resetting while the PingUploadWorker is running doesn't
    # delete the directory out from under the PingUploadWorker.
    p1 = PingUploadWorker._process()
    Glean._reset()

    p1.wait()
    assert p1.returncode == 0

    assert 10 == len(safe_httpserver.requests)


def test_set_application_build_id():
    Glean._reset()

    Glean._initialize_with_tempdir_for_testing(
        application_id="my-id",
        application_version="my-version",
        application_build_id="123ABC",
        upload_enabled=True,
    )

    assert (
        "123ABC" == _builtins.metrics.glean.internal.metrics.app_build.test_get_value()
    )


def wait_for_requests(server, n=1, timeout=2):
    start_time = time.time()
    while len(server.requests) < n:
        time.sleep(0.1)
        if time.time() - start_time > timeout:
            raise TimeoutError()


def test_set_application_id_and_version(safe_httpserver):
    safe_httpserver.serve_content(b"", code=200)
    Glean._reset()

    Glean._initialize_with_tempdir_for_testing(
        application_id="my-id",
        application_version="my-version",
        upload_enabled=True,
        configuration=Configuration(server_endpoint=safe_httpserver.url),
    )

    assert (
        "my-version"
        == _builtins.metrics.glean.internal.metrics.app_display_version.test_get_value()
    )

    _builtins.pings.baseline.submit()
    wait_for_requests(safe_httpserver)

    assert 1 == len(safe_httpserver.requests)

    request = safe_httpserver.requests[0]
    assert "baseline" in request.url
    assert "my-id" in request.url


def test_disabling_upload_sends_deletion_request(safe_httpserver):
    safe_httpserver.serve_content(b"", code=200)
    Glean._configuration.server_endpoint = safe_httpserver.url

    # Ensure nothing was received yet
    assert 0 == len(safe_httpserver.requests)

    # Disabling upload will trigger a deletion-request ping
    Glean.set_upload_enabled(False)
    assert 1 == len(safe_httpserver.requests)


def test_configuration_property(safe_httpserver):
    safe_httpserver.serve_content(b"", code=200)

    Glean._configuration.server_endpoint = safe_httpserver.url

    counter_metric = CounterMetricType(
        CommonMetricData(
            disabled=False,
            category="telemetry",
            lifetime=Lifetime.APPLICATION,
            name="counter_metric",
            send_in_pings=["baseline"],
            dynamic_label=None,
        )
    )

    counter_metric.add()

    _builtins.pings.baseline.submit()

    assert 1 == len(safe_httpserver.requests)

    request = safe_httpserver.requests[0]
    assert "baseline" in request.url


def test_sending_deletion_ping_if_disabled_outside_of_run(tmpdir, ping_schema_url):
    info_path = Path(str(tmpdir)) / "info.txt"
    data_dir = Path(str(tmpdir)) / "glean"

    Glean._reset()

    Glean.initialize(
        application_id=GLEAN_APP_ID,
        application_version=glean_version,
        upload_enabled=True,
        data_dir=data_dir,
        configuration=Configuration(ping_uploader=_RecordingUploader(info_path)),
    )

    Glean._reset()

    Glean.initialize(
        application_id=GLEAN_APP_ID,
        application_version=glean_version,
        upload_enabled=False,
        data_dir=data_dir,
        configuration=Configuration(ping_uploader=_RecordingUploader(info_path)),
    )

    while not info_path.exists():
        time.sleep(0.1)

    with info_path.open("r") as fd:
        url_path = fd.readline()
        serialized_ping = fd.readline()

    assert "deletion-request" == url_path.split("/")[3]

    json_content = json.loads(serialized_ping)

    assert 0 == validate_ping.validate_ping(
        io.StringIO(serialized_ping),
        sys.stdout,
        schema_url=ping_schema_url,
    )

    assert not json_content["client_info"]["client_id"].startswith("c0ffee")


def test_no_sending_deletion_ping_if_unchanged_outside_of_run(safe_httpserver, tmpdir):
    safe_httpserver.serve_content(b"", code=200)
    Glean._reset()
    config = Configuration(server_endpoint=safe_httpserver.url)

    Glean.initialize(
        application_id=GLEAN_APP_ID,
        application_version=glean_version,
        upload_enabled=False,
        data_dir=Path(str(tmpdir)),
        configuration=config,
    )

    assert 0 == len(safe_httpserver.requests)

    Glean._reset()

    Glean.initialize(
        application_id=GLEAN_APP_ID,
        application_version=glean_version,
        upload_enabled=False,
        data_dir=Path(str(tmpdir)),
        configuration=config,
    )

    assert 0 == len(safe_httpserver.requests)


def test_dont_allow_multiprocessing(monkeypatch, safe_httpserver):
    safe_httpserver.serve_content(b"", code=200)

    Glean._configuration.server_endpoint = safe_httpserver.url
    Glean._configuration._allow_multiprocessing = False

    # Monkey-patch the multiprocessing API to be broken so we can assert it isn't used
    def broken_process(*args, **kwargs):
        assert False, "shouldn't be called"  # noqa

    monkeypatch.setattr(subprocess, "Popen", broken_process)

    custom_ping = PingType(
        name="store1", include_client_id=True, send_if_empty=True, reason_codes=[]
    )

    custom_ping.submit()

    process = PingUploadWorker._process()
    process.wait()
    assert process.returncode == 0

    assert 1 == len(safe_httpserver.requests)


def test_clear_application_lifetime_metrics(tmpdir):
    Glean._reset()

    Glean.initialize(
        application_id=GLEAN_APP_ID,
        application_version=glean_version,
        upload_enabled=True,
        data_dir=Path(str(tmpdir)),
    )

    counter_metric = CounterMetricType(
        CommonMetricData(
            disabled=False,
            category="test.telemetry",
            lifetime=Lifetime.APPLICATION,
            name="lifetime_reset",
            send_in_pings=["store1"],
            dynamic_label=None,
        )
    )

    # Additionally get metrics using the loader.
    metrics = load_metrics(ROOT / "data" / "core.yaml", config={"allow_reserved": True})

    counter_metric.add(10)
    metrics.core_ping.seq.add(10)

    assert counter_metric.test_get_value() == 10

    assert metrics.core_ping.seq.test_get_value() == 10

    Glean._reset()

    Glean.initialize(
        application_id=GLEAN_APP_ID,
        application_version=glean_version,
        upload_enabled=True,
        data_dir=Path(str(tmpdir)),
    )

    assert not counter_metric.test_get_value()
    assert not metrics.core_ping.seq.test_get_value()


def test_presubmit_makes_a_valid_ping(tmpdir, ping_schema_url, monkeypatch):
    # Bug 1648140: Submitting a ping prior to initialize meant that the core
    # metrics wouldn't yet be set.

    info_path = Path(str(tmpdir)) / "info.txt"

    # This test relies on testing mode to be disabled, since we need to prove the
    # real-world async behaviour of this.
    Glean._reset()

    ping_name = "preinit_ping"
    ping = PingType(
        name=ping_name, include_client_id=True, send_if_empty=True, reason_codes=[]
    )

    # Submit a ping prior to calling initialize
    ping.submit()
    Glean._initialize_with_tempdir_for_testing(
        application_id=GLEAN_APP_ID,
        application_version=glean_version,
        upload_enabled=True,
        configuration=Glean._configuration,
    )

    monkeypatch.setattr(
        Glean._configuration, "ping_uploader", _RecordingUploader(info_path)
    )

    while not info_path.exists():
        time.sleep(0.1)

    with info_path.open("r") as fd:
        url_path = fd.readline()
        serialized_ping = fd.readline()

    assert ping_name == url_path.split("/")[3]

    assert 0 == validate_ping.validate_ping(
        io.StringIO(serialized_ping),
        sys.stdout,
        schema_url=ping_schema_url,
    )


def test_app_display_version_unknown():
    from glean import _builtins

    Glean._reset()
    Glean._initialize_with_tempdir_for_testing(
        application_id=GLEAN_APP_ID,
        application_version=None,
        upload_enabled=True,
    )

    assert (
        "Unknown"
        == _builtins.metrics.glean.internal.metrics.app_display_version.test_get_value()
    )


def test_flipping_upload_enabled_respects_order_of_events(tmpdir, monkeypatch):
    # This test relies on testing mode to be disabled, since we need to prove the
    # real-world async behaviour of this.
    Glean._reset()

    info_path = Path(str(tmpdir)) / "info.txt"

    # We create a ping and a metric before we initialize Glean
    ping = PingType(
        name="sample_ping_1",
        include_client_id=True,
        send_if_empty=True,
        reason_codes=[],
    )

    configuration = Glean._configuration
    configuration.ping_uploader = _RecordingUploader(info_path)
    Glean._initialize_with_tempdir_for_testing(
        application_id=GLEAN_APP_ID,
        application_version=glean_version,
        upload_enabled=True,
        configuration=Glean._configuration,
    )

    # Glean might still be initializing. Disable upload.
    Glean.set_upload_enabled(False)
    # Submit a custom ping.
    ping.submit()

    url_path, payload = wait_for_ping(info_path, max_wait=20)

    # Validate we got the deletion-request ping
    assert "deletion-request" == url_path.split("/")[3]


def test_data_dir_is_required():
    Glean._reset()

    with pytest.raises(TypeError):
        Glean.initialize(
            application_id=GLEAN_APP_ID,
            application_version=glean_version,
            upload_enabled=True,
            configuration=Glean._configuration,
        )


def wait_for_ping(info_path, max_wait=10) -> (str, str):
    while not info_path.exists():
        time.sleep(0.1)
        max_wait -= 1
        if max_wait == 0:
            break

    if not info_path.exists():
        raise RuntimeError("No ping received.")

    with info_path.open("r") as fd:
        url_path = fd.readline()
        serialized_ping = fd.readline()
        payload = json.loads(serialized_ping)

    os.remove(info_path)
    return (url_path, payload)


def test_client_activity_api(tmpdir, monkeypatch):
    Glean._reset()

    info_path = Path(str(tmpdir)) / "info.txt"

    # This test relies on testing mode to be disabled, since we need to prove the
    # real-world async behaviour of this.

    configuration = Glean._configuration
    configuration.ping_uploader = _RecordingUploader(info_path)

    Glean._testing_mode = False
    glean_set_test_mode(False)
    Glean._initialize_with_tempdir_for_testing(
        application_id=GLEAN_APP_ID,
        application_version=glean_version,
        upload_enabled=True,
        configuration=Glean._configuration,
    )

    # Making it active
    Glean.handle_client_active()

    url_path, payload = wait_for_ping(info_path, max_wait=20)
    assert "baseline" == url_path.split("/")[3]
    assert payload["ping_info"]["reason"] == "active"
    assert "timespan" not in payload["metrics"]

    # The upload process is fast, but not fast enough to communicate its status.
    # We give it just a blink of an eye to wind down.
    time.sleep(0.1)
    # Making it inactive
    Glean.handle_client_inactive()

    url_path, payload = wait_for_ping(info_path, max_wait=20)
    assert "baseline" == url_path.split("/")[3]
    assert payload["ping_info"]["reason"] == "inactive"
    assert "glean.baseline.duration" in payload["metrics"]["timespan"]

    # The upload process is fast, but not fast enough to communicate its status.
    # We give it just a blink of an eye to wind down.
    time.sleep(0.1)
    # Once more active
    Glean.handle_client_active()

    url_path, payload = wait_for_ping(info_path, max_wait=20)
    assert "baseline" == url_path.split("/")[3]
    assert payload["ping_info"]["reason"] == "active"
    assert "timespan" not in payload["metrics"]


def test_sending_of_custom_pings(safe_httpserver):
    safe_httpserver.serve_content(b"", code=200)
    Glean._configuration.server_endpoint = safe_httpserver.url

    counter_metric = CounterMetricType(
        CommonMetricData(
            disabled=False,
            category="telemetry",
            lifetime=Lifetime.APPLICATION,
            name="counter_metric",
            send_in_pings=["store1"],
            dynamic_label=None,
        )
    )

    custom_ping = PingType(
        name="store1", include_client_id=True, send_if_empty=False, reason_codes=[]
    )

    counter_metric.add()

    # Need a mutable object and plain booleans are not.
    callback_was_called = [False]

    def check_custom_ping(reason):
        assert reason is None
        assert 1 == counter_metric.test_get_value()
        callback_was_called[0] = True

    custom_ping.test_before_next_submit(check_custom_ping)
    custom_ping.submit()

    assert callback_was_called[0]

    assert 1 == len(safe_httpserver.requests)
