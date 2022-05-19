# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.

from glean import Glean
from glean import __version__ as glean_version
from glean import metrics
from glean.metrics import Lifetime, CommonMetricData
from glean.testing import ErrorType


def test_labeled_counter_type(ping_schema_url):
    labeled_counter_metric = metrics.LabeledCounterMetricType(
        CommonMetricData(
            disabled=False,
            category="telemetry",
            lifetime=Lifetime.APPLICATION,
            name="labeled_counter_metric",
            send_in_pings=["metrics"],
            dynamic_label=None,
        )
    )

    labeled_counter_metric["label1"].add(1)
    labeled_counter_metric["label2"].add(2)

    assert 1 == labeled_counter_metric["label1"].test_get_value()

    assert 2 == labeled_counter_metric["label2"].test_get_value()


def test_labeled_boolean_type(ping_schema_url):
    labeled_boolean_metric = metrics.LabeledBooleanMetricType(
        CommonMetricData(
            disabled=False,
            category="telemetry",
            lifetime=Lifetime.APPLICATION,
            name="labeled_boolean_metric",
            send_in_pings=["metrics"],
            dynamic_label=None,
        )
    )

    labeled_boolean_metric["label1"].set(True)
    labeled_boolean_metric["label2"].set(False)

    assert labeled_boolean_metric["label1"].test_get_value()

    assert not labeled_boolean_metric["label2"].test_get_value()


def test_labeled_string_type(ping_schema_url):
    labeled_string_metric = metrics.LabeledStringMetricType(
        CommonMetricData(
            disabled=False,
            category="telemetry",
            lifetime=Lifetime.APPLICATION,
            name="labeled_string_metric",
            send_in_pings=["metrics"],
            dynamic_label=None,
        )
    )

    labeled_string_metric["label1"].set("foo")
    labeled_string_metric["label2"].set("bar")

    assert "foo" == labeled_string_metric["label1"].test_get_value()

    assert "bar" == labeled_string_metric["label2"].test_get_value()


def test_other_label_with_predefined_labels(ping_schema_url):
    labeled_counter_metric = metrics.LabeledCounterMetricType(
        CommonMetricData(
            disabled=False,
            category="telemetry",
            lifetime=Lifetime.APPLICATION,
            name="labeled_counter_metric",
            send_in_pings=["metrics"],
            dynamic_label=None,
        ),
        labels=["foo", "bar", "baz"],
    )

    labeled_counter_metric["foo"].add(1)
    labeled_counter_metric["foo"].add(2)
    labeled_counter_metric["bar"].add(1)
    labeled_counter_metric["not_there"].add(1)
    labeled_counter_metric["also_not_there"].add(1)
    labeled_counter_metric["not_me"].add(1)

    assert 3 == labeled_counter_metric["foo"].test_get_value()
    assert 1 == labeled_counter_metric["bar"].test_get_value()
    assert not labeled_counter_metric["baz"].test_get_value()
    assert 3 == labeled_counter_metric["not_there"].test_get_value()


def test_other_label_without_predefined_labels(ping_schema_url):
    labeled_counter_metric = metrics.LabeledCounterMetricType(
        CommonMetricData(
            disabled=False,
            category="telemetry",
            lifetime=Lifetime.APPLICATION,
            name="labeled_counter_metric",
            send_in_pings=["metrics"],
            dynamic_label=None,
        )
    )

    for i in range(21):
        labeled_counter_metric[f"label_{i}"].add(1)

    labeled_counter_metric["label_0"].add(1)

    assert 2 == labeled_counter_metric["label_0"].test_get_value()
    for i in range(1, 16):
        assert 1 == labeled_counter_metric[f"label_{i}"].test_get_value()
    assert 5 == labeled_counter_metric["__other__"].test_get_value()


def test_other_label_without_predefined_labels_before_glean_init():
    labeled_counter_metric = metrics.LabeledCounterMetricType(
        CommonMetricData(
            disabled=False,
            category="telemetry",
            lifetime=Lifetime.APPLICATION,
            name="labeled_counter_metric",
            send_in_pings=["metrics"],
            dynamic_label=None,
        )
    )

    Glean._reset()

    for i in range(21):
        labeled_counter_metric[f"label_{i}"].add(1)
    labeled_counter_metric["label_0"].add(1)

    Glean._initialize_with_tempdir_for_testing(
        application_id="glean-python-test",
        application_version=glean_version,
        upload_enabled=True,
    )

    assert 2 == labeled_counter_metric["label_0"].test_get_value()
    for i in range(1, 16):
        assert 1 == labeled_counter_metric[f"label_{i}"].test_get_value()
    assert 5 == labeled_counter_metric["__other__"].test_get_value()


def test_invalid_labels_go_to_other():
    labeled_counter_metric = metrics.LabeledCounterMetricType(
        CommonMetricData(
            disabled=False,
            category="telemetry",
            lifetime=Lifetime.APPLICATION,
            name="labeled_counter_metric",
            send_in_pings=["metrics"],
            dynamic_label=None,
        )
    )

    labeled_counter_metric["notSnakeCase"].add(1)
    labeled_counter_metric[""].add(1)
    labeled_counter_metric["with/slash"].add(1)
    labeled_counter_metric["this_string_has_more_than_thirty_characters"].add(1)

    assert 4 == labeled_counter_metric.test_get_num_recorded_errors(
        ErrorType.INVALID_LABEL
    )


def test_rapidly_recreating_labeled_metrics_does_not_crash():
    """
    Regression test for bug 1733757.
    The underlying map implementation has an upper limit of entries it can handle,
    currently set to (1<<15)-1 = 32767.
    We used to create a new object every time a label was referenced,
    leading to exhausting the available storage in that map, which finally results in a panic.
    """

    labeled_counter_metric = metrics.LabeledCounterMetricType(
        CommonMetricData(
            category="telemetry",
            name="labeled_nocrash",
            send_in_pings=["metrics"],
            lifetime=Lifetime.APPLICATION,
            disabled=False,
            dynamic_label=None,
        ),
        labels=["foo"],
    )

    # We go higher than the maximum of `(1<<15)-1 = 32767`.
    # Python is slow, so we only go a tiny bit higher.
    max_attempts = (1 << 15) + 1  # 32769
    for _ in range(max_attempts):
        labeled_counter_metric["foo"].add(1)

    assert max_attempts == labeled_counter_metric["foo"].test_get_value()
