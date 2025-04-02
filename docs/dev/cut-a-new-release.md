# Glean release process

The Glean SDK consists of multiple libraries for different platforms and targets.
The main supported libraries are released as one.
Development happens on the main repository <https://github.com/mozilla/glean>.
See [Contributing](contributing.md) for how to contribute changes to the Glean SDK.

The development & release process roughly follows the [GitFlow model](https://nvie.com/posts/a-successful-git-branching-model/).

> **Note:** The rest of this section assumes that `upstream` points to the `https://github.com/mozilla/glean` repository,
> while `origin` points to the developer fork.
> For some developer workflows, `upstream` can be the same as `origin`.

**Table of Contents**:

* [Published artifacts](#published-artifacts)
* [Standard release](#standard-release)
* [Hotfix release for latest version](#hotfix-release-for-latest-version)
* [Hotfix release for previous version](#hotfix-release-for-previous-version)
* [Upgrading mozilla-central to a new version of Glean](#upgrading-mozilla-central-to-a-new-version-of-glean)
* [Recovering from a failed automated release](#recovering-from-a-failed-automated-release)

## Published artifacts

* The Kotlin libraries are published to [Mozilla Maven](https://maven.mozilla.org/?prefix=maven2/org/mozilla/telemetry/).
* Python bindings are published on PyPI: [glean-sdk](https://pypi.org/project/glean-sdk/).
* Swift package available as [mozilla/glean-swift](https://github.com/mozilla/glean-swift).
* Rust crates are published on crates.io: [glean](https://crates.io/crates/glean), [glean-core](https://crates.io/crates/glean-core), [glean-ffi](https://crates.io/crates/glean-ffi).

## Standard Release

Releases can only be done by one of the Glean maintainers.

* Main development branch: `main`
* Main release branch: `release`
* Specific release branch: `release-vX.Y.Z`
* Hotfix branch: `hotfix-X.Y.(Z+1)`

### Create a release branch

0. Announce your intention to create a release in the team chat.
1. Create a release branch from the `main` branch:
    ```
    git checkout -b release-v25.0.0 main
    ```
2. Update the changelog .
    1. Add any missing important changes under the `Unreleased changes` headline.
    2. Commit any changes to the changelog file due to the previous step.
3. Run `bin/prepare-release.sh <new version>` to bump the version number.
    1. The new version should be the next patch, minor or major version of what is currently released.
    2. Let it create a commit for you.
4. Push the new release branch:
    ```
    git push upstream release-v25.0.0
    ```
5. Open a pull request to merge back the specific release branch to the development branch: <https://github.com/mozilla/glean/compare/main...release-v25.0.0?expand=1>
    * Wait for CI to finish on that branch and ensure it's green.
    * Do not merge this PR yet!
6. Apply additional commits for bug fixes to this branch.
    * Adding large new features here is strictly prohibited. They need to go to the `main` branch and wait for the next release.

### Finish a release branch

When CI has finished and is green for your specific release branch, you are ready to cut a release.

1. Check out the main release branch:
    ```
    git checkout release
    ```
2. Merge the specific release branch:
    ```
    git merge --no-ff -X theirs release-v25.0.0
    ```
3. Push the main release branch:
    ```
    git push upstream release
    ```
4. Tag the release on GitHub:
    1. [Create a New Release](https://github.com/mozilla/glean/releases/new) in the GitHub UI (`Releases > Draft a New Release`).
    2. Enter `v<myversion>` as the tag. It's important this is the same as the version you specified to the `prepare_release.sh` script, with the `v` prefix added.
    3. Select the `release` branch as the target.
    4. Under the description, paste the contents of the release notes from `CHANGELOG.md`.
    5. Click the green `Publish Release` button.
5. Wait for the CI build to complete for the tag.
    * You can check [on CircleCI for the running build](https://circleci.com/gh/mozilla/glean).
    * You can find the TaskCluster task on the corresponding commit. See [How to find TaskCluster tasks](ci.md#how-to-find-taskcluster-tasks) for details.
    * On rare occasions CI tasks may fail.
      You can safely rerun them on CircleCI by selecting _Rerun_, then _Rerun workflow from failed_ on the top right on the failed task.
6. Merge the Pull Request opened previously.
    * This is important so that no changes are lost.
    * If this PR is "trivial" (no bugfixes or merge conflicts of note from earlier steps) you may land it without review.
    * There is a separate CircleCI task for the release branch, ensure that it also completes.
7. Once the above pull request lands, delete the specific release branch.
8. Pick a song and post a link in the team chat.
9. Post a message to [#glean:mozilla.org](https://chat.mozilla.org/#/room/#glean:mozilla.org) announcing the new release.
    * Include a copy of the release-specific changelog if you want to be fancy.

## Hotfix release for latest version

If the latest released version requires a bug fix, a hotfix branch is used.

### Create a hotfix branch

0. Announce your intention to create a release in the team chat.
1. Create a hotfix branch from the main release branch:
    ```
    git checkout -b hotfix-v25.0.1 release
    ```
3. Run `bin/prepare-release.sh <new version>` to bump the version number.
    1. The new version should be the next patch version of what is currently released.
    2. Let it create a commit for you.
4. Push the hotfix branch:
    ```
    git push upstream hotfix-v25.0.1
    ```
5. Create a local hotfix branch for bugfixes:
    ```
    git checkout -b bugfix hotfix-v25.0.1
    ```
5. Fix the bug and commit the fix in one or more separate commits.
6. Push your bug fixes and create a pull request against the hotfix branch: <https://github.com/mozilla/glean/compare/hotfix-v25.0.1...your-name:bugfix?expand=1>
7. When that pull request lands, wait for CI to finish on that branch and ensure it's green:
    * <https://circleci.com/gh/mozilla/glean/tree/hotfix-v25.0.1>
    * You can find the TaskCluster task on the corresponding commit. See [How to find TaskCluster tasks](ci.md#how-to-find-taskcluster-tasks) for details.

### Finish a hotfix branch

When CI has finished and is green for your hotfix branch, you are ready to cut a release, similar to a normal release:

1. Check out the main release branch:
    ```
    git checkout release
    ```
2. Merge the hotfix branch:
    ```
    git merge --no-ff hotfix-v25.0.1
    ```
3. Push the main release branch:
    ```
    git push upstream release
    ```
4. Tag the release on GitHub:
    1. [Create a New Release](https://github.com/mozilla/glean/releases/new) in the GitHub UI (`Releases > Draft a New Release`).
    2. Enter `v<myversion>` as the tag. It's important this is the same as the version you specified to the `prepare_release.sh` script, with the `v` prefix added.
    3. Select the `release` branch as the target.
    4. Under the description, paste the contents of the release notes from `CHANGELOG.md`.
5. Wait for the CI build to complete for the tag.
    * You can check [on CircleCI for the running build](https://circleci.com/gh/mozilla/glean).
    * You can find the TaskCluster task on the corresponding commit. See [How to find TaskCluster tasks](ci.md#how-to-find-taskcluster-tasks) for details.
6. Send a pull request to merge back the hotfix branch to the development branch: <https://github.com/mozilla/glean/compare/main...hotfix-v25.0.1?expand=1>
    * This is important so that no changes are lost.
    * This might have merge conflicts with the `main` branch, which you need to fix before it is merged.
7. Once the above pull request lands, delete the hotfix branch.

## Hotfix release for previous version

If you need to release a hotfix for a previously released version (that is: not the latest released version), you need a support branch.

> **Note**: This should rarely happen. We generally support only the latest released version of Glean.

### Create a support and hotfix branch

0. Announce your intention to create a release in the team chat.
1. Create a support branch from the version tag and push it:
    ```
    git checkout -b support/v24.0 v24.0.0
    git push upstream support/v24.0
    ```
2. Create a hotfix branch for this support branch:
    ```
    git checkout -b hotfix-v24.0.1 support/v24.0
    ```
3. Fix the bug and commit the fix in one or more separate commits into your hotfix branch.
4. Push your bug fixes and create a pull request against the support branch: <https://github.com/mozilla/glean/compare/support/v24.0...your-name:hotfix-v24.0.1?expand=1>
5. When that pull request lands, wait for CI to finish on that branch and ensure it's green:
    * <https://circleci.com/gh/mozilla/glean/tree/support/v24.0>
    * You can find the TaskCluster task on the corresponding commit. See [How to find TaskCluster tasks](ci.md#how-to-find-taskcluster-tasks) for details.

### Finish a support branch

1. Check out the support branch:
    ```
    git checkout support/v24.0
    ```
2. Update the changelog .
    1. Add any missing important changes under the `Unreleased changes` headline.
    2. Commit any changes to the changelog file due to the previous step.
3. Run `bin/prepare-release.sh <new version>` to bump the version number.
    1. The new version should be the next patch version of the support branch.
    2. Let it create a commit for you.
3. Push the support branch:
    ```
    git push upstream support/v24.0
    ```
4. Tag the release on GitHub:
    1. [Create a New Release](https://github.com/mozilla/glean/releases/new) in the GitHub UI (`Releases > Draft a New Release`).
    2. Enter `v<myversion>` as the tag. It's important this is the same as the version you specified to the `prepare_release.sh` script, with the `v` prefix added.
    3. Select the support branch (e.g. `support/v24.0`) as the target.
    4. Under the description, paste the contents of the release notes from `CHANGELOG.md`.
5. Wait for the CI build to complete for the tag.
    * You can check [on CircleCI for the running build](https://circleci.com/gh/mozilla/glean).
    * You can find the TaskCluster task on the corresponding commit. See [How to find TaskCluster tasks](ci.md#how-to-find-taskcluster-tasks) for details.
6. Send a pull request to merge back any bug fixes to the development branch: <https://github.com/mozilla/glean/compare/main...support/v24.0?expand=1>
    * This is important so that no changes are lost.
    * This might have merge conflicts with the `main` branch, which you need to fix before it is merged.
7. Once the above pull request lands, delete the support branch.

## Upgrading mozilla-central to a new version of Glean

Glean is integrated into Mozilla products in mozilla-central (Firefox Desktop, Firefox for Android, Focus for Android).
See [Updating the Glean SDK](https://firefox-source-docs.mozilla.org/toolkit/components/glean/dev/updating_sdk.html) on how to update the Glean SDK in mozilla-central.

## Recovering from a failed automated release

In rare circumstances the automated release on CI can fail to publish updated packages.
This usually can be fixed with some manual steps.

### CI re-run

In some circumstances it's possible to re-run the CI tasks.
This should be done if any of the tasks fail with an intermittent issues (e.g. broken network connectivity, package repository APIs unreachable, ...).

Find the release task on <https://app.circleci.com/pipelines/github/mozilla/glean/> and select "Rerun workflow from failed" from the "Rerun" drop-down.
Monitor the tasks for successful completion.

### Manual release

If any of the package releases fail for some other reason it might be possible to manually publish the package, using the same commands being run on CI.

#### Manual release: Rust crates

Make sure you're logged in to `crates.io`:

```
cargo login
```

Check out the newly created git tag in your Glean repository:

```
git checkout v63.0.0
```

Then execute the following commands:

```
pushd glean-core
cargo publish --verbose

pushd rlb
cargo publish --verbose
```

#### Manual release: Python package

The release process for Python packages is more involved and requires publishing pre-compiled packages for every platform.

CI already generates those packages, so if only the upload step fails try re-running the particular task or rerun the job with SSH enabled.
Grab the generated files from `target/wheels/` in the project directory.

Use `twine` locally to publish the file:

```
.venv/bin/python3 -m twine upload target/wheels/*
```

#### Manual release: Swift package

The Swift XCFramework is attached as an artifact to the [GitHub release](https://github.com/mozilla/glean/releases).
The Swift package is in an external repository: <https://github.com/mozilla/glean-swift>

If the step to create a new release tag on `glean-swift` fails, the step can be done manually:

1. Clone the `glean-swift` repository:

   ```
   git clone https://github.com/mozilla/glean-swift
   cd glean-swift
   ```

1. Run the publish step:

   ```
   ./bin/update.sh v63.0.0
   git push origin main
   git push origin v63.0.0
   ```

1. Verify the new tag is listed on <https://github.com/mozilla/glean-swift/tags>.
