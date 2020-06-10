# Running the tests

## Running all tests

The tests for all languages may be run from the command line:

```
make test
```

> **Windows Note:** On Windows, `make` is not available by default. While not required, installing `make` will allow you to use the convenience features in the `Makefile`.

## Running the Rust tests

The Rust tests may be run with the following command:

```
cargo test --all
```

Log output can be controlled via the environment variable `RUST_LOG` for the `glean_core` crate:

```
export RUST_LOG=glean_core=debug
```

When running tests with logging you need to tell `cargo` to not suppress output:

```
cargo test -- --nocapture
```

Tests run in parallel by default, leading to interleaving log lines.
This makes it harder to understand what's going on.
For debugging you can force single-threaded tests:

```
cargo test -- --nocapture --test-threads=1
```

## Running the Kotlin/Android tests

### From the command line

The full Android test suite may be run from the command line with:

```
./gradlew test
```

### From Android Studio

To run the full Android test suite, in the "Gradle" pane, navigate to `glean-core` -> `Tasks` -> `verification` and double-click either `testDebugUnitTest` or `testReleaseUnitTest` (depending on whether you want to run in Debug or Release mode).
You can save this task permanently by opening the task dropdown in the toolbar and selecting `"Save glean.rs:glean:android [testDebugUnitTest] Configuration"`.

To run a single Android test, navigate to the file containing the test, and right click on the green arrow in the left margin next to the test.  There you have a choice of running or debugging the test.

## Running the Swift/iOS tests

### From the command line

The full iOS test suite may be run from the command line with:

```
make test-swift
```

### From Xcode

To run the full iOS test suite, run tests in Xcode (`Product -> Test`).
To run a single Swift test, navigate to the file containing the test,
and click on the arrow in the left margin next to the test.

## Testing in CI

We run multiple tests on CI for every Pull Request and every commit to the `main` branch.
These include:

* Full Android tests
* Full iOS tests
* Rust tests
* Rust, Kotlin and Swift source code formatting
* Rust, Kotlin and Swift source code linting
* Generating documentation from Rust, Kotlin, Swift and the book
* Checking link validity of documentation
* Deploying generated documentation

These checks are required to pass before a Pull Request is merged.

### Documentation-only changes

Documentation is deployed from CI, we therefore need it to run on documentation changes.
However, some of the long-running code tests can be skipped.
For that add the following literal string to the last commit message to be pushed:

```
[doc only]
```

### Skipping CI completely

It is possible to completely skip running CI on a given push by including the following literal string in the commit message:

```
[ci skip]
```

This should only be used for metadata files, such as those in `.github`, `LICENSE` or `CODE_OF_CONDUCT.md`.
