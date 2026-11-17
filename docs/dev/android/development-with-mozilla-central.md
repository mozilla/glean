# Working on unreleased Glean code in mozilla-central

<!-- toc -->

> **Note**: This guide is for mozilla-central Android builds. If you only need to replace the Glean Rust parts, refer to [Developing with a local Glean build](https://firefox-source-docs.mozilla.org/toolkit/components/glean/dev/local_glean.html) in the Firefox Source Docs.

> **Note**: This guide only refers to building Fenix. It should work similarly for Focus Android.

## Preparation

This guide assumes you are already setup to build the mozilla-central repository for Android.
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
rust.targets=arm64

# For unit tests:
rust.targets=darwin-aarch64 # Or linux-*, windows-* (* = x86/x64)

# For emulator only:
rust.targets=x86

# Combine multiple targets: physical devices + macOS Mx host
rust.targets=arm64,darwin-aarch64
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

## Manual `mavenLocal()`

If the [Substituting projects](#substituting-projects) does not work you can manually switch to using `mavenLocal()`.

In the Glean repository:

1. Bump the version: `bin/prepare-release.sh 65.1.0`
    * Only increase the minor version to ensure it works transitively
1. Publish the Gradle project locally: `./gradlew publishToMavenLocal`
1. Check that the packages are available in `~/.m2/repository`

In `mozilla-central`:

1. In `gradle.properties` remove the line `org.gradle.configuration-cache=true`
    * Alternatively use `--no-build-cache --no-configuration-cache` on all Gradle invocations when building Fenix/Focus
1. In `gradle/libs.versions.toml` change the version of `mozilla-glean`
1. In the top-level `build.gradle` add `mavenLocal()` to the `repositories` block under `allprojects`, e.g.
    ```gradle
    allprojects {
        repositories {
            mavenLocal()
        }
    }
    ```
1. Further down in the top-level `build.gradle` disable the check in `verifyGleanVersion`, e.g.
    ```gradle
    @TaskAction
    void verifyGleanVersion() {
    }
    ```

Now you can build Fenix or Focus, for example with `./mach gradle fenix:assembleFenixDebug`.
After changes in the Glean repository publish a new package using `./gradlew publishToMavenLocal` and rebuild Fenix.

## Try runs with released versions of Glean

If you want to test out a new Glean release, especially a new major Glean version,
which also requires changes in application-services follow this process.

1. [Release Glean as usual](../cut-a-new-release.md)
1. [Update the Glean SDK in mozilla-central](https://firefox-source-docs.mozilla.org/toolkit/components/glean/dev/updating_sdk.html)
   ```
   ./mach update-glean 72.0.0
   ```
   and follow the printed steps.
1. Apply any required changes to how the Glean API is used in `third_party/application-services`.
   Any changes will need to be contributed to [mozilla/application-services](https://github.com/mozilla/application-services) before this Glean release can land in mozilla-central.
1. Select the `appservices-in-tree` tasks from a full fuzzy try (with duplicates):
   ```
   ./mach try fuzzy --full --include-duplicates
   ```
   e.g. `test-apk-fenix-debug-appservices-in-tree`
   You can select the Fenix specific ones in a single command with:
   ```
   ./mach try fuzzy --full --include-duplicates -q "'fenix 'appservices-in-tree"
   ```
   (Add `--no-push` to see which tasks this will launch)
1. In the treeherder job overview toggle Tier 3 jobs, so the launched tasks are visible.
   Any Tier 1/2 Android tasks might fail due to Glean version mismatches. This is expected.

## Try runs with development versions of Glean

> **Note**: This approach is hacky and might not work as expected or might require additional changes in other related
> Gradle build files.

Publish Glean to the local maven repository as above.

In `mozilla-central`:

1. Create a directory: `mkdir -p third_party/kotlin`
1. Add the local packages: `cp -a ~/.m2/repository/* third_party/kotlin`
1. Commit the new files: `git add third_party/kotlin && git commit -m "dev glean from local repo"`
1. Change the version number in `gradle/libs.versions.toml`
1. Add the following under `repositories `to the top-level `build.gradle`:
    ```gradle
    maven {
       url uri("${gradle.mozconfig.topsrcdir}/third_party/kotlin")
   }
   ```
1. Add the following to the `configureMavenRepositories` function in `mobile/android/gradle/mozconfig.gradle`:
    ```gradle
    handler.maven {
       url uri("${gradle.mozconfig.topsrcdir}/third_party/kotlin")
   }
   ```
