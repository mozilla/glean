# Android SDK / NDK versions

The Glean SDK implementation requires the following Android SDK/NDK tooling:

* SDK API 37.2
    * Look for `android-37.2` in the SDK manager
    * or install with: `sdkmanager --verbose "platforms;android-37.2"`
* Android Command line tools
    * Download link: <https://dl.google.com/android/repository/commandlinetools-linux-16111833_latest.zip>
* NDK r30
    * Download link: <https://dl.google.com/android/repository/android-ndk-r30-linux.zip>

For the full setup see [Setup the Android Build Environment](setup-android-build-environment.html).

The versions are defined in the following files.
All locations need to be updated on upgrades:

* Documentation
    * this file (`docs/dev/core/internal/sdk-ndk-versions.md`)
    * `dev/android/setup-android-build-environment.md`
* CI configuration
    * `.circleci/config.yml`
        * `sdkmanager 'build-tools;36.0.0'`
        * `image: circleci/android:2025.10.1-browsers`
    * `.github/workflows/test.yml`
        * `ANDROID_NDK: "ndk;30.0.16248370"`
    * `taskcluster/docker/linux/Dockerfile`.
        * `ENV ANDROID_BUILD_TOOLS "37.0.0"`
        * `ENV ANDROID_TOOLS_VERSION "16111833"`
        * `ENV ANDROID_PLATFORM_VERSION "37.2"`
        * `ENV ANDROID_NDK_VERSION "30.0.16248370"`
