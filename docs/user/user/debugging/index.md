# Debugging products using the Glean SDK

Glean provides a few debugging features to assist with debugging a product using Glean.

## Features

### Log Pings

Print the ping payload upon sending a ping.

### Debug View Tag

Tags all outgoing pings as debug pings to make them available for real-time validation, on the Glean Debug View.

#### Glean Debug View

The [Glean Debug View](https://debug-ping-preview.firebaseapp.com/) enables you to easily see in real-time what data your application is sending.

This data is what actually arrives in our data pipeline, shown in a web
interface that is automatically updated when new data arrives. Any data sent from a Glean-instrumented application usually shows up within 10 seconds,
updating the pages automatically. Pings are retained for 3 weeks.

#### Troubleshooting

If nothing is showing up on the dashboard after you set a `debugViewTag` and you see
`Glean must be enabled before sending pings.` in the logs, Glean is disabled. Check with
the application author on how to re-enable it.

### Source Tags

Tags outgoing pings with a maximum of 5 comma-separated tags.

### Send Ping

Sends a ping on demand.

## Debugging methods

Each Glean SDK may expose one or more of the following methods to
interact with and enable these debugging functionalities.

1. Enable debugging features through APIs exposed through the Glean singleton;
2. Enable debugging features through environment variables set at runtime;
3. Enable debugging features through platform specific tooling.

For methods 1. and 2., refer to the API reference section ["Debugging"](../../reference/debug/index.md)
for detailed information on how to use them.

For method 3. please refer to the platform specific pages on how to debug products using Glean.

### Platform Specific Information

1. [Debugging Android applications using the Glean SDK](./android.md)
2. [Debugging iOS applications using the Glean SDK](./ios.md)
3. [Debugging Python applications using the Glean SDK](./python.md)
4. [Debugging JavaScript applications using Glean.js](./javascript.md)

### Available debugging methods per platform

| | Glean API | Environment Variables | Platform Specific Tooling |
|-:|:-:|:-:|:-:|
| Kotlin | | | ✅ [^1] |
| Swift | ✅ | ✅ | ✅ [^2] |
| Python | | ✅ | |
| Rust | ✅ | ✅ | |
| JavaScript | ✅ | | |
| Firefox Desktop | | ✅ | ✅ [^3] |

[^1]: The Glean Kotlin SDK exposes the [`GleanDebugActivity`](./android.md) for interacting with debug features. Although it is technically possible to also use environment variables in Android, the Glean team is not aware of a proper way to set environment variables in Android devices or emulators.

[^2]: The Glean Swift SDK exposes a [custom URL format](./ios.md) for interacting with debug features.

[^3]: In Firefox Desktop, developers may use the interface exposed through `about:glean` to log, tag or send pings.
