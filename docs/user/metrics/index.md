# Metrics

There are different metrics to choose from, depending on what you want to achieve:

* [Events](event.md): Records events e.g. individual occurences of user actions,
  say every time a view was open and from where.
* [Booleans](boolean.md): Records a single truth value, for example "is a11y
  enabled?"
* [Strings](string.md): Records a single Unicode string value, for example the
  name of the OS.
* [String Lists](string_list.md): Records a list of Unicode string values, for
  example the list of enabled search engines.
* [Counters](counter.md): Used to count how often something happens, for
  example, how often a certain button was pressed.
* [Timespans](timespan.md): Used to measure how much time is spent in a single
  task.
* [Timing Distributions](timing_distribution.md): Used to record the
  distribution of multiple time measurements.
* [Datetimes](datetime.md): Used to record an absolute date and time, such as
  the time the user first ran the application.
* [UUIDs](uuid.md): Used to record universally unique identifiers (UUIDs), such
  as a client ID.
* [Labeled Metrics](labeled_metric.md): Used to record multiple metrics of the
  same type under different string labels, say every time you want to get a
  count of different error types in one metric.
