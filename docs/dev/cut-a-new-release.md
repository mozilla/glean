# Glean Release Process

> **Important:** The Glean SDK does not yet have a fully streamlined release process. All information here is preliminary.
> At this point the release process only produces release artifacts for the Kotlin library.
> The Rust crate is not released on its own. No artifacts for iOS are generated.

These are the steps needed to cut a new release from latest master.

1. Update the changelog.
    1. Add any missing important changes under the `Unreleased changes` headline
2. Run `bin/prepare-release.sh <new version>`
    1. The new version should be the next patch, minor or major version of what is currently released.
    2. Let it create a commit for you.
3. Land the new commit that perform the steps above. This takes a PR, typically, because of branch protection on master.
4. Cut the actual release.
    1. Click "Releases", and then "Draft a New Release" in the github UI.
    2. Enter `v<myversion>` as the tag. It's important this is the same as the tags you put in the links in the changelog.
    3. Under the description, paste the contents of the release notes from `CHANGELOG.md`.
    4. Note that the release is not available until the CI build completes for that tag.
        - You can check [on CircleCI for the running build](https://circleci.com/gh/mozilla/glean).
