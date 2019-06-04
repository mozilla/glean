# Running the tests

## Running all tests

The tests for all languages my be run from the command line (on Unix platforms):

```
make test
```

## Running the Rust tests

The Rust tests may be run with the following command:

```
cargo test --all
```

## Running the Kotlin/Android tests

### From the command line

The full Android test suite may be run from the command line with:

```
./gradlew test
```

### From Android Studio

To run the full Android test suite, in the "Gradle" pane, navigate to `glean-core` -> `Tasks` -> `verification` and double-click either `testDebugUnitTest` or `testReleaseUnitTest` (depending on whether you want to run in Debug or Release mode).
You can save this task permanently by opening the task dropdown in the toolbar and selecting "Save glean.rs:glean-core:android [testDebugUnitTest] Configuration".

To run a single Android test, navigate to the file containing the test, and right click on the green arrow in the left margin next to the test.  There you have a choice of running or debugging the test.
