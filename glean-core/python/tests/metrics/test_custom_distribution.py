# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.

from glean import metrics
from glean.metrics import Lifetime, CommonMetricData, HistogramType


def test_the_api_saves_to_its_storage_engine():
    metric = metrics.CustomDistributionMetricType(
        CommonMetricData(
            disabled=False,
            category="telemetry",
            lifetime=Lifetime.APPLICATION,
            name="custom_distribution",
            send_in_pings=["store1"],
            label=None,
        ),
        range_min=0,
        range_max=100,
        bucket_count=100,
        histogram_type=HistogramType.LINEAR,
    )

    metric.accumulate_samples([1, 2])
    metric.accumulate_single_sample(3)

    snapshot = metric.test_get_value()
    assert 1 + 2 + 3 == snapshot.sum
    assert 1 == snapshot.values[1]
    assert 1 == snapshot.values[2]
    assert 1 == snapshot.values[3]


def test_exponential_distribution():
    metric = metrics.CustomDistributionMetricType(
        CommonMetricData(
            disabled=False,
            category="telemetry",
            lifetime=Lifetime.APPLICATION,
            name="custom_distribution",
            send_in_pings=["store1"],
            label=None,
        ),
        range_min=0,
        range_max=100,
        bucket_count=10,
        histogram_type=HistogramType.EXPONENTIAL,
    )

    metric.accumulate_samples([1, 20, 50])

    snapshot = metric.test_get_value()
    assert 1 + 20 + 50 == snapshot.sum
    print(snapshot)
    assert 1 == snapshot.values[1]
    assert 1 == snapshot.values[16]
    assert 1 == snapshot.values[29]


def test_get_value_throws_if_nothing_is_stored():
    metric = metrics.CustomDistributionMetricType(
        CommonMetricData(
            disabled=False,
            category="telemetry",
            lifetime=Lifetime.APPLICATION,
            name="custom_distribution",
            send_in_pings=["store1"],
            label=None,
        ),
        range_min=0,
        range_max=100,
        bucket_count=100,
        histogram_type=HistogramType.LINEAR,
    )

    assert not metric.test_get_value()


def test_the_api_saves_to_secondary_pings():
    metric = metrics.CustomDistributionMetricType(
        CommonMetricData(
            disabled=False,
            category="telemetry",
            lifetime=Lifetime.APPLICATION,
            name="custom_distribution",
            send_in_pings=["store1", "store2", "store3"],
            label=None,
        ),
        range_min=0,
        range_max=100,
        bucket_count=100,
        histogram_type=HistogramType.LINEAR,
    )

    metric.accumulate_samples(list(range(1, 4)))

    for store in ["store1", "store2", "store3"]:
        snapshot = metric.test_get_value(store)
        assert 1 + 2 + 3 == snapshot.sum
        assert 1 == snapshot.values[1]
        assert 1 == snapshot.values[2]
        assert 1 == snapshot.values[3]
