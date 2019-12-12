# Debugging products using the Glean SDK

## Contents

1. [Debugging the Glean SDK in Android](#debugging-the-glean-sdk-in-android)
2. [Debugging the Glean SDK in iOS](#debugging-the-glean-sdk-in-ios)

### Important considerations when using Glean SDK debug tools

- Options that are set using the adb flags are not immediately reset and will persist until the application is closed or manually reset.

- There are a couple different ways in which to send pings using the Glean SDK debug tools.
    1. You can tag pings using the debug tools and trigger them manually using the UI.  This should always produce a ping with all required fields.
    2. You can tag _and_ send pings using the debug tools.  This has the side effect of potentially sending a ping which does not include all fields because `sendPing` triggers pings to be sent before certain application behaviors can occur which would record that information.  For example, `duration` is not calculated or included in a baseline ping sent with `sendPing` because it forces the ping to be sent before the `duration` metric has been recorded.  Keep in mind that there may be nothing to send, in which case no ping is generated.
    3. You can trigger a command while the instrumented application is still running.  This is useful for toggling commands or for triggering pings that have schedules that are difficult to trigger manually.  This is especially useful if you need to trigger a ping submission after some activity within the application, such as with the metrics ping.

### Glean Log messages

Glean logs warnings and errors through the platform-specific logging frameworks.  See the platform-specific instructions for information on how to view the logs.

## Debugging the Glean SDK in Android

The Glean SDK exports the `GleanDebugActivity` that can be used to toggle debugging features on or off.
Users can invoke this special activity, at run-time, using the following [`adb`](https://developer.android.com/studio/command-line/adb) command:

`adb shell am start -n [applicationId]/mozilla.telemetry.glean.debug.GleanDebugActivity [extra keys]`

> **Note:** In previous versions of Glean the activity was available with the name `mozilla.components.service.glean.debug.GleanDebugActivity`.
>
> If you're debugging an old build, try running:
> `adb shell am start -n [applicationId]/mozilla.components.service.glean.debug.GleanDebugActivity [extra keys]`

In the above:

- `[applicationId]` is the product's application id as defined in the manifest file and/or build script. For the Glean sample application, this is `org.mozilla.samples.glean` for a release build and `org.mozilla.samples.glean.debug` for a debug build.

- `[extra keys]` is a list of extra keys to be passed to the debug activity. See the [documentation](https://developer.android.com/studio/command-line/adb#IntentSpec) for the command line switches used to pass the extra keys.
  These are the currently supported keys:

|key|type|description|
|---|----|-----------|
| logPings | boolean (--ez) | If set to `true`, pings are dumped to logcat; defaults to `false` |
| sendPing | string (--es) | Sends the ping with the given name immediately |
| tagPings | string (--es) | Tags all outgoing pings as debug pings to make them available for real-time validation, on the [Glean Debug View](https://docs.telemetry.mozilla.org/concepts/glean/debug_ping_view.html). The value must match the pattern `[a-zA-Z0-9-]{1,20}` |

For example, to direct a release build of the Glean sample application to (1) dump pings to logcat, (2) tag the ping with the `test-metrics-ping` tag, and (3) send the "metrics" ping immediately, the following command can be used:

```shell
adb shell am start -n org.mozilla.samples.glean/mozilla.telemetry.glean.debug.GleanDebugActivity \
  --ez logPings true \
  --es sendPing metrics \
  --es tagPings test-metrics-ping
```

> **Note:** In previous versions of Glean the activity was available with the name `mozilla.components.service.glean.debug.GleanDebugActivity`.
>
> If you're debugging an old build, try running:
>
> ```shell
> adb shell am start -n org.mozilla.samples.glean/mozilla.components.service.glean.debug.GleanDebugActivity \
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

In the above `[applicationId]` is the product's application id as defined in the manifest file and/or build script. For the Glean sample application, this is `org.mozilla.samples.glean` for a release build and `org.mozilla.samples.glean.debug` for a debug build.

[pidcat]: https://github.com/JakeWharton/pidcat

## Debugging the Glean SDK in iOS

For debugging and validation purposes on iOS, Glean makes use of a custom URL scheme which is implemented _within the application_ that is consuming Glean.  Glean provides some convenience functions to facilitate this, but it's up to the consuming application to enable this functionality.  Applications that enable this Glean SDK feature will be able to launch the application from a URL with the Glean debug commands embedded in the URL itself.

### Available commands and query format

There are 3 available commands that you can use with the Glean SDK debug tools

- `logPings`: This is either true or false and will cause pings that are submitted to also be echoed to the device's log
- `tagPings`: This command will tag outgoing pings with the provided value, in order to identify them in the Glean Debug View. Tags need to be string with upper and lower case letters, numbers and dashes, with a max length of 20 characters.
- `sendPing`: This command expects a string name of a ping to force immediate collection and submission of.

The structure of the custom URL uses the following format:

`<protocol>://glean?<command 1>=<paramter 1>&<command 2>=<parameter 2> ...`

Where:

- `<protocol>` is the "Url Scheme" that has been added for your app (see Instrumenting the application below), such as `glean-sample-app`.
- This is followed by `://` and then `glean` which is required for the Glean SDK to recognize the command is meant for it to process.
- Following standard URL query format, the next character after `glean` is the `?` indicating the beginning of the query.
- This is followed by one or more queries in the form of `<command>=<parameter>`, where the command is one of the commands listed above, followed by an `=` and then the value or parameter to be used with the command.

There are a few things to consider when creating the custom URL:  

- Invalid commands will log an error and cause the entire url to be ignored.
- Not all commands are required to be encoded in the URL, you can mix and match the commands that you need.
- Multiple instances of commands are not allowed in the same url and, if present, will cause the entire url to be ignored.

### Instrumenting the application for Glean debug functionality

In order to enable the debugging features in a Glean SDK consuming iOS application, it is necessary to add some information to the application's `Info.plist`, and add a line and possibly an override for a function in the `AppDelegate.swift`.

#### Register custom URL scheme in `Info.plist`

> **Note:** If your application already has a custom URL scheme implemented, there is no need to implement a second scheme, you can simply use that and skip to the next section about adding the convenience method.  If the app doesn't have a custom URL scheme implemented, then you will need to perform the following instructions to register your app to recieve custom URLs.

Find and open the application's `Info.plist` and right click any blank area and select `Add Row` to create a new key.

You will be prompted to select a key from a drop-down menu, scroll down to and select `URL types`.  This creates an array item, which can be expanded by clicking the triangle disclosure icon.

Select `Item 0`, click on it and click the disclosure icon to expand it and show the `URL identifier` line.  Double-click the value field and fill in your identifier, typically the same as the bundle ID.

Right-click on `Item 0` and select `Add Row` from the context menu.  In the dropdown menu, select `URL Schemes` to add the item.

Click on the disclosure icon of `URL Schemes` to expand the item, double-click the value field of `Item 0` and key in the value for your application's custom scheme.  For instance, the Glean sample app uses `glean-sample-app`, which allows for custom URL's to be crafted using that as a protocol, for example: `glean-sample-app://glean?logPings=true`

#### Add the `Glean.handleCustomUrl()` convenience function and necessary overrides

In order to handle the incoming Glean SDK debug commands, it is necessary to implement the override in the application's `AppDelegate.swift` file.  Within that function, you can make use of the convenience function provided in Glean `handleCustomUrl(url: URL)`.  

An example of a simple implementation of this would look like this:

```Swift
func application(_: UIApplication,
                 open url: URL,
                 options _: [UIApplication.OpenURLOptionsKey: Any] = [:]) -> Bool {
    // ...

    // This does nothing if the url isn't meant for Glean.
    Glean.shared.handleCustomUrl(url: url)

    // ...

    return true
}
```

If you need additional help setting up a custom URL scheme in your application, please refer to [Apple's documentation](https://developer.apple.com/documentation/uikit/inter-process_communication/allowing_apps_and_websites_to_link_to_your_content/defining_a_custom_url_scheme_for_your_app).

### Invoking the Glean-iOS debug commands

Now that the app has the Glean SDK debug functionality enabled, there are a few ways in which we can invoke the debug commands.

#### Using a web browser

Perhaps the simplest way to invoke the Glean SDK debug functionality is to open a web browser and type/paste the custom URL into the address bar.  This is especially useful on an actual device because there isn't a good way to launch from the command line and process the URL for an actual device.

Using the glean-sample-app as an example: to activate ping logging, tag the pings to go to the Glean Debug View, and force the `events` ping to be sent, enter the following URL in a web browser on the iOS device:

```shell
glean-sample-app://glean?logPings=true&tagPings=My-ping-tag&sendPing=events
```

This should cause iOS to prompt you with a dialog asking if you want to open the URL in the Glean Sample App, and if you select "Okay" then it will launch (or resume if it's already running) the application with the indicated commands and parameters and immediately force the collection and submission of the events ping.

> **Note:** This method does not work if the browser you are using to input the command is the same application you are attempting to pass the Glean debug commands to.  So, you couldn't use Firefox for iOS to trigger commands within Firefox for iOS.

It is also possible to encode the URL into a 2D barcode or QR code and launch the app via the camera app.  After scanning the encoded URL, the dialog prompting to launch the app should appear as if the URL were entered into the browser address bar.

#### Using the command line

This method is useful for testing via the Simulator, which typically requires a Mac with Xcode installed, including the Xcode command line tools.  In order to perform the same command as above with using the browser to input the URL, you can use the following command in the command line terminal of the Mac:

```shell
xcrun simctl openurl booted "glean-sample-app://glean?logPings=true&tagPings=My-ping-tag&sendPing=events"
```

This will launch the simulator and again prompt the user with a dialog box asking if you want to open the URL in the Glean Sample App (or whichever app you are instrumenting and testing).

### Glean log messages

If debugging in the simulator, the logging messages can be seen in the console window within Xcode.

When running a Glean-powered app in the iOS Simulator or on a device connected to your computer via cable, the following steps will help in getting the logs:

1. Plug in the device and open Xcode
2. Choose Window -> Devices from the menu bar
3. Under the DEVICES section in the left column, choose the device
4. To see the device console, click the up-triangle at the bottom left of the right hand panel

> **Note:** It is helpful to filter the console window by adding the keyword "glean" to the filter box.  This will allow you to see only messages generated by the Glean SDK.

You can refer to [this Apple technical Q&A](https://developer.apple.com/library/archive/qa/qa1747/_index.html) for more information about debugging deployed iOS apps.
