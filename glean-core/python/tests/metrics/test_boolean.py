# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.


import pytest


from glean import metrics
from glean.metrics import Lifetime


def test_the_api_saves_to_its_storage_engine():
    boolean_metric = metrics.BooleanMetricType(
        disabled=False,
        category="telemetry",
        lifetime=Lifetime.APPLICATION,
        name="boolean_metric",
        send_in_pings=["store1"],
    )

    boolean_metric.set(True)

    assert boolean_metric.test_has_value()
    assert True is boolean_metric.test_get_value()

    boolean_metric.set(False)

    assert boolean_metric.test_has_value()
    assert False is boolean_metric.test_get_value()


def test_disabled_booleans_must_not_record_data():
    boolean_metric = metrics.BooleanMetricType(
        disabled=True,
        category="telemetry",
        lifetime=Lifetime.APPLICATION,
        name="boolean_metric",
        send_in_pings=["store1"],
    )

    boolean_metric.set(True)

    assert not boolean_metric.test_has_value()


def test_get_value_throws_if_nothing_is_stored():
    boolean_metric = metrics.BooleanMetricType(
        disabled=True,
        category="telemetry",
        lifetime=Lifetime.APPLICATION,
        name="boolean_metric",
        send_in_pings=["store1"],
    )

    with pytest.raises(ValueError):
        boolean_metric.test_get_value()


def test_the_api_saves_to_secondary_pings():
    boolean_metric = metrics.BooleanMetricType(
        disabled=False,
        category="telemetry",
        lifetime=Lifetime.APPLICATION,
        name="boolean_metric",
        send_in_pings=["store1", "store2"],
    )

    boolean_metric.set(True)

    assert boolean_metric.test_has_value("store2")
    assert True is boolean_metric.test_get_value("store2")

    boolean_metric.set(False)

    assert boolean_metric.test_has_value("store2")
    assert False is boolean_metric.test_get_value("store2")
