# Glean Release Process

> **Important:** The Glean SDK does not yet have a fully streamlined release process. All information here is preliminary.
> At this point the release process only produces release artifacts for the Kotlin library.
> The Rust crate is not released on its own. No artifacts for iOS are generated.

These are the steps needed to cut a new release from latest master.

1. Update the changelog.
    1. In `CHANGELOG.md`:
        1. Replace `# Unreleased Changes` with `# v<new-version-number> (_<current date>_)`.
        2. Replace `master` in the Full Changelog link to be `v<new-version-number>`. E.g. if you are releasing 0.1.0, the link should be
            ```
            [Full Changelog](https://github.com/mozilla/glean/compare/v0.0.1...v0.1.0)
            ```
            Note that this needs three dots (`...`) between the two tags (two dots is different). Yes, the second tag doesn't exist yet, you'll make it later.
        3. Optionally, go over the commits between the past release and this one and see if anything is worth including.
        4. Make sure the changelog follows the format of the other changelog entries.
            - Note that we try to provide PR or issue numbers (and links) for each change. Please add these if they are missing.

    2. Add a new `# Unreleased Changes` on top

2. Bump the versions
    * Rust crates: Bump `version` in [`glean-core/Cargo.toml`](https://github.com/mozilla/glean/blob/master/glean-core/Cargo.toml) and [`glean-core/ffi/Cargo.toml`](https://github.com/mozilla/glean/blob/master/glean-core/ffi/Cargo.toml).
    * Kotlin package: Bump `libraryVersion` in the top-level [.buildconfig.yml](https://github.com/mozilla/glean/blob/master/.buildconfig.yml) file.
    * Gradle plugin: Bump `project.ext.gleanVersion` in [`GleanGradlePlugin.groovy`](https://github.com/mozilla/glean/blob/master/gradle-plugin/src/main/groovy/mozilla/telemetry/glean-gradle-plugin/GleanGradlePlugin.groovy).
    * Be sure you're following semver, and if in doubt, ask.
3. Land the commits that perform the steps above. This takes a PR, typically, because of branch protection on master.
4. Cut the actual release.
    1. Click "Releases", and then "Draft a New Release" in the github UI.
    2. Enter `v<myversion>` as the tag. In the example above it would be `v0.1.0`. It's important this is the same as the tags you put in the links in the changelog.
    3. Under the description, paste the contents of the release notes from `CHANGELOG.md`.
    4. Note that the release is not available until the CI build completes for that tag.
        - You can check [on CircleCI for the running build](https://circleci.com/gh/mozilla/glean).
5. After the release, bump the versions to the next pre-release:
    * Rust crates: Use the next major version and append `-alpha.1`
    * Kotlin package: Use the next major version and append `-SNAPSHOT`
