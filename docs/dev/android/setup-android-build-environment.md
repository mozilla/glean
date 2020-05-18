# Setup the Android Build Environment

## Doing a local build of the Glean SDK:

This document describes how to make local builds of the Android bindings in this repository.
Most consumers of these bindings *do not* need to follow this process,
but will instead [use pre-built bindings](../../user/adding-glean-to-your-project.html).

## Prepare your build environment

Typically, this process only needs to be run once, although periodically you
may need to repeat some steps (e.g., Rust updates should be done periodically).

### Setting up Android dependencies

#### With Android Studio

The easiest way to install all the dependencies (and automatically
handle updates), is by using [Android Studio](https://developer.android.com/studio/index.html).
Once this is installed, start Android Studio and open the Glean project.
If Android Studio asks you to upgrade the version of Gradle, decline.

The following dependencies can be installed in Android Studio through `Tools > SDK Manager > SDK Tools`:

- Android SDK Tools (may already be selected)
- NDK r21
- CMake
- LLDB

You should be set to build Glean from within Android Studio now.

#### Manually

Set `JAVA_HOME` to be the location of Android Studio's JDK which can be found in Android Studio's "Project Structure" menu (you may need to escape spaces in the path).

Note down the location of your SDK.
If installed through Android Studio you will find the `Android SDK Location` in `Tools > SDK Manager`.

Set the `ANDROID_HOME` environment variable to that path.
Alternatively add the following line to the `local.properties` file in the root of the Glean checkout (create the file if it does not exist):

```
sdk.dir=/path/to/sdk
```

For the Android NDK:

1. Download NDK r21 from <https://developer.android.com/ndk/downloads>.
2. Extract it and put it somewhere (`$HOME/.android-ndk-r21` is a reasonable choice, but it doesn't matter).
3. Add the following line to the `local.properties` file in the root of the Glean checkout (create the file if it does not exist):
   ```
   ndk.dir=/path/to/.android-ndk-r21
   ```

### Setting up Rust

Rust can be installed using `rustup`, with the following commands:

```
curl https://sh.rustup.rs -sSf | sh
rustup update
```

Platform specific toolchains need to be installed for Rust.
This can be done using the `rustup target` command.
In order to enable building for real devices and Android emulators,
the following targets need to be installed:

```
rustup target add aarch64-linux-android
rustup target add armv7-linux-androideabi
rustup target add i686-linux-android
rustup target add x86_64-linux-android
```

## Building

Before building:

* Ensure your repository is up-to-date.
* Ensure Rust is up-to-date by running `rustup update`.

### With Android Studio

After importing the Glean project into Android Studio it should be all set up and you can build the project using `Build > Make Project`

### Manually

The builds are all performed by `./gradlew` and the general syntax used is `./gradlew project:task`

Build the whole project and run tests:

```
./gradlew build
```

You can see a list of projects by executing `./gradlew projects` and a list of tasks by executing `./gradlew tasks`.

## FAQ

- **Q: Android Studio complains about Python not being found when building.**
- A: Make sure that the `python` binary is on your `PATH`. On Windows, in addition to that,
it might be required to add its full path to the `rust.pythonCommand` entry in  `$project_root/local.properties`.
