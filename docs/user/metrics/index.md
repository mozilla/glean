# Metrics

There are different metrics to choose from, depending on what you want to achieve:

* [Events](event.md): Records events e.g. individual occurences of user actions,
  say every time a view was open and from where.
* [Boolean](boolean.md): Records a single truth value, for example "is a11y
  enabled?"
* [String](string.md): Records a single Unicode string value, for example the
  name of the OS.
* [String List](string_list.md): Records a list of Unicode string values, for
  example the list of enabled search engines.
* [Counter](counter.md): Used to count how often something happens, for
  example, how often a certain button was pressed.
* [Timespan](timespan.md): Used to measure how much time is spent in a single
  task.
* [Timing Distribution](timing_distribution.md): Used to record the
  distribution of multiple time measurements.
* [Datetime](datetime.md): Used to record an absolute date and time, such as
  the time the user first ran the application.
* [UUID](uuid.md): Used to record universally unique identifiers (UUIDs), such
  as a client ID.
* [Labeled Metrics](labeled_metric.md): Used to record multiple metrics of the
  same type under different string labels, say every time you want to get a
  count of different error types in one metric.
