# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.

from glean import metrics
from glean.metrics import Lifetime, CommonMetricData
from glean import testing


def test_the_api_saves_to_its_storage_engine():
    # Define a quantity metric, which will be stored in "store1"
    quantity_metric = metrics.QuantityMetricType(
        CommonMetricData(
            disabled=False,
            category="telemetry",
            lifetime=Lifetime.APPLICATION,
            name="quantity_metric",
            send_in_pings=["store1"],
            dynamic_label=None,
        )
    )

    assert quantity_metric.test_get_value() is None

    quantity_metric.set(1)

    # Check that the metric was properly recorded
    assert 1 == quantity_metric.test_get_value()

    quantity_metric.set(10)

    # Check that the metric was properly overwritten
    assert 10 == quantity_metric.test_get_value()


def test_disabled_quantities_must_not_record_data():
    # Define a quantity metric, which will be stored in "store1"
    quantity_metric = metrics.QuantityMetricType(
        CommonMetricData(
            disabled=True,
            category="telemetry",
            lifetime=Lifetime.APPLICATION,
            name="quantity_metric",
            send_in_pings=["store1"],
            dynamic_label=None,
        )
    )

    # Attempt to increment the quantity
    quantity_metric.set(1)
    # Check that nothing was recorded
    assert quantity_metric.test_get_value() is None


def test_get_value_throws_value_error_if_nothing_is_stored():
    # Define a quantity metric, which will be stored in "store1"
    quantity_metric = metrics.QuantityMetricType(
        CommonMetricData(
            disabled=True,
            category="telemetry",
            lifetime=Lifetime.APPLICATION,
            name="quantity_metric",
            send_in_pings=["store1"],
            dynamic_label=None,
        )
    )

    assert not quantity_metric.test_get_value()


def test_api_saves_to_secondary_pings():
    # Define a quantity metric, which will be stored in "store1" and "store2"
    quantity_metric = metrics.QuantityMetricType(
        CommonMetricData(
            disabled=False,
            category="telemetry",
            lifetime=Lifetime.APPLICATION,
            name="quantity_metric",
            send_in_pings=["store1", "store2"],
            dynamic_label=None,
        )
    )

    quantity_metric.set(1)

    # Check that the metric was properly recorded on the second ping
    assert 1 == quantity_metric.test_get_value("store2")

    quantity_metric.set(10)

    # Check that the metric was properly overwritten on the second ping
    assert 10 == quantity_metric.test_get_value("store2")


def test_negative_values_are_not_counted():
    # Define a quantity metric, which will be stored in "store1"
    quantity_metric = metrics.QuantityMetricType(
        CommonMetricData(
            disabled=False,
            category="telemetry",
            lifetime=Lifetime.APPLICATION,
            name="quantity_metric",
            send_in_pings=["store1"],
            dynamic_label=None,
        )
    )

    quantity_metric.set(1)

    # Check that the metric was properly recorded
    assert 1 == quantity_metric.test_get_value("store1")

    quantity_metric.set(-10)

    # Check that the quantity was NOT recorded
    assert 1 == quantity_metric.test_get_value("store1")
    assert 1 == quantity_metric.test_get_num_recorded_errors(testing.ErrorType.INVALID_VALUE)
