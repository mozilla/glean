# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.


import abc
from typing import Callable, List, Optional, Set, Type


from ..glean import Glean
from .. import _ffi
from .counter import CounterMetricType
from .lifetime import Lifetime
from ..testing import ErrorType


class LabeledMetricBase(abc.ABC):
    """
    This implements the developer-facing API for labeled metrics.

    Instances of this class type are automatically generated by
    `glean.load_metrics`, allowing developers to record values that were
    previously registered in the metrics.yaml file.

    Unlike most metric types, LabeledMetricType does not have its own
    corresponding storage, but records metrics for the underlying metric type T
    in the storage for that type. The only difference is that labeled metrics
    are stored with the special key `$category.$name/$label`. The collect
    method knows how to pull these special values back out of the individual
    metric storage and rearrange them correctly in the ping.
    """

    # The following 4 class attributes must be overridden by classes
    # inheriting from LabeledMetricBase:

    # The class of the concrete metric type
    _submetric_type = None  # type: Type

    # The FFI function to instantiate the labeled metric type
    _metric_type_instantiator = None  # type: Callable

    # The FFI function to get a concrete metric type from the labeled metric type
    _submetric_type_instantiator = None  # type: Callable

    # The FFI function for test_get_num_recorded_errors
    _test_get_num_recorded_errors_ffi = None  # type: Callable

    def __init__(
        self,
        disabled: bool,
        category: str,
        lifetime: Lifetime,
        name: str,
        send_in_pings: List[str],
        labels: Optional[Set[str]] = None,
    ):
        self._disabled = disabled
        self._send_in_pings = send_in_pings

        if labels is not None:
            label_list = _ffi.ffi_encode_vec_string(list(labels))
            label_len = len(labels)
        else:
            label_list = _ffi.ffi.NULL
            label_len = 0

        self._handle = self._metric_type_instantiator(
            _ffi.ffi_encode_string(category),
            _ffi.ffi_encode_string(name),
            _ffi.ffi_encode_vec_string(send_in_pings),
            len(send_in_pings),
            lifetime.value,
            disabled,
            label_list,
            label_len,
        )

    def __getitem__(self, item: str):
        """
        Get the specific metric for a given label.

        If a set of acceptable labels were specified in the metrics.yaml file,
        and the given label is not in the set, it will be recorded under the
        special `__other__`.

        If a set of acceptable labels was not specified in the metrics.yaml
        file, only the first 16 unique labels will be used. After that, any
        additional labels will be recorded under the special `__other__` label.

        Labels must be snake_case and less than 30 characters. If an invalid
        label is used, the metric will be recorded in the special `__other__`
        label.
        """
        handle = self._submetric_type_instantiator(
            self._handle, _ffi.ffi_encode_string(item)
        )
        metric = self._submetric_type.__new__(self._submetric_type)  # type: ignore
        metric._handle = handle
        metric._disabled = self._disabled
        metric._send_in_pings = self._send_in_pings
        return metric

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

        return self._test_get_num_recorded_errors_ffi(
            Glean._handle,
            self._handle,
            error_type.value,
            _ffi.ffi_encode_string(ping_name),
        )


class LabeledCounterMetricType(LabeledMetricBase):
    _submetric_type = CounterMetricType
    _metric_type_instantiator = _ffi.lib.glean_new_labeled_counter_metric
    _submetric_type_instantiator = _ffi.lib.glean_labeled_counter_metric_get
    _test_get_num_recorded_errors_ffi = (
        _ffi.lib.glean_labeled_counter_test_get_num_recorded_errors
    )


__all__ = ["LabeledCounterMetricType"]
