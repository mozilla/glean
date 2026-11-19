# Replacing the Glean Gradle plugin in mozilla-central

> **Note**: If you only need to replace the `glean_parser` Python parts used in the build see [Substituting `glean_parser`](glean-parser-substitution.md).
> The approached documented in this chapter is only necessary if you changed `GleanGradlePlugin.groovy`.

If you need to replace the Glean Gradle Plugin used by any component in mozilla-central, follow these steps:

1. In your Glean repository increment the version number in `.buildconfig.yml` to something unused:

   ```yaml
   libraryVersion: 99.0.0
   ```

1. Build the Glean Gradle plugin and publish the plugin locally:

   ```
   ./gradlew glean-gradle-plugin:publishToMavenLocal
   ```

1. In your `mozilla-central` checkout, give the plugin its own version in `gradle/libs.versions.toml`.
   Keep the `glean` version unchanged, so the Glean library and the `verifyGleanVersion` check are unaffected:

   ```toml
   mozilla-glean-gradle-plugin = { module = "org.mozilla.telemetry:glean-gradle-plugin", version = "99.0.0" }
   ```

1. Create a Gradle init script, for example `mavenlocal.gradle` outside the checkout, that adds `mavenLocal()` to the repositories every build script loads its plugins from:

   ```gradle
   allprojects {
       buildscript {
           repositories {
               mavenLocal()
           }
       }
   }
   ```

1. Build Fenix with the init script:

   ```
   ./mach gradle --init-script /path/to/mavenlocal.gradle buildEnvironment fenix:assembleDebug
   ```

   The `buildEnvironment` output lists the plugin version in use.

Building Fenix will now use your locally published Glean Gradle Plugin.
