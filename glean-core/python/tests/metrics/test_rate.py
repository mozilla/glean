# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.

from pathlib import Path

from glean import metrics
from glean.metrics import Lifetime, CommonMetricData
from glean.testing import ErrorType


ROOT = Path(__file__).parent


def test_rate_smoke():
    metric = metrics.RateMetricType(
        CommonMetricData(
            category="test",
            name="rate",
            lifetime=Lifetime.PING,
            send_in_pings=["store1"],
            label=None,
            disabled=False,
        ),
    )

    # Adding 0 doesn't error.
    metric.add_to_numerator(0)
    metric.add_to_denominator(0)
    assert 0 == metric.test_get_num_recorded_errors(ErrorType.INVALID_VALUE)

    # Adding a negative value errors.
    metric.add_to_numerator(-1)
    metric.add_to_denominator(-1)
    assert 2 == metric.test_get_num_recorded_errors(ErrorType.INVALID_VALUE)

    # Getting the value returns 0s if that's all we have.
    value = metric.test_get_value()
    assert 0 == value.numerator
    assert 0 == value.denominator

    # And normal values of course work.
    metric.add_to_numerator(22)
    metric.add_to_denominator(7)

    value = metric.test_get_value()
    assert 22 == value.numerator
    assert 7 == value.denominator


def test_numerator_smoke():
    metric = metrics.NumeratorMetricType(
        CommonMetricData(
            category="test",
            name="numerator",
            lifetime=Lifetime.PING,
            send_in_pings=["store1"],
            label=None,
            disabled=False,
        ),
    )

    # Adding 0 doesn't error.
    metric.add_to_numerator(0)
    assert 0 == metric.test_get_num_recorded_errors(ErrorType.INVALID_VALUE)

    # Adding a negative value errors.
    metric.add_to_numerator(-1)
    assert 1 == metric.test_get_num_recorded_errors(ErrorType.INVALID_VALUE)

    # Getting the value returns 0s if that's all we have.
    value = metric.test_get_value()
    assert 0 == value.numerator
    assert 0 == value.denominator

    # And normal values of course work.
    metric.add_to_numerator(22)

    value = metric.test_get_value()
    assert 22 == value.numerator
    assert 0 == value.denominator


def test_denominator_smoke():
    meta1 = CommonMetricData(
        category="test",
        name="rate1",
        lifetime=Lifetime.PING,
        send_in_pings=["store1"],
        label=None,
        disabled=False,
    )

    meta2 = CommonMetricData(
        category="test",
        name="rate2",
        lifetime=Lifetime.PING,
        send_in_pings=["store1"],
        label=None,
        disabled=False,
    )

    # This acts like a normal counter.
    denom = metrics.DenominatorMetricType(
        CommonMetricData(
            category="test",
            name="counter",
            lifetime=Lifetime.PING,
            send_in_pings=["store1"],
            label=None,
            disabled=False,
        ),
        [meta1, meta2],
    )

    num1 = metrics.NumeratorMetricType(meta1)
    num2 = metrics.NumeratorMetricType(meta2)

    num1.add_to_numerator(3)
    num2.add_to_numerator(5)

    denom.add(7)

    # no errors
    assert 0 == num1.test_get_num_recorded_errors(ErrorType.INVALID_VALUE)
    assert 0 == num2.test_get_num_recorded_errors(ErrorType.INVALID_VALUE)

    # Getting the value returns what is stored
    value = num1.test_get_value()
    assert 3 == value.numerator
    assert 7 == value.denominator

    value = num2.test_get_value()
    assert 5 == value.numerator
    assert 7 == value.denominator
