# Setup the Python Build Environment

This document describes how to set up an environment for the development of the Glean Python bindings.

Instructions for installing a copy of the Glean Python bindings into your own environment for use in your project are described in [adding Glean to your project](../../user/adding-glean-to-your-project.html).

## Prerequisites

Glean requires Python 3.6 or later.

Make sure it is installed on your system and accessible on the `PATH` as `python3`.

### Setting up Rust

If you've already set up Rust for building Glean for Android or iOS, you already have everything you need and can skip this section.

Rust can be installed using `rustup`, with the following commands:

- `curl https://sh.rustup.rs -sSf | sh`
- `rustup update`

## Create a virtual environment

It is recommended to do all Python development inside a virtual environment to make sure there are no interactions with other Python libraries installed on your system.

You may want to manage your own virtual environment if, for example, you are developing a library that is using Glean and you want to install Glean into it.  If you are just developing Glean itself, however, Glean's `Makefile` will handle the creation and use of a virtual environment automatically (though the `Makefile` unfortunately does not work on Microsoft Windows).

The instructions below all have both the manual instructions and the `Makefile` shortcuts.

### Manual method

The following instructions use the basic [virtual environment functionality that comes in the Python standard library](https://docs.python.org/3/library/venv.html).
Other higher level tools exist to manage virtual environments, such as [pipenv](https://pipenv.kennethreitz.org/en/latest/) and [conda](https://docs.conda.io/en/latest/).
These will work just as well, but are not documented here.
Using an alternative tool would replace the instructions in this section only, but all other instructions below would remain the same.

To create a virtual environment, enter the following command, replacing `<venv>` with the path to the virtual environment you want to create.  You will need to do this only once.

```bash
  $ python3 -m venv <venv>
```

Then activate the environment. You will need to do this for each new shell session when you want to develop Glean.
The command to use depends on your shell environment.

| Platform | Shell           | Command to activate virtual environment |
|----------|-----------------|-----------------------------------------|
| POSIX    | bash/zsh        | `source <venv>/bin/activate`            |
|          | fish            | `. <venv>/bin/activate.fish`            |
|          | csh/tcsh        | `source <venv>bin/activate.csh`         |
|          | PowerShell Core | `<venv>/bin/Activate.ps1`               |
| Windows  | cmd.exe         | `<venv>\Scripts\activate.bat`            |
|          | PowerShell      | `<venv>\Scripts\Activate.ps1`            |

Lastly, install Glean's Python development dependencies into the virtual environment.

```bash
cd glean-core/python
pip install -r requirements_dev.txt
```

### Makefile method

```bash
  $ make python-setup
```

The default location of the virtual environment used by the make file is `glean-core/python/.venvX.Y`, where `X.Y` is the version of Python in use. This makes it possible to build and test for multiple versions of Python in the same checkout.

> *Note:* If you wish to change the location of the virtual environment that the `Makefile` uses, pass the `GLEAN_PYENV` environment variable: `make python-setup GLEAN_PYENV=mypyenv`.

By default, the `Makefile` installs the latest version available of each of Glean's dependencies.  If you wish to install the minimum required versions instead, (useful primarily to ensure that Glean doesn't unintentionally use newer APIs in its dependencies) pass the `GLEAN_PYDEPS=min` environment variable: `make python-setup GLEAN_PYDEPS=min`.

## Build the Python bindings

### Manual method

Building the Python bindings also builds the Rust shared object for the Glean SDK core.

```bash
  $ cd glean-core/python
  $ python setup.py build install
```

### Makefile method

This will implicitly setup the Python environment, rebuild the Rust core and then build the Python bindings.

```bash
  $ make build-python
```

## Running the Python unit tests

### Manual method

Make sure the Python bindings are built, then:

```bash
  $ cd glean-core/python
  $ py.test
```

### Makefile method

The following will run the Python unit tests using `py.test`:

```bash
  $ make test-python
```

You can send extra parameters to the `py.test` command by setting the `PYTEST_ARGS` variable:

```bash
  $ make test-python PYTEST_ARGS="-s --pdb"
```

## Viewing logging output

Log messages (whether originating in Python or Rust) are emitted using the Python standard library's [`logging` module](https://docs.python.org/3/library/logging.html).
This module provides a lot of possibilities for customization, but the easiest way to control the log level globally is with [`logging.basicConfig`](https://docs.python.org/3/library/logging.html#logging.basicConfig):

```python
import logging
logging.basicConfig(level=logging.DEBUG)
```

## Linting, formatting and type checking

The Glean Python bindings use the following tools:

- Linting: [flake8](http://flake8.pycqa.org/en/latest/)
- Formatting: [black](https://black.readthedocs.io/en/stable/)
- Type-checking: [mypy](http://mypy-lang.org/)

### Manual method

```bash
  $ cd glean-core/python
  $ flake8 glean tests
  $ black glean tests
  $ mypy glean
```

### Makefile method

To just check the lints:

```bash
  $ make pythonlint
```

To reformat the Python files in-place:

```bash
  $ make pythonfmt
```

## Building the Python API docs

The Python API docs are built using [pdoc3](https://pdoc3.github.io/pdoc/).

### Manual method

```bash
  $ python -m pdoc --html glean --force -o build/docs/python
```

### Makefile method

```bash
  $ make python-docs
```

## Building wheels for Linux

Building on Linux using the above instructions will create Linux binaries that dynamically link against the version of `libc` installed on your machine. 
This generally will not be portable to other Linux distributions, and PyPI will not even allow it to be uploaded.
In order to create wheels that can be installed on the broadest range of Linux distributions, the Python Packaging Authority's [manylinux](https://github.com/pypa/manylinux) project maintains a Docker image for building compatible Linux wheels.

The CircleCI configuration handles making these wheels from tagged releases.
If you need to reproduce this locally, see the CircleCI job `pypi-linux-release` for an example of how this Docker image is used in practice.

## Building wheels for Windows

The official wheels for Windows are produced on a Linux virtual machine using the Mingw toolchain.

The CircleCI configuration handles making these wheels from tagged releases.
If you need to reproduce this locally, see the CircleCI job `pypi-windows-release` for an example of how this is done in practice.

