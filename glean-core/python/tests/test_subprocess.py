# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.


def test_subprocess_works():
    import glean  # noqa: F401
    import subprocess
    import sys

    # Importing glean shouldn't affect subprocess.
    output = (
        subprocess.check_output([sys.executable, "-c", "print('hello')"])
        .decode()
        .strip()
    )
    assert output == "hello"
