# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.


import locale
import sys
import time


def get_locale_tag() -> str:
    """
    Get a Gecko-compatible locale string (e.g. "es-ES", instead of the "es_ES")
    for the currently set locale.

    Returns:
        locale (str): The locale string.
    """
    value = locale.getlocale()[0]

    # The format of the locale string is platform depedent. At least on Linux,
    # often an understore is used between language and country, which is not
    # RFC 1766 compliant. Correct it here.
    value = value.replace("_", "-")

    return value


if sys.version_info >= (3, 7):

    def time_ms() -> int:
        """
        Get time from a monotonic timer in milliseconds.

        On Python prior to 3.7, this may have less than millisecond resolution.
        """
        return int(time.time_ns() / 1000)


else:

    def time_ms() -> int:
        """
        Get time from a monotonic timer in milliseconds.

        On Python prior to 3.7, this may have less than millisecond resolution.
        """
        return int(time.time() * 1000.0)
