# Replacing the Glean Gradle plugin in mozilla-central

> **Note**: If you only need to replace the `glean_parser` Python parts used in the build see [Substituting `glean_parser`](glean-parser-substitution.md).
> The approached documented in this chapter is only necessary if you changed `GleanGradlePlugin.groovy`.

If you need to replace the Glean Gradle Plugin used by any component in mozilla-central, follow these steps:

1. In your Glean repository increment the version number in `.buildconfig.yml` to something unused:

   ```yaml
   libraryVersion: 70.0.0
   ```

1. Build the Glean Gradle plugin and publish the plugin locally:

   ```
   ./gradlew glean-gradle-plugin:publishToMavenLocal
   ```

1. In your `mozilla-central` checkout, add the following line in `mobile/android/fenix/settings.gradle` file in the `pluginManagement` block:

   ```gradle
   mavenLocal()
   ```

1. Use the new version number where the plugin is imported in `mobile/android/fenix/build.gradle`:

   ```
   classpath "org.mozilla.telemetry:glean-gradle-plugin:70.0.0"
   ```

   This might need to be applied to the top-level `build.gradle` and other `build.gradle` files under `mobile/android` to apply to all components.


Building Fenix will now use your locally published Glean Gradle Plugin.
