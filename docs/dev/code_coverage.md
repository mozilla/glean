# Code Coverage

> In computer science, test coverage is a measure used to describe the degree to which the source code of a program is executed when a particular test suite runs.
> A program with high test coverage, measured as a percentage, has had more of its source code executed during testing,
> which suggests it has a lower chance of containing undetected software bugs compared to a program with low test coverage.
> ([Wikipedia](https://en.wikipedia.org/wiki/Code_coverage))

This chapter describes how to generate a traditional code coverage report over the Kotlin, Swift and Python code in the Glean SDK repository. To learn how to generate a coverage report about what metrics your project is testing, see the user documentation on [generating testing coverage reports](https://mozilla.github.io/glean/book/user/testing-metrics.html#generating-testing-coverage-reports).

## Generating Kotlin reports locally

Locally you can generate a coverage report with the following command:


```bash
./gradlew -Pcoverage :glean:build
```

After that you'll find an HTML report at the following location:

```
glean-core/android/build/reports/jacoco/jacocoTestReport/jacocoTestReport/html/index.html
```

## Generating Swift reports locally

Xcode automatically generates code coverage when running tests.
You can find the report in the Report Navigator (`View -> Navigators -> Show Report Navigator -> Coverage`).

## Generating Python reports locally

Python code coverage is determined using the [coverage.py](https://coverage.readthedocs.io/en/latest/) library.

Run

```bash
make coverage-python
```

to generate code coverage reports in the Glean virtual environment.

After running, the report will be in `htmlcov`.
