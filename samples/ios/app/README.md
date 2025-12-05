# A Glean sample app

A minimal example showcasing the integration of Glean into an iOS application.
This app does nothing, but enable Glean.

Glean sends a [`baseline`][] ping when the app is sent to background.

[`baseline`]: https://mozilla.github.io/glean/book/user/pings/baseline.html

## Build

1. Install Xcode 26.2 or higher.

2. Install the latest [Xcode developer tools](https://developer.apple.com/xcode/downloads/) from Apple.

3. Build the XCFramework to be consumed. Run the following from the root directory of this repository:

    ```
    ./bin/build-xcframework.sh
    ```

4. Open `samples/ios/app/glean-sample-app.xcodeproj` in Xcode.

5. Build/Run the `glean-sample-app` scheme in Xcode.
