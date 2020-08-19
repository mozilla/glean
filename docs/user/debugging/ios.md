# Enabling debugging features in iOS through environment variables

Debugging features in iOS can be enabled using environment variables.
For more information on the available features accessible through this method
and how to enable them, see [Enabling debugging features through environment variables](./index.md).

These environment variables must be set on the device that is running the application.

> **Note** To set environment variables to the process running your app in an iOS device or emulator you need to edit the scheme for your app. In the Xcode IDE, you can use the shortcut `Cmd + <` to open the scheme editor popup. The environment variables editor is under the `Arguments` tab on this popup.

# Debugging iOS applications using the Glean SDK

For debugging and validation purposes on iOS, the Glean SDK makes use of a custom URL scheme which is implemented _within the application_ that is consuming the Glean SDK.  The Glean SDK provides some convenience functions to facilitate this, but it's up to the consuming application to enable this functionality.  Applications that enable this Glean SDK feature will be able to launch the application from a URL with the Glean debug commands embedded in the URL itself.

### Available commands and query format

There are 4 available commands that you can use with the Glean SDK debug tools

- `logPings`: This is either true or false and will cause pings that are submitted to also be echoed to the device's log
- `debugViewTag`: This command will tag outgoing pings with the provided value, in order to identify them in the Glean Debug View. Tags need to be string with upper and lower case letters, numbers and dashes, with a max length of 20 characters. **Important**: in older versions of the Glean SDK, this was named `tagPings`.
- `sourceTags`: This command tags outgoing pings with a maximum of 5 comma-separated tags. The tags must match the pattern `[a-zA-Z0-9-]{1,20}`. The `automation` tag is meant for tagging pings generated on automation: such pings will be specially handled on the pipeline (i.e. discarded from [non-live views](https://docs.telemetry.mozilla.org/cookbooks/bigquery/querying.html#table-layout-and-naming)). Tags starting with `glean` are reserved for future use.
- `sendPing`: This command expects a string name of a ping to force immediate collection and submission of.

The structure of the custom URL uses the following format:

`<protocol>://glean?<command 1>=<paramter 1>&<command 2>=<parameter 2> ...`

Where:

- `<protocol>` is the "URL Scheme" that has been added for your app (see Instrumenting the application below), such as `glean-sample-app`.
- This is followed by `://` and then `glean` which is required for the Glean SDK to recognize the command is meant for it to process.
- Following standard URL query format, the next character after `glean` is the `?` indicating the beginning of the query.
- This is followed by one or more queries in the form of `<command>=<parameter>`, where the command is one of the commands listed above, followed by an `=` and then the value or parameter to be used with the command.

There are a few things to consider when creating the custom URL:

- Invalid commands will log an error and cause the entire URL to be ignored.
- Not all commands are required to be encoded in the URL, you can mix and match the commands that you need.
- Multiple instances of commands are not allowed in the same URL and, if present, will cause the entire URL to be ignored.
- The `logPings` command doesn't trigger ping submission and you won't see any output until a ping has been submitted. You can use the `sendPings` command to force a ping to be sent, but it could be more desirable to trigger the pings submission on their normal schedule. For instance, the `baseline` and `events` pings can be triggered by moving the app out of the foreground and the `metrics` ping can be triggered normally if it is overdue for the current calendar day. See the [ping documentation](../pings/index.md) for more information on ping scheduling to learn when pings are sent.
- Enabling debugging features through custom URLs overrides any debugging features set through environment variables.

### Instrumenting the application for Glean SDK debug functionality

In order to enable the debugging features in a Glean SDK consuming iOS application, it is necessary to add some information to the application's `Info.plist`, and add a line and possibly an override for a function in the `AppDelegate.swift`.

#### Register custom URL scheme in `Info.plist`

> **Note:** If your application already has a custom URL scheme implemented, there is no need to implement a second scheme, you can simply use that and skip to the next section about adding the convenience method.  If the app doesn't have a custom URL scheme implemented, then you will need to perform the following instructions to register your app to receive custom URLs.

Find and open the application's `Info.plist` and right click any blank area and select `Add Row` to create a new key.

You will be prompted to select a key from a drop-down menu, scroll down to and select `URL types`.  This creates an array item, which can be expanded by clicking the triangle disclosure icon.

Select `Item 0`, click on it and click the disclosure icon to expand it and show the `URL identifier` line.  Double-click the value field and fill in your identifier, typically the same as the bundle ID.

Right-click on `Item 0` and select `Add Row` from the context menu.  In the dropdown menu, select `URL Schemes` to add the item.

Click on the disclosure icon of `URL Schemes` to expand the item, double-click the value field of `Item 0` and key in the value for your application's custom scheme.  For instance, the Glean sample app uses `glean-sample-app`, which allows for custom URLs to be crafted using that as a protocol, for example: `glean-sample-app://glean?logPings=true`

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
glean-sample-app://glean?logPings=true&debugViewTag=My-ping-tag&sendPing=events
```

This should cause iOS to prompt you with a dialog asking if you want to open the URL in the Glean Sample App, and if you select "Okay" then it will launch (or resume if it's already running) the application with the indicated commands and parameters and immediately force the collection and submission of the events ping.

> **Note:** This method does not work if the browser you are using to input the command is the same application you are attempting to pass the Glean debug commands to.  So, you couldn't use Firefox for iOS to trigger commands within Firefox for iOS.

It is also possible to encode the URL into a 2D barcode or QR code and launch the app via the camera app.  After scanning the encoded URL, the dialog prompting to launch the app should appear as if the URL were entered into the browser address bar.

#### Using the command line

This method is useful for testing via the Simulator, which typically requires a Mac with Xcode installed, including the Xcode command line tools.  In order to perform the same command as above with using the browser to input the URL, you can use the following command in the command line terminal of the Mac:

```shell
xcrun simctl openurl booted "glean-sample-app://glean?logPings=true&debugViewTag=My-ping-tag&sendPing=events"
```

This will launch the simulator and again prompt the user with a dialog box asking if you want to open the URL in the Glean Sample App (or whichever app you are instrumenting and testing).

### Glean log messages

The Glean SDK integrates with the [unified logging system](https://developer.apple.com/documentation/os/logging/) available on iOS.
There are various ways to retrieve log information, see the [official documentation](https://developer.apple.com/documentation/os/logging/viewing_log_messages).

If debugging in the simulator, the logging messages can be seen in the console window within Xcode.

When running a Glean-powered app in the iOS Simulator or on a device connected to your computer via cable you can use `Console.app` to view the system log.
You can filter the logs with `category:glean` to only see logs from the Glean SDK.

You can also use the command line utility `log` to stream the log output.
Run the following in a shell:

```
log stream --predicate 'category contains "glean"'
```

See [Diagnosing Issues Using Crash Reports and Device Logs](https://developer.apple.com/library/archive/qa/qa1747/_index.html) for more information about debugging deployed iOS apps.
