# iOS build script configuration options

This chapter describes build configuration options that control the behavior of the Glean Swift SDK's `sdk_generator.sh`.
These options are not usually required for normal use.

Options can be passed as command-line flags.

## `--output <PATH>` / `-o <PATH`

Default: `$SOURCE_ROOT/$PROJECT/Generated`

The folder to place generated code in.

## `--build-date <TEXT>` / `-b <TEXT>`

Default: auto-generated.

Overwrite the auto-generated build date.

If set to `0` a static UNIX epoch time will be used.
If set to a ISO8601 datetime string (e.g. `2022-01-03T17:30:00`) it will use that date.
Note that any timezone offset will be ignored and UTC will be used.
For other values it will throw an error.

```sh
bash sdk_generator.sh --build-date 2022-01-03T17:30:00
```

## `--expire-by-version <INTEGER>`

Default: none.

Expire the metrics and pings by version, using the provided major version.

If enabled, expiring metrics or pings by date will produce an error.

```sh
bash sdk_generator.sh --expire-by-version 95
```

Different products have different ways to compute the product version at build-time.
For this reason the `sdk_generator.sh` script cannot provide an automated way to detect the product major version at build time.
When using the expiration by version feature in iOS,
products must provide the major version by themselves.

## `--markdown <PATH>` / `-m <PATH>`

Default: unset.

The Glean Swift SDK can automatically generate Markdown documentation for metrics and pings defined in the registry files, in addition to the metrics API code.
If set the documentation will be generated in the provided path.

```sh
bash sdk_generator.sh --markdown $SOURCE_ROOT/docs
```

In general this is not necessary for projects using Mozilla's data ingestion infrastructure:
in those cases human-readable documentation will automatically be viewable via the [Glean Dictionary](https://dictionary.telemetry.mozilla.org).

## `--glean-namespace <NAME>` / `-g <NAME>`

Default: `Glean`

The Glean namespace to use in generated code.

```sh
bash sdk_generator.sh --glean-namespace AnotherGlean
```

If you are using the combined release of application-services and the Glean Swift SDK you need to set the namespace to `MozillaAppServices`, e.g.:

```sh
bash sdk_generator.sh --glean-namespace MozillaAppServices
```
