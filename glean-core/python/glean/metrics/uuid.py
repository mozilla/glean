# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.


from typing import Optional, Union
import uuid


from .._uniffi import CommonMetricData
from .._uniffi import UuidMetric
from ..testing import ErrorType


class UuidMetricType:
    """
    This implements the developer facing API for recording UUID metrics.

    Instances of this class type are automatically generated by
    `glean.load_metrics`, allowing developers to record values that were
    previously registered in the metrics.yaml file.

    The UUID API exposes the `UuidMetricType.generate_and_set` and
    `UuidMetricType.set` methods.
    """

    def __init__(self, common_metric_data: CommonMetricData):
        self._inner = UuidMetric(common_metric_data)

    def generate_and_set(self) -> Optional[uuid.UUID]:
        """
        Generate a new UUID value and set it in the metric store.
        """
        id = self._inner.generate_and_set()
        return uuid.UUID("urn:uuid:" + id)

    def generate_once(self):
        """
        Generate a new random UUID if none is set yet.
        """
        self._inner.generate_once()

    def set(self, value: Union[uuid.UUID, str]) -> None:
        """
        Explicitly set an existing UUID value.

        Args:
            value (uuid.UUID): A valid UUID to set the metric to.
        """
        self._inner.set(str(value))

    def test_get_value(self, ping_name: Optional[str] = None) -> Optional[uuid.UUID]:
        id = self._inner.test_get_value()
        if id:
            return uuid.UUID("urn:uuid:" + id)
        else:
            return None

    def test_get_num_recorded_errors(self, error_type: ErrorType) -> int:
        return self._inner.test_get_num_recorded_errors(error_type)


__all__ = ["UuidMetricType"]
