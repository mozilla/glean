# Android SDK / NDK versions

The Glean SDK implementation is currently build against the following versions:

* SDK API 33
    * Look for `android-33` in the SDK manager
    * or install with: `sdkmanager --verbose "platforms;android-33"`
* Android Command line tools
    * Download link: <https://dl.google.com/android/repository/commandlinetools-linux-9477386_latest.zip>
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
        * `sdkmanager 'build-tools;33.0.2'`
        * `image: circleci/android:2023.02.1`
    * `taskcluster/docker/linux/Dockerfile`.
        * `ENV ANDROID_BUILD_TOOLS "33.0.2"`
        * `ENV ANDROID_SDK_VERSION "9477386"`
        * `ENV ANDROID_PLATFORM_VERSION "33"`
        * `ENV ANDROID_NDK_VERSION "25.2.9519653"`
