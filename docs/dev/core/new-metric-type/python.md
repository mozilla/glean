# Adding a new metric type - Python

## Re-export generated API

By default a metric type gets an auto-generated API from the definition in `glean.udl`.
If this API is sufficient it needs to be re-exported.

In `glean-core/python/glean/metrics/__init__.py` add a new re-export:

```Python
from .._uniffi import CounterMetric as CounterMetricType
```

## Extend and modify API

If the generated API is not sufficient, convenient or needs additional language-specific constructs or conversions the generated API can be wrapped.

Create a new Python file, e.g. `glean-core/python/glean/metrics/counter.py`.
Then create a new class, that delegates functionality to the generated metric type class.

```Python
from .._uniffi import CommonMetricData
from .._uniffi import CounterMetric


class CounterMetricType:
    def __init__(self, common_metric_data: CommonMetricData):
        self._inner = CounterMetric(common_metric_data)

    # Wrap existing functionality
    def add(self, amount = 1):
        self._inner.add(amount)

    # Add a new method
    def add_two(self):
        self._inner.add(2)
```

The new metric type also needs to be imported from `glean-core/python/glean/metrics/__init__.py`:

```Python
from .counter import CounterMetricType

__all__ = [
    "CounterMetricType",
    # ...
]
```

It also must be added to the `_TYPE_MAPPING` in `glean-core/python/glean/_loader.py`:

```Python
_TYPE_MAPPING = {
    "counter": metrics.CounterMetricType,
    # ...
}
```
