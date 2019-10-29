# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.


import glean


def test_smoke_test(tmpdir):
    """
    A very simple smoke test.
    """
    glean.Glean.initialize(glean.Configuration(), tmpdir)
    assert glean.Glean._handle != 0
