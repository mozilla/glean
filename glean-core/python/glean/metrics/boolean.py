# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.


from typing import List, Optional


from .. import _ffi
from .._dispatcher import Dispatcher
from ..testing import ErrorType


from .lifetime import Lifetime


class BooleanMetricType:
    """
    This implements the developer facing API for recording boolean metrics.

    Instances of this class type are automatically generated by
    `glean.load_metrics`, allowing developers to record values that were
    previously registered in the metrics.yaml file.

    The boolean API only exposes the `BooleanMetricType.set` method
    """

    def __init__(
        self,
        disabled: bool,
        category: str,
        lifetime: Lifetime,
        name: str,
        send_in_pings: List[str],
    ):
        self._disabled = disabled
        self._send_in_pings = send_in_pings

        self._handle = _ffi.lib.glean_new_boolean_metric(
            _ffi.ffi_encode_string(category),
            _ffi.ffi_encode_string(name),
            _ffi.ffi_encode_vec_string(send_in_pings),
            len(send_in_pings),
            lifetime.value,
            disabled,
        )

    def __del__(self):
        if getattr(self, "_handle", 0) != 0:
            _ffi.lib.glean_destroy_boolean_metric(self._handle)

    def set(self, value: bool):
        """
        Set a boolean value.

        Args:
            value (bool): This is a user-defined boolean value.
        """
        if self._disabled:
            return

        @Dispatcher.launch
        def set():
            _ffi.lib.glean_boolean_set(self._handle, value)

    def test_has_value(self, ping_name: Optional[str] = None) -> bool:
        """
        Tests whether a value is stored for the metric for testing purposes
        only.

        Args:
            ping_name (str): (default: first value in send_in_pings) The name
                of the ping to retrieve the metric for.

        Returns:
            has_value (bool): True if the metric value exists.
        """
        if ping_name is None:
            ping_name = self._send_in_pings[0]

        return bool(
            _ffi.lib.glean_boolean_test_has_value(
                self._handle, _ffi.ffi_encode_string(ping_name)
            )
        )

    def test_get_value(self, ping_name: Optional[str] = None) -> bool:
        """
        Returns the stored value for testing purposes only.

        Args:
            ping_name (str): (default: first value in send_in_pings) The name
                of the ping to retrieve the metric for.

        Returns:
            value (bool): value of the stored metric.
        """
        if ping_name is None:
            ping_name = self._send_in_pings[0]

        if not self.test_has_value(ping_name):
            raise ValueError("metric has no value")

        return bool(
            _ffi.lib.glean_boolean_test_get_value(
                self._handle, _ffi.ffi_encode_string(ping_name)
            )
        )


__all__ = ["BooleanMetricType"]
