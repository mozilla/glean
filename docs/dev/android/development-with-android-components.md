# Working on unreleased Glean code in android-components

This is a companion to the [equivalent instructions for the android-components repository](https://mozilla-mobile.github.io/android-components/contributing/testing-components-inside-app).

Modern Gradle supports [composite builds](https://docs.gradle.org/current/userguide/composite_builds.html), which allows to substitute on-disk projects for binary publications.  Composite builds transparently accomplish what is usually a frustrating loop of:
1. change library
1. publish library snapshot to the local Maven repository
1. consume library snapshot in application

> **Note**: this substitution-based approach will not work for testing updates to `glean_parser` as shipped in the Glean Gradle Plugin.
> For replacing `glean_parser` in a local build, see [Substituting `glean_parser`](glean-parser-substitution.md).

## Preparation

Clone the Glean SDK and android-components repositories:

```sh
git clone https://github.com/mozilla/glean
git clone https://github.com/mozilla-mobile/android-components
```

## Cargo build targets

By default when building Android Components using gradle, the rust-android plugin will compile Glean for every possible platform,
which might end up in build failures.
You can customize which targets are built in `glean/local.properties`
([more information](https://github.com/ncalexan/rust-android-gradle/blob/HEAD/README.md#specifying-local-targets)):

```
# For physical devices:
rust.targets=arm

# For unit tests:
# rust.targets=darwin # Or linux-*, windows-* (* = x86/x64)

# For emulator only:
rust.targets=x86
```

## Substituting projects

android-components has custom build logic for dealing with composite builds,
so you should be able to configure it by simply adding the path to the Glean repository in the correct `local.properties` file:

In `android-components/local.properties`:

```groovy
localProperties.autoPublish.glean.dir=../glean
```

This will auto-publish Glean SDK changes to a local repository and consume them in android-components.
