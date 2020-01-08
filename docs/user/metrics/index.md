# Metrics

There are different metrics to choose from, depending on what you want to achieve:

* [Boolean](boolean.md): Records a single truth value, for example "is a11y enabled?"

* [Counter](counter.md): Used to count how often something happens, for example, how often a certain button was pressed.

* [Labeled counter](labeled_counters.md): Used to count how often something happens, for example which kind of crash occurred (`"uncaught_exception"` or `"native_code_crash"`).

* [String](string.md): Records a single Unicode string value, for example the name of the OS.

* [Labeled strings](labeled_strings.md): Records multiple Unicode string values, for example to record which kind of error occurred in different stages of a login process.

* [String List](string_list.md): Records a list of Unicode string values, for example the list of enabled search engines.

* [Timespan](timespan.md): Used to measure how much time is spent in a single task.

* [Timing Distribution](timing_distribution.md): Used to record the distribution of multiple time measurements.

* [Memory Distribution](memory_distribution.md): Used to record the distribution of memory sizes.

* [UUID](uuid.md): Used to record universally unique identifiers (UUIDs), such as a client ID.

* [Datetime](datetime.md): Used to record an absolute date and time, such as the time the user first ran the application.

* [Events](event.md): Records events e.g. individual occurrences of user actions, say every time a view was open and from where.

* [Custom Distribution](custom_distribution.md): Used to record the distribution of a value that needs fine-grained control of how the histogram buckets are computed.

* [Quantity](quantity.md): Used to record a single non-negative integer value. For example, the width of the display in pixels.

## Labeled metrics

There are two types of metrics listed above - *labeled* and *unlabeled* metrics. If a metric is *labeled*, it means that for a single metric entry you define in `metrics.yaml`, you can record into multiple metrics under the same name, each of the same type and identified by a different string label.

This is useful when you need to break down metrics by a label known at build time or run time. For example:

- When you want to count a different set of sub-views that users interact with, you could use `viewCount["view1"].add()` and `viewCount["view2"].add()`.

- When you want to count errors that might occur for a feature, you could use `errorCount[errorName].add()`.

Labeled metrics come in two forms:

- **Static labels**: The labels are specified at build time in the `metrics.yaml` file.
  If a label that isn't part of this set is used at run time, it is converted to the special label `__other__`.
  
- **Dynamic labels**: The labels aren't known at build time, so are set at run time.
  Only the first 16 labels seen by the Glean SDK will be tracked. After that, any additional labels are converted to the special label `__other__`.

> **Note**: Be careful with using arbitrary strings as labels and make sure they can't accidentally contain identifying data (like directory paths or user input).
