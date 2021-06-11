# Debugging

Different platforms have different ways to enable each debug functionality. They may be
enabled

- through APIs exposed on the Glean singleton,
- through environment variables set at run time,
- or through platform specific debug tools.

## Platform Specific Information

Check out the platform specific guides on how to use Glean's debug functionalities.

1. [Debugging Android applications using the Glean SDK](../../user/debugging/android.md)
2. [Debugging iOS applications using the Glean SDK](../../user/debugging/ios.md)
3. [Debugging Python applications using the Glean SDK](../../user/debugging/python.md)
4. [Debugging JavaScript applications using Glean.js](../../user/debugging/javascript.md)

## Features

The Glean SDK provides four debugging features.

### [Log Pings](./logPings.md)

This is either true or false and will cause all subsequent pings that are submitted, to also be echoed to the device's log.

### [Debug View Tag](./debugViewTag.md)

This will tag all subsequent outgoing pings with the provided value, in order to identify them in the [Glean Debug View](../../user/debugging/index.html#glean-debug-view).

### [Source Tags](./logPings.md)

This will tag all subsequent outgoing pings with a maximum of 5 comma-separated tags.

### Send Pings

_This feature is only available through the Android and iOS debug tools._

This expects the name of a ping and forces its immediate submission.
