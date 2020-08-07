# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.


import time


import pytest


from glean import metrics
from glean.metrics import Lifetime, TimeUnit
from glean import testing


def test_the_api_saves_to_its_storage_engine():
    timespan_metric = metrics.TimespanMetricType(
        disabled=False,
        category="telemetry",
        lifetime=Lifetime.APPLICATION,
        name="timespan_metric",
        send_in_pings=["store1"],
        time_unit=TimeUnit.MILLISECOND,
    )

    timespan_metric.start()
    timespan_metric.stop()

    assert timespan_metric.test_has_value()
    assert timespan_metric.test_get_value() >= 0


def test_disabled_timespans_must_not_record_data():
    timespan_metric = metrics.TimespanMetricType(
        disabled=True,
        category="telemetry",
        lifetime=Lifetime.APPLICATION,
        name="timespan_metric",
        send_in_pings=["store1"],
        time_unit=TimeUnit.MILLISECOND,
    )

    timespan_metric.start()
    timespan_metric.stop()

    assert not timespan_metric.test_has_value()


def test_the_api_must_correctly_cancel():
    timespan_metric = metrics.TimespanMetricType(
        disabled=False,
        category="telemetry",
        lifetime=Lifetime.APPLICATION,
        name="timespan_metric",
        send_in_pings=["store1"],
        time_unit=TimeUnit.MILLISECOND,
    )

    timespan_metric.start()
    timespan_metric.cancel()
    timespan_metric.stop()

    assert not timespan_metric.test_has_value()
    assert 1 == timespan_metric.test_get_num_recorded_errors(
        testing.ErrorType.INVALID_STATE
    )


def test_get_value_throws_if_nothing_is_stored():
    timespan_metric = metrics.TimespanMetricType(
        disabled=True,
        category="telemetry",
        lifetime=Lifetime.APPLICATION,
        name="timespan_metric",
        send_in_pings=["store1"],
        time_unit=TimeUnit.MILLISECOND,
    )

    with pytest.raises(ValueError):
        timespan_metric.test_get_value()


def test_the_api_saves_to_secondary_pings():
    timespan_metric = metrics.TimespanMetricType(
        disabled=False,
        category="telemetry",
        lifetime=Lifetime.APPLICATION,
        name="timespan_metric",
        send_in_pings=["store1", "store2"],
        time_unit=TimeUnit.MILLISECOND,
    )

    timespan_metric.start()
    timespan_metric.stop()

    assert timespan_metric.test_has_value("store2")
    assert timespan_metric.test_get_value("store2") >= 0


def test_records_an_error_if_started_twice():
    timespan_metric = metrics.TimespanMetricType(
        disabled=False,
        category="telemetry",
        lifetime=Lifetime.APPLICATION,
        name="timespan_metric",
        send_in_pings=["store1", "store2"],
        time_unit=TimeUnit.MILLISECOND,
    )

    timespan_metric.start()
    timespan_metric.start()
    timespan_metric.stop()

    assert timespan_metric.test_has_value("store2")
    assert timespan_metric.test_get_value("store2") >= 0
    assert 1 == timespan_metric.test_get_num_recorded_errors(
        testing.ErrorType.INVALID_STATE
    )


def test_value_unchanged_if_stopped_twice():
    timespan_metric = metrics.TimespanMetricType(
        disabled=False,
        category="telemetry",
        lifetime=Lifetime.APPLICATION,
        name="timespan_metric",
        send_in_pings=["store1"],
        time_unit=TimeUnit.MILLISECOND,
    )

    timespan_metric.start()
    timespan_metric.stop()

    assert timespan_metric.test_has_value()
    value = timespan_metric.test_get_value()
    assert value >= 0

    timespan_metric.stop()

    assert value == timespan_metric.test_get_value()


def test_set_raw_nanos():
    timespan_nanos = 6 * 1000000000

    timespan_metric = metrics.TimespanMetricType(
        disabled=False,
        category="telemetry",
        lifetime=Lifetime.APPLICATION,
        name="timespan_metric",
        send_in_pings=["store1"],
        time_unit=TimeUnit.SECOND,
    )

    timespan_metric.set_raw_nanos(timespan_nanos)
    assert 6 == timespan_metric.test_get_value()


def test_set_raw_nanos_followed_by_other_api():
    timespan_nanos = 6 * 1000000000

    timespan_metric = metrics.TimespanMetricType(
        disabled=False,
        category="telemetry",
        lifetime=Lifetime.APPLICATION,
        name="timespan_metric",
        send_in_pings=["store1"],
        time_unit=TimeUnit.SECOND,
    )

    timespan_metric.set_raw_nanos(timespan_nanos)
    assert 6 == timespan_metric.test_get_value()

    timespan_metric.start()
    timespan_metric.stop()
    assert 6 == timespan_metric.test_get_value()


def test_set_raw_nanos_does_not_overwrite_value():
    timespan_nanos = 6 * 1000000000

    timespan_metric = metrics.TimespanMetricType(
        disabled=False,
        category="telemetry",
        lifetime=Lifetime.APPLICATION,
        name="timespan_metric",
        send_in_pings=["store1"],
        time_unit=TimeUnit.SECOND,
    )

    timespan_metric.start()
    timespan_metric.stop()
    value = timespan_metric.test_get_value()

    timespan_metric.set_raw_nanos(timespan_nanos)
    assert value == timespan_metric.test_get_value()


def test_set_raw_nanos_does_nothing_when_timer_is_running():
    timespan_nanos = 6 * 1000000000

    timespan_metric = metrics.TimespanMetricType(
        disabled=False,
        category="telemetry",
        lifetime=Lifetime.APPLICATION,
        name="timespan_metric",
        send_in_pings=["store1"],
        time_unit=TimeUnit.SECOND,
    )

    timespan_metric.start()
    timespan_metric.set_raw_nanos(timespan_nanos)
    timespan_metric.stop()

    assert 6 != timespan_metric.test_get_value()


def test_measure():
    timespan_metric = metrics.TimespanMetricType(
        disabled=False,
        category="telemetry",
        lifetime=Lifetime.APPLICATION,
        name="timespan_metric",
        send_in_pings=["store1"],
        time_unit=TimeUnit.NANOSECOND,
    )

    with timespan_metric.measure():
        time.sleep(0.1)

    assert timespan_metric.test_has_value()
    assert 0 < timespan_metric.test_get_value()


def test_measure_exception():
    timespan_metric = metrics.TimespanMetricType(
        disabled=False,
        category="telemetry",
        lifetime=Lifetime.APPLICATION,
        name="timespan_metric",
        send_in_pings=["store1"],
        time_unit=TimeUnit.NANOSECOND,
    )

    with pytest.raises(ValueError):
        with timespan_metric.measure():
            time.sleep(0.1)
            raise ValueError()

    assert not timespan_metric.test_has_value()
