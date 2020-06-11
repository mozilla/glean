# Adding Glean to your project

## Before using Glean

Products using the Glean SDK to collect telemetry **must**:

- add documentation for any new metric collected with the library in its repository (see [an example](pings/index.md));
- include the markdown-formatted documentation generated from the `metrics.yaml` and `pings.yaml` files in the project's documentation;
- go through data review for the newly collected data by following [this process](https://wiki.mozilla.org/Firefox/Data_Collection);
- provide a way for users to turn data collection off (e.g. providing settings to control
  `Glean.setUploadEnabled()`).

## Usage

### Integrating with your project

{{#include ../tab_header.md}}

<div data-lang="Kotlin" class="tab">

#### Setting up the dependency

Glean is published on [maven.mozilla.org](https://maven.mozilla.org/).
To use it, you need to add the following to your project's top-level build file,
in the `allprojects` block (see e.g. [Glean's own `build.gradle`](https://github.com/mozilla/glean/blob/main/build.gradle)):

```Groovy
repositories {
    maven {
       url "https://maven.mozilla.org/maven2"
    }
}
```

Each module that uses Glean needs to specify it in its build file, in the `dependencies` block.
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


</div>

<div data-lang="Swift" class="tab">

#### Setting up the dependency

Glean can be consumed through [Carthage](https://github.com/Carthage/Carthage), a dependency manager for macOS and iOS.
For consuming the latest version of Glean, add the following line to your `Cartfile`:

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
</div>

<div data-lang="Python" class="tab">

We recommend using a virtual environment for your work to isolate the dependencies for your project. There are many popular abstractions on top of virtual environments in the Python ecosystem which can help manage your project dependencies.

The Python Glean bindings currently have [prebuilt wheels on PyPI for x86_64 Windows, Linux and macOS](https://pypi.org/project/glean-sdk/#files).

If you're running one of those platforms and have your virtual environment set up and activated, you can install Glean into it using:

```bash
$ python -m pip install glean_sdk
```

If you are not on one of these platforms, you will need to build the Glean Python bindings from source using [these instructions](../dev/python/setting-up-python-build-environment.html).

</div>

{{#include ../tab_footer.md}}

### Adding new metrics

All metrics that your project collects must be defined in a `metrics.yaml` file.

The format of that file is documented [with `glean_parser`](https://mozilla.github.io/glean_parser/metrics-yaml.html).
To learn more, see [adding new metrics](adding-new-metrics.md).

> **Important**: as stated [before](adding-glean-to-your-project.md#before-using-glean), any new data collection requires documentation and data-review.
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
            classpath "org.mozilla.components:tooling-gradle-plugin:{android-components-version}"
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
Set any [additional parameters](android-build-configuration-options.md) to control the behavior of the Glean Gradle plugin before calling `apply plugin`.


```Groovy
// Optionally, set any parameters to send to the plugin.
ext.gleanGenerateMarkdownDocs = true
apply plugin: "org.mozilla.telemetry.glean-gradle-plugin"
```

> **Note:** Earlier versions of Glean used a Gradle script (`sdk_generator.gradle`) rather than a Gradle plugin. Its use is deprecated and projects should be updated to use the Gradle plugin as described above.

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

The documentation for your application or library's metrics and pings are written in `metrics.yaml` and `pings.yaml`. However, you should also provide human-readable markdown files based on this information, and this is a requirement for Mozilla projects using Glean. For other languages and platforms, this transformation is done automatically as part of the build. However, for Python the integration to automatically generate docs is an additional step.

Glean provides a commandline tool for automatically generating markdown documentation from your `metrics.yaml` and `pings.yaml` files. To perform that translation, run `glean_parser`'s `translate` command:

```sh
python3 -m glean_parser translate -f markdown -o docs metrics.yaml pings.yaml
```

To get more help about the commandline options:

```sh
python3 -m glean_parser translate --help
```

We recommend integrating this step into your project's documentation build. The details of that integration is left to you, since it depends on the documentation tool being used and how your project is set up.

</div>

{{#include ../tab_footer.md}}

### Adding custom pings

Please refer to the [custom pings documentation](pings/custom.md).

> **Important**: as stated [before](adding-glean-to-your-project.md#before-using-glean), any new data collection requires documentation and data-review.
> This is also required for any new metric automatically collected by the Glean SDK.

### Parallelism

All of Glean's target languages use a separate worker thread to do most of Glean's work, including any I/O. This thread is fully managed by Glean as an implementation detail. Therefore, users should be free to use the Glean API wherever it is most convenient, without worrying about the performance impact of updating metrics and sending pings.

{{#include ../tab_header.md}}

<div data-lang="Python" class="tab">
When using the Python bindings, Glean's work is done on a separate thread, managed by Glean itself.
Glean releases the Global Interpreter Lock (GIL), therefore your application's threads should not be in contention with Glean's thread.

Glean installs an [`atexit` handler](https://docs.python.org/3/library/atexit.html) so the Glean thread can attempt to cleanly shut down when your application exits.
This handler will wait up to 1 second for any pending work to complete.

In addition, by default ping uploading is performed in a separate child process. This process will continue to upload any pending pings even after the main process shuts down.

Note that this approach may not work with applications built using [`PyInstaller`](https://www.pyinstaller.org/) or similar tools which bundle an application together with a Python interpreter. For these cases (and any others which we haven't thought of), there is an option to ensure that ping uploading occurs in the main process. To do this, set the `allow_multiprocessing` parameter on the `glean.Configuration` object to `False`.
</div>

{{#include ../tab_footer.md}}

### Testing metrics

In order to make testing metrics easier 'out of the box', all metrics include a set of test API functions in order to facilitate unit testing.  These include functions to test whether a value has been stored, and functions to retrieve the stored value for validation.  For more information, please refer to [Unit testing Glean metrics](testing-metrics.md).

### Adding metadata about your project to the pipeline

In order for data to be collected from your project, its application id must be registered in the pipeline.

[File a data engineering bug][dataeng-bug] to enable your product's application id.

[dataeng-bug]: https://bugzilla.mozilla.org/enter_bug.cgi?product=Data%20Platform%20and%20Tools&component=General&short_desc=Glean:%20Enable%20application%20id%20org.mozilla.myProduct&comment=%2A%2A%20Please%20needinfo%20mreid%40mozilla.com%20when%20filing%20this%20bug%20for%20the%20fastest%20response%20%2A%2A
