# Android SDK / NDK versions

The Glean SDK implementation is currently build against the following versions:

* SDK API 34
    * Look for `android-34` in the SDK manager
    * or install with: `sdkmanager --verbose "platforms;android-34"`
* Android Command line tools
    * Download link: <https://dl.google.com/android/repository/commandlinetools-linux-11076708_latest.zip>
* NDK r25
    * Download link: <https://dl.google.com/android/repository/android-ndk-r25c-linux.zip>

For the full setup see [Setup the Android Build Environment](setup-android-build-environment.html).

The versions are defined in the following files.
All locations need to be updated on upgrades:

* Documentation
    * this file (`docs/dev/core/internal/sdk-ndk-versions.md`)
    * `dev/android/setup-android-build-environment.md`
* CI configuration
    * `.circleci/config.yml`
        * `sdkmanager 'build-tools;34.0.0'`
        * `image: circleci/android:2024.01.1-browsers`
    * `taskcluster/docker/linux/Dockerfile`.
        * `ENV ANDROID_BUILD_TOOLS "34.0.0"`
        * `ENV ANDROID_SDK_VERSION "11076708"`
        * `ENV ANDROID_PLATFORM_VERSION "34"`
        * `ENV ANDROID_NDK_VERSION "25.2.9519653"`
