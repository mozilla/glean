# Adding Glean to your Python project

This page provides a step-by-step guide on how to integrate the Glean library into a Python project.

Nevertheless this is just one of the required steps for integrating Glean successfully into a project. Check you the full [Glean integration checklist](./index.md) for a comprehensive list of all the steps involved in doing so.


## Setting up the dependency

We recommend using a virtual environment for your work to isolate the dependencies for your project. There are many popular abstractions on top of virtual environments in the Python ecosystem which can help manage your project dependencies.

The Glean Python SDK currently has [prebuilt wheels on PyPI for Windows (i686 and x86_64), Linux/glibc (x86_64) and macOS (x86_64)](https://pypi.org/project/glean-sdk/#files).
For other platforms, including BSD or Linux distributions that don't use glibc, such as Alpine Linux, the `glean_sdk` package will be built from source on your machine.
This requires that Cargo and Rust are already installed.
The easiest way to do this is through [rustup](https://rustup.rs/).

Once you have your virtual environment set up and activated, you can install the Glean Python SDK into it using:

```bash
$ python -m pip install glean_sdk
```

{{#include ../../../shared/blockquote-warning.html}}

##### Important

> Installing Python wheels is still a rapidly evolving feature of the Python package ecosystem.
> If the above command fails, try upgrading `pip`:
> ```bash
> python -m pip install --upgrade pip
> ```

{{#include ../../../shared/blockquote-warning.html}}

##### Important

> The Glean Python SDK make extensive use of type annotations to catch type related errors at build time. We highly recommend adding [mypy](https://mypy-lang.org) to your continuous integration workflow to catch errors related to type mismatches early.

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

For Mozilla projects, this SDK documentation is automatically published on the [Glean Dictionary](https://dictionary.telemetry.mozilla.org). For non-Mozilla products, it is recommended to generate markdown-based documentation of your metrics and pings into the repository. For most languages and platforms, this transformation can be done automatically as part of the build. However, for some SDKs the integration to automatically generate docs is an additional step.

The Glean Python SDK provides a commandline tool for automatically generating markdown documentation from your `metrics.yaml` and `pings.yaml` files. To perform that translation, run `glean_parser`'s `translate` command:

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

### Parallelism

Most Glean SDKs use a separate worker thread to do most of its work, including any I/O. This thread is fully managed by the SDK as an implementation detail. Therefore, users should feel free to use the Glean SDKs wherever they are most convenient, without worrying about the performance impact of updating metrics and sending pings.

Since the Glean SDKs perform disk and networking I/O, they try to do as much of their work as possible on separate threads and processes.
Since there are complex trade-offs and corner cases to support Python parallelism, it is hard to design a one-size-fits-all approach.

#### Default behavior

When using the Python SDK, most of the Glean's work is done on a separate thread, managed by the SDK itself.
The SDK releases the Global Interpreter Lock (GIL) for most of its operations, therefore your application's threads should not be in contention with the Glean's worker thread.

The Glean Python SDK installs an [`atexit` handler](https://docs.python.org/3/library/atexit.html) so that its worker thread can cleanly finish when your application exits.
This handler will wait up to 30 seconds for any pending work to complete.

By default, ping uploading is performed in a separate child process. This process will continue to upload any pending pings even after the main process shuts down. This is important for commandline tools where you want to return control to the shell as soon as possible and not be delayed by network connectivity.

#### Cases where subprocesses aren't possible

The default approach may not work with applications built using [`PyInstaller`](https://www.pyinstaller.org/) or similar tools which bundle an application together with a Python interpreter making it impossible to spawn new subprocesses of that interpreter. For these cases, there is an option to ensure that ping uploading occurs in the main process. To do this, set the `allow_multiprocessing` parameter on the `glean.Configuration` object to `False`.

#### Using the `multiprocessing` module

Additionally, the default approach does not work if your application uses the `multiprocessing` module for parallelism.
The Glean Python SDK can not wait to finish its work in a `multiprocessing` subprocess, since `atexit` handlers are not supported in that context.  
Therefore, if the Glean Python SDK detects that it is running in a `multiprocessing` subprocess, all of its work that would normally run on a worker thread will run on the main thread.
In practice, this should not be a performance issue: since the work is already in a subprocess, it will not block the main process of your application.
