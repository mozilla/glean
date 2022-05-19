# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.


import logging
from pathlib import Path
import uuid
import time


from glean import Glean
from glean import _builtins
from glean import metrics
from glean.metrics import CounterMetricType, Lifetime, CommonMetricData
from glean._process_dispatcher import ProcessDispatcher
from glean.net import PingUploadWorker
from glean.net.http_client import HttpClientUploader
from glean.net import ping_uploader
from glean import __version__ as glean_version


GLEAN_APP_ID = "glean-python-test"


def get_upload_failure_metric():
    return metrics.LabeledCounterMetricType(
        CommonMetricData(
            disabled=False,
            send_in_pings=["metrics"],
            name="ping_upload_failure",
            category="glean.upload",
            lifetime=metrics.Lifetime.PING,
            dynamic_label=None,
        ),
        labels=[
            "status_code_4xx",
            "status_code_5xx",
            "status_code_unknown",
            "unrecoverable",
            "recoverable",
        ],
    )


def test_recording_upload_errors_doesnt_clobber_database(
    tmpdir, safe_httpserver, monkeypatch
):
    """
    Test that running the ping uploader subprocess doesn't clobber the
    database. If, under some bug, the subprocess had "upload_enabled" set to
    True, it could record upload errors in the database, clobbering any metrics
    that might have meanwhile been recorded in the main process.

    This test is known to fail if "upload_enabled" is set to `True` in the
    subprocess.
    """
    tmpdir = Path(tmpdir)

    Glean._reset()
    Glean.initialize(
        application_id=GLEAN_APP_ID,
        application_version=glean_version,
        upload_enabled=True,
        data_dir=tmpdir,
    )

    counter_metric = CounterMetricType(
        CommonMetricData(
            disabled=False,
            category="telemetry",
            lifetime=Lifetime.PING,
            name="counter_metric",
            send_in_pings=["baseline"],
            dynamic_label=None,
        )
    )
    counter_metric.add(10)

    safe_httpserver.serve_content(b"", code=400)

    # Force the ping upload worker into a separate process
    monkeypatch.setattr(PingUploadWorker, "process", PingUploadWorker._process)
    Glean._configuration._server_endpoint = safe_httpserver.url
    _builtins.pings.baseline.submit()
    # `Ping.submit()` is async on the Rust dispatcher.
    # We briefly wait to give it a chance to trigger.
    time.sleep(0.5)
    ProcessDispatcher._wait_for_last_process()

    assert 1 == len(safe_httpserver.requests)

    # Force a reload of the database from disk
    Glean._reset()
    Glean.initialize(
        application_id=GLEAN_APP_ID,
        application_version=glean_version,
        upload_enabled=True,
        data_dir=tmpdir,
    )

    metric = get_upload_failure_metric()
    assert not metric["status_code_4xx"].test_get_value()


def test_400_error(safe_httpserver):
    safe_httpserver.serve_content(b"", code=400)

    response = HttpClientUploader.upload(
        url=safe_httpserver.url, data=b"{}", headers={}
    )

    assert isinstance(response, ping_uploader.UploadResult.HTTP_STATUS)
    assert 400 == response.code
    assert 1 == len(safe_httpserver.requests)


def test_500_error(safe_httpserver):
    safe_httpserver.serve_content(b"", code=500)

    response = HttpClientUploader.upload(
        url=safe_httpserver.url, data=b"{}", headers={}
    )

    assert isinstance(response, ping_uploader.UploadResult.HTTP_STATUS)
    assert 500 == response.code
    assert 1 == len(safe_httpserver.requests)


def test_unknown_scheme():
    response = HttpClientUploader.upload(
        url="ftp://example.com/", data=b"{}", headers={}
    )

    assert isinstance(response, ping_uploader.UploadResult.UNRECOVERABLE_FAILURE)


def test_ping_upload_worker_single_process(safe_httpserver):
    safe_httpserver.serve_content(b"", code=200)
    Glean._configuration.server_endpoint = safe_httpserver.url

    # This test requires us to write a few files in the pending pings
    # directory, to which language bindings have theoretically no access.
    # Manually create the path to that directory, at the risk of breaking
    # the test in the future, if that changes in the Rust code.
    pings_dir = Glean._data_dir / "pending_pings"
    pings_dir.mkdir()

    for _ in range(5):
        with (pings_dir / str(uuid.uuid4())).open("wb") as fd:
            fd.write(b"/data/path/\n")
            fd.write(b"{}\n")

    # Fire off the first PingUploaderWorker process to handle the existing
    # pings. Then, in parallel, write more pings and fire off "new"
    # PingUploadWorker processes (some of which will be no-ops and just keeping
    # the existing worker running).
    p1 = PingUploadWorker._process()

    processes = []
    for _ in range(5):
        with (pings_dir / str(uuid.uuid4())).open("wb") as fd:
            fd.write(b"/data/path/\n")
            fd.write(b"{}\n")

        processes.append(PingUploadWorker._process())

    p1.wait()
    assert p1.returncode == 0

    for p in processes:
        p.wait()
        assert p.returncode == 0

    assert 10 == len(safe_httpserver.requests)


def test_unknown_url_no_exception():
    # Shouldn't leak any socket or HTTPExceptions
    response = HttpClientUploader.upload(
        url="http://nowhere.example.com", data=b"{}", headers={}
    )

    assert isinstance(response, ping_uploader.UploadResult.RECOVERABLE_FAILURE)


def test_log_on_success(safe_httpserver, capfd):
    # We can't use caplog to catch log messages from the subprocess, but we can
    # use capsys to catch its stderr.

    safe_httpserver.serve_content(b"", code=200)
    Glean._configuration.server_endpoint = safe_httpserver.url
    Glean._simple_log_level = logging.INFO

    pings_dir = Glean._data_dir / "pending_pings"
    pings_dir.mkdir()

    with (pings_dir / str(uuid.uuid4())).open("wb") as fd:
        fd.write(b"/data/path/\n")
        fd.write(b"{}\n")

    p1 = PingUploadWorker._process()
    p1.wait()
    assert p1.returncode == 0

    assert 1 == len(safe_httpserver.requests)

    captured = capfd.readouterr()

    assert "successfully sent 200" in str(captured.err)
