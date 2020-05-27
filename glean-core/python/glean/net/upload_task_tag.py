# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.

from enum import IntEnum


class UploadTaskTag(IntEnum):
    """
    An enumeration for the different upload tasks that the Glean uploader supports.

    This *MUST* have the same order as the variants in `glean-core/ffi/src/upload.rs`.
    """

    UPLOAD = 0
    """
    Ping data is available for upload
    """

    WAIT = 1
    """
    Caller needs to wait before requesting new data
    """

    DONE = 2
    """
    No data available
    """
