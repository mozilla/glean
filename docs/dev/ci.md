# How the Glean CI works

Current build status: [![Build Status](https://img.shields.io/circleci/build/github/mozilla/glean/main)](https://circleci.com/gh/mozilla/glean)

The `mozilla/glean` repository uses both [CircleCI] and [TaskCluster] to run tests on each Pull Request,
each merge to the `main` branch and on releases.

## Which tasks we run

### Pull Request - testing

For each opened pull request we run a number of checks to ensure consistent formatting, no lint failures and that tests across all language bindings pass.
These checks are required to pass before a Pull Request is merged.
This includes by default the following tasks:

On CircleCI:

* Formatting & lints for Rust, Kotlin, Swift & Python
* Consistency checks for generated header files, license checks across dependencies and a schema check
* Builds & tests for Rust, Kotlin, Python & C#
    * The Python language bindings are build for multiple platforms and Python versions
* Documentation generation for all languages and the book, including spell check & link check
* By default no Swift/iOS tests are launched. These can be run manually. See below.

On TaskCluster:

* A full Android build & test

For all tasks you can always find the full log and generated artifacts by following the "Details" links on each task.

#### iOS tests on Pull Request

Due to the long runtime and costs iOS tests are not run by default on Pull Requests.
Admins of the `mozilla/glean` repository can run them on demand.

1. On the pull request scroll to the list of all checks running on that PR on the bottom.
2. Find the job titled "ci/circleci: iOS/hold".
3. Click "Details" to get to Circle CI.
4. Click on the "hold" task, then approve it.
5. The iOS tasks will now run.

### Merge to `main`

Current build status: [![Build Status](https://img.shields.io/circleci/build/github/mozilla/glean/main)](https://circleci.com/gh/mozilla/glean)

When pull requests are merged to the `main` branch we run the same checks as we do on pull requests.
Additionally we run the following tasks:

* Documentation deployment
* iOS build, unit & integration test

If you notice that they fail please take a look and [open a bug][newbugzilla] to investigate build failures.

### Releases

When [cutting a new release](cut-a-new-release.md) of the Glean SDK our CI setup is responsible for building, packaging and uploading release builds.
We don't run the full test suite again.

We run the following tasks:

On CircleCI:

* Build the iOS framework & push to [a GitHub release][glean-releases]
* Build the Python wheel & push to [a GitHub release][glean-releases]

On TaskCluster:

* Build & package Glean for Android and push a [release to Maven][maven]
* Build & package the Glean Gradle Plugin and push a [release to Maven][maven]

On release the full TaskCluster suite takes up to 30 minutes to run.

#### How to find TaskCluster tasks

1. Go to [GitHub releases][glean-releases] and find the version
2. Go to the release commit, indicated by its commit id on the left
3. Click the green tick or red cross left of the commit title to open the list of all checks.
4. Find the "Decision task" and click "Details"
5. From the list of tasks pick the one you want to look at and follow the "View task in Taskcluster" link.

## Special behavior

### Documentation-only changes

Documentation is deployed from CI, we therefore need it to run on documentation changes.
However, some of the long-running code tests can be skipped.
For that add the following literal string to the last commit message of your pull request:

```
[doc only]
```

### Skipping CI completely

It is possible to completely skip running CI checks on a pull request.

To skip tasks on CircleCI include the following literal string in the commit message.  
To skip tasks on TaskCluster add the following to the title of your pull request.

```
[ci skip]
```

This should only be used for metadata files, such as those in `.github`, `LICENSE` or `CODE_OF_CONDUCT.md`.

### Release-like Android builds

Release builds are done on [TaskCluster].
As it builds for multiple platforms this task is quite long and does not run on pull requests by default.

It is possible trigger the task on pull requests.
Add the following literal string to the pull request **title**:

```
[ci full]
```

If added after initially opening the pull request, you need to close, then re-open the pull request to trigger a new build.

The `Decision Task` will spawn `module-build-glean` and `module-build-glean-gradle-plugin` tasks.
When they finish you will find the generated files under `Artifacts` on TaskCluster.

[CircleCI]: https://circleci.com
[TaskCluster]: https://taskcluster.net/
[newbugzilla]: https://bugzilla.mozilla.org/enter_bug.cgi?product=Data+Platform+and+Tools&component=Glean%3A+SDK&priority=P3&status_whiteboard=%5Btelemetry%3Aglean-rs%3Am%3F%5D
[glean-releases]: https://github.com/mozilla/glean/releases
[maven]: https://maven.mozilla.org/?prefix=maven2/org/mozilla/telemetry/
