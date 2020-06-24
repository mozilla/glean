# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.


import uuid


from glean import Glean
from glean.net import PingUploadWorker
from glean.net.http_client import HttpClientUploader
from glean.net import ping_uploader


def test_400_error(safe_httpserver):
    safe_httpserver.serve_content(b"", code=400)

    response = HttpClientUploader.upload(
        url=safe_httpserver.url, data=b"{}", headers=[]
    )

    assert type(response) is ping_uploader.HttpResponse
    assert 400 == response._status_code
    assert 1 == len(safe_httpserver.requests)


def test_500_error(safe_httpserver):
    safe_httpserver.serve_content(b"", code=500)

    response = HttpClientUploader.upload(
        url=safe_httpserver.url, data=b"{}", headers=[]
    )

    assert type(response) is ping_uploader.HttpResponse
    assert 500 == response._status_code
    assert 1 == len(safe_httpserver.requests)


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

    for i in range(10):
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
