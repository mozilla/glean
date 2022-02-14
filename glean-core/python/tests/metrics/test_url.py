# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.


from glean import metrics
from glean.metrics import Lifetime, CommonMetricData
from glean import testing


def test_the_api_saves_to_its_storage_engine():
    url_metric = metrics.UrlMetricType(
        CommonMetricData(
            disabled=False,
            category="telemetry",
            lifetime=Lifetime.APPLICATION,
            name="url_metric",
            send_in_pings=["store1"],
            dynamic_label=None,
        )
    )

    url_metric.set("glean://testing")

    assert "glean://testing" == url_metric.test_get_value()

    url_metric.set("glean://overriddenValue")

    assert "glean://overriddenValue" == url_metric.test_get_value()


def test_disabled_urls_must_not_record_data():
    url_metric = metrics.UrlMetricType(
        CommonMetricData(
            disabled=True,
            category="telemetry",
            lifetime=Lifetime.APPLICATION,
            name="url_metric",
            send_in_pings=["store1"],
            dynamic_label=None,
        )
    )

    url_metric.set("glean://testing")

    assert not url_metric.test_get_value()


def test_the_api_saves_to_secondary_pings():
    url_metric = metrics.UrlMetricType(
        CommonMetricData(
            disabled=False,
            category="telemetry",
            lifetime=Lifetime.APPLICATION,
            name="url_metric",
            send_in_pings=["store1", "store2"],
            dynamic_label=None,
        )
    )

    url_metric.set("glean://value")

    assert "glean://value" == url_metric.test_get_value("store2")

    url_metric.set("glean://overriddenValue")

    assert "glean://overriddenValue" == url_metric.test_get_value("store2")


def test_setting_a_long_url_records_an_error():
    url_metric = metrics.UrlMetricType(
        CommonMetricData(
            disabled=False,
            category="telemetry",
            lifetime=Lifetime.APPLICATION,
            name="url_metric",
            send_in_pings=["store1", "store2"],
            dynamic_label=None,
        )
    )

    url_metric.set("glean://" + "testing" * 2000)

    assert 1 == url_metric.test_get_num_recorded_errors(
        testing.ErrorType.INVALID_OVERFLOW
    )


def test_setting_a_url_as_none():
    url_metric = metrics.UrlMetricType(
        CommonMetricData(
            disabled=False,
            category="telemetry",
            lifetime=Lifetime.APPLICATION,
            name="url_metric",
            send_in_pings=["store1", "store2"],
            dynamic_label=None,
        )
    )

    url_metric.set(None)

    assert not url_metric.test_get_value()
