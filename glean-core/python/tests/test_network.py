# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.


import uuid


from glean import Glean
from glean import metrics
from glean._process_dispatcher import ProcessDispatcher
from glean.net import PingUploadWorker
from glean.net.http_client import HttpClientUploader
from glean.net import ping_uploader


def get_upload_failure_metric():
    return metrics.LabeledCounterMetricType(
        disabled=False,
        send_in_pings=["metrics"],
        name="ping_upload_failure",
        category="glean.upload",
        labels=[
            "status_code_4xx",
            "status_code_5xx",
            "status_code_unknown",
            "unrecoverable",
            "recoverable",
        ],
        lifetime=metrics.Lifetime.PING,
    )


def test_400_error(safe_httpserver):
    safe_httpserver.serve_content(b"", code=400)

    response = HttpClientUploader.upload(
        url=safe_httpserver.url, data=b"{}", headers=[]
    )

    assert type(response) is ping_uploader.HttpResponse
    assert 400 == response._status_code
    assert 1 == len(safe_httpserver.requests)


def test_400_error_submit(safe_httpserver, monkeypatch):
    safe_httpserver.serve_content(b"", code=400)

    # Force the ping upload worker into a separate process
    monkeypatch.setattr(PingUploadWorker, "process", PingUploadWorker._process)
    Glean._configuration._server_endpoint = safe_httpserver.url
    Glean._submit_ping_by_name("baseline")
    ProcessDispatcher._wait_for_last_process()

    assert 1 == len(safe_httpserver.requests)

    metric = get_upload_failure_metric()
    assert 1 == metric["status_code_4xx"].test_get_value()
    assert not metric["status_code_5xx"].test_has_value()


def test_500_error(safe_httpserver):
    safe_httpserver.serve_content(b"", code=500)

    response = HttpClientUploader.upload(
        url=safe_httpserver.url, data=b"{}", headers=[]
    )

    assert type(response) is ping_uploader.HttpResponse
    assert 500 == response._status_code
    assert 1 == len(safe_httpserver.requests)


def test_500_error_submit(safe_httpserver, monkeypatch):
    safe_httpserver.serve_content(b"", code=500)

    # Force the ping upload worker into a separate process
    monkeypatch.setattr(PingUploadWorker, "process", PingUploadWorker._process)
    Glean._configuration._server_endpoint = safe_httpserver.url
    Glean._submit_ping_by_name("baseline")
    ProcessDispatcher._wait_for_last_process()

    # This kind of recoverable error will be tried 10 times
    # The number of retries is defined on glean-core
    assert 3 == len(safe_httpserver.requests)

    metric = get_upload_failure_metric()
    assert not metric["status_code_4xx"].test_has_value()
    assert 3 == metric["status_code_5xx"].test_get_value()


def test_500_error_submit_concurrent_writing(slow_httpserver, monkeypatch):
    # This tests that concurrently writing to the database from the main process
    # and the ping uploading subprocess.
    slow_httpserver.serve_content(b"", code=500)

    counter = metrics.CounterMetricType(
        disabled=False,
        category="test",
        name="counter",
        send_in_pings=["metrics"],
        lifetime=metrics.Lifetime.PING,
    )

    # Force the ping upload worker into a separate process
    monkeypatch.setattr(PingUploadWorker, "process", PingUploadWorker._process)
    Glean._configuration._server_endpoint = slow_httpserver.url
    Glean._submit_ping_by_name("baseline")

    # While the uploader is running, increment the counter as fast as we can
    times = 0
    last_process = ProcessDispatcher._last_process
    while last_process.poll() is None:
        counter.add()
        times += 1

    # This kind of recoverable error will be tried 3 times
    # The number of retries is defined on glean-core
    assert 3 == len(slow_httpserver.requests)

    metric = get_upload_failure_metric()
    assert not metric["status_code_4xx"].test_has_value()
    assert 3 == metric["status_code_5xx"].test_get_value()

    assert times > 0
    assert times == counter.test_get_value()


def test_unknown_scheme():
    response = HttpClientUploader.upload(
        url="ftp://example.com/", data=b"{}", headers=[]
    )

    assert type(response) is ping_uploader.UnrecoverableFailure


def test_ping_upload_worker_single_process(safe_httpserver):
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

    # Fire off two PingUploadWorker processing tasks at the same time. If
    # working correctly, p1 should finish entirely before p2 starts.
    # If these actually run in parallel, one or the other will try to send
    # deleted queued ping files and fail.
    p1 = PingUploadWorker._process()
    p2 = PingUploadWorker._process()

    p1.wait()
    assert p1.returncode == 0

    p2.wait()
    assert p2.returncode == 0

    assert 10 == len(safe_httpserver.requests)


def test_unknown_url_no_exception():
    # Shouldn't leak any socket or HTTPExceptions
    response = HttpClientUploader.upload(
        url="http://nowhere.example.com", data=b"{}", headers=[]
    )

    assert type(response) is ping_uploader.RecoverableFailure
