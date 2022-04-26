# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.


import logging
import socket
import time
from pathlib import Path

import pytest

import pytest_localserver.http

from glean import config
from glean import testing
from glean import __version__ as glean_version

# This defines the location of the JSON schema used to validate the pings
# created during unit testing. This uses the vendored schema.
#
# Use `bin/update-schema.sh latest` to update it to the latest upstream version.`
this_dir = Path(__file__)
# removing the file name and 3 layers of directories
GLEAN_PING_SCHEMA_PATH = (
    this_dir.parent.parent.parent.parent / "glean.1.schema.json"
).resolve()

# Turn on all logging when running the unit tests
logging.getLogger(None).setLevel(logging.INFO)


# This will be run before every test in the entire test suite
def pytest_runtest_setup(item):
    testing.reset_glean(
        application_id="glean-python-test", application_version=glean_version
    )


# The pytest-localserver server occasionally deadlocks.  This is a workaround posted at
#    https://bitbucket.org/pytest-dev/pytest-localserver/issues/19/flaky-test-failures-due-to-server-not
@pytest.fixture
def safe_httpserver(httpserver):
    wait_for_server(httpserver)
    return httpserver


@pytest.fixture
def slow_httpserver(httpserver):
    """
    An httpserver that takes 0.5 seconds to respond.
    """
    wait_for_server(httpserver)

    orig_call = httpserver.__call__

    def __call__(self, *args, **kwargs):
        time.sleep(0.5)
        return orig_call(*args, **kwargs)

    httpserver.__call__ = __call__

    return httpserver


@pytest.fixture
def safe_httpsserver(httpsserver):
    wait_for_server(httpsserver)
    return httpsserver


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
