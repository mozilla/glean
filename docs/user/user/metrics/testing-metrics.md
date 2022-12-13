# Unit testing Glean metrics

In order to support unit testing inside of client applications using the Glean SDK,
a set of testing API functions have been included.
The intent is to make the Glean SDKs easier to test 'out of the box'
in any client application it may be used in.
These functions expose a way to inspect and validate recorded metric values within the client application. but are restricted to test code only.
(Outside of a testing context, Glean APIs are otherwise write-only so that it can enforce semantics and constraints about data).

To encourage using the testing APIs, it is also possible to [generate testing coverage reports](#generating-testing-coverage-reports) to show which metrics in your project are tested.

## Example of using the test API

In order to enable metrics testing APIs in each SDK, Glean must be reset and put in testing mode.
For documentation on how to do that, refer to [Initializing - Testing API](../../reference/general/initializing.md#testing-api).

Check out full examples of using the metric testing API on each Glean SDK.
All examples omit the step of resetting Glean for tests to focus solely on metrics unit testing.

{{#include ../../../shared/tab_header.md}}

<div data-lang="Kotlin" class="tab">

```kotlin
// Record a metric value with extra to validate against
GleanMetrics.BrowserEngagement.click.record(
    BrowserEngagementExtras(font = "Courier")
)

// Record more events without extras attached
BrowserEngagement.click.record()
BrowserEngagement.click.record()

// Retrieve a snapshot of the recorded events
val events = BrowserEngagement.click.testGetValue()!!

// Check if we collected all 3 events in the snapshot
assertEquals(3, events.size)

// Check extra key/value for first event in the list
assertEquals("Courier", events.elementAt(0).extra["font"])
```

</div>

<div data-lang="Swift" class="tab">

```Swift
// Record a metric value with extra to validate against
GleanMetrics.BrowserEngagement.click.record([.font: "Courier"])

// Record more events without extras attached
BrowserEngagement.click.record()
BrowserEngagement.click.record()

// Retrieve a snapshot of the recorded events
let events = BrowserEngagement.click.testGetValue()!

// Check if we collected all 3 events in the snapshot
XCTAssertEqual(3, events.count)

// Check extra key/value for first event in the list
XCTAssertEqual("Courier", events[0].extra?["font"])
```

</div>

<div data-lang="Python" class="tab">

```python
from glean import load_metrics
metrics = load_metrics("metrics.yaml")

# Record a metric value with extra to validate against
metrics.url.visit.add(1)

# Check if we collected any events into the 'click' metric
assert metrics.url.visit.test_get_value() is not Null

# Retrieve a snapshot of the recorded events
assert 1 == metrics.url.visit.test_get_value()
```

</div>

{{#include ../../../shared/tab_footer.md}}

## Generating testing coverage reports

Glean can generate coverage reports to track which metrics are tested in your unit test suite.

There are three steps to integrate it into your continuous integration workflow: recording coverage, post-processing the results, and uploading the results.

### Recording coverage

Glean testing coverage is enabled by setting the `GLEAN_TEST_COVERAGE` environment variable to the name of a file to store results.
It is good practice to set it to the absolute path to a file, since some testing harnesses (such as `cargo test`) may change the current working directory.

```bash
GLEAN_TEST_COVERAGE=$(realpath glean_coverage.txt) make test
```

### Post-processing the results

A post-processing step is required to convert the raw output in the file specified by `GLEAN_TEST_COVERAGE` into usable output for coverage reporting tools. Currently, the only coverage reporting tool supported is [codecov.io](https://codecov.io).

This post-processor is available in the `coverage` subcommand in the [`glean_parser`](https://github.com/mozilla/glean_parser) tool.

For some build systems, `glean_parser` is already installed for you by the build system integration at the following locations:

- On Android/Gradle, `$GRADLE_HOME/glean/bootstrap-4.5.11/Miniconda3/bin/glean_parser`
- On iOS, `$PROJECT_ROOT/.venv/bin/glean_parser`
- For other systems, install `glean_parser` using `pip install glean_parser`

The `glean_parser coverage` command requires the following parameters:

  - `-f`: The output format to produce, for example `codecovio` to produce [codecov.io](https://codecov.io)'s custom format.
  - `-o`: The path to the output file, for example `codecov.json`.
  - `-c`: The input raw coverage file. `glean_coverage.txt` in the example above.
  - A list of the `metrics.yaml` files in your repository.
  
For example, to produce output for [codecov.io](https://codecov.io):

```bash
glean_parser coverage -f codecovio -o glean_coverage.json -c glean_coverage.txt app/metrics.yaml
```

In this example, the `glean_coverage.json` file is now ready for uploading to codecov.io.

### Uploading coverage

If using `codecov.io`, the uploader doesn't send coverage results for YAML files by default. Pass the `-X yaml` option to the uploader to make sure they are included:

```bash
bash <(curl -s https://codecov.io/bash) -X yaml
```
