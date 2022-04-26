# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.


import pytest


from glean import metrics
from glean.metrics import Lifetime, MemoryUnit
from glean import testing


def test_the_api_saves_to_its_storage_engine():
    metric = metrics.MemoryDistributionMetricType(
        disabled=False,
        category="telemetry",
        lifetime=Lifetime.APPLICATION,
        name="memory_distribution",
        send_in_pings=["store1"],
        memory_unit=MemoryUnit.KILOBYTE,
    )

    for i in range(1, 4):
        metric.accumulate(i)

    kb = 1024

    assert metric.test_has_value()
    snapshot = metric.test_get_value()
    assert 1 * kb + 2 * kb + 3 * kb == snapshot.sum
    assert 1 == snapshot.values[1023]
    assert 1 == snapshot.values[2047]
    assert 1 == snapshot.values[3024]


def test_values_are_truncated_to_1tb():
    metric = metrics.MemoryDistributionMetricType(
        disabled=False,
        category="telemetry",
        lifetime=Lifetime.APPLICATION,
        name="memory_distribution",
        send_in_pings=["store1"],
        memory_unit=MemoryUnit.GIGABYTE,
    )

    metric.accumulate(2048)

    assert metric.test_has_value()
    snapshot = metric.test_get_value()
    assert 1 << 40 == snapshot.sum
    assert 1 == snapshot.values[(1 << 40) - 1]
    assert 1 == metric.test_get_num_recorded_errors(testing.ErrorType.INVALID_VALUE)


def test_disabled_memory_distributions_must_not_record_data():
    metric = metrics.MemoryDistributionMetricType(
        disabled=True,
        category="telemetry",
        lifetime=Lifetime.APPLICATION,
        name="memory_distribution",
        send_in_pings=["store1"],
        memory_unit=MemoryUnit.KILOBYTE,
    )

    metric.accumulate(1)

    assert not metric.test_has_value()


def test_get_value_throws_if_nothing_is_stored():
    metric = metrics.MemoryDistributionMetricType(
        disabled=False,
        category="telemetry",
        lifetime=Lifetime.APPLICATION,
        name="memory_distribution",
        send_in_pings=["store1"],
        memory_unit=MemoryUnit.KILOBYTE,
    )

    with pytest.raises(ValueError):
        metric.test_get_value()


def test_the_api_saves_to_secondary_pings():
    metric = metrics.MemoryDistributionMetricType(
        disabled=False,
        category="telemetry",
        lifetime=Lifetime.APPLICATION,
        name="memory_distribution",
        send_in_pings=["store1", "store2", "store3"],
        memory_unit=MemoryUnit.KILOBYTE,
    )

    for i in range(1, 4):
        metric.accumulate(i)

    for store in ["store1", "store2", "store3"]:
        assert metric.test_has_value(store)
        snapshot = metric.test_get_value(store)
        assert 6144 == snapshot.sum
        assert 1 == snapshot.values[1023]
        assert 1 == snapshot.values[2047]
        assert 1 == snapshot.values[3024]
