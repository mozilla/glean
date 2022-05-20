# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.

from glean import metrics
from glean.metrics import Lifetime, CommonMetricData
from glean import testing


def test_the_api_saves_to_its_storage_engine():
    # Define a counter metric, which will be stored in "store1"
    counter_metric = metrics.CounterMetricType(
        CommonMetricData(
            disabled=False,
            category="telemetry",
            lifetime=Lifetime.APPLICATION,
            name="counter_metric",
            send_in_pings=["store1"],
            dynamic_label=None,
        )
    )

    assert counter_metric.test_get_value() is None

    counter_metric.add()

    # Check that the count was incremented and properly recorded
    assert 1 == counter_metric.test_get_value()

    counter_metric.add(10)

    # Check that the count was incremented and properly recorded. This second
    # call will check calling add() with 10 to test increment by other amount.
    assert 11 == counter_metric.test_get_value()


def test_disabled_counters_must_not_record_data():
    # Define a counter metric, which will be stored in "store1"
    counter_metric = metrics.CounterMetricType(
        CommonMetricData(
            disabled=True,
            category="telemetry",
            lifetime=Lifetime.APPLICATION,
            name="counter_metric",
            send_in_pings=["store1"],
            dynamic_label=None,
        )
    )

    # Attempt to increment the counter
    counter_metric.add(1)
    # Check that nothing was recorded
    assert counter_metric.test_get_value() is None


def test_get_value_throws_value_error_if_nothing_is_stored():
    # Define a counter metric, which will be stored in "store1"
    counter_metric = metrics.CounterMetricType(
        CommonMetricData(
            disabled=True,
            category="telemetry",
            lifetime=Lifetime.APPLICATION,
            name="counter_metric",
            send_in_pings=["store1"],
            dynamic_label=None,
        )
    )

    assert counter_metric.test_get_value() is None


def test_api_saves_to_secondary_pings():
    # Define a counter metric, which will be stored in "store1" and "store2"
    counter_metric = metrics.CounterMetricType(
        CommonMetricData(
            disabled=False,
            category="telemetry",
            lifetime=Lifetime.APPLICATION,
            name="counter_metric",
            send_in_pings=["store1", "store2"],
            dynamic_label=None,
        )
    )

    counter_metric.add()

    # Check that the count was incremented and properly recorded for the second
    # ping
    assert 1 == counter_metric.test_get_value("store2")

    counter_metric.add(10)

    # Check that the count was incremented and properly recorded for the second
    # ping
    assert 11 == counter_metric.test_get_value("store2")


def test_negative_values_are_not_counted():
    # Define a counter metric, which will be stored in "store1"
    counter_metric = metrics.CounterMetricType(
        CommonMetricData(
            disabled=False,
            category="telemetry",
            lifetime=Lifetime.APPLICATION,
            name="counter_metric",
            send_in_pings=["store1"],
            dynamic_label=None,
        )
    )

    counter_metric.add()

    # Check that the counter was incremented
    assert 1 == counter_metric.test_get_value("store1")

    counter_metric.add(-10)

    # Check that the counter was not incremented
    assert 1 == counter_metric.test_get_value("store1")
    assert 1 == counter_metric.test_get_num_recorded_errors(
        testing.ErrorType.INVALID_VALUE
    )
