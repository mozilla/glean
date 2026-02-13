# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.

from pathlib import Path
from typing import Optional
from dataclasses import dataclass

from glean import metrics
from glean.metrics import Lifetime, CommonMetricData
from glean import testing


ROOT = Path(__file__).parent


class BalloonsObject(list, metrics.ObjectSerialize):
    pass


@dataclass
class BalloonsObjectItem(metrics.ObjectSerialize):
    colour: Optional[str] = None
    diameter: Optional[int] = None


@dataclass
class OtherObject(metrics.ObjectSerialize):
    name: Optional[str] = None
    description: Optional[str] = None


def test_the_api_records_to_its_storage_engine():
    metric = metrics.ObjectMetricType[BalloonsObject](
        CommonMetricData(
            category="test",
            name="baloon",
            lifetime=Lifetime.PING,
            send_in_pings=["store1"],
            label=None,
            disabled=False,
        ),
        BalloonsObject,
    )

    balloons = BalloonsObject()
    balloons.append(BalloonsObjectItem(colour="red", diameter=5))
    balloons.append(BalloonsObjectItem(colour="green"))
    metric.set(balloons)

    snapshot = metric.test_get_value()

    exp = [{"colour": "red", "diameter": 5}, {"colour": "green"}]

    assert exp == snapshot


def test_object_must_not_record_if_disabled():
    metric = metrics.ObjectMetricType[BalloonsObject](
        CommonMetricData(
            category="test",
            name="baloon",
            lifetime=Lifetime.PING,
            send_in_pings=["store1"],
            label=None,
            disabled=True,
        ),
        BalloonsObject,
    )

    balloons = BalloonsObject()
    balloons.append(BalloonsObjectItem(colour="yellow", diameter=10))
    metric.set(balloons)

    assert metric.test_get_value() is None


def test_object_get_value_returns_nil_if_nothing_is_stored():
    metric = metrics.ObjectMetricType[BalloonsObject](
        CommonMetricData(
            category="test",
            name="baloon",
            lifetime=Lifetime.PING,
            send_in_pings=["store1"],
            label=None,
            disabled=True,
        ),
        BalloonsObject,
    )

    assert metric.test_get_value() is None


def test_object_saves_to_secondary_pings():
    metric = metrics.ObjectMetricType[BalloonsObject](
        CommonMetricData(
            category="test",
            name="baloon",
            lifetime=Lifetime.PING,
            send_in_pings=["store1", "store2"],
            label=None,
            disabled=False,
        ),
        BalloonsObject,
    )

    balloons = BalloonsObject()
    balloons.append(BalloonsObjectItem(colour="red", diameter=5))
    balloons.append(BalloonsObjectItem(colour="green"))
    metric.set(balloons)

    exp = [{"colour": "red", "diameter": 5}, {"colour": "green"}]

    snapshot = metric.test_get_value("store1")
    assert exp == snapshot

    snapshot = metric.test_get_value("store2")
    assert exp == snapshot


def test_wrong_object_records_an_error():
    metric = metrics.ObjectMetricType[BalloonsObject](
        CommonMetricData(
            category="test",
            name="baloon",
            lifetime=Lifetime.PING,
            send_in_pings=["store1"],
            label=None,
            disabled=False,
        ),
        BalloonsObject,
    )

    other = OtherObject(name="unknown", description="should give an error")
    metric.set(other)

    assert 1 == metric.test_get_num_recorded_errors(testing.ErrorType.INVALID_VALUE)

    snapshot = metric.test_get_value()
    assert snapshot is None
