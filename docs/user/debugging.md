# Debugging products using the Glean SDK

The Glean SDK exports the `GleanDebugActivity` that can be used to toggle debugging features on or off.
Users can invoke this special activity, at run-time, using the following [`adb`](https://developer.android.com/studio/command-line/adb) command:

`adb shell am start -n [applicationId]/mozilla.components.service.glean.debug.GleanDebugActivity [extra keys]`

In the above:

- `[applicationId]` is the product's application id as defined in the manifest file and/or build script. For the Glean sample application, this is `org.mozilla.samples.glean` for a release build and `org.mozilla.samples.glean.debug` for a debug build.

- `[extra keys]` is a list of extra keys to be passed to the debug activity. See the [documentation](https://developer.android.com/studio/command-line/adb#IntentSpec) for the command line switches used to pass the extra keys.
  These are the currently supported keys:

|key|type|description|
|---|----|-----------|
| logPings | boolean (--ez) | If set to `true`, pings are dumped to logcat; defaults to `false` |
| sendPing | string (--es) | Sends the ping with the given name immediately |
| tagPings | string (--es) | Tags all outgoing pings as debug pings to make them available for real-time validation. The value must match the pattern `[a-zA-Z0-9-]{1,20}` |

For example, to direct a release build of the Glean sample application to (1) dump pings to logcat, (2) tag the ping with the `test-metrics-ping` tag, and (3) send the "metrics" ping immediately, the following command can be used:

```
adb shell am start -n org.mozilla.samples.glean/mozilla.components.service.glean.debug.GleanDebugActivity \
  --ez logPings true \
  --es sendPing metrics \
  --es tagPings test-metrics-ping
```

### Important GleanDebugActivity notes!

- Options that are set using the adb flags are not immediately reset and will persist until the application is closed or manually reset.

- There are a couple different ways in which to send pings through the GleanDebugActivity.
    1. You can use the `GleanDebugActivity` in order to tag pings and trigger them manually using the UI.  This should always produce a ping with all required fields.
    2. You can use the `GleanDebugActivity` to tag _and_ send pings.  This has the side effect of potentially sending a ping which does not include all fields because `sendPings` triggers pings to be sent before certain application behaviors can occur which would record that information.  For example, `duration` is not calculated or included in a baseline ping sent with `sendPing` because it forces the ping to be sent before the `duration` metric has been recorded.

## Glean Log messages

Glean logs warnings and errors through the Android logging framework.
When running a Glean-powered app in the Android emulator or on a device connected to your computer via cable, there are several ways to read the log output.

### Android Studio

Android Studio can show the logs of a connected emulator or device.
To display the log messages for an app:

1. Run an app on your device.
2. Click **View > Tool Windows > Logcat** (or click **Logcat** in the tool window bar).

The Logcat window will show all log messages and allows to filter those by the application ID.
Select the application ID of the product you're debugging.

More information can be found in the [View Logs with Logcat][] help article.

[View Logs with Logcat]: https://developer.android.com/studio/debug/am-logcat

### Command line

On the command line you can show all of the log output using:

```
adb logcat
```

This is the unfiltered output of all log messages.
You can match for `glean` using grep:

```
adb logcat | grep -i glean
```

A simple way to filter for only the application that is being debugged is by using [pidcat][], a wrapper around `adb`, which adds colors and proper filtering by application ID and log level.
Run it like this to filter for an application:

```
pidcat [applicationId]
```

In the above `[applicationId]` is the product's application id as defined in the manifest file and/or build script. For the Glean sample application, this is `org.mozilla.samples.glean` for a release build and `org.mozilla.samples.glean.debug` for a debug build.

[pidcat]: https://github.com/JakeWharton/pidcat
