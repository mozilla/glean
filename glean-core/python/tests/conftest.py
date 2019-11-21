# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.


import logging
import socket
import time


import pytest


from glean import testing
from glean import __version__ as glean_version


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
def safe_httpsserver(httpsserver):
    wait_for_server(httpsserver)
    return httpsserver


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
