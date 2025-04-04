# Adding Glean to your Kotlin project

This page provides a step-by-step guide on how to integrate the Glean library into a Kotlin project.

Nevertheless this is just one of the required steps for integrating Glean successfully into a project. Check you the full [Glean integration checklist](./index.md) for a comprehensive list of all the steps involved in doing so.

Currently, this SDK only supports the Android platform.

## Setting up the dependency

The Glean Kotlin SDK is published on [maven.mozilla.org](https://maven.mozilla.org/).
To use it, you need to add the following to your project's top-level build file,
in the `allprojects` block (see e.g. [Glean SDK's own `build.gradle`](https://github.com/mozilla/glean/blob/main/build.gradle)):

```Groovy
repositories {
    maven {
       url "https://maven.mozilla.org/maven2"
    }
}
```

Each module that uses the Glean Kotlin SDK needs to specify it in its build file, in the `dependencies` block.
Add this to your Gradle configuration:

```Groovy
implementation "org.mozilla.telemetry:glean:{latest-version}"
```

{{#include ../../../shared/blockquote-warning.html}}

##### Pick the correct version

> The `{latest-version}` placeholder in the above link should be replaced with the [latest version of the Glean SDK](https://github.com/mozilla/glean/releases).
>
> For example, if version *63.0.0* is the latest version, then the include directive becomes:
>
> ```Groovy
> implementation "org.mozilla.telemetry:glean:63.0.0"
> ```

{{#include ../../../shared/blockquote-info.html}}

##### Size impact on the application APK

> The Glean Kotlin SDK APK ships binary libraries for all the supported platforms. Each library file measures about 600KB. If the final APK size of the consuming project is a concern, please enable [ABI splits](https://developer.android.com/studio/build/configure-apk-splits#configure-abi-split).

### Dependency for local testing

Due to its use of a native library you will need additional setup to allow local testing.

Add the following lines to your `dependencies` block:

```Groovy
testImplementation "org.mozilla.telemetry:glean-forUnitTests:${project.ext.glean_version}"
```

**Note:** Always use `org.mozilla.telemetry:glean-forUnitTests`.
This package is standalone and its version will be exported from the main Glean package automatically.

## Setting up metrics and pings code generation

In order for the Glean Kotlin SDK to generate an API for your metrics, two Gradle plugins must be included in your build:

- The [Glean Gradle plugin](https://github.com/mozilla/glean/tree/main/gradle-plugin/)
- JetBrains' [Python envs plugin](https://github.com/JetBrains/gradle-python-envs/)

The Glean Gradle plugin is distributed through Mozilla's Maven, so we need to tell your build where to look for it by adding the following to the top of your `build.gradle`:

```Groovy
buildscript {
    repositories {
        maven {
            url "https://maven.mozilla.org/maven2"
        }

        dependencies {
            classpath "org.mozilla.telemetry:glean-gradle-plugin:{latest-version}"
        }
    }
}
```

{{#include ../../../shared/blockquote-warning.html}}

##### Important

> As above, the `{latest-components-version}` placeholder in the above link should be replaced with the version number of the Glean SDK used in your project.

The JetBrains Python plugin is distributed in the Gradle plugin repository, so it can be included with:

```Groovy
plugins {
    id "com.jetbrains.python.envs" version "0.0.26"
}
```

Right before the end of the same file, we need to apply the Glean Gradle plugin.
Set any [additional parameters](../../language-bindings/android/android-build-configuration-options.md) to control the behavior of the Glean Gradle plugin before calling `apply plugin`.


```Groovy
// Optionally, set any parameters to send to the plugin.
ext.gleanGenerateMarkdownDocs = true
apply plugin: "org.mozilla.telemetry.glean-gradle-plugin"
```

{{#include ../../../shared/blockquote-info.html}}

##### Rosetta 2 required on Apple Silicon

> On Apple Silicon machines (M1/M2/M3 MacBooks and iMacs) Rosetta 2 is required for the bundled Python.
> See the [Apple documentation about Rosetta 2](https://support.apple.com/en-us/HT211861)
> and [Bug 1775420](https://bugzilla.mozilla.org/show_bug.cgi?id=1775420) for details.  
> You can install it with `softwareupdate --install-rosetta`

{{#include ../../../shared/blockquote-info.html}}

##### Offline builds

> The Glean Gradle plugin has limited support for [offline builds](../../language-bindings/android/android-offline-builds.md) of applications that use the Glean SDK.
