# Android build script configuration options

This chapter describes build configuration options that control the behavior of the Glean Kotlin SDK's Gradle plugin.
These options are not usually required for normal use.

Options can be turned on by setting a variable on the Gradle [`ext`](https://docs.gradle.org/current/dsl/org.gradle.api.plugins.ExtraPropertiesExtension.html) object *before* applying the Glean Gradle plugin.

## `gleanBuildDate`

Overwrite the auto-generated build date.

If set to `0` a static UNIX epoch time will be used.
If set to a ISO8601 datetime string it will use that date.
Note that any timezone offset will be ignored and UTC will be used.
For other values it will throw an error.

```groovy
ext.gleanBuildDate = "2022-01-03T17:30:00"
```

## `allowMetricsFromAAR`

Normally, the Glean Kotlin SDK looks for `metrics.yaml` and `pings.yaml` files in the root directory of the Glean-using project.
However, in some cases, these files may need to ship inside the dependencies of the project.
For example, this is used in the `engine-gecko` component to grab the `metrics.yaml` from the `geckoview` AAR.

```groovy
ext.allowMetricsFromAAR = true
```

When this flag is set, every direct dependency of your library will be searched for a `metrics.yaml` file, and those metrics will be treated as the metrics as if they were defined by your library.
That is, API wrappers accessible from your library will be generated for those metrics.

The `metrics.yaml` can be added to the dependency itself by calling this on each relevant build variant:

```groovy
variant.packageLibraryProvider.get().from("${topsrcdir}/path/metrics.yaml")
```

## `gleanGenerateMarkdownDocs`


The Glean Kotlin SDK can automatically generate Markdown documentation for metrics and pings defined in the registry files, in addition to the metrics API code.

```groovy
ext.gleanGenerateMarkdownDocs = true
```

Flipping the feature to `true` will generate a `metrics.md` file in `$projectDir/docs` at build-time. In general this is not necessary for projects using Mozilla's data ingestion infrastructure: in those cases human-readable documentation will automatically be viewable via the [Glean Dictionary](https://dictionary.telemetry.mozilla.org).

## `gleanDocsDirectory`

The `gleanDocsDirectory` can be used to customize the path of the documentation output directory.
If `gleanGenerateMarkdownDocs` is disabled, it does nothing.
Please note that only the `metrics.md` will be overwritten: any other file available in the target directory will be preserved.

```groovy
ext.gleanDocsDirectory = "$rootDir/docs/user/telemetry"
```

## `gleanYamlFiles`

By default, the Glean Gradle plugin will look for `metrics.yaml` and `pings.yaml` files in the same directory that the plugin is included from in your application or library.
To override this, `ext.gleanYamlFiles` may be set to a list of explicit paths.

```groovy
ext.gleanYamlFiles = ["$rootDir/glean-core/metrics.yaml", "$rootDir/glean-core/pings.yaml"]
```

## `gleanExpireByVersion`

Expire the metrics and pings by version, using the provided major version.

If enabled, expiring metrics or pings by date will produce an error.

```groovy
ext.gleanExpireByVersion = 25
```

Different products have different ways to compute the product version at build-time.
For this reason the Glean Gradle plugin cannot provide an automated way to detect the product major version at build time.
When using the expiration by version feature in Android, products must provide the major version by themselves.
