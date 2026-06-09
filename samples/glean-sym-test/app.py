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
    # * It contains 2 metrics with the expected values
    path = os.path.join(data_path, "sent_pings")
    for root, dirs, files in os.walk(path):
        assert len(files) == 1
        assert "prototype-" in files[0]

        sent_ping = os.path.join(path, files[0])
        data = open(sent_ping).read()
        end_first_object = data.find("}")
        payload = json.loads(data[end_first_object + 1 :])
        counter = payload["metrics"]["counter"]

        assert 1 == counter["test.metrics.sample_counter"]
        assert amount == counter["dylib.counting"]
