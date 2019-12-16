# Debugging products using the Glean SDK

## Contents

1. [Debugging the Glean SDK in Android](android.md)
2. [Debugging the Glean SDK in iOS](ios.md)

### Important considerations when using Glean SDK debug tools

- Options that are set using the flags are not immediately reset and will persist until the application is closed or manually reset.

- There are a couple different ways in which to send pings using the Glean SDK debug tools.
    1. You can tag pings using the debug tools and trigger them manually using the UI.  This should always produce a ping with all required fields.
    2. You can tag _and_ send pings using the debug tools.  This has the side effect of potentially sending a ping which does not include all fields because `sendPing` triggers pings to be sent before certain application behaviors can occur which would record that information.  For example, `duration` is not calculated or included in a baseline ping sent with `sendPing` because it forces the ping to be sent before the `duration` metric has been recorded.  Keep in mind that there may be nothing to send, in which case no ping is generated.
    3. You can trigger a command while the instrumented application is still running.  This is useful for toggling commands or for triggering pings that have schedules that are difficult to trigger manually.  This is especially useful if you need to trigger a ping submission after some activity within the application, such as with the metrics ping.

### Glean Log messages

Glean logs warnings and errors through the platform-specific logging frameworks.  See the platform-specific instructions for information on how to view the logs.
