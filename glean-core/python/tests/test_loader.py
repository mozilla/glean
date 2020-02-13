# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.


from pathlib import Path


import pytest


from glean import load_metrics
from glean._loader import UnsupportedMetricType
from glean import _builtins


ROOT = Path(__file__).parent


def test_builtin_pings():
    assert set(dir(_builtins.pings)).issuperset(set(["metrics", "baseline", "events"]))


def test_unknown_metric_type():
    metrics = load_metrics(ROOT / "data" / "core.yaml", config={"allow_reserved": True})

    assert isinstance(metrics.environment.profile_date, UnsupportedMetricType)

    with pytest.raises(TypeError):
        metrics.environment.profile_date.set("foo")


def test_working_metric():
    metrics = load_metrics(ROOT / "data" / "core.yaml", config={"allow_reserved": True})

    assert metrics.core_ping.flash_usage.__doc__.startswith(
        "The number of times the flash plugin"
    )

    metrics.core_ping.flash_usage.add(1)

    assert 1 == metrics.core_ping.flash_usage.test_get_value()


def test_glinter_error(capsys):
    load_metrics(ROOT / "data" / "glinter.yaml")

    captured = capsys.readouterr()
    assert "BUG_NUMBER" in captured.err
