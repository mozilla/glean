# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.

import uuid

import pytest

from glean import metrics
from glean.metrics import Lifetime


def test_the_api_saves_to_its_storage_engine():
    uuid_metric = metrics.UuidMetricType(
        disabled=False,
        category="telemetry",
        lifetime=Lifetime.APPLICATION,
        name="uuid_metric",
        send_in_pings=["store1"],
    )

    # Check that there is no UUID recorded
    assert not uuid_metric.test_has_value()

    # Record two UUIDs of the same type, with a little delay
    uuid1 = uuid_metric.generate_and_set()

    # Check that the data has been properly recorded
    assert uuid_metric.test_has_value()
    assert uuid1 == uuid_metric.test_get_value()

    uuid2 = uuid.UUID("urn:uuid:ce2adeb8-843a-4232-87a5-a099ed1e7bb3")
    uuid_metric.set(uuid2)

    # Check that the data was properly recorded
    assert uuid_metric.test_has_value()
    assert uuid2 == uuid_metric.test_get_value()


def test_disabled_uuids_must_not_record_data():
    uuid_metric = metrics.UuidMetricType(
        disabled=True,
        category="telemetry",
        lifetime=Lifetime.PING,
        name="uuid_metric",
        send_in_pings=["store1"],
    )

    uuid_metric.generate_and_set()
    assert not uuid_metric.test_has_value()


def test_test_get_value_throws_exception_if_nothing_is_stored():
    uuid_metric = metrics.UuidMetricType(
        disabled=True,
        category="telemetry",
        lifetime=Lifetime.PING,
        name="uuid_metric",
        send_in_pings=["store1"],
    )

    with pytest.raises(ValueError):
        uuid_metric.test_get_value()


def test_the_api_saves_to_secondary_pings():
    uuid_metric = metrics.UuidMetricType(
        disabled=False,
        category="telemetry",
        lifetime=Lifetime.PING,
        name="uuid_metric",
        send_in_pings=["store1", "store2"],
    )

    uuid1 = uuid_metric.generate_and_set()

    # Check that the data was properly recorded
    assert uuid_metric.test_has_value("store2")
    assert uuid1 == uuid_metric.test_get_value("store2")

    uuid2 = uuid.UUID("urn:uuid:ce2adeb8-843a-4232-87a5-a099ed1e7bb3")
    uuid_metric.set(uuid2)

    # Check that the data was properly recorded
    assert uuid_metric.test_has_value("store2")
    assert uuid2 == uuid_metric.test_get_value("store2")


def test_invalid_uuid_must_not_crash():
    uuid_metric = metrics.UuidMetricType(
        disabled=False,
        category="telemetry",
        lifetime=Lifetime.PING,
        name="uuid_metric",
        send_in_pings=["store1"],
    )

    # Attempt to set an invalid UUID.
    uuid_metric.set("well, this is not a UUID")

    # Check that no value was stored.
    assert not uuid_metric.test_has_value()
