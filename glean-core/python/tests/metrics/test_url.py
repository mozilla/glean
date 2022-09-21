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

    # Whenever the URL is longer than our MAX_URL_LENGTH, we truncate the URL to the
    # MAX_URL_LENGTH.
    #
    # This 8-character string was chosen so we could have an even number that is
    # a divisor of our MAX_URL_LENGTH.
    long_path_base = "abcdefgh"

    # Using 2000 creates a string > 16000 characters, well over MAX_URL_LENGTH.
    test_url = "glean://" + (long_path_base * 2000)
    url_metric.set(test_url)

    # "glean://" is 8 characters
    # "abcdefgh" (long_path_base) is 8 characters
    # `long_path_base` is repeated 1023 times (8184)
    # 8 + 8184 = 8192 (MAX_URL_LENGTH)
    expected = "glean://" + (long_path_base * 1023)

    assert expected == url_metric.test_get_value()
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
