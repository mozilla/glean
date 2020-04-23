# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.


import uuid


from glean import Glean
from glean.net import PingUploadWorker
from glean.net.http_client import HttpClientUploader


def test_invalid_filename():
    pings_dir = PingUploadWorker.storage_directory()
    pings_dir.mkdir()

    with (pings_dir / "ping").open("wb") as fd:
        fd.write(b"\n")

    assert PingUploadWorker._test_process_sync()

    assert 0 == len(list(pings_dir.iterdir()))


def test_invalid_content():
    pings_dir = PingUploadWorker.storage_directory()
    pings_dir.mkdir()

    with (pings_dir / str(uuid.uuid4())).open("wb") as fd:
        fd.write(b"\n")

    assert not PingUploadWorker.process()

    assert 0 == len(list(pings_dir.iterdir()))


def test_400_error(safe_httpserver):
    safe_httpserver.serve_content(b"", code=400)
    Glean._configuration.server_endpoint = safe_httpserver.url

    pings_dir = PingUploadWorker.storage_directory()
    pings_dir.mkdir()

    with (pings_dir / str(uuid.uuid4())).open("wb") as fd:
        fd.write(b"/data/path/\n")
        fd.write(b"{}\n")

    assert PingUploadWorker._test_process_sync()

    assert 0 == len(list(pings_dir.iterdir()))

    assert 1 == len(safe_httpserver.requests)


def test_500_error(safe_httpserver):
    safe_httpserver.serve_content(b"", code=500)
    Glean._configuration.server_endpoint = safe_httpserver.url

    pings_dir = PingUploadWorker.storage_directory()
    pings_dir.mkdir()

    with (pings_dir / str(uuid.uuid4())).open("wb") as fd:
        fd.write(b"/data/path/\n")
        fd.write(b"{}\n")

    assert not PingUploadWorker._test_process_sync()

    assert 1 == len(list(pings_dir.iterdir()))

    assert 1 == len(safe_httpserver.requests)


def test_unknown_scheme():
    Glean._configuration.server_endpoint = "ftp://example.com/"

    pings_dir = PingUploadWorker.storage_directory()
    pings_dir.mkdir()

    with (pings_dir / str(uuid.uuid4())).open("wb") as fd:
        fd.write(b"/data/path/\n")
        fd.write(b"{}\n")

    assert False is PingUploadWorker._test_process_sync()


def test_ping_upload_worker_single_process(safe_httpserver):
    safe_httpserver.serve_content(b"", code=200)
    Glean._configuration.server_endpoint = safe_httpserver.url

    pings_dir = PingUploadWorker.storage_directory()
    pings_dir.mkdir()

    for i in range(100):
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

    assert 100 == len(safe_httpserver.requests)


def test_unknown_url_no_exception():
    # Shouldn't leak any socket or HTTPExceptions
    assert not HttpClientUploader.upload("http://nowhere.example.com", "{}", [])
