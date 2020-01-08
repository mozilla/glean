# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.


from pathlib import Path
from typing import Optional


from glean import Configuration
from .error_type import ErrorType  # noqa


def reset_glean(
    *,
    application_id: str,
    application_version: str,
    configuration: Optional[Configuration] = None,
    clear_stores: bool = True
):
    """
    Resets the Glean singleton.

    Args:
        application_id (str): The application id to use when sending pings.
        application_version (str): The version of the application sending
            Glean data.
        configuration (glean.config.Configuration): (optional) An object with
            global settings.
    """
    from glean import Glean

    data_dir = None  # type: Optional[Path]
    if not clear_stores:
        Glean._destroy_data_dir = False
        data_dir = Glean._data_dir

    Glean.reset()
    Glean.initialize(
        application_id=application_id,
        application_version=application_version,
        upload_enabled=True,
        configuration=configuration,
        data_dir=data_dir,
    )


__all__ = ["reset_glean", "ErrorType"]
