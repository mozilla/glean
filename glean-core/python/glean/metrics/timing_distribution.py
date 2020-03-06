# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.


from typing import List, Optional


from .. import _ffi
from .._dispatcher import Dispatcher
from ..testing import ErrorType
from .. import util


from .distribution_data import DistributionData
from .lifetime import Lifetime
from .timeunit import TimeUnit


class TimingDistributionMetricType:
    """
    This implements the developer facing API for recording timingdistribution metrics.

    Instances of this class type are automatically generated by
    `glean.load_metrics`, allowing developers to record values that were
    previously registered in the metrics.yaml file.
    """

    def __init__(
        self,
        disabled: bool,
        category: str,
        lifetime: Lifetime,
        name: str,
        send_in_pings: List[str],
        time_unit: TimeUnit,
    ):
        self._disabled = disabled
        self._send_in_pings = send_in_pings

        self._handle = _ffi.lib.glean_new_timing_distribution_metric(
            _ffi.ffi_encode_string(category),
            _ffi.ffi_encode_string(name),
            _ffi.ffi_encode_vec_string(send_in_pings),
            len(send_in_pings),
            lifetime.value,
            disabled,
            time_unit.value,
        )

    def __del__(self):
        if getattr(self, "_handle", 0) != 0:
            _ffi.lib.glean_destroy_timing_distribution_metric(self._handle)

    def start(self) -> Optional[int]:
        """
        Start tracking time. This records an error if it’s already tracking
        time (i.e. start was already called with no corresponding
        `stop_and_ccumulate`): in that case the original start time will be
        preserved.

        Returns:
            timer_id: The object to associate with this timing.
        """
        if self._disabled:
            return None

        # The Rust code for `start` runs async and we need to use the same
        # clock for start and stop. Therefore we take the time on the Python
        # side.
        start_time = util.time_ns()

        # No dispatcher, we need the return value
        return _ffi.lib.glean_timing_distribution_set_start(self._handle, start_time)

    def stop_and_accumulate(self, timer_id: Optional[int]) -> None:
        """
        Stop tracking time for the provided metric and associated timer id. Add a
        count to the corresponding bucket in the timing distribution.
        This will record an error if no `start` was called.

        Args:
            timer_id: The timer id associated with this timing. This allows for
                concurrent timing of events associated with different ids to
                the same timespan metric.
        """
        # `start` may have returned None.
        # Accepting that means users of this API don't need to do a None check.
        if self._disabled or timer_id is None:
            return

        # The Rust code runs async and might be delayed. We need the time as
        # precisely as possible. We also need the same clock for start and stop
        # (`start` takes the time on the Python side).
        stop_time = util.time_ns()

        @Dispatcher.launch
        def stop():
            _ffi.lib.glean_timing_distribution_set_stop_and_accumulate(
                self._handle, timer_id, stop_time
            )

    def cancel(self, timer_id: Optional[int]) -> None:
        """
        Abort a previous `start` call. No error is recorded if no `start` was called.

        Args:
            timer_id: The timer id associated with this timing. This allows for
                concurrent timing of events associated with different ids to
                the same timing distribution metric.
        """
        # `start` may have returned None.
        # Accepting that means users of this API don't need to do a None check.
        if self._disabled or timer_id is None:
            return

        @Dispatcher.launch
        def cancel():
            _ffi.lib.glean_timing_distribution_cancel(self._handle, timer_id)

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
            _ffi.lib.glean_timing_distribution_test_has_value(
                self._handle, _ffi.ffi_encode_string(ping_name)
            )
        )

    def test_get_value(self, ping_name: Optional[str] = None) -> DistributionData:
        """
        Returns the stored value for testing purposes only.

        Args:
            ping_name (str): (default: first value in send_in_pings) The name
                of the ping to retrieve the metric for.

        Returns:
            value (DistriubutionData): value of the stored metric.
        """
        if ping_name is None:
            ping_name = self._send_in_pings[0]

        if not self.test_has_value(ping_name):
            raise ValueError("metric has no value")

        return DistributionData.from_json_string(
            _ffi.ffi_decode_string(
                _ffi.lib.glean_timing_distribution_test_get_value_as_json_string(
                    self._handle, _ffi.ffi_encode_string(ping_name)
                )
            )
        )

    def test_get_num_recorded_errors(
        self, error_type: ErrorType, ping_name: Optional[str] = None
    ) -> int:
        """
        Returns the number of errors recorded for the given metric.

        Args:
            error_type (ErrorType): The type of error recorded.
            ping_name (str): (default: first value in send_in_pings) The name
                of the ping to retrieve the metric for.

        Returns:
            num_errors (int): The number of errors recorded for the metric for
                the given error type.
        """
        if ping_name is None:
            ping_name = self._send_in_pings[0]

        return _ffi.lib.glean_timing_distribution_get_num_recorded_errors(
            self._handle, error_type.value, _ffi.ffi_encode_string(ping_name),
        )


__all__ = ["TimingDistributionMetricType"]
