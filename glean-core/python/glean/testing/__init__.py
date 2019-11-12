# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.


from typing import Optional


from glean import Configuration
from .error_type import ErrorType  # noqa


def reset_glean(
    application_id: str,
    application_version: str,
    configuration: Optional[Configuration] = None,
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

    Glean.reset()
    Glean.initialize(application_id, application_version, configuration)


__all__ = ["reset_glean", "ErrorType"]
