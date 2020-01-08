# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.


import io
import json
import sys


from glean_parser import validate_ping


from glean import Glean
from glean import __version__ as glean_version
from glean import _builtins
from glean import metrics
from glean._dispatcher import Dispatcher
from glean.metrics import Lifetime
from glean.testing import ErrorType


def test_labeled_counter_type():
    labeled_counter_metric = metrics.LabeledCounterMetricType(
        disabled=False,
        category="telemetry",
        lifetime=Lifetime.APPLICATION,
        name="labeled_counter_metric",
        send_in_pings=["metrics"],
    )

    labeled_counter_metric["label1"].add(1)
    labeled_counter_metric["label2"].add(2)

    assert labeled_counter_metric["label1"].test_has_value()
    assert 1 == labeled_counter_metric["label1"].test_get_value()

    assert labeled_counter_metric["label2"].test_has_value()
    assert 2 == labeled_counter_metric["label2"].test_get_value()

    json_content = Glean.test_collect(_builtins.pings.metrics)

    assert 0 == validate_ping.validate_ping(io.StringIO(json_content), sys.stdout)

    tree = json.loads(json_content)

    assert (
        1
        == tree["metrics"]["labeled_counter"]["telemetry.labeled_counter_metric"][
            "label1"
        ]
    )
    assert (
        2
        == tree["metrics"]["labeled_counter"]["telemetry.labeled_counter_metric"][
            "label2"
        ]
    )


def test_other_label_with_predefined_labels():
    labeled_counter_metric = metrics.LabeledCounterMetricType(
        disabled=False,
        category="telemetry",
        lifetime=Lifetime.APPLICATION,
        name="labeled_counter_metric",
        send_in_pings=["metrics"],
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
    assert not labeled_counter_metric["baz"].test_has_value()
    assert 3 == labeled_counter_metric["not_there"].test_get_value()

    json_content = Glean.test_collect(_builtins.pings.metrics)

    assert 0 == validate_ping.validate_ping(io.StringIO(json_content), sys.stdout)

    tree = json.loads(json_content)

    assert (
        3
        == tree["metrics"]["labeled_counter"]["telemetry.labeled_counter_metric"]["foo"]
    )
    assert (
        1
        == tree["metrics"]["labeled_counter"]["telemetry.labeled_counter_metric"]["bar"]
    )
    assert (
        3
        == tree["metrics"]["labeled_counter"]["telemetry.labeled_counter_metric"][
            "__other__"
        ]
    )


def test_other_label_without_predefined_labels():
    labeled_counter_metric = metrics.LabeledCounterMetricType(
        disabled=False,
        category="telemetry",
        lifetime=Lifetime.APPLICATION,
        name="labeled_counter_metric",
        send_in_pings=["metrics"],
    )

    for i in range(21):
        labeled_counter_metric["label_{}".format(i)].add(1)

    labeled_counter_metric["label_0"].add(1)

    assert 2 == labeled_counter_metric["label_0"].test_get_value()
    for i in range(1, 16):
        assert 1 == labeled_counter_metric["label_{}".format(i)].test_get_value()
    assert 5 == labeled_counter_metric["__other__"].test_get_value()

    json_content = Glean.test_collect(_builtins.pings.metrics)

    assert 0 == validate_ping.validate_ping(io.StringIO(json_content), sys.stdout)


def test_other_label_without_predefined_labels_before_glean_init():
    labeled_counter_metric = metrics.LabeledCounterMetricType(
        disabled=False,
        category="telemetry",
        lifetime=Lifetime.APPLICATION,
        name="labeled_counter_metric",
        send_in_pings=["metrics"],
    )

    Glean.reset()
    Dispatcher.set_task_queueing(True)

    for i in range(21):
        labeled_counter_metric["label_{}".format(i)].add(1)
    labeled_counter_metric["label_0"].add(1)

    Glean.initialize(
        application_id="glean-python-test", application_version=glean_version
    )

    assert 2 == labeled_counter_metric["label_0"].test_get_value()
    for i in range(1, 16):
        assert 1 == labeled_counter_metric["label_{}".format(i)].test_get_value()
    assert 5 == labeled_counter_metric["__other__"].test_get_value()


def test_invalid_labels_go_to_other():
    labeled_counter_metric = metrics.LabeledCounterMetricType(
        disabled=False,
        category="telemetry",
        lifetime=Lifetime.APPLICATION,
        name="labeled_counter_metric",
        send_in_pings=["metrics"],
    )

    labeled_counter_metric["notSnakeCase"].add(1)
    labeled_counter_metric[""].add(1)
    labeled_counter_metric["with/slash"].add(1)
    labeled_counter_metric["this_string_has_more_than_thirty_characters"].add(1)

    assert 4 == labeled_counter_metric.test_get_num_recorded_errors(
        ErrorType.INVALID_LABEL
    )
