# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.


import pytest


from glean import metrics
from glean.metrics import Lifetime, TimeUnit
from glean import testing
from glean import _util


def test_the_api_saves_to_its_storage_engine(monkeypatch):
    metric = metrics.TimingDistributionMetricType(
        disabled=False,
        category="telemetry",
        lifetime=Lifetime.APPLICATION,
        name="timing_distribution",
        send_in_pings=["store1"],
        time_unit=TimeUnit.NANOSECOND,
    )

    override_time = 0

    def override_time_ns():
        return override_time

    monkeypatch.setattr(_util, "time_ns", override_time_ns)

    for i in range(1, 4):
        override_time = 0
        timer_id = metric.start()
        override_time = i
        metric.stop_and_accumulate(timer_id)

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
        time_unit=TimeUnit.NANOSECOND,
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
        time_unit=TimeUnit.NANOSECOND,
    )

    with pytest.raises(ValueError):
        metric.test_get_value()


def test_api_saves_to_secondary_pings(monkeypatch):
    metric = metrics.TimingDistributionMetricType(
        disabled=False,
        category="telemetry",
        lifetime=Lifetime.APPLICATION,
        name="timing_distribution",
        send_in_pings=["store1", "store2", "store3"],
        time_unit=TimeUnit.NANOSECOND,
    )

    override_time = 0

    def override_time_ns():
        return override_time

    monkeypatch.setattr(_util, "time_ns", override_time_ns)

    for i in range(1, 4):
        override_time = 0
        timer_id = metric.start()
        override_time = i
        metric.stop_and_accumulate(timer_id)

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
        time_unit=TimeUnit.NANOSECOND,
    )

    metric.stop_and_accumulate(-1)
    assert 1 == metric.test_get_num_recorded_errors(testing.ErrorType.INVALID_STATE)


def test_time_unit_controls_truncation(monkeypatch):
    max_sample_time = 1000 * 1000 * 1000 * 60 * 10

    override_time = 0

    def override_time_ns():
        return override_time

    monkeypatch.setattr(_util, "time_ns", override_time_ns)

    for unit in [TimeUnit.NANOSECOND, TimeUnit.MICROSECOND, TimeUnit.MILLISECOND]:
        metric = metrics.TimingDistributionMetricType(
            disabled=False,
            category="telemetry",
            lifetime=Lifetime.APPLICATION,
            name=f"timing_distribution_{unit.name}",
            send_in_pings=["baseline"],
            time_unit=unit,
        )

        for value in [
            1,
            100,
            100000,
            max_sample_time,
            max_sample_time * 1000,
            max_sample_time * 1000000,
        ]:
            override_time = 0
            timer_id = metric.start()
            override_time = value
            metric.stop_and_accumulate(timer_id)

        snapshot = metric.test_get_value()
        assert len(snapshot.values) < 318


def test_measure(monkeypatch):
    """
    Test the TimingDistributionMetricType.measure context manager.
    """
    override_time = 0

    def override_time_ns():
        return override_time

    monkeypatch.setattr(_util, "time_ns", override_time_ns)

    metric = metrics.TimingDistributionMetricType(
        disabled=False,
        category="telemetry",
        lifetime=Lifetime.APPLICATION,
        name="timing_distribution",
        send_in_pings=["baseline"],
        time_unit=TimeUnit.NANOSECOND,
    )

    with metric.measure():
        # Move the "virtual timer" forward
        override_time = 1000

    snapshot = metric.test_get_value()
    assert snapshot.sum == 1000


def test_measure_exception(monkeypatch):
    """
    Test the TimingDistributionMetricType.measure context manager.
    """
    override_time = 0

    def override_time_ns():
        return override_time

    monkeypatch.setattr(_util, "time_ns", override_time_ns)

    metric = metrics.TimingDistributionMetricType(
        disabled=False,
        category="telemetry",
        lifetime=Lifetime.APPLICATION,
        name="timing_distribution",
        send_in_pings=["baseline"],
        time_unit=TimeUnit.NANOSECOND,
    )

    with pytest.raises(ValueError):
        with metric.measure():
            raise ValueError()

    assert not metric.test_has_value()
