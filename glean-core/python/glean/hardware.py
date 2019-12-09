# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.

"""
This module provides a cross-platform abstraction to get system model and
manufacturer information.
"""

import dataclasses
from pathlib import Path
import sys


# Loosely based on the Java code found here:
#    https://github.com/oshi/oshi/pull/264


@dataclasses.dataclass
class SystemInformation:
    """
    Stores information about the model and manufacturer of the system.
    """

    model: str
    """
    The model name of the current system
    """

    manufacturer: str
    """
    The manufacturer of the current system
    """


if sys.platform.startswith("linux"):

    def get_system_information():
        def get_value(path):
            try:
                with open(dmi_root / path, "rb") as fd:
                    return fd.read().decode("ascii", "replace").strip()
            except IOError:
                return "error"

        dmi_root = Path("/sys/devices/virtual/dmi/id")

        model = get_value("product_name")
        manufacturer = get_value("sys_vendor")

        return SystemInformation(model=model, manufacturer=manufacturer)


elif sys.platform == "darwin":

    def get_system_information():
        def get_value(name):
            for line in sysinfo.splitlines():
                line = line.strip()
                if line.startswith(name):
                    return line[len(name) :].decode("ascii", "replace").strip()  # noqa
            return "error"

        import subprocess

        try:
            sysinfo = subprocess.check_output(["system_profiler", "SPHardwareDataType"])
        except subprocess.CalledProcessError:
            sysinfo = ""

        model = get_value(b"Model Identifier: ")
        manufacturer = "Apple Inc."

        return SystemInformation(model=model, manufacturer=manufacturer)


elif sys.platform.startswith("win"):

    def get_system_information():
        def get_value(name):
            try:
                output = subprocess.check_output(["WMIC", "CSPRODUCT", "GET", name])
            except subprocess.CalledProcessError:
                return "error"
            lines = output.splitlines()
            if lines < 3:
                return "error"
            return lines[2].strip().decode("ascii", "replace")

        import subprocess

        model = get_value("NAME")
        manufacturer = get_value("VENDOR")

        return SystemInformation(model=model, manufacturer=manufacturer)


else:

    def get_system_information():
        return SystemInformation(model="unknown", manufacturer="unknown")
