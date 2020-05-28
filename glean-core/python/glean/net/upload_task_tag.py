# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.

from enum import IntEnum


from .. import _ffi


class UploadTaskTag(IntEnum):
    """
    An enumeration for the different upload tasks that the Glean uploader supports.
    """

    UPLOAD = _ffi.lib.FfiPingUploadTask_Upload
    """
    Ping data is available for upload
    """

    WAIT = _ffi.lib.FfiPingUploadTask_Wait
    """
    Caller needs to wait before requesting new data
    """

    DONE = _ffi.lib.FfiPingUploadTask_Done
    """
    No data available
    """
