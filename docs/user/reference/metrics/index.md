# Metrics

> **Not sure which metric type to use?** These docs contain a [series of questions](../../user/metrics/adding-new-metrics.html#choosing-a-metric-type) that can help. Reference information about each metric type is linked below.

The parameters available that apply to any metric type are in the [metric parameters page](../yaml/index.html).

There are different metrics to choose from, depending on what you want to achieve:

* [Boolean](boolean.md): Records a single truth value, for example "is a11y enabled?"

* [Labeled boolean](labeled_booleans.md): Records truth values for a set of labels, for example "which a11y features are enabled?"

* [Counter](counter.md): Used to count how often something happens, for example, how often a certain button was pressed.

* [Labeled counter](labeled_counters.md): Used to count how often something happens, for example which kind of crash occurred (`"uncaught_exception"` or `"native_code_crash"`).

* [String](string.md): Records a single Unicode string value, for example the name of the OS.

* [Labeled strings](labeled_strings.md): Records multiple Unicode string values, for example to record which kind of error occurred in different stages of a login process.

* [String List](string_list.md): Records a list of Unicode string values, for example the list of enabled search engines.

* [Timespan](timespan.md): Used to measure how much time is spent in a single task.

* [Timing Distribution](timing_distribution.md): Used to record the distribution of multiple time measurements.

* [Memory Distribution](memory_distribution.md): Used to record the distribution of memory sizes.

* [UUID](uuid.md): Used to record universally unique identifiers (UUIDs), such as a client ID.

* [URL](url.md): Used to record URL-like strings.

* [Datetime](datetime.md): Used to record an absolute date and time, such as the time the user first ran the application.

* [Events](event.md): Records events e.g. individual occurrences of user actions, say every time a view was open and from where.

* [Custom Distribution](custom_distribution.md): Used to record the distribution of a value that needs fine-grained control of how the histogram buckets are computed.  **Custom distributions are only available for values that come from Gecko.**

* [Quantity](quantity.md): Used to record a single non-negative integer value. For example, the width of the display in pixels.

* [Rate](rate.md): Used to record the rate something happens relative to some other thing.
  For example, the number of HTTP connections that experienced an error relative to the number of total HTTP connections made.

* [Text](text.md): Records a single long Unicode text, used when the limits on `String` are too low.

## Labeled metrics

There are two types of metrics listed above - *labeled* and *unlabeled* metrics. If a metric is *labeled*, it means that for a single metric entry you define in `metrics.yaml`, you can record into multiple metrics under the same name, each of the same type and identified by a different string label.

This is useful when you need to break down metrics by a label known at build time or run time. For example:

- When you want to count a different set of sub-views that users interact with, you could use `viewCount["view1"].add()` and `viewCount["view2"].add()`.

- When you want to count errors that might occur for a feature, you could use `errorCount[errorName].add()`.

Labeled metrics come in two forms:

- **Static labels**: The labels are specified at build time in the `metrics.yaml` file, in the `labels` parameter.
  If a label that isn't part of this set is used at run time, it is converted to the special label `__other__`.
  The number of static labels is limited to 100 per metric.

- **Dynamic labels**: The labels aren't known at build time, so are set at run time.
  Only the first 16 labels seen by Glean will be tracked. After that, any additional labels are converted to the special label `__other__`.

> **Note**: Be careful with using arbitrary strings as labels and make sure they can't accidentally contain identifying data (like directory paths or user input).

### Label format

To ensure maximum support in database columns, labels must be made up of dot-separated identifiers with lowercase ASCII alphanumerics, containing underscores and dashes.

Specifically, they must conform to this regular expression:

```
^[a-z_][a-z0-9_-]{0,29}(\\.[a-z_][a-z0-9_-]{0,29})*$
```

Check your label:
<input type="text" label="Label" id="label">
<span id="result">unchecked</span>

## Adding or changing metric types
Glean has a [well-defined process](https://wiki.mozilla.org/Glean/Adding_or_changing_Glean_metric_types) for requesting changes to existing metric types or suggesting the implementation of new metric types:

1.  Glean consumers need to file a bug in the [Data platforms & tools::Glean Metric Types](https://bugzilla.mozilla.org/enter_bug.cgi?product=Data%20Platform%20and%20Tools&component=Glean%20Metric%20Types) component, filling in the provided form;
2.  The triage owner of the Bugzilla component prioritizes this within 6 business days and kicks off the [decision making process](https://wiki.mozilla.org/Glean/Adding_or_changing_Glean_metric_types#The_decision_making_process).
3.  Once the decision process is completed, the bug is closed with a comment outlining the decision that was made.


## Deprecated metrics

- [JWE](https://docs.google.com/document/d/1nntNIiE6braTGzoKf-lx21OVDd8ssyeIeJu3jnQQfEE/edit?usp=sharing): Deprecated in [v37.0.0](https://github.com/mozilla/glean/blob/main/CHANGELOG.md#v3700-2021-04-30)
