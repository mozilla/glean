# Using locally-published Glean in Fenix

> Note: This is a bit tedious, and you might like to try the substitution-based approach documented in
> [Working on unreleased Glean code in android-components](./development-with-android-components.md).
> That approach is still fairly new, and the local-publishing approach in this document is necessary if it fails.

> Note: This is Fenix-specific only in that some links on the page go to the `mozilla-mobile/fenix` repository,
> however these steps should work for e.g. `reference-browser`, as well.
> (Same goes for Lockwise, or any other consumer of Glean, but they may use a different structure -- Lockwise has no Dependencies.kt, for example)

## Preparation

Clone the Glean SDK, android-components and Fenix repositories:

```sh
git clone https://github.com/mozilla/glean
git clone https://github.com/mozilla-mobile/android-components
git clone https://github.com/mozilla-mobile/fenix/
```

## Local publishing


1. Inside the `glean` repository root:
    1. In [`.buildconfig.yml`][glean-yaml], change
       `libraryVersion` to end in `-TESTING$N`[^1],
       where `$N` is some number that you haven't used for this before.

       Example: `libraryVersion: 22.0.0-TESTING1`
    2. Check your `local.properties` file,
       and add `rust.targets=x86` if you're testing on the emulator,
       `rust.targets=arm` if you're testing on 32-bit arm (arm64 for 64-bit arm, etc).
       This will make the build that's done in the next step much faster.
    3. Run `./gradlew publishToMavenLocal`. This may take a few minutes.

2. Inside the `android-components` repository root:
    1. In [`.buildconfig.yml`][android-components-yaml], change
       `componentsVersion` to end in `-TESTING$N`[^1],
       where `$N` is some number that you haven't used for this before.

       Example: `componentsVersion: 24.0.0-TESTING1`
    2. Inside [`buildSrc/src/main/java/Dependencies.kt`][android-components-deps],
       change `mozilla_glean` to reference the `libraryVersion` you published in step 2 part 1.

       Example: `const val mozilla_glean = "22.0.0-TESTING1"`

    3. Inside [`build.gradle`][android-components-build-gradle], add
       `mavenLocal()` inside `allprojects { repositories { <here> } }`.

    4. Add the following block in the [`settings.gradle`](https://github.com/mozilla-mobile/android-components/blob/d67f83af679a2e847e5bd284ea4a30b412403241/settings.gradle#L7) file, before any other statement in the script, in order to have the Glean Gradle plugin loaded from the local Maven repository.
       ```
       pluginManagement {
         repositories {
             mavenLocal()
             gradlePluginPortal()
         }
       }
       ```

    5. Inside the android-component's `local.properties` file, ensure
       `substitutions.glean.dir` is *NOT* set.

    6. Run `./gradlew publishToMavenLocal`.

3. Inside the `fenix` repository root:
    1. Inside [`build.gradle`][fenix-build-gradle] you need to add `mavenLocal()` twice.  
       First add `mavenLocal()` inside `buildscript` like this:
       ```
       buildscript {
           repositories {
               mavenLocal()
               ...
       ```

       Second add `mavenLocal()` inside `allprojects`:
       ```
       allprojects {
           repositories {
               mavenLocal()
               ...
       ```

    2. Inside [`buildSrc/src/main/java/AndroidComponents.kt`][fenix-deps], change
       `VERSION` to the version you defined in step 3 part 1.

       Example:
       ```
       const val VERSION = "24.0.0-TESTING1"
       ```

You should now be able to build and run Fenix (assuming you could before all this).

## Caveats

1. This assumes you have followed the [Android/Rust build setup](setup-android-build-environment.md)
2. Make sure you're fully up to date in all repositories, unless you know you need to not be.
3. This omits the steps if changes needed because, e.g. Glean made a breaking change to an API used in android-components.
   These should be understandable to fix, you usually should be able to find a PR with the fixes somewhere in the android-component's list of pending PRs
   (or, failing that, a description of what to do in the Glean changelog).
4. Ask in the [#glean channel on chat.mozilla.org](https://chat.mozilla.org/#/room/#glean:mozilla.org).

## Notes

This document is based on the equivalent documentation for application-services:
[Using locally-published components in Fenix](https://github.com/mozilla/application-services/blob/HEAD/docs/howtos/locally-published-components-in-fenix.md)

---

[^1]: It doesn't have to end with `-TESTING$N`, it only needs to have the format `-someidentifier`.
`-SNAPSHOT$N` is also very common to use, however without the numeric suffix, this has specific meaning to gradle,
so we avoid it.
Additionally, while the `$N` we have used in our running example has matched
(e.g. all of the identifiers ended in `-TESTING1`, this is not required, so long as you match everything up correctly at the end).

[glean-yaml]: https://github.com/mozilla/glean/blob/main/.buildconfig.yml#L1
[android-components-yaml]: https://github.com/mozilla-mobile/android-components/blob/HEAD/.buildconfig.yml#L1
[android-components-deps]: https://github.com/mozilla-mobile/android-components/blob/50a2f28027f291bf1c6056d42b55e75ba3c050db/buildSrc/src/main/java/Dependencies.kt#L32
[android-components-build-gradle]: https://github.com/mozilla-mobile/android-components/blob/b98206cf8de818499bdc87c00de942a41f8aa2fb/build.gradle#L28
[fenix-build-gradle]: https://github.com/mozilla-mobile/fenix/blob/c21b41c1b23e6e25e0b5a8392f0c6dcb5ad25473/build.gradle#L5
[fenix-deps]: https://github.com/mozilla-mobile/fenix/blob/c21b41c1b23e6e25e0b5a8392f0c6dcb5ad25473/buildSrc/src/main/java/AndroidComponents.kt#L6
