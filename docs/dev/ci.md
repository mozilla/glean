# How the Glean CI works

Current build status: [![Test](https://github.com/mozilla/glean/actions/workflows/test.yml/badge.svg?branch=main)](https://github.com/mozilla/glean/actions/workflows/test.yml?query=branch%3Amain)

The `mozilla/glean` repository uses [GitHub Actions] and [TaskCluster] to run tests on each Pull Request and each merge to the `main` branch.
Releases are built on [TaskCluster] and [CircleCI].

## Which tasks we run

### Pull Request - testing

For each opened pull request we run a number of checks to ensure consistent formatting, no lint failures and that tests across all language bindings pass.
These checks are required to pass before a Pull Request is merged.
This includes the following tasks:

On GitHub Actions:

* Formatting & lints for Rust, Kotlin, Swift, Python & YAML
* License checks across dependencies, a schema check and a check of the internal metrics documentation
* Builds & tests for Rust, Kotlin, Swift & Python
* Documentation generation for Rust, Python and the book, including spell check & link check

Each task only runs when the pull request changes files it covers, as listed in `.github/changed-files.yaml`.
Changes to `.github/workflows/test.yml` run all of them.

On TaskCluster:

* A full Android build & test

For all tasks you can always find the full log and generated artifacts by following the "Details" links on each task.

### Merge to `main`

When pull requests are merged to the `main` branch we run all of the pull request checks, regardless of which files changed.
Additionally we deploy the documentation.

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

### Skipping CI completely

It is possible to completely skip running CI checks on a pull request.

To skip tasks on GitHub Actions include the following literal string in the commit message.  
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
[GitHub Actions]: https://docs.github.com/actions
[TaskCluster]: https://taskcluster.net/
[newbugzilla]: https://bugzilla.mozilla.org/enter_bug.cgi?product=Data+Platform+and+Tools&component=Glean%3A+SDK&priority=P3&status_whiteboard=%5Btelemetry%3Aglean-rs%3Am%3F%5D
[glean-releases]: https://github.com/mozilla/glean/releases
[maven]: https://maven.mozilla.org/?prefix=maven2/org/mozilla/telemetry/
