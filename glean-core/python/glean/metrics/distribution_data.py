# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.


import json
from typing import Dict


class DistributionData:
    """
    This class represents the structure of a distribution according to the
    pipeline schema. It is meant to help serialize and deserialize data to the
    correct format for transport and storage, as well as including a helper
    function to calculate the bucket sizes.
    """

    def __init__(self, values: Dict[int, int], sum: int):
        """
        Args:
            values: a map containing the bucket index mapped to the accumulated
                count
            sum: the accumulated sum of all the samples in the distribution
        """
        self._values = values
        self._sum = sum

    @property
    def values(self) -> Dict[int, int]:
        return self._values

    @property
    def sum(self) -> int:
        return self._sum

    @property
    def count(self) -> int:
        return sum(self._values.values())

    @classmethod
    def from_json_string(cls, json_string: str) -> "DistributionData":
        """
        Factory function that takes stringified JSON and converts it back into a
        `DistributionData`.  This tries to read all values and attempts to
        use a default where no value exists.

        Args:
            json: Stringified JSON value representing a `DistributionData` object.

        Returns:
            distribution_data: A `DistributionData`.
        """

        json_object = json.loads(json_string)

        values = dict((int(k), int(v)) for (k, v) in json_object["values"].items())
        sum = int(json_object["sum"])

        return cls(values, sum)
