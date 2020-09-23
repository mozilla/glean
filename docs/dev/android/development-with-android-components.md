# Working on unreleased Glean code in android-components

This is a companion to the [equivalent instructions for the android-components repository](https://mozilla-mobile.github.io/android-components/contributing/testing-components-inside-app).

Modern Gradle supports [composite builds](https://docs.gradle.org/current/userguide/composite_builds.html), which allows to substitute on-disk projects for binary publications.  Composite builds transparently accomplish what is usually a frustrating loop of:
1. change library
1. publish library snapshot to the local Maven repository
1. consume library snapshot in application

> **Note**: this substitution-based approach will not work for testing updates to the Glean Gradle Plugin.
> For that to work, the library and the plugin need to be published to a local maven and manually imported
> in the consuming projects.
> Please refer to [Using locally-published Glean in Fenix](./locally-published-components-in-fenix.md) for
> how to do that.

## Preparation

Clone the Glean SDK and android-components repositories:

```sh
git clone https://github.com/mozilla/glean
git clone https://github.com/mozilla-mobile/android-components
```

## Cargo build targets

By default when building Android Components using gradle, the gradle-cargo plugin will compile Glean for every possible platform,
which might end up in failures.
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
substitutions.glean.dir=../glean
```

If this doesn't seem to work, or if you need to configure composite builds for a project that does not contain this custom logic,
add the following to `settings.gradle`:

In `android-components/settings.gradle`:
```groovy
includeBuild('../glean') {
  dependencySubstitution {
    substitute module('org.mozilla.telemetry:glean') with project(':glean')
    substitute module('org.mozilla.telemetry:glean-forUnitTests') with project(':glean')
  }
}
```

Composite builds will ensure the Glean SDK is build as part of tasks inside `android-components`.

## Caveat

There's a big gotcha with library substitutions: the Gradle build computes lazily, and AARs don't include their transitive dependencies' JNI libraries.
This means that in `android-components`, `./gradlew :service-glean:assembleDebug` **does not** invoke `:glean:cargoBuild`,
even though `:service-glean` depends on the substitution for `:glean` and even if the inputs to Cargo have changed!
It's the final consumer of the `:service-glean` project (or publication) that will incorporate the JNI libraries.

In practice that means _you should always be targeting something that produces an APK_: a test or a sample module.
Then you should find that the `cargoBuild` tasks are invoked as you expect.

Inside the `android-components` repository `./gradlew :samples-glean:connectedAndroidTest` should work.
Other tests like `:service-glean:testDebugUnitTest` or `:support-sync-telemetry:testDebugUnitTest` will currently fail, because the JNI libraries are not included.


## Notes

This document is based on the equivalent documentation for application-services:
[Development with the Reference Browser](https://github.com/mozilla/application-services/blob/HEAD/docs/howtos/working-with-reference-browser.md)

1. Transitive substitutions (as shown above) work but require newer Gradle versions (4.10+).
