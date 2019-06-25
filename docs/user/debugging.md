# Debugging products using Glean

Glean exports the `GleanDebugActivity` that can be used to toggle debugging features on or off. 
Users can invoke this special activity, at run-time, using the following [`adb`](https://developer.android.com/studio/command-line/adb) command:

`adb shell am start -n [applicationId]/mozilla.components.service.glean.debug.GleanDebugActivity [extra keys]`

In the above:

- `[applicationId]` is the product's application id as defined in the manifest file and/or build script. For the Glean sample application, this is `org.mozilla.samples.glean` for a release build and `org.mozilla.samples.glean.debug` for a debug build.

- `[extra keys]` is a list of extra keys to be passed to the debug activity. See the [documentation](https://developer.android.com/studio/command-line/adb#IntentSpec) for the command line switches used to pass the extra keys. 
  These are the currently supported keys:

    |key|type|description|
    |---|----|-----------|
    | logPings | boolean (--ez) | If set to `true`, Glean dumps pings to logcat; defaults to `false` |
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
