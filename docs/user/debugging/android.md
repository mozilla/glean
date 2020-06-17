# Debugging Android applications using the Glean SDK

The Glean SDK exports the `GleanDebugActivity` that can be used to toggle debugging features on or off.
Users can invoke this special activity, at run-time, using the following [`adb`](https://developer.android.com/studio/command-line/adb) command:

`adb shell am start -n [applicationId]/mozilla.telemetry.glean.debug.GleanDebugActivity [extra keys]`

> **Note:** In previous versions of Glean the activity was available with the name `mozilla.components.service.glean.debug.GleanDebugActivity`.
>
> If you're debugging an old build, try running:
> `adb shell am start -n [applicationId]/mozilla.components.service.glean.debug.GleanDebugActivity [extra keys]`

In the above:

- `[applicationId]` is the product's application id as defined in the manifest file and/or build script. For the Glean sample application, this is `org.mozilla.samples.gleancore` for a release build and `org.mozilla.samples.gleancore.debug` for a debug build.

- `[extra keys]` is a list of extra keys to be passed to the debug activity. See the [documentation](https://developer.android.com/studio/command-line/adb#IntentSpec) for the command line switches used to pass the extra keys.
  These are the currently supported keys:

|key|type|description|
|---|----|-----------|
| `logPings` | boolean (`--ez`)  | If set to `true`, pings are dumped to logcat; defaults to `false` |
| `sendPing` | string (`--es`)  | Sends the ping with the given name immediately |
| `tagPings` | string (`--es`)  | Tags all outgoing pings as debug pings to make them available for real-time validation, on the [Glean Debug View](./debug-ping-view.md). The value must match the pattern `[a-zA-Z0-9-]{1,20}` |

> **Note:** Due to limitations on Android logcat message size, pings larger than 4KB are broken into multiple log messages when using `logPings`.

For example, to direct a release build of the Glean sample application to (1) dump pings to logcat, (2) tag the ping with the `test-metrics-ping` tag, and (3) send the "metrics" ping immediately, the following command can be used:

```shell
adb shell am start -n org.mozilla.samples.gleancore/mozilla.telemetry.glean.debug.GleanDebugActivity \
  --ez logPings true \
  --es sendPing metrics \
  --es tagPings test-metrics-ping
```

The `logPings` command doesn't trigger ping submission and you won't see any output until a ping has been submitted. You can use the `sendPing` command to force a ping to be sent, but it could be more desirable to trigger the pings submission on their normal schedule. For instance, the `baseline` and `events` pings can be triggered by moving the app out of the foreground and the `metrics` ping can be triggered normally if it is overdue for the current calendar day.

Note that if no metrics have been collected, then no pings will be sent *unless* [`send_if_empty` is set on your ping](../pings/custom.md#defining-a-custom-ping). See the [ping documentation](../pings/index.md) for more information on ping scheduling to learn when pings are sent.

Options that are set using the `adb` flags are not immediately reset and will
persist until the application is closed or manually reset.

> **Note:** In previous versions of Glean the activity was available with the name `mozilla.components.service.glean.debug.GleanDebugActivity`.
>
> If you're debugging an old build, try running:
>
> ```shell
> adb shell am start -n org.mozilla.samples.gleancore/mozilla.components.service.glean.debug.GleanDebugActivity \
>   --ez logPings true \
>   --es sendPing metrics \
>   --es tagPings test-metrics-ping
> ```

### Glean Log messages

When running a Glean-powered app in the Android emulator or on a device connected to your computer via cable, there are several ways to read the log output.

#### Android Studio

Android Studio can show the logs of a connected emulator or device.
To display the log messages for an app:

1. Run an app on your device.
2. Click **View > Tool Windows > Logcat** (or click **Logcat** in the tool window bar).

The Logcat window will show all log messages and allows to filter those by the application ID.
Select the application ID of the product you're debugging.
You can also filter by `Glean` only.

More information can be found in the [View Logs with Logcat][] help article.

[View Logs with Logcat]: https://developer.android.com/studio/debug/am-logcat

#### Command line

On the command line you can show all of the log output using:

```shell
adb logcat
```

This is the unfiltered output of all log messages.
You can match for `glean` using grep:

```shell
adb logcat | grep -i glean
```

A simple way to filter for only the application that is being debugged is by using [pidcat][], a wrapper around `adb`, which adds colors and proper filtering by application ID and log level.
Run it like this to filter for an application:

```shell
pidcat [applicationId]
```

In the above `[applicationId]` is the product's application id as defined in the manifest file and/or build script. For the Glean sample application, this is `org.mozilla.samples.gleancore` for a release build and `org.mozilla.samples.gleancore.debug` for a debug build.

[pidcat]: https://github.com/JakeWharton/pidcat
