## Offline builds of Android applications that use Glean

The Glean SDK has basic support for building Android applications that use Glean in offline mode.

The Glean SDK uses a Python script, [`glean_parser`](https://github.com/mozilla/glean_parser/) to generate code for metrics from the `metrics.yaml` and `pings.yaml` files when Glean-using applications are built. When online, the pieces necessary to run this script are installed automatically.

For offline builds, the Python environment, and packages of `glean_parser` and its dependencies must be provided prior to building the Glean-using application.

To build a Glean-using application in offline mode, do the following:

- Install Python 3.6 or later and ensure it's on the `PATH`.

  - On Linux, installing Python from your Linux distribution's package manager is usually sufficient.

  - On macOS, installing Python from [`homebrew`](https://brew.sh/) is known to work, but other package managers may also work.

  - On Windows, we recommend installing one of the official Python installers from [python.org](https://python.org).

- Determine the version of `glean_parser` required.

  - It can be really difficult to manually determine the version of `glean_parser` that is required for a given application, since it needs to be tracked through `android-components`, to `glean-core` and finally to `glean_parser`. The required version of `glean_parser` can be determined by running the following at the top-level of the Glean-using application:

    ```sh
    $ ./gradlew | grep "Requires glean_parser"
    Requires glean_parser==1.28.1
    ```

- Download packages for `glean_parser` and its dependencies:

  - In the root directory of the Glean-using project, create a directory called `glean-wheels` and `cd` into it.

  - Download packages for `glean_parser` and its dependencies, replacing `X.Y.Z` with the correct version of `glean_parser`:

    ```sh
    $ python3 -m pip download glean_parser==X.Y.Z
    ```

- Build the Glean-using project using `./gradlew`, but passing in the `--offline` flag.

There are a couple of environment variables that control offline building:

- To override the location of the Python interpreter to use, set the `GLEAN_PYTHON` environment variable. If unset, the first Python interpreter on the `PATH` will be used.

- To override the location of the downloaded Python wheels, set the `GLEAN_PYTHON_WHEELS_DIR` environment variable.  If unset `${projectDir}/glean-wheels` will be used.
