# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.


import dataclasses
from typing import Dict


@dataclasses.dataclass
class RecordedExperimentData:
    """
    Deserialized experiment data.
    """

    branch: str
    """
    The experiment's branch, as set through
    `glean.Glean.set_experiment_active`.
    """

    extra: Dict[str, str]
    """
    And extra data associated with this experiment through
    `glean.Glean.set_experiment_active`.
    """


__all__ = ["RecordedExperimentData"]
