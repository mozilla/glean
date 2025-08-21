# Working on unreleased Glean code in mozilla-central

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

## Try runs with development versions of Glean

Publish Glean to the local maven repository as above.

In `mozilla-central`:

1. Create a directory: `mkdir -p third_party/kotlin`
1. Add the local packages: `cp -a ~/.m2/repository/* third_party/kotlin`
1. Commit the new files: `git add third_party/kotlin && git commit -m "dev glean from local repo"`
1. Change the version number in `gradle/libs.versions.toml`
1. Add the following under `repositories `to the top-level `build.gradle` and nested `build.gradle` files:
    ```gradle
    maven {
       url uri("${gradle.mozconfig.topsrcdir}/third_party/kotlin")
   }
   ```

The nested `build.gradle` files are:

* `mobile/android/fenix/build.gradle`
* `mobile/android/focus-android/build.gradle`
* `mobile/android/android-components/build.gradle`

You can now launch try tasks with the modified Glean.

After changes in the Glean repository publish a new package using `./gradlew publishToMavenLocal`,
then copy the files again with `cp -a ~/.m2/repository/* third_party/kotlin`.
Commit the changes and push to try again.
