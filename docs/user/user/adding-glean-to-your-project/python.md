# Adding Glean to your Python project

Python projects use the Python Language Bindings provided by the Glean SDK.

## Setting up the dependency

We recommend using a virtual environment for your work to isolate the dependencies for your project. There are many popular abstractions on top of virtual environments in the Python ecosystem which can help manage your project dependencies.

The Glean SDK Python bindings currently have [prebuilt wheels on PyPI for Windows (i686 and x86_64), Linux/glibc (x86_64) and macOS (x86_64)](https://pypi.org/project/glean-sdk/#files).
For other platforms, including BSD or Linux distributions that don't use glibc, such as Alpine Linux, the `glean_sdk` package will be built from source on your machine.
This requires that Cargo and Rust are already installed.
The easiest way to do this is through [rustup](https://rustup.rs/).

Once you have your virtual environment set up and activated, you can install the Glean SDK into it using:

```bash
$ python -m pip install glean_sdk
```

{{#include ../../../shared/blockquote-warning.html}}

##### Important

> The Glean SDK Python bindings make extensive use of type annotations to catch type related errors at build time. We highly recommend adding [mypy](https://mypy-lang.org) to your continuous integration workflow to catch errors related to type mismatches early.

## Consuming YAML registry files

For Python, the `metrics.yaml` file must be available and loaded at runtime.

If your project is a script (i.e. just Python files in a directory), you can load the `metrics.yaml` using:

```Python
from glean import load_metrics

metrics = load_metrics("metrics.yaml")

# Use a metric on the returned object
metrics.your_category.your_metric.set("value")
```

If your project is a distributable Python package, you need to include the `metrics.yaml` file using [one of the myriad ways to include data in a Python package](https://setuptools.readthedocs.io/en/latest/setuptools.html#including-data-files) and then use [`pkg_resources.resource_filename()`](https://setuptools.readthedocs.io/en/latest/pkg_resources.html#resource-extraction) to get the filename at runtime.

```Python
from glean import load_metrics
from pkg_resources import resource_filename

metrics = load_metrics(resource_filename(__name__, "metrics.yaml"))

# Use a metric on the returned object
metrics.your_category.your_metric.set("value")
```

### Automation steps

#### Documentation

The documentation for your application or library's metrics and pings are written in `metrics.yaml` and `pings.yaml`. 

For Mozilla projects, this SDK documentation is automatically published on the [Glean Dictionary](https://dictionary.telemetry.mozilla.org). For non-Mozilla products, it is recommended to generate markdown-based documentation of your metrics and pings into the repository. For most languages and platforms, this transformation can be done automatically as part of the build. However, for some language bindings the integration to automatically generate docs is an additional step.

The Glean SDK provides a commandline tool for automatically generating markdown documentation from your `metrics.yaml` and `pings.yaml` files. To perform that translation, run `glean_parser`'s `translate` command:

```sh
python3 -m glean_parser translate -f markdown -o docs metrics.yaml pings.yaml
```

To get more help about the commandline options:

```sh
python3 -m glean_parser translate --help
```

We recommend integrating this step into your project's documentation build. The details of that integration is left to you, since it depends on the documentation tool being used and how your project is set up.

#### Metrics linting

Glean includes a "linter" for `metrics.yaml` and `pings.yaml` files called the `glinter` that catches a number of common mistakes in these files.

As part of your continuous integration, you should run the following on your `metrics.yaml` and `pings.yaml` files:

```sh
python3 -m glean_parser glinter metrics.yaml pings.yaml
```
