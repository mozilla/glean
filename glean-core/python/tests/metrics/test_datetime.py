# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.

import datetime

from glean import metrics
from glean.metrics import Lifetime, CommonMetricData


def test_the_api_saves_to_its_storage_engine():
    # Define a datetime metric, which will be stored in "store1"
    datetime_metric = metrics.DatetimeMetricType(
        CommonMetricData(
            disabled=False,
            category="telemetry",
            lifetime=Lifetime.APPLICATION,
            name="datetime_metric",
            send_in_pings=["store1"],
            dynamic_label=None,
        ),
        time_unit=metrics.TimeUnit.MINUTE,
    )

    assert datetime_metric.test_get_value() is None

    # Regression test: We previously failed to record the timezone.
    # We don't have a _fixed_ local timezone,
    # but we know how to get it:
    expected_dt = datetime.datetime.now(datetime.timezone.utc).astimezone()
    expected_off = expected_dt.tzinfo.utcoffset(expected_dt).seconds

    datetime_metric.set()
    assert datetime_metric.test_get_value()
    value = datetime_metric.test_get_value()
    # Check that the set datetime contains the local timezone offset.
    assert expected_off == value.tzinfo.utcoffset(value).seconds

    value = datetime.datetime(
        2004, 12, 9, 8, 3, 29, tzinfo=datetime.timezone(datetime.timedelta(hours=16))
    )
    datetime_metric.set(value)
    assert "2004-12-09T08:03:00+16:00" == datetime_metric.test_get_value_as_str()
    assert value.replace(second=0) == datetime_metric.test_get_value()

    value2 = datetime.datetime(1993, 2, 23, 5, 43, tzinfo=datetime.timezone.utc)
    datetime_metric.set(value2)
    assert "1993-02-23T05:43:00+00:00" == datetime_metric.test_get_value_as_str()
    assert value2.replace(second=0) == datetime_metric.test_get_value()

    # A date prior to the UNIX epoch
    value3 = datetime.datetime(1969, 8, 20, 17, 3, tzinfo=datetime.timezone.utc)
    datetime_metric.set(value3)
    assert "1969-08-20T17:03:00+00:00" == datetime_metric.test_get_value_as_str()
    assert value3.replace(second=0) == datetime_metric.test_get_value()

    # A date following 2038 (the extent of 32-bits after the UNIX epoch
    value4 = datetime.datetime(2039, 7, 20, 20, 17, 3, tzinfo=datetime.timezone.utc)
    datetime_metric.set(value4)
    assert "2039-07-20T20:17:00+00:00" == datetime_metric.test_get_value_as_str()
    assert value4.replace(second=0) == datetime_metric.test_get_value()


def test_disabled_datetimes_must_not_record_data():
    datetime_metric = metrics.DatetimeMetricType(
        CommonMetricData(
            disabled=True,
            category="telemetry",
            lifetime=Lifetime.APPLICATION,
            name="datetime_metric",
            send_in_pings=["store1"],
            dynamic_label=None,
        ),
        time_unit=metrics.TimeUnit.MINUTE,
    )

    datetime_metric.set()
    assert datetime_metric.test_get_value() is None
