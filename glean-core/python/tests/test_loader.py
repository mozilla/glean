# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.


from pathlib import Path


from glean import load_metrics, load_pings
from glean.metrics.ping import PingType
from glean import _builtins


ROOT = Path(__file__).parent


def test_builtin_pings():
    assert set(dir(_builtins.pings)).issuperset(
        set(["metrics", "baseline", "events", "deletion_request"])
    )


def test_working_metric():
    metrics = load_metrics(ROOT / "data" / "core.yaml", config={"allow_reserved": True})

    assert metrics.core_ping.flash_usage.__doc__.startswith(
        "The number of times the flash plugin"
    )

    metrics.core_ping.flash_usage.add(1)

    assert 1 == metrics.core_ping.flash_usage.test_get_value()


def test_kebab_case_pings():
    pings = load_pings(ROOT / "data" / "pings.yaml")

    assert isinstance(pings.kebab_case, PingType)
