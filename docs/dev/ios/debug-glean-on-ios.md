# Debug an iOS application against different builds of Glean

At times it may be necessary to debug against a local build of Glean or another git fork or branch in order to test new features or specific versions of Glean.

Glean is shipped as a Swift Package (as [glean-swift](https://github.com/mozilla/glean-swift)).
To replace it in consuming application you need to locally build an XCFramework and replace the Swift package with it.
The following steps guide you through this process.

## Build the XCFramework

In your local Glean checkout run:

```
make build-xcframework
```

This will create the XCFramework in `build/archives/Glean.xcframework`

## Add the Glean XCFramework to the consuming application

1. Open the project in Xcode.
1. Remove the `glean-swift` package dependency in the project's settings.
1. Clear out any build artifacts. Select the `Product` menu, then `Clean Build Folder`.
1. Right-click in the file tree and select "Add files".
1. Select the directory generated in your local Glean checkout, `build/archives/Glean.xcframework`.
1. In the project settings, select your target.
1. In the General section of that target, look for the "Frameworks, Libraries, and Embedded Content" section.
1. Click the `+` (plus) symbol and add the `Glean.xcframework`.
1. Run a full build. Select the `Product` menu, then `Build`.

The application is now built with the locally built Glean XCFramework.

After making changes to Glean you will have to rebuild the XCFramework,
then clean out build artifacts in the consuming application and rebuild the project.
