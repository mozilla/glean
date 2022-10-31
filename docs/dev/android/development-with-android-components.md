# Working on unreleased Glean code in android-components

This is a companion to the [equivalent instructions for the android-components repository](https://mozilla-mobile.github.io/android-components/contributing/testing-components-inside-app).

Modern Gradle supports [composite builds](https://docs.gradle.org/current/userguide/composite_builds.html), which allows to substitute on-disk projects for binary publications.  Composite builds transparently accomplish what is usually a frustrating loop of:
1. change library
1. publish library snapshot to the local Maven repository
1. consume library snapshot in application

## Preparation

Clone the Glean SDK and android-components repositories:

```sh
git clone https://github.com/mozilla/glean
git clone https://github.com/mozilla-mobile/firefox-android
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

In `firefox-android/android-components/local.properties`:

```groovy
autoPublish.glean.dir=../glean
```

This will auto-publish Glean SDK changes to a local repository and consume them in android-components.

### Replacing the Glean Gradle plugin

> **Note**: If you only need to replace the `glean_parser` used in the build see [Substituting `glean_parser`](glean-parser-substitution.md).
> This is only necessary if you changed `GleanGradlePlugin.groovy`.

If you need to replace the Glean Gradle Plugin used by other components within Android Components, follow these steps:

1. In your Glean repository increment the version number in `.buildconfig.yml`

   ```yaml
   libraryVersion: 60.0.0
   ```

2. Build and publish the plugin locally:

   ```
   ./gradlew publishToMavenLocal
   ```

3. In the Android Components repository change the required Glean version `buildSrc/src/main/java/Dependencies.kt`:

   ```kotlin
   const val mozilla_glean = "60.0.0"
   ```

4. In the Android Components repository add the following at the top of the `settings.gradle` file:

   ```gradle
   pluginManagement {
       repositories {
           mavenLocal()
           gradlePluginPortal()
       }
   }
   ```

Building any component will now use your locally published Glean Gradle Plugin (and Glean SDK).
