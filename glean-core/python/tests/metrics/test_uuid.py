# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.

from pathlib import Path
import uuid

from glean import Glean
from glean import _builtins
from glean import metrics
from glean import testing
from glean.metrics import Lifetime, CommonMetricData
from glean.testing import _RecordingUploader
from glean._uniffi import glean_set_test_mode


def test_the_api_saves_to_its_storage_engine():
    uuid_metric = metrics.UuidMetricType(
        CommonMetricData(
            disabled=False,
            category="telemetry",
            lifetime=Lifetime.APPLICATION,
            name="uuid_metric",
            send_in_pings=["store1"],
            dynamic_label=None,
        )
    )

    # Check that there is no UUID recorded
    assert not uuid_metric.test_get_value()

    # Record two UUIDs of the same type, with a little delay
    uuid1 = uuid_metric.generate_and_set()

    # Check that the data has been properly recorded
    assert uuid1 == uuid_metric.test_get_value()

    uuid2 = uuid.UUID("urn:uuid:ce2adeb8-843a-4232-87a5-a099ed1e7bb3")
    uuid_metric.set(uuid2)

    # Check that the data was properly recorded
    assert uuid2 == uuid_metric.test_get_value()


def test_disabled_uuids_must_not_record_data():
    uuid_metric = metrics.UuidMetricType(
        CommonMetricData(
            disabled=True,
            category="telemetry",
            lifetime=Lifetime.PING,
            name="uuid_metric",
            send_in_pings=["store1"],
            dynamic_label=None,
        )
    )

    uuid_metric.generate_and_set()
    assert not uuid_metric.test_get_value()


def test_test_get_value_throws_exception_if_nothing_is_stored():
    uuid_metric = metrics.UuidMetricType(
        CommonMetricData(
            disabled=True,
            category="telemetry",
            lifetime=Lifetime.PING,
            name="uuid_metric",
            send_in_pings=["store1"],
            dynamic_label=None,
        )
    )

    assert not uuid_metric.test_get_value()


def test_the_api_saves_to_secondary_pings():
    uuid_metric = metrics.UuidMetricType(
        CommonMetricData(
            disabled=False,
            category="telemetry",
            lifetime=Lifetime.PING,
            name="uuid_metric",
            send_in_pings=["store1", "store2"],
            dynamic_label=None,
        )
    )

    uuid1 = uuid_metric.generate_and_set()

    # Check that the data was properly recorded
    assert uuid1 == uuid_metric.test_get_value("store2")

    uuid2 = uuid.UUID("urn:uuid:ce2adeb8-843a-4232-87a5-a099ed1e7bb3")
    uuid_metric.set(uuid2)

    # Check that the data was properly recorded
    assert uuid2 == uuid_metric.test_get_value("store2")


def test_invalid_uuid_must_not_crash():
    uuid_metric = metrics.UuidMetricType(
        CommonMetricData(
            disabled=False,
            category="telemetry",
            lifetime=Lifetime.PING,
            name="uuid_metric",
            send_in_pings=["store1"],
            dynamic_label=None,
        )
    )

    # Attempt to set an invalid UUID.
    uuid_metric.set("well, this is not a UUID")

    # Check that no value was stored.
    assert not uuid_metric.test_get_value()


def test_invalid_uuid_string():
    uuid_metric = metrics.UuidMetricType(
        CommonMetricData(
            disabled=False,
            category="telemetry",
            lifetime=Lifetime.PING,
            name="uuid_metric",
            send_in_pings=["store1"],
            dynamic_label=None,
        )
    )

    uuid_metric.set("NOT-A-UUID!!!")
    assert uuid_metric.test_get_num_recorded_errors(testing.ErrorType.INVALID_VALUE) == 1


def test_what_looks_like_it_might_be_uuid(tmpdir, helpers):
    import hashlib

    Glean._reset()

    glean_set_test_mode(True)

    info_path = Path(str(tmpdir)) / "info.txt"

    # This test relies on testing mode to be disabled, since we need to prove the
    # real-world async behaviour of this.

    configuration = Glean._configuration
    configuration.ping_uploader = _RecordingUploader(info_path)
    Glean._initialize_with_tempdir_for_testing(
        application_id="glean-python-test",
        application_version="0.0.1",
        upload_enabled=True,
        configuration=Glean._configuration,
    )

    chksum_uuid = metrics.UuidMetricType(
        CommonMetricData(
            disabled=False,
            category="c",
            lifetime=Lifetime.PING,
            name="chksum",
            send_in_pings=["metrics"],
            dynamic_label=None,
        )
    )

    random_uuid = metrics.UuidMetricType(
        CommonMetricData(
            disabled=False,
            category="c",
            lifetime=Lifetime.PING,
            name="random",
            send_in_pings=["metrics"],
            dynamic_label=None,
        )
    )

    valid_uuid = metrics.UuidMetricType(
        CommonMetricData(
            disabled=False,
            category="c",
            lifetime=Lifetime.PING,
            name="valid",
            send_in_pings=["metrics"],
            dynamic_label=None,
        )
    )

    # We can handle anything that _looks_ like a UUID,
    # that is:
    # * A 32-byte hex string
    # * A hyphenated UUID (5 groups of characters plus `-`, total of 36 characters)
    # * `urn:uuid:$uuid`
    # Why? Because that's what people use.
    chksum = hashlib.md5("glean".encode("utf-8")).hexdigest()
    random = "dd296ebb49b2456eaf3b99d7486ab9c0"  # generated using `uuid.uuid4().hex`
    valid = "dd296ebb-49b2-456e-af3b-99d7486ab9c0"  # above but hyphenated

    # A character too long and you are out!
    random_uuid.set(random + "a")
    assert random_uuid.test_get_num_recorded_errors(testing.ErrorType.INVALID_VALUE) == 1

    chksum_uuid.set(chksum)
    random_uuid.set(random)
    valid_uuid.set(valid)

    # We check the actual payload to verify how it is encoded.
    _builtins.pings.metrics.submit()

    url_path, payload = helpers.wait_for_ping(info_path)

    assert "metrics" == url_path.split("/")[3]

    uuids = payload["metrics"]["uuid"]

    assert uuids["c.chksum"] == "39621ca5-f9d2-ef5c-d021-afc9a789535e"
    assert uuids["c.random"] == valid
    assert uuids["c.valid"] == valid
