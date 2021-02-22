# Code Coverage

> In computer science, test coverage is a measure used to describe the degree to which the source code of a program is executed when a particular test suite runs.
> A program with high test coverage, measured as a percentage, has had more of its source code executed during testing,
> which suggests it has a lower chance of containing undetected software bugs compared to a program with low test coverage.
> ([Wikipedia](https://en.wikipedia.org/wiki/Code_coverage))

## Generating Kotlin reports locally

Locally you can generate a coverage report with the following command:


```bash
./gradlew -Pcoverage :glean:build
```

After that you'll find an HTML report at the following location:

```
glean-core/android/build/reports/jacoco/jacocoTestReport/jacocoTestReport/html/index.html
```

## Generating Rust reports locally

> Generating the Rust coverage report requires a significant amount of RAM during the build.

We use [grcov](https://github.com/mozilla/grcov) to collect and aggregate code coverage information.
Releases can be found on the [grcov Release page](https://github.com/mozilla/grcov/releases).

The build process requires a Rust Nightly version. Install it using `rustup`:

```bash
rustup toolchain add nightly
```

To generate an HTML report, `genhtml` from the `lcov` package is required. Install it through your system's package manager.

After installation you can build the Rust code and generate a report:

```bash
export CARGO_INCREMENTAL=0
export RUSTFLAGS='-Zprofile -Ccodegen-units=1 -Cinline-threshold=0 -Clink-dead-code -Coverflow-checks=off -Zno-landing-pads'
cargo +nightly test

zip -0 ccov.zip `find . \( -name "glean*.gc*" \) -print`
grcov ccov.zip -s . -t lcov --llvm --branch --ignore-not-existing --ignore-dir "/*" -o lcov.info
genhtml -o report/ --show-details --highlight --ignore-errors source --legend lcov.info
```

After that you'll find an HTML report at the following location:

```
report/index.html
```

## Generating Swift reports locally

Xcode automatically generates code coverage when running tests.
You can find the report in the Report Navigator (`View -> Navigators -> Show Report Navigator -> Coverage`).

## Generating Python reports locally

Python code coverage is determined using the [coverage.py](https://coverage.readthedocs.io/en/latest/) library.

Run

```bash
make python-coverage
```

to generate code coverage reports in the Glean virtual environment.

After running, the report will be in `htmlcov`.
