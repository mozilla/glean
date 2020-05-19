# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.


"""
Utilities for loading metrics.yaml and pings.yaml files and creating a tree
of metric types.
"""


import enum
import io
from pathlib import Path
import sys
from typing import Any, Generator, List, Tuple, Union


from glean_parser.parser import parse_objects  # type: ignore
import glean_parser.lint  # type: ignore
import glean_parser.metrics  # type: ignore
from glean_parser.util import Camelize  # type: ignore


from . import metrics


# A mapping from the name of the metric type as it appears in the metrics.yaml
# to the Python class for that metric type.
_TYPE_MAPPING = {
    "boolean": metrics.BooleanMetricType,
    "counter": metrics.CounterMetricType,
    "datetime": metrics.DatetimeMetricType,
    "event": metrics.EventMetricType,
    "labeled_boolean": metrics.LabeledBooleanMetricType,
    "labeled_counter": metrics.LabeledCounterMetricType,
    "labeled_string": metrics.LabeledStringMetricType,
    "memory_distribution": metrics.MemoryDistributionMetricType,
    "ping": metrics.PingType,
    "string": metrics.StringMetricType,
    "string_list": metrics.StringListMetricType,
    "timespan": metrics.TimespanMetricType,
    "timing_distribution": metrics.TimingDistributionMetricType,
    "uuid": metrics.UuidMetricType,
}


# The arguments that should be passed to the constructor for the metric types.
_ARGS = [
    "allowed_extra_keys",
    "bucket_count",
    "category",
    "disabled",
    "histogram_type",
    "include_client_id",
    "send_if_empty",
    "lifetime",
    "memory_unit",
    "name",
    "range_max",
    "range_min",
    "reason_codes",
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
            "The metric type '{}' is not supported by the Glean Python bindings".format(
                self._type
            )
        )


def _get_metric_objects(
    name: str, metric: glean_parser.metrics.Metric
) -> Generator[Tuple[str, Any], None, None]:
    """
    Given a `glean_parser.metrics.Metric` instance, return the Glean Python
    bindings metric instances for the metric.
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

    yield name, glean_metric

    # Events and Pings also need to define an enumeration
    if metric.type == "event":
        enum_name = name + "_keys"
        class_name = Camelize(enum_name)
        values = dict((x.upper(), i) for (i, x) in enumerate(metric.allowed_extra_keys))
        keys_enum = enum.Enum(class_name, values)  # type: ignore
        yield enum_name, keys_enum
    elif metric.type == "ping":
        enum_name = name + "_reason_codes"
        class_name = Camelize(enum_name)
        values = dict((x.upper(), i) for (i, x) in enumerate(metric.reason_codes))
        keys_enum = enum.Enum(class_name, values)  # type: ignore
        yield enum_name, keys_enum


def load_metrics(
    filepath: Union[Union[str, Path], List[Union[str, Path]]], config: dict = {}
) -> Any:
    """
    Load metrics from a `metrics.yaml` file.

    Args:
        filepath (Path): The path to the file, or a list of paths, to load.
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

    # Just print glinter warnings to stderr
    glinter_warnings = io.StringIO()
    if glean_parser.lint.glinter(filepath, config, file=glinter_warnings):
        sys.stderr.write(glinter_warnings.getvalue())

    result = parse_objects(filepath, config)

    errors = list(result)
    if len(errors):
        raise ValueError("\n\n".join(errors))

    metrics = result.value
    if len(metrics) == 0:
        raise ValueError("Didn't find any metrics in '{}'".format(filepath))

    root = type("Metrics", (object,), {})

    for category_name, category in metrics.items():
        cursor = root
        for part in category_name.split("."):
            if not hasattr(cursor, part):
                setattr(cursor, part, type(category_name, (object,), {}))
            cursor = getattr(cursor, part)
        for name, metric in category.items():
            for actual_name, glean_metric in _get_metric_objects(name, metric):
                setattr(cursor, actual_name, glean_metric)

    return root


def load_pings(
    filepath: Union[Union[str, Path], List[Union[str, Path]]], config: dict = {}
) -> Any:
    """
    Load pings from a `pings.yaml` file.

    Args:
        filepath (Path): The path to the file, or a list of paths, to load.
        config (dict): A dictionary of options that change parsing behavior.
            These are documented in glean_parser:
            https://mozilla.github.io/glean_parser/glean_parser.html#glean_parser.parser.parse_objects
    Returns:
        pings (object): An object where the attributes are pings, as defined in
            the `pings.yaml` file.
    Example:
        >>> pings = load_pings("pings.yaml")
        >>> pings.baseline.submit()
    """
    metrics = load_metrics(filepath, config)

    return metrics.pings


__all__ = ["load_metrics", "load_pings"]
