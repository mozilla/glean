# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.


import pytest


from glean import metrics
from glean.metrics import Lifetime
from glean import testing


def test_the_api_saves_to_its_storage_engine_by_first_adding_then_setting():
    metric = metrics.StringListMetricType(
        disabled=False,
        category="telemetry",
        lifetime=Lifetime.APPLICATION,
        name="string_list_metric",
        send_in_pings=["store1"],
    )

    metric.add("value1")
    metric.add("value2")
    metric.add("value3")

    assert metric.test_has_value()
    snapshot = metric.test_get_value()
    assert ["value1", "value2", "value3"] == snapshot

    metric.set(["other1", "other2", "other3"])

    assert metric.test_has_value()
    snapshot2 = metric.test_get_value()
    assert ["other1", "other2", "other3"] == snapshot2


def test_the_api_saves_to_its_storage_engine_by_first_setting_then_adding():
    metric = metrics.StringListMetricType(
        disabled=False,
        category="telemetry",
        lifetime=Lifetime.APPLICATION,
        name="string_list_metric",
        send_in_pings=["store1"],
    )

    metric.set(["value1", "value2", "value3"])

    assert metric.test_has_value()
    snapshot = metric.test_get_value()
    assert ["value1", "value2", "value3"] == snapshot

    metric.add("added1")

    assert metric.test_has_value()
    snapshot = metric.test_get_value()
    assert ["value1", "value2", "value3", "added1"] == snapshot


def test_disabled_lists_must_not_record_data():
    metric = metrics.StringListMetricType(
        disabled=True,
        category="telemetry",
        lifetime=Lifetime.APPLICATION,
        name="string_list_metric",
        send_in_pings=["store1"],
    )

    metric.set(["value1", "value2", "value3"])

    assert not metric.test_has_value()

    metric.add("value4")

    assert not metric.test_has_value()


def test_test_get_value_throws():
    metric = metrics.StringListMetricType(
        disabled=False,
        category="telemetry",
        lifetime=Lifetime.APPLICATION,
        name="string_list_metric",
        send_in_pings=["store1"],
    )

    with pytest.raises(ValueError):
        metric.test_get_value()


def test_api_saves_to_secondary_pings():
    metric = metrics.StringListMetricType(
        disabled=False,
        category="telemetry",
        lifetime=Lifetime.APPLICATION,
        name="string_list_metric",
        send_in_pings=["store1", "store2"],
    )

    metric.add("value1")
    metric.add("value2")
    metric.add("value3")

    for store in ["store1", "store2"]:
        assert metric.test_has_value(store)
        assert ["value1", "value2", "value3"] == metric.test_get_value(store)

    metric.set(["other1", "other2", "other3"])

    for store in ["store1", "store2"]:
        assert metric.test_has_value(store)
        assert ["other1", "other2", "other3"] == metric.test_get_value(store)


def test_long_string_lists_are_truncated():
    metric = metrics.StringListMetricType(
        disabled=False,
        category="telemetry",
        lifetime=Lifetime.APPLICATION,
        name="string_list_metric",
        send_in_pings=["store1"],
    )

    for i in range(21):
        metric.add(f"value{i}")

    snapshot = metric.test_get_value()
    assert 20 == len(snapshot)

    assert 1 == metric.test_get_num_recorded_errors(testing.ErrorType.INVALID_VALUE)
