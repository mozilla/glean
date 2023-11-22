# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.


import locale


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


class classproperty:
    """
    Decorator for creating a property on a class (rather than an instance).
    """

    def __init__(self, f):
        self.f = f

    def __get__(self, obj, owner):
        return self.f(owner)
