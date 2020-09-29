# Glean SDK

![Glean logo](glean.jpeg)

The `Glean SDK` is a modern approach for a Telemetry library and is part of the [Glean project](https://docs.telemetry.mozilla.org/concepts/glean/glean.html).

To contact us you can:
- Find us in the [#glean channel on chat.mozilla.org](https://chat.mozilla.org/#/room/#glean:mozilla.org).
- To report issues or request changes, file a bug in [Bugzilla in Data Platform & Tools :: Glean: SDK](https://bugzilla.mozilla.org/enter_bug.cgi?product=Data+Platform+and+Tools&component=Glean%3A+SDK&priority=P3&status_whiteboard=%5Btelemetry%3Aglean-rs%3Am%3F%5D).
- Send an email to *glean-team@mozilla.com*.
- The Glean SDK team is: *:janerik*, *:dexter*, *:travis*, *:mdroettboom*, *:gfritzsche*, *:chutten*, *:brizental*.

The source code is available [on GitHub](https://github.com/mozilla/glean/).

## Using this book

This book is specifically about the `Glean SDK` (the client side code for collecting telemetry). Documentation about the broader end-to-end [Glean project](https://docs.telemetry.mozilla.org/concepts/glean/glean.html).

This book is divided into 5 main chapters:

### [Using the Glean SDK](user/index.html)

If you want to use the Glean SDK to report data then this is the section you should read.
It explains the first steps from integrating Glean into your project,
contains details about all available metric types
and how to do send your own custom pings.

### [Metrics collected by the Glean SDK](user/collected-metrics/metrics.md)

This chapter lists all metrics collected by the Glean SDK itself.

### [Developing the Glean SDK](dev/testing.md)

This chapter describes how to develop the Glean SDK and its various implementations.
This is relevant if you plan to contribute to the Glean SDK code base.

### [API Reference Documentation](api/index.md)

Reference documentation for the API in its various language bindings.

### [This Week in Glean](appendix/twig.md)

“This Week in Glean” is a series of blog posts that the Glean Team at Mozilla is using to try to communicate better about our work.
They could be release notes, documentation, hopes, dreams, or whatever: so long as it is inspired by Glean.

## License

The Glean SDK Source Code is subject to the terms of the Mozilla Public License v2.0.
You can obtain a copy of the MPL at <https://mozilla.org/MPL/2.0/>.
