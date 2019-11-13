# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.


import logging


from glean import testing
from glean import __version__ as glean_version


# Turn on all logging when running the unit tests
logging.getLogger(None).setLevel(logging.INFO)


# This will be run before every test in the entire test suite
def pytest_runtest_setup(item):
    testing.reset_glean("glean-python-test", glean_version)
