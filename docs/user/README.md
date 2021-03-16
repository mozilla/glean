# Introduction

Glean is a modern approach for a telemetry library
and is part of the [Glean project](https://docs.telemetry.mozilla.org/concepts/glean/glean.html).

![Glean logo](glean.jpeg)

There are two implementations of Glean, with support for 5 different programming languages in total.
Both implementations strive to contain the same features with similar, but idiomatic APIs.

Unless clearly stated otherwise, regard the text in this book as valid for both clients
and all the supported programming languages and environments.

### [The Glean SDK](https://github.com/mozilla/glean)

The Glean SDK is an implementation of Glean in Rust, with language bindings for **Kotlin**,
**Python**, **Rust** and **Swift**.

For development documentation on the `Glean SDK`,
refer to [the Glean SDK development book](../dev/index.html).

To report issues or request changes on the Glean SDK,
file a bug in [Bugzilla in Data Platform & Tools :: Glean: SDK](https://bugzilla.mozilla.org/enter_bug.cgi?product=Data+Platform+and+Tools&component=Glean%3A+SDK&priority=P3&status_whiteboard=%5Btelemetry%3Aglean-rs%3Am%3F%5D).

### [Glean.js](https://github.com/mozilla/glean.js)

Glean.js is an implementation of Glean in **Javascript**. Currently, it only has support
for usage in web extensions.

For development documentation on `Glean.js`,
refer to [the Glean.js development documentation](https://github.com/mozilla/glean.js/tree/main/docs).

To report issues or request changes on Glean.js,
file a bug in [Bugzilla in Data Platform & Tools :: Glean.js][gleanjs-bugs].

> **Note** Glean.js is still in development and does not provide all the features the Glean SDK does.
> Feature parity will be worked on after initial validation. Do not hesitate to [file a bug][gleanjs-bugs]
> if you want to use Glean.js and is missing some key Glean feature.
## Sections

### [Using Glean](./user/index.html)

In this section we describe how to use Glean in your own libraries and applications.
It explains the first steps of integrating Glean into your project, choosing the right metric type for you,
debugging products that use Glean and Glean's built-in error reporting mechanism.
If you want to start using Glean to report data, this is the section you should read.

### [Metric Types](./user/metrics/index.html)

This sections lists all the metric types provided by Glean, with examples on how to define them
and record data using them. Before diving into Glean's metric types details, don't forget to
read the [Choosing a metric type](https://mozilla.github.io/glean/book/user/adding-new-metrics.html#choosing-a-metric-type) page.

### [Pings](./user/pings/index.html)

This section goes through what is a ping and how to define custom pings. A Glean client may provide
off-the-shelf pings, such as the [`metrics`](https://mozilla.github.io/glean/book/user/pings/metrics.html)
or [`baseline`](https://mozilla.github.io/glean/book/user/pings/baseline.html) pings. In this section,
you will also find the descriptions and the schedules of each of these pings.

### Appendix

#### [Glossary](./appendix/glossary.html)

In this book we use a lot of Glean specific terminology. In the glossary, we go through
many of the terms used throughout this book and describe exactly what we mean when we use them.

#### [Changelog](./appendix/changelog.html)

This section contains detailed notes about changes in Glean, per release.

#### [This Week in Glean](./appendix/twig.html)

“This Week in Glean” is a series of blog posts that the Glean Team at Mozilla is using to try
to communicate better about our work. They could be release notes, documentation, hopes, dreams,
or whatever: so long as it is inspired by Glean.

## Contact

To contact the Glean team you can:

- Find us in the [#glean channel on chat.mozilla.org](https://chat.mozilla.org/#/room/#glean:mozilla.org).
- Send an email to *glean-team@mozilla.com*.
- The Glean SDK team is: *:janerik*, *:dexter*, *:travis*, *:mdroettboom*, *:gfritzsche*, *:chutten*, *:brizental*.

## License

Glean.js and the Glean SDK Source Code is subject to the terms of the Mozilla Public License v2.0.
You can obtain a copy of the MPL at <https://mozilla.org/MPL/2.0/>.

[gleanjs-bugs]: https://bugzilla.mozilla.org/enter_bug.cgi?product=Data+Platform+and+Tools&component=Glean.js&priority=P4&status_whiteboard=%5Btelemetry%3Aglean-js%3Am%3F%5D
