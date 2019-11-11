# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.


"""
Utilities for loading metrics.yaml and pings.yaml files and creating a tree
of metric types.
"""


from pathlib import Path
from typing import Any, List, Union


from glean_parser.parser import parse_objects  # type: ignore
import glean_parser.metrics  # type: ignore


from . import metrics


# A mapping from the name of the metric type as it appears in the metrics.yaml
# to the Python class for that metric type.
_TYPE_MAPPING = {
    "counter": metrics.CounterMetricType,
    "labeled_counter": metrics.LabeledCounterMetricType,
    "ping": metrics.PingType,
    "string": metrics.StringMetricType,
}


# The arguments that should be passed to the constructor for the metric types.
_ARGS = [
    "allowed_extra_keys",
    "bucket_count",
    "category",
    "disabled",
    "histogram_type",
    "include_client_id",
    "lifetime",
    "memory_unit",
    "name",
    "range_max",
    "range_min",
    "send_in_pings",
    "time_unit",
]


class UnsupportedMetricType:
    """
    A placeholder class for unsupported metric types.

    It raises a `TypeError` when trying to do anything with it, but this lets
    us load the entire `metrics.yaml` even when it contains metric types that
    aren't yet implemented.
    """

    def __init__(self, type: str):
        self._type = type

    def __getattr__(self, attr):
        raise TypeError(
            f"The metric type '{self._type}' is not supported by the "
            "Glean Python bindings"
        )


def _get_metric_object(metric: glean_parser.metrics.Metric) -> Any:
    """
    Given a `glean_parser.metrics.Metric` instance, return a Glean Python
    bindings metric instance.
    """
    args = {}
    for arg in _ARGS:
        if hasattr(metric, arg):
            args[arg] = getattr(metric, arg)

    metric_type = _TYPE_MAPPING.get(metric.type)

    if metric_type is None:
        glean_metric = UnsupportedMetricType(metric.type)
    else:
        glean_metric = metric_type(**args)

    glean_metric.__doc__ = metric.description

    return glean_metric


def load_metrics(
    filepath: Union[Union[str, Path], List[Union[str, Path]]], config: dict = {}
) -> Any:
    """
    Load metrics from a `metrics.yaml` file.

    Args:
        filepath: The path to the file, or a list of paths, to load.
        config (dict): A dictionary of options that change parsing behavior.
            These are documented in glean_parser:
            https://mozilla.github.io/glean_parser/glean_parser.html#glean_parser.parser.parse_objects
    Returns:
        metrics (object): An object containing a tree of metrics, as defined in
            the `metrics.yaml` file.
    Example:
        >>> metrics = load_metrics("metrics.yaml")
        >>> metrics.category.name.set("value")
    """
    if not isinstance(filepath, list):
        filepath = [filepath]

    filepath = [Path(x) for x in filepath]

    result = parse_objects(filepath, config)

    errors = list(result)
    if len(errors):
        raise ValueError("\n\n".join(errors))

    metrics = result.value
    if len(metrics) == 0:
        raise ValueError(f"Didn't find any metrics in '{filepath}'")

    root = type("Metrics", (object,), {})

    for category_name, category in metrics.items():
        cursor = root
        for part in category_name.split("."):
            if not hasattr(cursor, part):
                setattr(cursor, part, type(category_name, (object,), {}))
            cursor = getattr(cursor, part)
        for name, metric in category.items():
            setattr(cursor, name, _get_metric_object(metric))

    return root


def load_pings(
    filepath: Union[Union[str, Path], List[Union[str, Path]]], config: dict = {}
) -> Any:
    """
    Load pings from a `pings.yaml` file.

    Args:
        filepath: The path to the file, or a list of paths, to load.
        config (dict): A dictionary of options that change parsing behavior.
            These are documented in glean_parser:
            https://mozilla.github.io/glean_parser/glean_parser.html#glean_parser.parser.parse_objects
    Returns:
        pings (object): An object where the attributes are pings, as defined in
            the `pings.yaml` file.
    Example:
        >>> pings = load_pings("pings.yaml")
        >>> pings.baseline.send()
    """
    metrics = load_metrics(filepath, config)

    return metrics.pings


__all__ = ["load_metrics", "load_pings"]
