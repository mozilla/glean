# Introduction

Glean is a modern approach for a Telemetry library and is part of the [Glean project](https://docs.telemetry.mozilla.org/concepts/glean/glean.html).

## Contact

To contact the Glean team you can:

- Find us in the [#glean channel on chat.mozilla.org](https://chat.mozilla.org/#/room/#glean:mozilla.org).
- Send an email to *glean-team@mozilla.com*.
- The Glean SDK team is: *:janerik*, *:dexter*, *:travis*, *:mdroettboom*, *:gfritzsche*, *:chutten*, *:brizental*.

![Glean logo](glean.jpeg)

## Sections

### [Using Glean](./user/index.html)

If you want to start using Glean to report data then this is the section you should read. It explains the first steps of integrating Glean into your project, choosing the right metric type for you, debugging products that use Glean and even Glean's internal error reporting mechanism.

### [Metric Types](./user/metrics/index.html)

This sections lists all the metric types provided by Glean, with examples on how to define them and record data using them. Before diving into Glean's metric types details, don't forget to read the [Choosing a metric type](https://mozilla.github.io/glean/book/user/adding-new-metrics.html#choosing-a-metric-type).

### [Pings](./user/pings/index.html)

This section goes through what is a ping and how to define custom pings. A Glean client may provide off-the-shelf pings, such as the [`metrics`](https://mozilla.github.io/glean/book/user/pings/metrics.html) or [`baseline`](https://mozilla.github.io/glean/book/user/pings/baseline.html) pings. In this section you also will find the descriptions and the schedules of each of these pings.

### Appendix

#### [Glossary](./appendix/glossary.html)

In this book we use a lot of Glean specific terminology. In the glossary we go through many of the terms used throughout this book and describe exactly what we mean when we use them.

#### [Changelog](./appendix/changelog.html)

This section contains detailed notes about changes in Glean, per release.

#### [This Week in Glean](./appendix/twig.html)

“This Week in Glean” is a series of blog posts that the Glean Team at Mozilla is using to try to communicate better about our work. They could be release notes, documentation, hopes, dreams, or whatever: so long as it is inspired by Glean.

## License

Glean.js and the Glean SDK Source Code is subject to the terms of the Mozilla Public License v2.0. You can obtain a copy of the MPL at <https://mozilla.org/MPL/2.0/>.
