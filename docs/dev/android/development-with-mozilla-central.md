# Working on unreleased Glean code in mozilla-central

> **Note**: This guide is for mozilla-central Android builds. If you only need to replace the Glean Rust parts, refer to [Developing with a local Glean build](https://firefox-source-docs.mozilla.org/toolkit/components/glean/dev/local_glean.html) in the Firefox Source Docs.

> **Note**: This guide only refers to building Fenix. It should work similarly for Focus Android.

## Preparation

This guide assumes you already have are setup to build the mozilla-central repository for Android.
Refer to [Getting Set Up To Work On The Firefox Codebase](https://firefox-source-docs.mozilla.org/setup/index.html) for more.

Clone the Glean SDK repositories:

```sh
git clone https://github.com/mozilla/glean
```

## Cargo build targets

By default when building Fenix using gradle, the rust-android plugin will compile Glean for every possible platform,
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

Fenix has custom build logic for dealing with composite builds,
so you should be able to configure it by simply adding the path to the Glean repository in the correct `local.properties` file:

In `mozilla-central/mobile/android/fenix/local.properties`:

```groovy
autoPublish.glean.dir=../../../../glean
```

Make sure to use the correct path pointing to your Glean checkout.

> **Note**: This substitution-based approach will not work for testing updates to the Glean Gradle plugin or `glean_parser` as shipped in the Glean Gradle Plugin.
>
> See [Replacing the Glean Gradle plugin in mozilla-central](gradle-plugin-in-mc.md).  
> For replacing `glean_parser` in a local build, see [Substituting `glean_parser`](glean-parser-substitution.md).
