# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.


import datetime


import pytest


from glean import metrics
from glean.metrics import Lifetime


def test_the_api_saves_to_its_storage_engine():
    # Define a datetime metric, which will be stored in "store1"
    datetime_metric = metrics.DatetimeMetricType(
        disabled=False,
        category="telemetry",
        lifetime=Lifetime.APPLICATION,
        name="datetime_metric",
        send_in_pings=["store1"],
        time_unit=metrics.TimeUnit.MINUTE,
    )

    assert datetime_metric.test_has_value() is False

    datetime_metric.set()
    assert datetime_metric.test_has_value() is True

    value = datetime.datetime(
        2004, 12, 9, 8, 3, 29, tzinfo=datetime.timezone(datetime.timedelta(hours=16))
    )
    datetime_metric.set(value)
    assert datetime_metric.test_has_value() is True
    assert "2004-12-09T08:03+16:00" == datetime_metric.test_get_value_as_str()
    assert value.replace(second=0) == datetime_metric.test_get_value()

    value2 = datetime.datetime(1993, 2, 23, 5, 43, tzinfo=datetime.timezone.utc)
    datetime_metric.set(value2)
    assert datetime_metric.test_has_value() is True
    assert "1993-02-23T05:43+00:00" == datetime_metric.test_get_value_as_str()
    assert value2.replace(second=0) == datetime_metric.test_get_value()

    # A date prior to the UNIX epoch
    value3 = datetime.datetime(1969, 8, 20, 17, 3, tzinfo=datetime.timezone.utc)
    datetime_metric.set(value3)
    assert datetime_metric.test_has_value() is True
    assert "1969-08-20T17:03+00:00" == datetime_metric.test_get_value_as_str()
    assert value3.replace(second=0) == datetime_metric.test_get_value()

    # A date following 2038 (the extent of 32-bits after the UNIX epoch
    value4 = datetime.datetime(2039, 7, 20, 20, 17, 3, tzinfo=datetime.timezone.utc)
    datetime_metric.set(value4)
    assert datetime_metric.test_has_value() is True
    assert "2039-07-20T20:17+00:00" == datetime_metric.test_get_value_as_str()
    assert value4.replace(second=0) == datetime_metric.test_get_value()


def test_disabled_datetimes_must_not_record_data():
    datetime_metric = metrics.DatetimeMetricType(
        disabled=True,
        category="telemetry",
        lifetime=Lifetime.APPLICATION,
        name="datetime_metric",
        send_in_pings=["store1"],
        time_unit=metrics.TimeUnit.MINUTE,
    )

    datetime_metric.set()
    assert False is datetime_metric.test_has_value()

    with pytest.raises(ValueError):
        datetime_metric.test_get_value()
