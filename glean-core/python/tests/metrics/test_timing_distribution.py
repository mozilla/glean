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
    assert 3 == snapshot.count


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
        assert 3 == snapshot.count


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

    metric.stop_and_accumulate(TimerId(id=0))
    assert 1 == metric.test_get_num_recorded_errors(testing.ErrorType.INVALID_STATE)


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
        iteration_guard = 10**8  # this guard should be
        # about 100 times as long as the loop
        iterations = 0
        end_time = time.perf_counter() + 0.1
        while time.perf_counter() < end_time:
            if iterations > iteration_guard:
                raise Exception("Loop to accrue time exceeded guard")
            iterations += 1

    snapshot = metric.test_get_value()
    # more than 0.1s = 100ms = 10^8 nanoseconds
    # less than 0.2s = 200ms = 2*10^8 nanoseconds
    assert snapshot.sum > 10**8, "Measured value below minimum time"
    assert snapshot.sum < 2 * 10**8, "Measured value above maximum time"


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


def test_the_accumulate_apis_record_data():
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

    metric.accumulate_samples([1, 2, 3])

    snapshot = metric.test_get_value()
    assert 6 == snapshot.sum
    assert 3 == snapshot.count

    metric.accumulate_single_sample(4)

    snapshot = metric.test_get_value()
    assert 10 == snapshot.sum
    assert 4 == snapshot.count
