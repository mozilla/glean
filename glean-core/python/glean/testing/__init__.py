# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.


from .error_type import ErrorType  # noqa


def reset_glean():
    """
    Resets the Glean singleton.
    """
    from glean import Configuration, Glean
    from glean import _dispatcher

    Glean.reset()
    _dispatcher.Dispatcher = _dispatcher.DispatcherInternal()
    Glean.initialize(Configuration())


__all__ = ["reset_glean", "ErrorType"]
