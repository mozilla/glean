# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.


from typing import Dict


class RecordedExperimentData:
    """
    Deserialized experiment data.
    """

    def __init__(self, branch: str, extra: Dict[str, str] = None):
        """
        Args:
            branch (str): The experiment's branch.
            extra (dict of str->str): Any extra data associated with this
                experiment.
        """
        self._branch = branch
        if extra is None:
            extra = {}
        self._extra = extra

    @property
    def branch(self) -> str:
        """The experiment's branch."""
        return self._branch

    @property
    def extra(self) -> Dict[str, str]:
        """Any extra data associated with this experiment."""
        return self._extra


__all__ = ["RecordedExperimentData"]
