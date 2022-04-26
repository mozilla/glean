# YAML Registry Format

User defined Glean pings and metrics are declared in YAML files, which must be parsed by
[`glean_parser`](https://pypi.org/project/glean-parser/) to generate public APIs
for said metrics and pings.

These files also serve the purpose of documenting metrics and pings. They are consumed by the
[`probe-scraper`](https://github.com/mozilla/probe-scraper) tool, which generates a REST API to
access metrics and pings information consumed by most other tools in the Glean ecosystem, such as
[GLAM](https://glam.telemetry.mozilla.org/) and [the Glean Dictionary](https://dictionary.telemetry.mozilla.org/).

Moreover, for products that do not wish to use the Glean Dictionary as their metrics and pings documentation source, `glean_parser` provides an option to generate Markdown documentation for metrics and pings based on these files. For more information of that, refer to the help output
of the `translate` command, by running in your terminal:

```bash
$ glean_parser translate --help
```

## `metrics.yaml` file

For a full reference on the `metrics.yaml` format, refer to the
[Metrics YAML Registry Format](metrics.md) page.

## `pings.yaml` file

For a full reference on the `pings.yaml` format, refer to the
[Pings YAML Registry Format](pings.md) page.

## `tags.yaml` file

For a full reference on the `tags.yaml` format, refer to the
[Tags YAML Registry Format](tags.md) page.
