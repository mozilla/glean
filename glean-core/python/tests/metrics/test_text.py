# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.

from pathlib import Path

from glean import metrics
from glean.metrics import Lifetime, CommonMetricData
from glean.testing import ErrorType


ROOT = Path(__file__).parent


def test_text_smoke():
    metric = metrics.TextMetricType(
        CommonMetricData(
            category="test",
            name="text",
            lifetime=Lifetime.PING,
            send_in_pings=["store1"],
            dynamic_label=None,
            disabled=False,
        ),
    )

    metric.set("hello world")
    assert 0 == metric.test_get_num_recorded_errors(ErrorType.INVALID_OVERFLOW)

    assert "hello world" == metric.test_get_value()


def test_text_truncation():
    metric = metrics.TextMetricType(
        CommonMetricData(
            category="test",
            name="text",
            lifetime=Lifetime.PING,
            send_in_pings=["store1"],
            dynamic_label=None,
            disabled=False,
        ),
    )

    test_string = "01234567890" * (200 * 1024)
    metric.set(test_string)

    assert test_string[: (200 * 1024)] == metric.test_get_value()
    assert 1 == metric.test_get_num_recorded_errors(ErrorType.INVALID_OVERFLOW)
