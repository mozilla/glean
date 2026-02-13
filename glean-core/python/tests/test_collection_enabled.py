# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.


from pathlib import Path
import pytest
import sys


from glean import Configuration, Glean, load_pings
from glean import __version__ as glean_version
from glean.metrics import (
    CounterMetricType,
    CommonMetricData,
    Lifetime,
    PingType,
)
from glean.testing import _RecordingUploader

GLEAN_APP_ID = "glean-python-test"
ROOT = Path(__file__).parent


@pytest.mark.skipif(sys.platform == "win32", reason="bug 1933943: Windows failures")
def test_pings_with_follows_false_follow_their_own_setting(tmpdir, helpers):
    info_path = Path(str(tmpdir)) / "info.txt"
    data_dir = Path(str(tmpdir)) / "glean"

    Glean._reset()
    Glean.initialize(
        application_id=GLEAN_APP_ID,
        application_version=glean_version,
        upload_enabled=True,
        data_dir=data_dir,
        configuration=Configuration(ping_uploader=_RecordingUploader(info_path)),
    )

    ping = PingType(
        "nofollows",
        include_client_id=False,
        send_if_empty=True,
        precise_timestamps=True,
        include_info_sections=False,
        enabled=False,
        schedules_pings=[],
        reason_codes=[],
        follows_collection_enabled=False,
        uploader_capabilities=[],
    )

    counter_metric = CounterMetricType(
        CommonMetricData(
            disabled=False,
            category="local",
            lifetime=Lifetime.PING,
            name="counter",
            send_in_pings=["nofollows"],
            label=None,
        )
    )

    counter_metric.add(1)
    assert not counter_metric.test_get_value()
    ping.submit()
    assert not counter_metric.test_get_value()

    ping.set_enabled(True)
    counter_metric.add(2)
    assert 2 == counter_metric.test_get_value()
    ping.submit()
    assert not counter_metric.test_get_value()

    url_path, payload = helpers.wait_for_ping(info_path)

    assert "nofollows" == url_path.split("/")[3]
    assert 2 == payload["metrics"]["counter"]["local.counter"]


@pytest.mark.skipif(sys.platform == "win32", reason="bug 1933943: Windows failures")
def test_loader_sets_flags(tmpdir, helpers):
    info_path = Path(str(tmpdir)) / "info.txt"
    data_dir = Path(str(tmpdir)) / "glean"

    Glean._reset()

    pings = load_pings(ROOT / "data" / "pings.yaml")

    counter_metric = CounterMetricType(
        CommonMetricData(
            disabled=False,
            category="local",
            lifetime=Lifetime.PING,
            name="counter",
            send_in_pings=["nofollows-defined"],
            label=None,
        )
    )

    Glean.initialize(
        application_id=GLEAN_APP_ID,
        application_version=glean_version,
        upload_enabled=True,
        data_dir=data_dir,
        configuration=Configuration(ping_uploader=_RecordingUploader(info_path)),
    )

    counter_metric.add(1)
    assert not counter_metric.test_get_value()
    pings.nofollows_defined.submit()
    assert not counter_metric.test_get_value()

    pings.nofollows_defined.set_enabled(True)
    counter_metric.add(2)
    assert 2 == counter_metric.test_get_value()
    pings.nofollows_defined.submit()
    assert not counter_metric.test_get_value()

    url_path, payload = helpers.wait_for_ping(info_path)

    assert "nofollows-defined" == url_path.split("/")[3]
    assert 2 == payload["metrics"]["counter"]["local.counter"]
