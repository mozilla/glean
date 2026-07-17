# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at https://mozilla.org/MPL/2.0/.

import ctypes
import json
import os
import platform
import tempfile
from ctypes import cdll


def library_name(name):
    suffix = "dylib" if platform.system() == "Darwin" else "so"
    return f"lib{name}.{suffix}"

def check_ping_data(ping_type, sent_ping, amount):
    data = open(sent_ping).read()
    end_first_object = data.find("}")
    payload = json.loads(data[end_first_object + 1 :])
    counter = payload["metrics"]["counter"]
    events = payload["events"]

    if ping_type == "prototype":
        assert 1 == counter["test.metrics.sample_counter"]

    assert amount == counter["dylib.counting"]

    assert 2 == len(events)

    no_extra = events[0]
    assert "event" == no_extra["name"]

    with_extra = events[1]
    assert "event_with_extras" == with_extra["name"]
    extras = with_extra["extra"]
    assert "true", extras["is_set"]

def test_run():
    xul = cdll.LoadLibrary(library_name("xul"))
    services = cdll.LoadLibrary(library_name("services"))

    with tempfile.TemporaryDirectory() as data_path:
        startup_fn = xul.startup
        startup_fn.argtypes = [ctypes.c_char_p]
        startup_fn(str.encode(data_path))

        services_record = services.record
        services_record.argtypes = [ctypes.c_int32]

        amount = 31
        services_record(amount)

        xul.submit()
        xul.shutdown()

        # Check that
        # * We submitted one ping only
        # * It's the `prototype` ping
        # * It contains several metrics with the expected values
        path = os.path.join(data_path, "sent_pings")
        for root, dirs, files in os.walk(path):
            assert len(files) == 2
            files = sorted(files)

            assert "prototype-" in files[0]
            sent_ping = os.path.join(path, files[0])
            check_ping_data("prototype", sent_ping, amount)

            assert "services-info-" in files[1]
            sent_ping = os.path.join(path, files[1])
            check_ping_data("services-info", sent_ping, amount)
