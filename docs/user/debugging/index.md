# Debugging products using the Glean SDK

## Platform-specific debugging

1. [Debugging Android applications using the Glean SDK](android.md)
2. [Debugging iOS applications using the Glean SDK](ios.md)
3. [Debugging Python applications using the Glean SDK](python.md)

## General debugging information

### Available debugging features

The Glean SDK has 4 available debugging features:

- `logPings`: This is either `true` or `false` and will cause all subsequent pings that are submitted, to also be echoed to the device's log. Once enabled, the only way to disable this feature is to restart or manually reset the application.
- `debugViewTag`: This will tag all subsequent outgoing pings with the provided value, in order to identify them in the Glean Debug View. Once enabled, the only way to disable this feature is to restart or manually reset the application.
- `sourceTags`: This will tag all subsequent outgoing pings with a maximum of 5 comma-separated tags. Once enabled, the only way to disable this feature is to restart or manually reset the application.
- `sendPing`: This expects the name of a ping and forces its immediate collection and submission. _This feature is only available for Android and iOS_.

Different platforms may have different ways to enable these features.

### Enabling debugging features through environment variables

Some of the debugging features described above may be enabled using environment variables:

- `logPings`: May be set by the `GLEAN_LOG_PINGS` environment variable. The accepted values are `true` or `false`. Any other value will be ignored.
- `debugViewTag`: May be set by the `GLEAN_DEBUG_VIEW_TAG` environment variable. Any valid HTTP header value is expected here (e.g. any value that matches the regex `[a-zA-Z0-9-]{1,20}`). Invalid values will be ignored.
- `sourceTags`: May be set by the `GLEAN_SOURCE_TAGS` environment variable. A comma-separated list of valid HTTP header values is expected here (e.g. any value that matches the regex `[a-zA-Z0-9-]{1,20}`). Invalid values will be ignored. The special value of `automation` is meant for tagging pings generated on automation: such pings will be specially handled on the pipeline (i.e. discarded from [non-live views](https://docs.telemetry.mozilla.org/cookbooks/bigquery/querying.html#table-layout-and-naming)).

These variables must be set at runtime, not at compile time. They will be checked upon Glean initialization.

Enabling debugging features using environment variables is available for all supported platforms.

> **Note** Although it is technically possible to use the environment variables described here to enable debugging features in Android, the Glean team is not currently aware of a proper way to set environment variables in Android devices or emulators.

### Important considerations when using Glean SDK debug features

- Enabled features will persist until the application is closed or manually reset. When enabled by environment variables, the variables need to be cleared upon resetting for the feature to be disabled.

- There are a couple different ways in which to send pings using the Glean SDK debug tools.
    1. You can tag pings using the debug tools and trigger them manually using the UI. This should always produce a ping with all required fields.
    2. You can tag _and_ send pings using the debug tools.  This has the side effect of potentially sending a ping which does not include all fields because `sendPing` triggers pings to be sent before certain application behaviors can occur which would record that information.  For example, `duration` is not calculated or included in a baseline ping sent with `sendPing` because it forces the ping to be sent before the `duration` metric has been recorded.  Keep in mind that there may be nothing to send, in which case no ping is generated.
    3. You can trigger a command while the instrumented application is still running.  This is useful for toggling commands or for triggering pings that have schedules that are difficult to trigger manually.  This is especially useful if you need to trigger a ping submission after some activity within the application, such as with the metrics ping.

### Glean SDK Log messages

The Glean SDK logs warnings and errors through the platform-specific logging frameworks.  See the platform-specific instructions for information on how to view the logs.

### Implementation details

See [Debug Pings](../../dev/core/internal/debug-pings.md).
