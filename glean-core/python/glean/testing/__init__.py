# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.


from .error_type import ErrorType  # noqa


def reset_glean():
    """
    Resets the Glean singleton.
    """
    from pathlib import Path
    import shutil
    import tempfile

    from glean import Configuration, Glean

    if Glean._handle != 0:
        shutil.rmtree(Glean._data_dir)
        Glean._handle = 0

    tmpdir = Path(tempfile.TemporaryDirectory().name)
    Glean.initialize(Configuration(), tmpdir)


__all__ = ["reset_glean", "ErrorType"]
