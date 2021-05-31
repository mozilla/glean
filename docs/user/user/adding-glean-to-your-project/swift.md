# Adding Glean to your Swift project

This page provides a step-by-step guide on how to integrate the Glean library into a Swift project.

Nevertheless this is just one of the required steps for integrating Glean successfully into a project. Check you the full [Glean integration checklist](./index.md) for a comprehensive list of all the steps involved in doing so.


Currently, these bindings only support the iOS platform.

## Requirements

* Python >= 3.6.

## Setting up the dependency

The Glean SDK can be consumed through [Carthage](https://github.com/Carthage/Carthage), a dependency manager for macOS and iOS.
For consuming the latest version of the Glean SDK, add the following line to your `Cartfile`:

```
github "mozilla/glean" "{latest-version}"
```

{{#include ../../../shared/blockquote-warning.html}}

##### Pick the correct version

> The `{latest-version}` placeholder should be replaced with the version number of the latest Glean SDK release.
> You can find the version number on the [release page](https://github.com/mozilla/glean/releases/latest).

Then check out and build the new dependency:

```
carthage update --platform iOS
```

## Integrating with the build system

For integration with the build system you can follow the [Carthage Quick Start steps](https://github.com/Carthage/Carthage#quick-start).

1. After building the dependency one drag the built `.framework` binaries from `Carthage/Build/iOS` into your application's Xcode project.
2. On your application targets' Build Phases settings tab, click the `+` icon and choose `New Run Script Phase`.
   If you already use Carthage for other dependencies, extend the existing step.
   Create a Run Script in which you specify your shell (ex: `/bin/sh`), add the following contents to the script area below the shell:

   ```
   /usr/local/bin/carthage copy-frameworks
   ```

3. Add the path to the Glean framework under "Input Files":

   ```
   $(SRCROOT)/Carthage/Build/iOS/Glean.framework
   ```

4. Add the paths to the copied framework to the "Output Files":

   ```
   $(BUILT_PRODUCTS_DIR)/$(FRAMEWORKS_FOLDER_PATH)/Glean.framework
   ```

## Combined usage with application-services

If your application uses both the Glean SDK and [application-services](https://github.com/mozilla/application-services)
you can use a combined release to reduce the memory usage and startup time impact.

In your `Cartfile` require only `application-services`, e.g.:

```
github "mozilla/application-services" ~> "{latest-version}"
```

{{#include ../../../shared/blockquote-warning.html}}

##### Pick the correct version

> The `{latest-version}` placeholder should be replaced with the version number of the latest application-services release.
> You can find the version number on the [release page](https://github.com/mozilla/application-services/releases/latest).

Then check out and build the new dependency:

```
carthage update --platform iOS
```

## Setting up metrics and pings code generation

The `metrics.yaml` file is parsed at build time and Swift code is generated.
Add a new `metrics.yaml` file to your Xcode project.

Follow these steps to automatically run the parser at build time:

1. Download the `sdk_generator.sh` script from the Glean repository:
   ```
   https://raw.githubusercontent.com/mozilla/glean/{latest-release}/glean-core/ios/sdk_generator.sh
   ```

{{#include ../../../shared/blockquote-warning.html}}

##### Pick the correct version

> As above, the `{latest-version}` placeholder should be replaced with the version number of Glean SDK release used in this project.

2. Add the `sdk_generator.sh` file to your Xcode project.
3. On your application targets' Build Phases settings tab, click the `+` icon and choose `New Run Script Phase`.
   Create a Run Script in which you specify your shell (ex: `/bin/sh`), add the following contents to the script area below the shell:

   ```
   bash $PWD/sdk_generator.sh
   ```

{{#include ../../../shared/blockquote-warning.html}}

##### Using with application-services

> If you are using the combined release of application-services and the Glean SDK you need to set the namespace to `MozillaAppServices`, e.g.:
>
> ```
> bash $PWD/sdk_generator.sh --glean-namespace MozillaAppServices
> ```

3. Add the path to your `metrics.yaml` and (optionally) `pings.yaml` under "Input files":

   ```
   $(SRCROOT)/{project-name}/metrics.yaml
   $(SRCROOT)/{project-name}/pings.yaml
   ```

4. Add the path to the generated code file to the "Output Files":

   ```
   $(SRCROOT)/{project-name}/Generated/Metrics.swift
   ```

{{#include ../../../shared/blockquote-info.html}}

##### The generated API

> The parser now generates a single file called `Metrics.swift` (since Glean v31.0.0).

5. If you are using Git, add the following lines to your `.gitignore` file:

   ```
   .venv/
   {project-name}/Generated
   ```

   This will ignore files that are generated at build time by the `sdk_generator.sh` script.
   They don't need to be kept in version control, as they can be re-generated from your `metrics.yaml` and `pings.yaml` files.

{{#include ../../../shared/blockquote-info.html}}

##### Glean and embedded extensions

> Metric collection is a no-op in [application extensions](https://developer.apple.com/library/archive/documentation/General/Conceptual/ExtensibilityPG/ExtensionOverview.html#//apple_ref/doc/uid/TP40014214-CH2-SW2) and Glean will not run. Since extensions run in a separate sandbox and process from the application, Glean would run in an extension as if it were a completely separate application with different client ids and storage. This complicates things because Glean doesn’t know or care about other processes. Because of this, Glean is purposefully prevented from running in an application extension and if metrics need to be collected from extensions, it's up to the integrating application to pass the information to the base application to record in Glean.
