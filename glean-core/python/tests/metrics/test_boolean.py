# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.

from glean import metrics
from glean.metrics import Lifetime, CommonMetricData


def test_the_api_saves_to_its_storage_engine():
    boolean_metric = metrics.BooleanMetricType(
        CommonMetricData(
            disabled=False,
            category="telemetry",
            lifetime=Lifetime.APPLICATION,
            name="boolean_metric",
            send_in_pings=["store1"],
            dynamic_label=None,
        )
    )

    boolean_metric.set(True)

    assert True is boolean_metric.test_get_value()

    boolean_metric.set(False)

    assert False is boolean_metric.test_get_value()


def test_disabled_booleans_must_not_record_data():
    boolean_metric = metrics.BooleanMetricType(
        CommonMetricData(
            disabled=True,
            category="telemetry",
            lifetime=Lifetime.APPLICATION,
            name="boolean_metric",
            send_in_pings=["store1"],
            dynamic_label=None,
        )
    )

    boolean_metric.set(True)

    assert not boolean_metric.test_get_value()


def test_get_value_throws_if_nothing_is_stored():
    boolean_metric = metrics.BooleanMetricType(
        CommonMetricData(
            disabled=True,
            category="telemetry",
            lifetime=Lifetime.APPLICATION,
            name="boolean_metric",
            send_in_pings=["store1"],
            dynamic_label=None,
        )
    )

    assert not boolean_metric.test_get_value()


def test_the_api_saves_to_secondary_pings():
    boolean_metric = metrics.BooleanMetricType(
        CommonMetricData(
            disabled=False,
            category="telemetry",
            lifetime=Lifetime.APPLICATION,
            name="boolean_metric",
            send_in_pings=["store1", "store2"],
            dynamic_label=None,
        )
    )

    boolean_metric.set(True)

    assert True is boolean_metric.test_get_value("store2")

    boolean_metric.set(False)

    assert False is boolean_metric.test_get_value("store2")
