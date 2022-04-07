# Command Line Interface

The `@mozilla/glean` package exposes the `glean` CLI.
This utility installs `glean_parser` in a virtual environment
and allows users to execute `glean_parser` command through it.

In order to get a list of all commands available run `npx glean --help`.

## Customizing virtual environment

The `glean` CLI will look for / create a `.venv` virtual environment
at the directory it is called from and install `glean_parser` there by default.
However, it is possible to customize the directory in which it looks for a virtual environment,
by setting the `VIRTUAL_ENV` environment variable. Example:

```bash
VIRTUAL_ENV="my/venv/path" npx glean --help
```

{{#include ../../../shared/blockquote-info.html}}

### What if the `glean` command is called from inside an active virtual environment?

> The `VIRTUAL_ENV` environment variable [is automatically set while a virtual environment in active](https://docs.python.org/3/library/venv.html), in which case Glean will not create a new environment, but use the currently active one.

## Preventing automatic installation of `glean_parser`

In order to prevent the `glean` CLI from installing `glean_parser` itself, it must be provided
with the path to a virtual environment that already has `glean_parser` installed.

When doing that it is important to keep the separate installation of `glean_parser` in sync with
the version expected by the Glean SDK version in use. The `glean` CLI exposes the `--glean-parser-version` command that returns the expected version of `glean_parser`
for such use cases.

With this command it is possible to something like:

```bash
pip install -U glean_parser==$(npx glean --glean-parser-version)
```
