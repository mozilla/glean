# Android build script configuration options

This chapter describes build configuration options that control the behavior of the Glean SDK's Gradle plugin.
These options are not usually required for normal use.

Options can be turned on by setting a variable on the Gradle [`ext`](https://docs.gradle.org/current/dsl/org.gradle.api.plugins.ExtraPropertiesExtension.html) object *before* applying the Glean Gradle plugin.

## `allowMetricsFromAAR`

Normally, the Glean SDK looks for `metrics.yaml` and `pings.yaml` files in the root directory of the Glean-using project.
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

The Glean SDK can automatically generate Markdown documentation for metrics and pings defined in the registry files, in addition to the metrics API code.

```groovy
ext.gleanGenerateMarkdownDocs = true
```

Flipping the feature to `true` will generate a `metrics.md` file in `$projectDir/docs` at build-time.

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
