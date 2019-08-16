# Android build script configuration options

This chapter describes build configuration options that control the behavior of Glean's `sdk_generator.gradle` script.
These options are not usually required for normal use.

## `allowMetricsFromAAR`

Normally, Glean looks for `metrics.yaml` and `pings.yaml` files in the root directory of the Glean-using project.
However, in some cases, these files may need to ship inside the dependencies of the project.
For example, this is used in the `engine-gecko` component to grab the `metrics.yaml` from the `geckoview` AAR.

To turn on this behavior, set a variable on the Gradle [ext](https://docs.gradle.org/current/dsl/org.gradle.api.plugins.ExtraPropertiesExtension.html) object:

```groovy
ext.allowMetricsFromAAR = true
```

When this flag is set, every direct dependency of your library will be searched for a `metrics.yaml` file, and those metrics will be treated as the metrics as if they were defined by your library.
That is, API wrappers accessible from your library will be generated for those metrics.

The `metrics.yaml` can be added to the dependency itself by calling this on each relevant build variant:

```groovy
variant.packageLibraryProvider.get().from("${topsrcdir}/path/metrics.yaml")
```
