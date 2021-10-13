# Adding Glean to your Qt/QML project

This page provides a step-by-step guide on how to integrate the [Glean.js](https://github.com/mozilla/glean.js/) library into a Qt/QML project.

Nevertheless this is just one of the required steps for integrating Glean successfully into a project. Check you the full [Glean integration checklist](./index.md) for a comprehensive list of all the steps involved in doing so.

## Requirements

* Python >= 3.6
* Qt >= 5.15.2

## Setting up the dependency

Glean.js' Qt/QML build is distributed as an asset with every Glean.js release. In order to download
the latest version visit [https://github.com/mozilla/glean.js/releases/latest](https://github.com/mozilla/glean.js/releases/latest).

Glean.js is a [QML module](https://doc.qt.io/qt-5/qtqml-modules-topic.html),
so extract the contents of the downloaded file wherever you keep your other modules.
Make sure that whichever directory that module is placed in, is part of the
[QML Import Path](https://doc.qt.io/qt-5/qtqml-syntax-imports.html#qml-import-path).

After doing that, import Glean like so:

```qml
import org.mozilla.Glean <version>
```

{{#include ../../../shared/blockquote-info.html}}

### Picking the correct version

> The `<version>` number is the version of the release you downloaded minus its patch version.
> For example, if you downloaded Glean.js version `0.15.0` your import statement will be:
>
> ```qml
> import org.mozilla.Glean 0.15
> ```

## Consuming YAML registry files

Qt/QML projects need to setup metrics and pings code generation manually.

First install the [`glean_parser`](https://mozilla.github.io/glean_parser/) CLI tool.

```bash
pip install glean_parser
```

{{#include ../../../shared/blockquote-warning.html}}

### Make sure you have the correct `glean_parser` version!

> Qt/QML support was added to `glean_parser` in version **3.5.0**.

Then call `glean_parser` from the command line:

```bash
glean_parser translate path/to/metrics.yaml path/to/pings.yaml \
  -f javascript \
  -o path/to/generated/files \
  --option platform=qt \
  --option version=0.15
```

The `translate` command will takes a list of YAML registry file paths and an output path and parse
the given YAML registry files into QML JavaScript files.

The generated folder will be a QML module. Make sure wherever the generated module is placed is also
part of the [QML Import Path](https://doc.qt.io/qt-5/qtqml-syntax-imports.html#qml-import-path).

Notice that when building for Qt/QML it is mandatory to give the `translate` command two extra options.

#### `--option platform=qt`

This option is what changes the output file from standard JavaScript to QML JavaScript.

#### `--option version=<version>`

The version passed to this option will be the version of the generated QML module.

## Automation steps

### Documentation

{{#include ../../../shared/blockquote-warning.html}}

#### Prefer using the Glean Dictionary

> While it is still possible to generate Markdown documentation, if working on a public Mozilla project rely on the [Glean Dictionary] for documentation.
> Your product will be automatically indexed by the Glean Dictionary after it gets enabled in the pipeline.

One of the commands provided by `glean_parser` allows users to generate Markdown documentation based
on the contents of their YAML registry files. To perform that translation, use the `translate` command
with a different output format, as shown below.

```bash
glean_parser translate path/to/metrics.yaml path/to/pings.yaml \
  -f markdown \
  -o path/to/docs
```

### YAML registry files linting

`glean_parser` includes a "linter" for the YAML registry files called the `glinter` that catches a
number of common mistakes in these files. To run the linter use the `glinter` command.

```bash
glean_parser glinter path/to/metrics.yaml path/to/pings.yaml
```

[Glean Dictionary]: https://dictionary.telemetry.mozilla.org

## Debugging

By default, the Glean.js QML module uses a minified version of the Glean.js library.
It may be useful to use the unminified version of the library in order to get proper
line numbers and function names when debugging crashes.

The bundle provided contains the unminified version of the library.
In order to use it, open the `glean.js` file inside the included module and change the line:

```
.import "glean.lib.js" as Glean
```

to

```
.import "glean.dev.js" as Glean
```

## Troubleshooting

### `submitPing` may cause crashes when debugging iOS devices

The [`submitPing`](../../reference/pings/index.md) function hits a
[known bug](https://bugreports.qt.io/browse/QTBUG-96788) in the Qt JavaScript interpreter.

This bug is only reproduced in iOS devices, it does not happen in emulators. It also
**only happens when using the Qt debug library for iOS**.

There is no way around this bug other than avoiding the Qt debug library for iOS altogether until
it is fixed. Refer to the [the Qt debugging documentation](https://doc.qt.io/qt-5/debug.html#debugging-in-macos-and-xcode) on how to do that.
