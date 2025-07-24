# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.


import json
import logging
import os
import socket
import time
from pathlib import Path

import pytest

import pytest_localserver.http

from glean import config
from glean import testing
from glean import __version__ as glean_version
from glean.metrics import PingType

# This defines the location of the JSON schema used to validate the pings
# created during unit testing. This uses the vendored schema.
#
# Use `bin/update-schema.sh latest` to update it to the latest upstream version.`
this_dir = Path(__file__)
# removing the file name and 3 layers of directories
GLEAN_PING_SCHEMA_PATH = (this_dir.parent.parent.parent.parent / "glean.1.schema.json").resolve()

# Turn on all logging when running the unit tests
logging.getLogger(None).setLevel(logging.INFO)


# This will be run before every test in the entire test suite
def pytest_runtest_setup(item):
    PingType(
        name="store1",
        include_client_id=True,
        send_if_empty=False,
        precise_timestamps=True,
        include_info_sections=True,
        schedules_pings=[],
        reason_codes=[],
        uploader_capabilities=[],
    )
    PingType(
        name="store2",
        include_client_id=True,
        send_if_empty=False,
        precise_timestamps=True,
        include_info_sections=True,
        schedules_pings=[],
        reason_codes=[],
        uploader_capabilities=[],
    )
    PingType(
        name="store3",
        include_client_id=True,
        send_if_empty=False,
        precise_timestamps=True,
        include_info_sections=True,
        schedules_pings=[],
        reason_codes=[],
        uploader_capabilities=[],
    )
    testing.reset_glean(application_id="glean-python-test", application_version=glean_version)


# The pytest-localserver server occasionally deadlocks.  This is a workaround posted at
#    https://bitbucket.org/pytest-dev/pytest-localserver/issues/19/flaky-test-failures-due-to-server-not
@pytest.fixture
def safe_httpserver(httpserver):
    wait_for_server(httpserver)
    return httpserver


@pytest.fixture
def ping_schema_url():
    return str(GLEAN_PING_SCHEMA_PATH)


def wait_for_server(httpserver, timeout=30):
    start_time = time.time()
    while True:
        try:
            sock = socket.create_connection(httpserver.server_address, timeout=0.1)
            sock.close()
            break
        except socket.error:
            if time.time() - start_time > timeout:
                raise TimeoutError()


class Helpers:
    @staticmethod
    def wait_for_ping(path, timeout=2) -> (str, str):
        """
        Wait for a ping to appear in `path` for at most `timeout` seconds.

        Raises a `TimeoutError` if the file doesn't exist within the timeout.

        Returns a tuple of (url path, payload).
        """

        start_time = time.time()
        while not path.exists():
            time.sleep(0.1)
            if time.time() - start_time > timeout:
                raise TimeoutError(f"No ping appeared in {path} within {timeout} seconds")

        with path.open("r") as fd:
            url_path = fd.readline()
            serialized_ping = fd.readline()
            payload = json.loads(serialized_ping)

        os.remove(path)
        return (url_path, payload)


@pytest.fixture
def helpers():
    return Helpers


# Setup a default webserver that pings will go to by default, so we don't hit
# the real telemetry endpoint from unit tests. Some tests that care about the
# pings actually being sent may still set up their own webservers using the
# `safe_httpserver` fixture that overrides this one. This is just to catch the
# pings from the majority of unit tests that don't care by default.
default_server = pytest_localserver.http.ContentServer()
default_server.daemon = True
default_server.start()
wait_for_server(default_server)
config.DEFAULT_TELEMETRY_ENDPOINT = default_server.url
