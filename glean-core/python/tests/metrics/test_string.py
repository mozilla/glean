# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.


from glean import metrics
from glean.metrics import Lifetime, CommonMetricData
from glean import testing


def test_the_api_saves_to_its_storage_engine():
    string_metric = metrics.StringMetricType(
        CommonMetricData(
            disabled=False,
            category="telemetry",
            lifetime=Lifetime.APPLICATION,
            name="string_metric",
            send_in_pings=["store1"],
            dynamic_label=None,
        )
    )

    string_metric.set("value")

    assert "value" == string_metric.test_get_value()

    string_metric.set("overriddenValue")

    assert "overriddenValue" == string_metric.test_get_value()


def test_disabled_strings_must_not_record_data():
    string_metric = metrics.StringMetricType(
        CommonMetricData(
            disabled=True,
            category="telemetry",
            lifetime=Lifetime.APPLICATION,
            name="string_metric",
            send_in_pings=["store1"],
            dynamic_label=None,
        )
    )

    string_metric.set("value")

    assert not string_metric.test_get_value()


def test_the_api_saves_to_secondary_pings():
    string_metric = metrics.StringMetricType(
        CommonMetricData(
            disabled=False,
            category="telemetry",
            lifetime=Lifetime.APPLICATION,
            name="string_metric",
            send_in_pings=["store1", "store2"],
            dynamic_label=None,
        )
    )

    string_metric.set("value")

    assert "value" == string_metric.test_get_value("store2")

    string_metric.set("overriddenValue")

    assert "overriddenValue" == string_metric.test_get_value("store2")


def test_setting_a_long_string_records_an_error():
    string_metric = metrics.StringMetricType(
        CommonMetricData(
            disabled=False,
            category="telemetry",
            lifetime=Lifetime.APPLICATION,
            name="string_metric",
            send_in_pings=["store1", "store2"],
            dynamic_label=None,
        )
    )

    string_metric.set("0123456789" * 11)

    assert 1 == string_metric.test_get_num_recorded_errors(
        testing.ErrorType.INVALID_OVERFLOW
    )


def test_setting_a_string_as_none():
    string_metric = metrics.StringMetricType(
        CommonMetricData(
            disabled=False,
            category="telemetry",
            lifetime=Lifetime.APPLICATION,
            name="string_metric",
            send_in_pings=["store1", "store2"],
            dynamic_label=None,
        )
    )

    string_metric.set(None)

    assert not string_metric.test_get_value()
