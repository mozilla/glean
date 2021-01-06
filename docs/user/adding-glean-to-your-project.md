# Adding Glean to your project

## Glean integration checklist

The Glean integration checklist can help to ensure your Glean SDK-using product is meeting all of the recommended guidelines.

Products (applications or libraries) using the Glean SDK to collect telemetry **must**:

1. [Integrate the Glean SDK into the build system](#integrating-with-your-project). Since the Glean SDK does some code generation for your metrics at build time, this requires a few more steps than just adding a library.

2. Include the markdown-formatted documentation generated from the `metrics.yaml` and `pings.yaml` files in the project's documentation.

3. Go through [data review process](https://wiki.mozilla.org/Firefox/Data_Collection) for all newly collected data.

4. Ensure that telemetry coming from automated testing or continuous integration is either not sent to the telemetry server or [tagged with the `automation` tag using the `sourceTag` feature](debugging/index.md#available-debugging-features).

5. [File a data engineering bug][dataeng-bug] to enable your product's application id.

Additionally, applications (but not libraries) **must**:

6. Provide a way for users to turn data collection off (e.g. providing settings to control `Glean.setUploadEnabled()`). The exact method used is application-specific.

## Usage

### Integrating with your project

{{#include ../tab_header.md}}

<div data-lang="Kotlin" class="tab">

#### Setting up the dependency

The Glean SDK is published on [maven.mozilla.org](https://maven.mozilla.org/).
To use it, you need to add the following to your project's top-level build file,
in the `allprojects` block (see e.g. [Glean SDK's own `build.gradle`](https://github.com/mozilla/glean/blob/main/build.gradle)):

```Groovy
repositories {
    maven {
       url "https://maven.mozilla.org/maven2"
    }
}
```

Each module that uses Glean SDK needs to specify it in its build file, in the `dependencies` block.
Add this to your Gradle configuration:

```Groovy
implementation "org.mozilla.components:service-glean:{latest-version}"
```

> **Important:** the `{latest-version}` placeholder in the above link should be replaced with the version of Android Components used by the project.

The Glean SDK is released as part of [android-components](https://github.com/mozilla-mobile/android-components).  Therefore, it follows android-components' versions.
The [android-components release page](https://github.com/mozilla-mobile/android-components/releases/) can be used to determine the latest version.

For example, if version *33.0.0* is used, then the include directive becomes:

```Groovy
implementation "org.mozilla.components:service-glean:33.0.0"
```

> **Size impact on the application APK**: the Glean SDK APK ships binary libraries for all the supported platforms. Each library file measures about 600KB. If the final APK size of the consuming project is a concern, please enable [ABI splits](https://developer.android.com/studio/build/configure-apk-splits#configure-abi-split).

</div>

<div data-lang="Swift" class="tab">

#### Requirements

* Python >= 3.6.

#### Setting up the dependency

The Glean SDK can be consumed through [Carthage](https://github.com/Carthage/Carthage), a dependency manager for macOS and iOS.
For consuming the latest version of the Glean SDK, add the following line to your `Cartfile`:

```
github "mozilla/glean" "{latest-version}"
```

> **Important:** the `{latest-version}` placeholder should be replaced with the version number of the latest Glean SDK release.
> You can find the version number on the [release page](https://github.com/mozilla/glean/releases/latest).

Then check out and build the new dependency:

```
carthage update --platform iOS
```


#### Integrating with the build system

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

#### Combined usage with application-services

If your application uses both the Glean SDK and [application-services](https://github.com/mozilla/application-services)
you can use a combined release to reduce the memory usage and startup time impact.

In your `Cartfile` require only `application-services`, e.g.:

```
github "mozilla/application-services" ~> "{latest-version}"
```

> **Important:** the `{latest-version}` placeholder should be replaced with the version number of the latest application-services release.
> You can find the version number on the [release page](https://github.com/mozilla/application-services/releases/latest).

Then check out and build the new dependency:

```
carthage update --platform iOS
```

</div>

<div data-lang="Python" class="tab">

We recommend using a virtual environment for your work to isolate the dependencies for your project. There are many popular abstractions on top of virtual environments in the Python ecosystem which can help manage your project dependencies.

The Glean SDK Python bindings currently have [prebuilt wheels on PyPI for Windows (i686 and x86_64), Linux/glibc (x86_64) and macOS (x86_64)](https://pypi.org/project/glean-sdk/#files).
For other platforms, including *BSD or Linux distributions that don't use glibc, such as Alpine Linux, the `glean_sdk` package will be built from source on your machine.
This requires that Cargo and Rust are already installed.
The easiest way to do this is through [rustup](https://rustup.rs/).

Once you have your virtual environment set up and activated, you can install the Glean SDK into it using:

```bash
$ python -m pip install glean_sdk
```

The Glean SDK Python bindings make extensive use of type annotations to catch type related errors at build time. We highly recommend adding [mypy](https://mypy-lang.org) to your continuous integration workflow to catch errors related to type mismatches early.

</div>

<div data-lang="C#" class="tab">

TODO. To be implemented in [bug 1643568](https://bugzilla.mozilla.org/show_bug.cgi?id=1643568).

</div>

{{#include ../tab_footer.md}}

### Adding new metrics

All metrics that your project collects must be defined in a `metrics.yaml` file.

To learn more, see [adding new metrics](adding-new-metrics.md).
See the [metric parameters](metric-parameters.md) documentation which provides reference information about the contents of that file.

> **Important**: as stated [before](adding-glean-to-your-project.md#glean-integration-checklist), any new data collection requires documentation and data-review.
> This is also required for any new metric automatically collected by the Glean SDK.

{{#include ../tab_header.md}}

<div data-lang="Kotlin" class="tab">

In order for the Glean SDK to generate an API for your metrics, two Gradle plugins must be included in your build:

- The [Glean Gradle plugin](https://github.com/mozilla/glean/tree/main/gradle-plugin/)
- JetBrains' [Python envs plugin](https://github.com/JetBrains/gradle-python-envs/)

The Glean Gradle plugin is distributed through Mozilla's Maven, so we need to tell your build where to look for it by adding the following to the top of your `build.gradle`:

```
buildscript {
    repositories {
        // Include the next clause if you are tracking snapshots of android components
        maven {
            url "https://snapshots.maven.mozilla.org/maven2"
        }
        maven {
            url "https://maven.mozilla.org/maven2"
        }

        dependencies {
            classpath "org.mozilla.components:tooling-glean-gradle:{android-components-version}"
        }
    }
}
```

> **Important:** as above, the `{android-components-version}` placeholder in the above link should be replaced with the version number of android components used in your project.

The JetBrains Python plugin is distributed in the Gradle plugin repository, so it can be included with:

```Groovy
plugins {
    id "com.jetbrains.python.envs" version "0.0.26"
}
```

Right before the end of the same file, we need to apply the Glean Gradle plugin.
Set any [additional parameters](android/android-build-configuration-options.md) to control the behavior of the Glean Gradle plugin before calling `apply plugin`.


```Groovy
// Optionally, set any parameters to send to the plugin.
ext.gleanGenerateMarkdownDocs = true
apply plugin: "org.mozilla.telemetry.glean-gradle-plugin"
```

> **Note:** Earlier versions of Glean used a Gradle script (`sdk_generator.gradle`) rather than a Gradle plugin. Its use is deprecated and projects should be updated to use the Gradle plugin as described above.

> **Note:** The Glean Gradle plugin has limited support for [offline builds](android/android-offline-builds.md) of applications that use the Glean SDK.

</div>

<div data-lang="Swift" class="tab">

The `metrics.yaml` file is parsed at build time and Swift code is generated.
Add a new `metrics.yaml` file to your Xcode project.

Follow these steps to automatically run the parser at build time:

1. Download the `sdk_generator.sh` script from the Glean repository:
   ```
   https://raw.githubusercontent.com/mozilla/glean/{latest-release}/glean-core/ios/sdk_generator.sh
   ```

    > **Important:** as above, the `{latest-version}` placeholder should be replaced with the version number of Glean SDK release used in this project.

2. Add the `sdk_generator.sh` file to your Xcode project.
3. On your application targets' Build Phases settings tab, click the `+` icon and choose `New Run Script Phase`.
   Create a Run Script in which you specify your shell (ex: `/bin/sh`), add the following contents to the script area below the shell:

   ```
   bash $PWD/sdk_generator.sh
   ```

   > **Note:** If you are using the combined release of application-services and the Glean SDK you need to set the namespace to `MozillaAppServices`, e.g.:
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

   > **Important**: The parser now generates a single file called `Metrics.swift` (since Glean v31.0.0).

5. If you are using Git, add the following lines to your `.gitignore` file:

   ```
   .venv/
   {project-name}/Generated
   ```

   This will ignore files that are generated at build time by the `sdk_generator.sh` script.
   They don't need to be kept in version control, as they can be re-generated from your `metrics.yaml` and `pings.yaml` files.

> **Important information about Glean and embedded extensions:** Metric collection is a no-op in [application extensions](https://developer.apple.com/library/archive/documentation/General/Conceptual/ExtensibilityPG/ExtensionOverview.html#//apple_ref/doc/uid/TP40014214-CH2-SW2) and Glean will not run. Since extensions run in a separate sandbox and process from the application, Glean would run in an extension as if it were a completely separate application with different client ids and storage. This complicates things because Glean doesn’t know or care about other processes. Because of this, Glean is purposefully prevented from running in an application extension and if metrics need to be collected from extensions, it's up to the integrating application to pass the information to the base application to record in Glean.

</div>

<div data-lang="Python" class="tab">

For Python, the `metrics.yaml` file must be available and loaded at runtime.

If your project is a script (i.e. just Python files in a directory), you can load the `metrics.yaml` using:

```Python
from glean import load_metrics

metrics = load_metrics("metrics.yaml")

# Use a metric on the returned object
metrics.your_category.your_metric.set("value")
```

If your project is a distributable Python package, you need to include the `metrics.yaml` file using [one of the myriad ways to include data in a Python package](https://setuptools.readthedocs.io/en/latest/setuptools.html#including-data-files) and then use [`pkg_resources.resource_filename()`](https://setuptools.readthedocs.io/en/latest/pkg_resources.html#resource-extraction) to get the filename at runtime.

```Python
from glean import load_metrics
from pkg_resources import resource_filename

metrics = load_metrics(resource_filename(__name__, "metrics.yaml"))

# Use a metric on the returned object
metrics.your_category.your_metric.set("value")
```

### Automation steps

#### Documentation

The documentation for your application or library's metrics and pings are written in `metrics.yaml` and `pings.yaml`. However, you should also provide human-readable markdown files based on this information, and this is a requirement for Mozilla projects using the Glean SDK. For other languages and platforms, this transformation is done automatically as part of the build. However, for Python the integration to automatically generate docs is an additional step.

The Glean SDK provides a commandline tool for automatically generating markdown documentation from your `metrics.yaml` and `pings.yaml` files. To perform that translation, run `glean_parser`'s `translate` command:

```sh
python3 -m glean_parser translate -f markdown -o docs metrics.yaml pings.yaml
```

To get more help about the commandline options:

```sh
python3 -m glean_parser translate --help
```

We recommend integrating this step into your project's documentation build. The details of that integration is left to you, since it depends on the documentation tool being used and how your project is set up.

#### Metrics linting

Glean includes a "linter" for `metrics.yaml` and `pings.yaml` files called the `glinter` that catches a number of common mistakes in these files.

As part of your continuous integration, you should run the following on your `metrics.yaml` and `pings.yaml` files:

```sh
python3 -m glean_parser glinter metrics.yaml pings.yaml
```

</div>

<div data-lang="C#" class="tab">

A new build target needs to be added to the project `csproj` file in order to generate the metrics and pings APIs from the registry files (e.g. `metrics.yaml`, `pings.yaml`).

```xml
<Project>
  <!-- ... other directives ... -->

  <Target Name="GleanIntegration" BeforeTargets="CoreCompile">
    <ItemGroup>
      <!--
        Note that the two files are not required: Glean will work just fine
        with just the 'metrics.yaml'. A 'pings.yaml' is only required if custom
        pings are defined.
        Please also note that more than one metrics file can be added.
      -->
      <GleanRegistryFiles Include="metrics.yaml" />
      <GleanRegistryFiles Include="pings.yaml" />
    </ItemGroup>
    <!-- This is what actually runs the parser. -->
    <GleanParser RegistryFiles="@(GleanRegistryFiles)" OutputPath="$(IntermediateOutputPath)Glean" Namespace="csharp.GleanMetrics" />

    <!--
      And this adds the generated files to the project, so that they can be found by
      the compiler and Intellisense.
    -->
    <ItemGroup>
      <Compile Include="$(IntermediateOutputPath)Glean\**\*.cs" />
    </ItemGroup>
  </Target>
</Project>
```

This is using the Python 3 interpreter found in `PATH` under the hood. The `GLEAN_PYTHON` environment variable can be used to provide the location of the Python 3 interpreter.

</div>

{{#include ../tab_footer.md}}

### Adding custom pings

Please refer to the [custom pings documentation](pings/custom.md).

> **Important**: as stated [before](adding-glean-to-your-project.md#glean-integration-checklist), any new data collection requires documentation and data-review.
> This is also required for any new metric automatically collected by the Glean SDK.

### Parallelism

All of the Glean SDK's target languages use a separate worker thread to do most of its work, including any I/O. This thread is fully managed by the Glean SDK as an implementation detail. Therefore, users should feel free to use the Glean SDK wherever it is most convenient, without worrying about the performance impact of updating metrics and sending pings.

{{#include ../tab_header.md}}

<div data-lang="Python" class="tab">
Since the Glean SDK performs disk and networking I/O, it tries to do as much of its work as possible on separate threads and processes.
Since there are complex trade-offs and corner cases to support Python parallelism, it is hard to design a one-size-fits-all approach.

#### Default behavior

When using the Python bindings, most of the Glean SDK's work is done on a separate thread, managed by the Glean SDK itself.
The Glean SDK releases the Global Interpreter Lock (GIL) for most of its operations, therefore your application's threads should not be in contention with the Glean SDK's worker thread.

The Glean SDK installs an [`atexit` handler](https://docs.python.org/3/library/atexit.html) so that its worker thread can cleanly finish when your application exits.
This handler will wait up to 30 seconds for any pending work to complete.

By default, ping uploading is performed in a separate child process. This process will continue to upload any pending pings even after the main process shuts down. This is important for commandline tools where you want to return control to the shell as soon as possible and not be delayed by network connectivity.

#### Cases where subprocesses aren't possible

The default approach may not work with applications built using [`PyInstaller`](https://www.pyinstaller.org/) or similar tools which bundle an application together with a Python interpreter making it impossible to spawn new subprocesses of that interpreter. For these cases, there is an option to ensure that ping uploading occurs in the main process. To do this, set the `allow_multiprocessing` parameter on the `glean.Configuration` object to `False`.

#### Using the `multiprocessing` module

Additionally, the default approach does not work if your application uses the `multiprocessing` module for parallelism.
The Glean SDK can not wait to finish its work in a `multiprocessing` subprocess, since `atexit` handlers are not supported in that context.  
Therefore, if the Glean SDK detects that it is running in a `multiprocessing` subprocess, all of its work that would normally run on a worker thread will run on the main thread.
In practice, this should not be a performance issue: since the work is already in a subprocess, it will not block the main process of your application.
</div>

{{#include ../tab_footer.md}}

### Testing metrics

In order to make testing metrics easier 'out of the box', all metrics include a set of test API functions in order to facilitate unit testing.  These include functions to test whether a value has been stored, and functions to retrieve the stored value for validation.  For more information, please refer to [Unit testing Glean metrics](testing-metrics.md).

[dataeng-bug]: https://bugzilla.mozilla.org/enter_bug.cgi?assigned_to=nobody%40mozilla.org&bug_ignored=0&bug_severity=--&bug_status=NEW&bug_type=task&cf_fx_iteration=---&cf_fx_points=---&comment=Application%20friendly%20name%3A%20my_app_name%0D%0AApplication%20ID%3A%20org.mozilla.my_app_id%0D%0ADescription%3A%20Brief%20description%20of%20your%20application%0D%0AGit%20Repository%20URL%3A%20https%3A%2F%2Fgithub.com%2Fmozilla%2Fmy_app_name%0D%0ALocations%20of%20%60metrics.yaml%60%20files%20%28can%20be%20many%29%3A%0D%0A%20%20-%20src%2Fmetrics.yaml%0D%0ALocations%20of%20%60pings.yaml%60%20files%20%28can%20be%20many%29%3A%0D%0A%20-%20src%2Fpings.yaml%0D%0ADependencies%2A%3A%0D%0A%20-%20org.mozilla.components%3Aservice-glean%0D%0ARetention%20Days%3A%20N%2A%2A%0D%0A%0D%0A%0D%0A%2A%20Dependencies%20can%20be%20found%20%5Bin%20the%20Glean%20repositories%5D%28https%3A%2F%2Fprobeinfo.telemetry.mozilla.org%2Fglean%2Frepositories%29.%20Each%20dependency%20must%20be%20listed%20explicitly.%20For%20example%2C%20the%20default%20Glean%20probes%20will%20only%20be%20included%20if%20glean%20itself%20is%20a%20dependency.%0D%0A%0D%0A%2A%2A%20Number%20of%20days%20that%20raw%20data%20will%20be%20retained.%20A%20good%20default%20is%20180.%0D%0A%0D%0AIf%20you%20need%20new%20dependencies%2C%20please%20file%20new%20bugs%20for%20them%2C%20separately%20from%20this%20one.%20For%20any%20questions%2C%20ask%20in%20the%20%23glean%20channel.&component=General&contenttypemethod=list&contenttypeselection=text%2Fplain&defined_groups=1&filed_via=standard_form&flag_type-4=X&flag_type-607=X&flag_type-800=X&flag_type-803=X&flag_type-936=X&form_name=enter_bug&maketemplate=Remember%20values%20as%20bookmarkable%20template&needinfo_from=fbertsch%40mozilla.com%2C%20&op_sys=Unspecified&priority=--&product=Data%20Platform%20and%20Tools&rep_platform=Unspecified&short_desc=Enable%20new%20Glean%20App%20my_app_name&target_milestone=---&version=unspecified
