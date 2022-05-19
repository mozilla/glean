# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.


import pytest
import time


from glean import metrics
from glean.metrics import Lifetime, TimeUnit, TimerId, CommonMetricData
from glean import testing


def test_the_api_saves_to_its_storage_engine():
    metric = metrics.TimingDistributionMetricType(
        CommonMetricData(
            disabled=False,
            category="telemetry",
            lifetime=Lifetime.APPLICATION,
            name="timing_distribution",
            send_in_pings=["store1"],
            dynamic_label=None,
        ),
        time_unit=TimeUnit.NANOSECOND,
    )

    for _ in range(1, 4):
        timer_id = metric.start()
        metric.stop_and_accumulate(timer_id)

    snapshot = metric.test_get_value()
    assert 0 < snapshot.sum

    count = sum([v for v in snapshot.values.values()])
    assert 3 == count


def test_disabled_timing_distributions_must_not_record_data():
    metric = metrics.TimingDistributionMetricType(
        CommonMetricData(
            disabled=True,
            category="telemetry",
            lifetime=Lifetime.APPLICATION,
            name="timing_distribution",
            send_in_pings=["store1"],
            dynamic_label=None,
        ),
        time_unit=TimeUnit.NANOSECOND,
    )

    timer_id = metric.start()
    metric.stop_and_accumulate(timer_id)

    assert not metric.test_get_value()


def test_get_value_throws():
    metric = metrics.TimingDistributionMetricType(
        CommonMetricData(
            disabled=False,
            category="telemetry",
            lifetime=Lifetime.APPLICATION,
            name="timing_distribution",
            send_in_pings=["store1"],
            dynamic_label=None,
        ),
        time_unit=TimeUnit.NANOSECOND,
    )

    assert not metric.test_get_value()


def test_api_saves_to_secondary_pings():
    metric = metrics.TimingDistributionMetricType(
        CommonMetricData(
            disabled=False,
            category="telemetry",
            lifetime=Lifetime.APPLICATION,
            name="timing_distribution",
            send_in_pings=["store1", "store2", "store3"],
            dynamic_label=None,
        ),
        time_unit=TimeUnit.NANOSECOND,
    )

    for _ in range(1, 4):
        timer_id = metric.start()
        metric.stop_and_accumulate(timer_id)

    for store in ["store1", "store2", "store3"]:
        snapshot = metric.test_get_value(store)
        assert 0 < snapshot.sum
        count = sum([v for v in snapshot.values.values()])
        assert 3 == count


def test_stopping_a_non_existent_timer_records_an_error():
    metric = metrics.TimingDistributionMetricType(
        CommonMetricData(
            disabled=False,
            category="telemetry",
            lifetime=Lifetime.APPLICATION,
            name="timing_distribution",
            send_in_pings=["store1", "store2", "store3"],
            dynamic_label=None,
        ),
        time_unit=TimeUnit.NANOSECOND,
    )

    metric.stop_and_accumulate(TimerId(0))
    assert 1 == metric.test_get_num_recorded_errors(testing.ErrorType.INVALID_STATE)


# Doesn't really test anything anymore
@pytest.mark.skip
def test_time_unit_controls_truncation():
    max_sample_time = 1000 * 1000 * 1000 * 60 * 10

    for unit in [TimeUnit.NANOSECOND, TimeUnit.MICROSECOND, TimeUnit.MILLISECOND]:
        metric = metrics.TimingDistributionMetricType(
            CommonMetricData(
                disabled=False,
                category="telemetry",
                lifetime=Lifetime.APPLICATION,
                name=f"timing_distribution_{unit.name}",
                send_in_pings=["baseline"],
                dynamic_label=None,
            ),
            time_unit=unit,
        )

        for _value in [
            1,
            100,
            100000,
            max_sample_time,
            max_sample_time * 1000,
            max_sample_time * 1000000,
        ]:
            timer_id = metric.start()
            metric.stop_and_accumulate(timer_id)

        snapshot = metric.test_get_value()
        assert len(snapshot.values) < 318


def test_measure():
    """
    Test the TimingDistributionMetricType.measure context manager.
    """
    metric = metrics.TimingDistributionMetricType(
        CommonMetricData(
            disabled=False,
            category="telemetry",
            lifetime=Lifetime.APPLICATION,
            name="timing_distribution",
            send_in_pings=["baseline"],
            dynamic_label=None,
        ),
        time_unit=TimeUnit.NANOSECOND,
    )

    with metric.measure():
        time.sleep(0.1)

    snapshot = metric.test_get_value()
    # more than 0.1s = 100ms = 10^8 nanoseconds
    # less than 0.2s = 200ms = 2*10^8 nanoseconds
    assert 10**8 < snapshot.sum
    assert 2 * 10**8 > snapshot.sum


def test_measure_exception():
    """
    Test the TimingDistributionMetricType.measure context manager.
    """
    metric = metrics.TimingDistributionMetricType(
        CommonMetricData(
            disabled=False,
            category="telemetry",
            lifetime=Lifetime.APPLICATION,
            name="timing_distribution",
            send_in_pings=["baseline"],
            dynamic_label=None,
        ),
        time_unit=TimeUnit.NANOSECOND,
    )

    with pytest.raises(ValueError):
        with metric.measure():
            raise ValueError()

    assert not metric.test_get_value()
