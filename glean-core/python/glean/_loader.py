# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.


"""
Utilities for loading metrics.yaml and pings.yaml files and creating a tree
of metric types.
"""


import enum
from pathlib import Path
from typing import Any, Dict, Generator, List, Optional, Tuple, Union


from glean_parser.parser import parse_objects  # type: ignore
import glean_parser.lint  # type: ignore
import glean_parser.metrics as gp_metrics  # type: ignore
from glean_parser.util import Camelize, extra_metric_args  # type: ignore


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
    "quantity": metrics.QuantityMetricType,
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

_ARG_CONVERSION = {
    "lifetime": {
        gp_metrics.Lifetime.ping: metrics.Lifetime.PING,
        gp_metrics.Lifetime.application: metrics.Lifetime.APPLICATION,
        gp_metrics.Lifetime.user: metrics.Lifetime.USER,
    },
    "time_unit": {
        gp_metrics.TimeUnit.nanosecond: metrics.TimeUnit.NANOSECOND,
        gp_metrics.TimeUnit.microsecond: metrics.TimeUnit.MICROSECOND,
        gp_metrics.TimeUnit.millisecond: metrics.TimeUnit.MILLISECOND,
        gp_metrics.TimeUnit.second: metrics.TimeUnit.SECOND,
        gp_metrics.TimeUnit.minute: metrics.TimeUnit.MINUTE,
        gp_metrics.TimeUnit.hour: metrics.TimeUnit.HOUR,
        gp_metrics.TimeUnit.day: metrics.TimeUnit.DAY,
    },
    "memory_unit": {
        gp_metrics.MemoryUnit.byte: metrics.MemoryUnit.BYTE,
        gp_metrics.MemoryUnit.kilobyte: metrics.MemoryUnit.KILOBYTE,
        gp_metrics.MemoryUnit.megabyte: metrics.MemoryUnit.MEGABYTE,
        gp_metrics.MemoryUnit.gigabyte: metrics.MemoryUnit.GIGABYTE,
    },
}


def getattr_conv(metric, arg):
    """
    Get an attribute from the parsed metric
    and optionally convert it to the correct glean-py type.
    """

    val = getattr(metric, arg)
    if arg in _ARG_CONVERSION:
        val = _ARG_CONVERSION[arg].get(val, val)

    return val


def _normalize_name(name):
    """
    Convert kebab-case to snake_case.
    """
    return name.replace("-", "_")


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
            f"The metric type '{self._type}' is not supported by the Glean Python bindings"
        )


def _event_extra_factory(name: str, argnames: List[Tuple[str, str]]) -> Any:
    """
    Generate a new class, inheriting from `metrics.EventExtras`
    and implementing the `to_ffi_extra` method,
    which serializes expected attributes to pass over FFI.
    """

    def __init__(self, **kwargs):
        for key, value in kwargs.items():
            typ = next((t for (k, t) in argnames if key == k), None)
            if typ is None:
                raise TypeError(
                    f"Argument '{key}' not valid for {self.__class__.__name__}"
                )
            elif typ == "boolean" and isinstance(value, bool):
                pass
            elif typ == "string" and isinstance(value, str):
                pass
            elif typ == "quantity" and isinstance(value, int):
                pass
            else:
                raise TypeError(
                    f"Field '{key}' requires type {typ} in {self.__class__.__name__}"
                )
            setattr(self, key, value)

    def to_ffi_extra(self):
        extras = {}

        for name, typ in argnames:
            attr = getattr(self, name, None)
            if attr is not None:
                if typ == "boolean" and isinstance(attr, bool):
                    # Special-case needed for booleans to turn them lowercase (true/false)
                    extras[name] = str(attr).lower()
                elif typ == "string" and isinstance(attr, str):
                    extras[name] = str(attr)
                elif typ == "quantity" and isinstance(attr, int):
                    extras[name] = str(attr)
                # Don't support other data types
                else:
                    raise TypeError(f"Type {type(attr)} not supported for {name}")

        return extras

    attr = {name: None for (name, _) in argnames}  # type: Dict[str, Any]
    attr["__init__"] = __init__
    attr["to_ffi_extra"] = to_ffi_extra
    newclass = type(name, (metrics.EventExtras,), attr)
    return newclass


def _split_ctor_args(args: Dict[str, Any]) -> Tuple[Dict[str, Any], Dict[str, Any]]:
    meta_args = {}
    extra_args = {}
    for k, v in args.items():
        if k in extra_metric_args:
            extra_args[k] = v
        else:
            meta_args[k] = v

    return (meta_args, extra_args)


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
            args[arg] = getattr_conv(metric, arg)

    metric_type = _TYPE_MAPPING.get(metric.type)

    if metric_type is None:
        glean_metric = UnsupportedMetricType(metric.type)  # type: ignore
    elif metric.type == "ping":
        # Special-case Ping, doesn't take CommonMetricData
        glean_metric = metrics.PingType(**args)  # type: ignore
    else:
        # Hack for the time being.
        if "dynamic_label" not in args:
            args["dynamic_label"] = None
        meta_args, rest = _split_ctor_args(args)
        glean_metric = metric_type(metrics.CommonMetricData(**meta_args), **rest)

    glean_metric.__doc__ = metric.description

    yield name, glean_metric

    # Events and Pings also need to define an enumeration
    if metric.type == "event":
        if metric.has_extra_types:
            class_name = name + "_extra"
            class_name = Camelize(class_name)
            values = metric.allowed_extra_keys_with_types
            keys_class = _event_extra_factory(class_name, values)  # type: ignore
            yield class_name, keys_class
        else:
            enum_name = name + "_keys"
            class_name = Camelize(enum_name)
            values = dict((x.upper(), x) for x in metric.allowed_extra_keys)
            keys_enum = enum.Enum(class_name, values)  # type: ignore
            yield enum_name, keys_enum
    elif metric.type == "ping":
        enum_name = name + "_reason_codes"
        class_name = Camelize(enum_name)
        values = dict((x.upper(), i) for (i, x) in enumerate(metric.reason_codes))
        keys_enum = enum.Enum(class_name, values)  # type: ignore
        yield enum_name, keys_enum


def load_metrics(
    filepath: Union[Union[str, Path], List[Union[str, Path]]],
    config: Optional[dict] = None,
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
    if config is None:
        config = {}

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
            for actual_name, glean_metric in _get_metric_objects(name, metric):
                setattr(cursor, _normalize_name(actual_name), glean_metric)

    return root


def load_pings(
    filepath: Union[Union[str, Path], List[Union[str, Path]]],
    config: Optional[dict] = None,
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
