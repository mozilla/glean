# Glean Release Process

These are the steps needed to cut a new release from latest master.

1. Update the changelog.
    1. Add any missing important changes under the `Unreleased changes` headline
2. Run `bin/prepare-release.sh <new version>`
    1. The new version should be the next patch, minor or major version of what is currently released.
    2. Let it create a commit for you.
3. Land the new commit that performs the steps above. This takes a PR, typically, because of branch protection on master.
4. Cut the actual release.
    1. Click "Releases", and then "Draft a New Release" in the github UI.
    2. Enter `v<myversion>` as the tag. It's important this is the same as the tags you put in the links in the changelog.
    3. Under the description, paste the contents of the release notes from `CHANGELOG.md`.
    4. Note that the release is not available until the CI build completes for that tag.
        - You can check [on CircleCI for the running build](https://circleci.com/gh/mozilla/glean).
    5. Release the Rust crates:

       ```
       cd glean-core
       cargo publish --verbose
       cd ffi
       cargo publish --verbose
       ```

### Published artifacts

* The Kotlin libaries are published: [GitHub Releases](https://github.com/mozilla/glean/releases), [Mozilla Maven](https://maven.mozilla.org/?prefix=maven2/org/mozilla/telemetry/).
* Python bindings are published on PyPi: [glean-sdk](https://pypi.org/project/glean-sdk/).
* Artifacts for iOS wil be [generated on release soon](https://bugzilla.mozilla.org/show_bug.cgi?id=1598276).
* Rust crates are published on crates.io: [glean-core](https://crates.io/crates/glean-core), [glean-ffi](https://crates.io/crates/glean-ffi).

## New point-releases

If the new release is based on the current master, just follow the above release process.

Otherwise follow these steps to release a point-release on top an older release that is behind latest master:

1. Ensure your fixes are landed on master first (if required).
2. Create a new branch named `release-vX.Y` which will be used for all point-releases on the `vX.Y` series. Example:

   ```
   git checkout -b release-v19.1 v19.0.0
   git push -u origin release-v19.1
   ```

3. Make a new branch with any fixes to be included in the release, *remembering not to make any breaking API changes.*.
   This may involve cherry-picking fixes from master, or developing a new fix directly against the branch.
   Example:

   ```
   git checkout -b fixes-for-v1.19.1 release-v19.1
   git cherry-pick 37d35304a4d1d285c8f6f3ce3df3c412fcd2d6c6
   git push -u origin fixes-for-v1.19.1
   ```
4. Follow the above steps for cutting a new release, except that:
    * When opening a PR to land the commits, target the `release-vX.Y` branch rather than master.
    * When cutting the new release via github's UI, target the `release-vX.Y` branch rather than master.

> **Note:** Point-releases for older versions should be rarely required.
> We support the last released version and fixes should go there whenever possible.
