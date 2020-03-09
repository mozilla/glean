# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.


import pytest


from glean import metrics
from glean.metrics import Lifetime, TimeUnit
from glean import testing
from glean import _util


def test_the_api_saves_to_its_storage_engine():
    metric = metrics.TimingDistributionMetricType(
        disabled=False,
        category="telemetry",
        lifetime=Lifetime.APPLICATION,
        name="timing_distribution",
        send_in_pings=["store1"],
        time_unit=TimeUnit.MILLISECOND,
    )

    override_time = 0

    def override_time_ns():
        return override_time

    original_time_ns = _util.time_ns
    _util.time_ns = override_time_ns

    try:
        for i in range(1, 4):
            override_time = 0
            timer_id = metric.start()
            override_time = i
            metric.stop_and_accumulate(timer_id)

    finally:
        _util.time_ns = original_time_ns

    assert metric.test_has_value()
    snapshot = metric.test_get_value()
    assert 6 == snapshot.sum
    assert {1: 1, 2: 1, 3: 1, 4: 0} == snapshot.values


def test_disabled_timing_distributions_must_not_record_data():
    metric = metrics.TimingDistributionMetricType(
        disabled=True,
        category="telemetry",
        lifetime=Lifetime.APPLICATION,
        name="timing_distribution",
        send_in_pings=["store1"],
        time_unit=TimeUnit.MILLISECOND,
    )

    timer_id = metric.start()
    metric.stop_and_accumulate(timer_id)

    assert not metric.test_has_value()


def test_get_value_throws():
    metric = metrics.TimingDistributionMetricType(
        disabled=False,
        category="telemetry",
        lifetime=Lifetime.APPLICATION,
        name="timing_distribution",
        send_in_pings=["store1"],
        time_unit=TimeUnit.MILLISECOND,
    )

    with pytest.raises(ValueError):
        metric.test_get_value()


def test_api_saves_to_secondary_pings():
    metric = metrics.TimingDistributionMetricType(
        disabled=False,
        category="telemetry",
        lifetime=Lifetime.APPLICATION,
        name="timing_distribution",
        send_in_pings=["store1", "store2", "store3"],
        time_unit=TimeUnit.MILLISECOND,
    )

    override_time = 0

    def override_time_ns():
        return override_time

    original_time_ns = _util.time_ns
    _util.time_ns = override_time_ns

    try:
        for i in range(1, 4):
            override_time = 0
            timer_id = metric.start()
            override_time = i
            metric.stop_and_accumulate(timer_id)

    finally:
        _util.time_ns = original_time_ns

    for store in ["store1", "store2", "store3"]:
        assert metric.test_has_value(store)
        snapshot = metric.test_get_value(store)
        assert 6 == snapshot.sum
        assert {1: 1, 2: 1, 3: 1, 4: 0} == snapshot.values


def test_stopping_a_non_existent_timer_records_an_error():
    metric = metrics.TimingDistributionMetricType(
        disabled=False,
        category="telemetry",
        lifetime=Lifetime.APPLICATION,
        name="timing_distribution",
        send_in_pings=["store1", "store2", "store3"],
        time_unit=TimeUnit.SECOND,
    )

    metric.stop_and_accumulate(-1)
    assert 1 == metric.test_get_num_recorded_errors(testing.ErrorType.INVALID_STATE)
