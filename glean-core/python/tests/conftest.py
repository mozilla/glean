# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.


import logging
import socket
import time


import pytest


from glean import testing
from glean import __version__ as glean_version

# !IMPORTANT!
# Everytime this hash is changed it should also be changed in
# glean-core/android/build.gradle
GLEAN_PING_SCHEMA_GIT_HASH = "63dcb4285b73c0c625cbee46cf1fe506b7f4c5f6"
GLEAN_PING_SCHEMA_URL = (
    "https://raw.githubusercontent.com/mozilla-services/"
    "mozilla-pipeline-schemas/{}/schemas/glean/glean/"
    "glean.1.schema.json"
).format(GLEAN_PING_SCHEMA_GIT_HASH)

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


@pytest.fixture
def ping_schema_url():
    return GLEAN_PING_SCHEMA_URL


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
