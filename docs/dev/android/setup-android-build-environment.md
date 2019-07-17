# Setup the Android Build Environment

> **Important:** This documentation refers to setting up the development environment for the [cross-platform implementation of the Glean SDK](https://github.com/mozilla/glean).  Instructions for setting up the implementation in android-components is documented in the [android-components repository](https://github.com/mozilla-mobile/android-components).

## Doing a local build of the Android Components:

This document describes how to make local builds of the Android bindings in
this repository. Most consumers of these bindings *do not* need to follow
this process, but will instead use pre-built bindings [todo: link to this]

## Prepare your build environment

Typically, this process only needs to be run once, although periodically you
may need to repeat some steps (eg, rust updates should be done periodically)

### Setting up Android dependencies

At the end of this process you should have the following environment variables set up.

- `ANDROID_HOME`
- `NDK_HOME`
- `JAVA_HOME`

The easiest way to install all the dependencies (and automatically
handle updates), is by using [Android Studio](https://developer.android.com/studio/index.html).
Once this is installed, it must be run and the Glean project opened to complete initial setup.
If Android Studio asks you to upgrade the verison of gradle, decline.

The following dependencies can be installed in Android Studio through `Tools > SDK Manager > SDK Tools`:

- Android SDK Tools (may already be selected)
- NDK
- CMake
- LLDB

With the dependencies installed, note down the `Android SDK Location` in `Tools > SDK Manager`.
Set the `ANDROID_HOME` environment variable to that path.
The `NDK_HOME` can be set to `NDK_HOME=$ANDROID_HOME/ndk-bundle`.
Set `JAVA_HOME` to be the location of Android Studio's JDK which can be found in Glean's "Project Structure" menu. (You may need to escape spaces in the path).

### Setting up Rust

Rust can be installed using `rustup`, with the following commands:

- `curl https://sh.rustup.rs -sSf | sh`
- `rustup update`

Platform specific toolchains need to be installed for Rust. This can be
done using the `rustup target` command. In order to enable building to real
devices and Android emulators, the following targets need to be installed:

- `rustup target add aarch64-linux-android`
- `rustup target add armv7-linux-androideabi`
- `rustup target add i686-linux-android`
- `rustup target add x86_64-linux-android`

The `mdbook` crate is required in order to build documentation:

- `cargo install mdbook`

## Building

This should be relatively straightforward and painless:

1. Ensure your repository is up-to-date.

2. Ensure rust is up-to-date by running `rustup`

3. The builds are all performed by `./gradlew` and the general syntax used is
   `./gradlew project:task`

   You can see a list of projects by executing `./gradlew projects` and a list
   of tasks by executing `./gradlew tasks`.

The above can be skipped if using `Android Studio`. The project directory can be imported
and all the build details can be left to the IDE.

## FAQ

- **Q: Android Studio complains about Python not being found when building.**
- A: Make sure that the `python` binary is on your `PATH`. On Windows, in addition to that,
it might be required to add its full path to the `rust.pythonCommand` entry in  `$project_root/local.properties`.
