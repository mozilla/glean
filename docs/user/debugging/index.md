# Debugging products using the Glean SDK

## Platform-specific debugging

1. [Debugging the Glean SDK in Android](android.md)
2. [Debugging the Glean SDK in iOS](ios.md)

## General debugging information

### Available commands and query format

There are 3 available commands that you can use with the Glean SDK debug tools

- `logPings`: This is either true or false and will cause pings that are submitted to also be echoed to the device's log
- `tagPings`: This command will tag outgoing pings with the provided value, in order to identify them in the Glean Debug View. Tags need to be string with upper and lower case letters, numbers and dashes, with a max length of 20 characters.
- `sendPing`: This command expects a string name of a ping to force immediate collection and submission of.

Different platforms have different ways to send these commands.

### Important considerations when using Glean SDK debug tools

- Options that are set using the flags are not immediately reset and will persist until the application is closed or manually reset.

- There are a couple different ways in which to send pings using the Glean SDK debug tools.
    1. You can tag pings using the debug tools and trigger them manually using the UI.  This should always produce a ping with all required fields.
    2. You can tag _and_ send pings using the debug tools.  This has the side effect of potentially sending a ping which does not include all fields because `sendPing` triggers pings to be sent before certain application behaviors can occur which would record that information.  For example, `duration` is not calculated or included in a baseline ping sent with `sendPing` because it forces the ping to be sent before the `duration` metric has been recorded.  Keep in mind that there may be nothing to send, in which case no ping is generated.
    3. You can trigger a command while the instrumented application is still running.  This is useful for toggling commands or for triggering pings that have schedules that are difficult to trigger manually.  This is especially useful if you need to trigger a ping submission after some activity within the application, such as with the metrics ping.

### Glean Log messages

Glean logs warnings and errors through the platform-specific logging frameworks.  See the platform-specific instructions for information on how to view the logs.
