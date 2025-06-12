# Adding Glean to your Swift project

This page provides a step-by-step guide on how to integrate the Glean library into a Swift project.

Nevertheless this is just one of the required steps for integrating Glean successfully into a project. Check you the full [Glean integration checklist](./index.md) for a comprehensive list of all the steps involved in doing so.

Currently, this SDK only supports the iOS platform.

## Requirements

* Python >= 3.9.

## Setting up the dependency

The Glean Swift SDK can be consumed as a Swift Package.
In your Xcode project add a new package dependency:

```
https://github.com/mozilla/glean-swift
```

Use the dependency rule "Up to Next Major Version".
Xcode will automatically fetch the latest version for you.

The Glean library will be automatically available to your code when you import it:

```swift
import Glean
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

> As above, the `{latest-version}` placeholder should be replaced with the version number of Glean Swift SDK release used in this project.

2. Add the `sdk_generator.sh` file to your Xcode project.
3. On your application targets' Build Phases settings tab, click the `+` icon and choose `New Run Script Phase`.
   Create a Run Script in which you specify your shell (ex: `/bin/sh`), add the following contents to the script area below the shell:

   ```
   bash $PWD/sdk_generator.sh
   ```

   Set [additional options](../../language-bindings/ios/ios-build-configuration-options.md) to control the behavior of the script.

4. Add the path to your `metrics.yaml` and (optionally) `pings.yaml` and `tags.yaml` under "Input files":

   ```
   $(SRCROOT)/{project-name}/metrics.yaml
   $(SRCROOT)/{project-name}/pings.yaml
   $(SRCROOT)/{project-name}/tags.yaml
   ```

5. Add the path to the generated code file to the "Output Files":

   ```
   $(SRCROOT)/{project-name}/Generated/Metrics.swift
   ```

6. If you are using Git, add the following lines to your `.gitignore` file:

   ```
   .venv/
   {project-name}/Generated
   ```

   This will ignore files that are generated at build time by the `sdk_generator.sh` script.
   They don't need to be kept in version control, as they can be re-generated from your `metrics.yaml` and `pings.yaml` files.

{{#include ../../../shared/blockquote-info.html}}

##### Glean and embedded extensions

> Metric collection is a no-op in [application extensions](https://developer.apple.com/library/archive/documentation/General/Conceptual/ExtensibilityPG/ExtensionOverview.html#//apple_ref/doc/uid/TP40014214-CH2-SW2) and Glean will not run. Since extensions run in a separate sandbox and process from the application, Glean would run in an extension as if it were a completely separate application with different client ids and storage. This complicates things because Glean doesn’t know or care about other processes. Because of this, Glean is purposefully prevented from running in an application extension and if metrics need to be collected from extensions, it's up to the integrating application to pass the information to the base application to record in Glean.
