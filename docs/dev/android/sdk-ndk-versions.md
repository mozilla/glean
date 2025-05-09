# Android SDK / NDK versions

The Glean SDK implementation requires the following Android SDK/NDK tooling:

* SDK API 35
    * Look for `android-35` in the SDK manager
    * or install with: `sdkmanager --verbose "platforms;android-35"`
* Android Command line tools
    * Download link: <https://dl.google.com/android/repository/commandlinetools-linux-12700392_latest.zip>
* NDK r28b
    * Download link: <https://dl.google.com/android/repository/android-ndk-r28b-linux.zip>

For the full setup see [Setup the Android Build Environment](setup-android-build-environment.html).

The versions are defined in the following files.
All locations need to be updated on upgrades:

* Documentation
    * this file (`docs/dev/core/internal/sdk-ndk-versions.md`)
    * `dev/android/setup-android-build-environment.md`
* CI configuration
    * `.circleci/config.yml`
        * `sdkmanager 'build-tools;35.0.0'`
        * `image: circleci/android:2025.04.1-browsers`
    * `taskcluster/docker/linux/Dockerfile`.
        * `ENV ANDROID_BUILD_TOOLS "35.0.0"`
        * `ENV ANDROID_SDK_VERSION "12700392"`
        * `ENV ANDROID_PLATFORM_VERSION "35"`
        * `ENV ANDROID_NDK_VERSION "28.1.13356709"`
