# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.

from glean import metrics
from glean.metrics import Lifetime, CommonMetricData
from glean import testing


def test_the_api_saves_to_its_storage_engine_by_first_adding_then_setting():
    metric = metrics.StringListMetricType(
        CommonMetricData(
            disabled=False,
            category="telemetry",
            lifetime=Lifetime.APPLICATION,
            name="string_list_metric",
            send_in_pings=["store1"],
            dynamic_label=None,
        )
    )

    metric.add("value1")
    metric.add("value2")
    metric.add("value3")

    snapshot = metric.test_get_value()
    assert ["value1", "value2", "value3"] == snapshot

    metric.set(["other1", "other2", "other3"])

    snapshot2 = metric.test_get_value()
    assert ["other1", "other2", "other3"] == snapshot2


def test_the_api_saves_to_its_storage_engine_by_first_setting_then_adding():
    metric = metrics.StringListMetricType(
        CommonMetricData(
            disabled=False,
            category="telemetry",
            lifetime=Lifetime.APPLICATION,
            name="string_list_metric",
            send_in_pings=["store1"],
            dynamic_label=None,
        )
    )

    metric.set(["value1", "value2", "value3"])

    snapshot = metric.test_get_value()
    assert ["value1", "value2", "value3"] == snapshot

    metric.add("added1")

    snapshot = metric.test_get_value()
    assert ["value1", "value2", "value3", "added1"] == snapshot


def test_disabled_lists_must_not_record_data():
    metric = metrics.StringListMetricType(
        CommonMetricData(
            disabled=True,
            category="telemetry",
            lifetime=Lifetime.APPLICATION,
            name="string_list_metric",
            send_in_pings=["store1"],
            dynamic_label=None,
        )
    )

    metric.set(["value1", "value2", "value3"])

    assert not metric.test_get_value()

    metric.add("value4")

    assert not metric.test_get_value()


def test_test_get_value_throws():
    metric = metrics.StringListMetricType(
        CommonMetricData(
            disabled=False,
            category="telemetry",
            lifetime=Lifetime.APPLICATION,
            name="string_list_metric",
            send_in_pings=["store1"],
            dynamic_label=None,
        )
    )

    assert not metric.test_get_value()


def test_api_saves_to_secondary_pings():
    metric = metrics.StringListMetricType(
        CommonMetricData(
            disabled=False,
            category="telemetry",
            lifetime=Lifetime.APPLICATION,
            name="string_list_metric",
            send_in_pings=["store1", "store2"],
            dynamic_label=None,
        )
    )

    metric.add("value1")
    metric.add("value2")
    metric.add("value3")

    for store in ["store1", "store2"]:
        assert ["value1", "value2", "value3"] == metric.test_get_value(store)

    metric.set(["other1", "other2", "other3"])

    for store in ["store1", "store2"]:
        assert ["other1", "other2", "other3"] == metric.test_get_value(store)


def test_long_string_lists_are_truncated():
    metric = metrics.StringListMetricType(
        CommonMetricData(
            disabled=False,
            category="telemetry",
            lifetime=Lifetime.APPLICATION,
            name="string_list_metric",
            send_in_pings=["store1"],
            dynamic_label=None,
        )
    )

    for i in range(21):
        metric.add(f"value{i}")

    snapshot = metric.test_get_value()
    assert 20 == len(snapshot)

    assert 1 == metric.test_get_num_recorded_errors(testing.ErrorType.INVALID_VALUE)
