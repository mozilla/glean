# Android SDK / NDK versions

The Glean SDK implementation is currently build against the following versions:

* SDK API 28
    * Look for `android-28` in the SDK manager
    * or install with: `sdkmanager --verbose "platforms;android-28"`
* Android build tools 28.0.3
    * Download link: <https://dl.google.com/android/repository/sdk-tools-linux-3859397.zip>
* NDK r21
    * Download link: <https://dl.google.com/android/repository/android-ndk-r21-linux-x86_64.zip>

For the full setup see [Setup the Android Build Environment](setup-android-build-environment.html).

The versions are defined in the following files.
All locations need to be updated on upgrades:

* Documentation
    * this file (`docs/dev/core/internal/sdk-ndk-versions.md`)
    * `dev/android/setup-android-build-environment.md`
* CI configuration
    * `.circleci/config.yml`
        * `sdkmanager 'build-tools;28.0.3'`
        * `image: circleci/android:api-28-ndk`
    * `taskcluster/docker/linux/Dockerfile`.
        * `ENV ANDROID_BUILD_TOOLS "28.0.3"`
        * `ENV ANDROID_SDK_VERSION "3859397"`
        * `ENV ANDROID_PLATFORM_VERSION "28"`
        * `ENV ANDROID_NDK_VERSION "r21"`
