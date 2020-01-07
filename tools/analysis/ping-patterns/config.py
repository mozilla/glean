# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.

"""
Configuration constants for the analysis script.
"""


import datetime


FIRST_DATE = datetime.datetime.fromisoformat("2019-11-01")
"""
The earliest date to include in the analysis. This helps to remove clients with
wildly incorrect clocks.
"""

FIXES = [
    ("Double scheduling of metrics ping", 191115),  # Double-schedule of metrics ping
    ("Proguard rule to retain lifetime API", 191116),  # Proguard rule
    ("Avoid reflection in the lifetime API", 191120),  # Non-reflection-based API
]
"""
A list of Fenix revisions to highlight in the output. Each entry is a tuple
`(description, version)` where `version` is in Fenix nightly version format:
YYMMDD.
"""

FIXES = sorted(FIXES, key=lambda x: x[1])
