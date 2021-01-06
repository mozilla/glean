# Glean release process

The Glean SDK consists of multiple libraries for different platforms and targets.
The main supported libraries are released as one.
Development happens on the main repository <https://github.com/mozilla/glean>.
See [Contributing](../contributing.md) for how to contribute changes to the Glean SDK.

The development & release process roughly follows the [GitFlow model](https://nvie.com/posts/a-successful-git-branching-model/).

> **Note:** The rest of this section assumes that `upstream` points to the `https://github.com/mozilla/glean` repository,
> while `origin` points to the developer fork.
> For some developer workflows, `upstream` can be the same as `origin`.

**Table of Contents**:

* [Published artifacts](#published-artifacts)
* [Standard release](#standard-release)
* [Hotfix release for latest version](#hotfix-release-for-latest-version)
* [Hotfix release for previous version](#hotfix-release-for-previous-version)
* [Upgrading android-components to a new version of Glean](#upgrading-android-components-to-a-new-version-of-glean)

## Published artifacts

* The Kotlin libraries are published to [GitHub Releases](https://github.com/mozilla/glean/releases) and [Mozilla Maven](https://maven.mozilla.org/?prefix=maven2/org/mozilla/telemetry/).
* Python bindings are published on PyPI: [glean-sdk](https://pypi.org/project/glean-sdk/).
* iOS framework artifacts: [GitHub Releases](https://github.com/mozilla/glean/releases).
* Rust crates are published on crates.io: [glean-core](https://crates.io/crates/glean-core), [glean-ffi](https://crates.io/crates/glean-ffi).

## Standard Release

Releases can only be done by one of the Glean maintainers.

* Main development branch: `main`
* Main release branch: `release`
* Specific release branch: `release-vX.Y.Z`
* Hotfix branch: `hotfix-X.Y.(Z+1)`

### Create a release branch

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
5. Wait for CI to finish on that branch and ensure it's green:
    * <https://circleci.com/gh/mozilla/glean/tree/release-v25.0.0>
    * You can find the TaskCluster task on the corresponding commit. See [How to find TaskCluster tasks](ci.md#how-to-find-taskcluster-tasks) for details.
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
    git merge --no-ff release-v25.0.0
    ```
3. Push the main release branch:
    ```
    git push upstream release
    ```
4. Tag the release on GitHub:
    1. [Draft a New Release](https://github.com/mozilla/glean/releases/new) in the GitHub UI (`Releases > Draft a New Release`).
    2. Enter `v<myversion>` as the tag. It's important this is the same as the version you specified to the `prepare_release.sh` script, with the `v` prefix added.
    3. Select the `release` branch as the target.
     4. Under the description, paste the contents of the release notes from `CHANGELOG.md`.
5. Wait for the CI build to complete for the tag.
    * You can check [on CircleCI for the running build](https://circleci.com/gh/mozilla/glean).
    * You can find the TaskCluster task on the corresponding commit. See [How to find TaskCluster tasks](ci.md#how-to-find-taskcluster-tasks) for details.
6. Send a pull request to merge back the specific release branch to the development branch: <https://github.com/mozilla/glean/compare/main...release-v25.0.0?expand=1>
    * This is important so that no changes are lost.
    * This might have merge conflicts with the `main` branch, which you need to fix before it is merged.
7. Once the above pull request lands, delete the specific release branch.
8. Update `glean-ffi` in the iOS megazord. See the [application-services documentation for that](https://github.com/mozilla/application-services/blob/main/megazords/ios/README.md#glean-component).

## Hotfix release for latest version

If the latest released version requires a bug fix, a hotfix branch is used.

### Create a hotfix branch

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
    1. [Draft a New Release](https://github.com/mozilla/glean/releases/new) in the GitHub UI (`Releases > Draft a New Release`).
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
8. Update `glean-ffi` in the iOS megazord. See the [application-services documentation for that](https://github.com/mozilla/application-services/blob/main/megazords/ios/README.md#glean-component).

## Hotfix release for previous version

If you need to release a hotfix for a previously released version (that is: not the latest released version), you need a support branch.

> **Note**: This should rarely happen. We generally support only the latest released version of Glean.

### Create a support and hotfix branch

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
    1. [Draft a New Release](https://github.com/mozilla/glean/releases/new) in the GitHub UI (`Releases > Draft a New Release`).
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

## Upgrading android-components to a new version of Glean

On Android, Mozilla products consume the Glean SDK through its wrapper in [`android-components`](https://github.com/mozilla-mobile/android-components).
Therefore, when a new Glean SDK release is made, `android-components` must also be updated.

After following one of the above instructions to make a Glean SDK release:

1. Ensure that CI has completed and the artifacts are published on [Mozilla's Maven repository](https://maven.mozilla.org/?prefix=maven2/org/mozilla/telemetry/).

2. Create a pull request against `android-components` to update the Glean version with the following changes:

   - The Glean version is updated in the `mozilla_glean` variable in the [`buildSrc/src/main/java/Dependencies.kt`](https://github.com/mozilla-mobile/android-components/blob/69546999739ab19d21425e9a98e107e438a3f905/buildSrc/src/main/java/Dependencies.kt#L32) file.

   - The relevant parts of the Glean changelog copied into the top part of the [`android-components` changelog](https://github.com/mozilla-mobile/android-components/blob/main/docs/changelog.md).
     This involves copying the Android-specific changes and the general changes to Glean, but can omit other platform-specific changes.

**IMPORTANT:** Until the [Glean Gradle plugin work is complete](https://bugzilla.mozilla.org/show_bug.cgi?id=1592947), all downstream consumers of android-components will also need to update their version of Glean to match the version used in android-components so that their unit tests can run correctly.

In Fenix, for example, the [Glean version is specified here](https://github.com/mozilla-mobile/fenix/blob/5e4ef202b85e273cf46ec5c7ec1b80f30ca4e77c/buildSrc/src/main/java/Dependencies.kt#L44).
