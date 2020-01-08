# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.


import uuid


import pytest


from glean import Glean
from glean.net import PingUploadWorker


def test_invalid_filename():
    pings_dir = PingUploadWorker.storage_directory()
    pings_dir.mkdir()

    with (pings_dir / "ping").open("wb") as fd:
        fd.write(b"\n")

    assert PingUploadWorker.process()

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

    assert PingUploadWorker.process()

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

    assert not PingUploadWorker.process()

    assert 1 == len(list(pings_dir.iterdir()))

    assert 1 == len(safe_httpserver.requests)


def test_unknown_scheme():
    Glean._configuration.server_endpoint = "ftp://example.com/"

    pings_dir = PingUploadWorker.storage_directory()
    pings_dir.mkdir()

    with (pings_dir / str(uuid.uuid4())).open("wb") as fd:
        fd.write(b"/data/path/\n")
        fd.write(b"{}\n")

    with pytest.raises(ValueError):
        PingUploadWorker.process()
