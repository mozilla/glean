# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.


import functools
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
    # getdefaultlocale() returns the default locale specified for a user on the
    # system, and isn't affected by the locale that may have been explicitly
    # set by the application. This is used primarily to have a cross-platform
    # way to get the locale in RFC 1766 format.
    value = locale.getdefaultlocale()[0]

    # In some contexts, especially on Windows, there is no locale set. Use "und"
    # to indicate "undetermined", as recommended by the Unicode TR35:
    # https://unicode.org/reports/tr35/#Unknown_or_Invalid_Identifiers
    if value is None:
        return "und"

    # The format of the locale string is platform depedent. At least on Linux,
    # often an understore is used between language and country, which is not
    # RFC 1766 compliant. Correct it here.
    value = value.replace("_", "-")

    return value


if sys.version_info >= (3, 7):

    time_ns = time.monotonic_ns


else:

    def time_ns() -> int:
        """
        Get time from a monotonic timer in nanoseconds.

        On Python prior to 3.7, this may have less than nanosecond resolution.
        """
        return int(time.monotonic() * 1000000000.0)


class classproperty:
    """
    Decorator for creating a property on a class (rather than an instance).
    """

    def __init__(self, f):
        self.f = f

    def __get__(self, obj, owner):
        return self.f(owner)


def record_error(self):
    """
    A decorator to put around functions that should catch exceptions and report
    them through the Glean error reporting system.

    Example:

       @record_error(self)
       def do_work():
           self.record_metric()
    """

    def callable(func):
        @functools.wraps(func)
        def wrapped(*args, **kwargs):
            from .testing import ErrorType

            try:
                return func(*args, **kwargs)
            except Exception as e:
                self._record_error(
                    ErrorType.UNEXPECTED_EXCEPTION,
                    f"{self.__class__.__name__} threw exception: {e}",
                    1,
                )
                raise

        return wrapped

    return callable
