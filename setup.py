# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.

"""
A basic top-level setup.py script that delegates to the real one in
glean-core/python/setup.py

This is used to generate the source package for glean_sdk on PyPI.
"""

from pathlib import Path
import sys

sys.path.insert(0, str((Path(__file__).parent / "glean-core" / "python").resolve()))
from setup import *
