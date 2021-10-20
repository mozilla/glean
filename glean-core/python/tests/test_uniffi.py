# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.


import io
import os
import json
from pathlib import Path
import re
import shutil
import subprocess
import sys
import time
import uuid


import pytest


from glean import _uniffi
from glean import Configuration, Glean, load_metrics
from glean import __version__ as glean_version
from glean.metrics import (
    CounterMetricType,
    CommonMetricData,
    Lifetime,
)

GLEAN_APP_ID = "glean-python-test"


ROOT = Path(__file__).parent


def test_smoke():
    Glean._initialize_with_tempdir_for_testing(
        application_id=GLEAN_APP_ID,
        application_version=glean_version,
        upload_enabled=True,
    )

    counter = CounterMetricType(CommonMetricData(
        disabled=False,
        category="telemetry",
        lifetime=Lifetime.APPLICATION,
        name="counter_metric",
        send_in_pings=["store1"],
        dynamic_label=None
    ))
    counter.add()

    assert 1 == counter.test_get_value()

def test_smoke_experiment_api():
    Glean._initialize_with_tempdir_for_testing(
        application_id=GLEAN_APP_ID,
        application_version=glean_version,
        upload_enabled=True,
    )

    Glean.set_experiment_active("my-experiment", "control")
    assert Glean.test_is_experiment_active("my-experiment")

    Glean.set_experiment_inactive("my-experiment")
    assert not Glean.test_is_experiment_active("my-experiment")

    Glean.set_experiment_active("my-experiment", "control", {"report": "nothing"})
    assert Glean.test_is_experiment_active("my-experiment")
    experiment = Glean.test_get_experiment_data("my-experiment")
    assert "control" == experiment.branch
    assert {"report": "nothing"} == experiment.extra
