# A Glean sample app

A minimal example showcasing the integration of Glean into an iOS application.
This app does nothing, but enable Glean.

Glean sends a [`baseline`][] ping when the app is sent to background.

[`baseline`]: https://mozilla.github.io/glean/book/user/pings/baseline.html

## Build

1. Install Xcode version 11.0 + Swift 5.1.

2. Install the latest [Xcode developer tools](https://developer.apple.com/xcode/downloads/) from Apple.

3. Install Carthage:

    ```
    brew update
    brew install carthage
    ```

4. Pull in the sample app's dependencies:

    ```
    ./bootstrap.sh
    ```

6. Open `glean-sample-app.xcodeproj` in Xcode.

7. Build/Run the `glean-sample-app` scheme in Xcode.
