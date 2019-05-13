<!-- MarkdownTOC autolink="true" -->

- [Setup the Android Build Environment](#setup-the-android-build-environment)
  - [Doing a local build of the Android Components:](#doing-a-local-build-of-the-android-components)
  - [Prepare your build environment](#prepare-your-build-environment)
    - [Setting up Android dependencies](#setting-up-android-dependencies)
    - [Setting up Rust](#setting-up-rust)
  - [Building](#building)
    - [Publishing the components to your local maven repository](#publishing-the-components-to-your-local-maven-repository)
    - [Other build types](#other-build-types)
- [Using Windows](#using-windows)
  - [Setting up the build environment](#setting-up-the-build-environment)
  - [Configure Maven](#configure-maven)
  - [FAQ](#faq)

<!-- /MarkdownTOC -->


# Setup the Android Build Environment

## Doing a local build of the Android Components:

This document describes how to make local builds of the Android components in
this repository. Most consumers of these components *do not* need to follow
this process, but will instead use pre-built components [todo: link to this]

## Prepare your build environment

Typically, this process only needs to be run once, although periodically you
may need to repeat some steps (eg, rust updates should be done periodically)

### Setting up Android dependencies

At the end of this process you should have the following environment variables set up.

- `ANDROID_HOME`
- `NDK_HOME`

The easiest way to install all the dependencies (and automatically
handle updates), is by using [Android Studio](https://developer.android.com/studio/index.html).
Once this is installed, the following dependencies can be installed through it:

- Android SDK Tools
- NDK
- CMake
- LLDB

To install a dependency, open Android Studio and select `Tools > SDK Manager > SDK Tools`. Then tick the boxes corrensponding to the dependencies listed above.

With the dependencies installed, note down the `Android SDK Location` in `Tools > SDK Manager`. Set the `ANDROID_HOME` environment variable to that path. The `NDK_HOME` can be set to `NDK_HOME=$ANDROID_HOME/ndk-bundle`.

### Setting up Rust

Rust can be installed using `rustup`, with the following commands:

- `curl https://sh.rustup.rs -sSf | sh`
- `rustup update`

Platform specific toolchains need to be installed in Rust. This can be
done using `rustup target` command. In order to enable building to real
devices and Android emulators, the following targets need to be installed:

- `rustup target add aarch64-linux-android`
- `rustup target add armv7-linux-androideabi`
- `rustup target add i686-linux-android`
- `rustup target add x86_64-linux-android`

## Building

This should be relatively straightforward and painless:

1. Ensure your clone repo is up-to-date.

2. Ensure rust is up-to-date by running `rustup`

3. The builds are all performed by `./gradlew` and the general syntax used is
   `./gradlew project:task`

   You can see a list of projects by executing `./gradlew projects` and a list
   of tasks by executing `./gradlew tasks`.

The above can be skipped if using `Android Studio`, as the project directory can be imported
and all the build details can be left to the IDE.

### Publishing the components to your local maven repository

The easiest way to use the build is to have your Android project reference the component from your local maven repository - this is done by the `publishToMavenLocal` task - so:

    ./gradlew publishToMavenLocal

should work. Check your `~/.m2` directory (which is your local maven repo) for the components.

You can also publish single projects - eg:

    ./gradlew service-sync-places:publishToMavenLocal

For more information about using the local maven repo, see this [android components guide](https://mozilla-mobile.github.io/android-components/contributing/testing-components-inside-app)

### Other build types

If you just want the build artifacts, you probably want one of the `assemble` tasks - either
   `assembleDebug` or `assembleRelease`.

For example, to build a debug version of the places library, the command you
want is `./gradlew places-library:assembleDebug`

After building, you should find the built artifact under the `target` directory,
with sub-directories for each Android architecture. For example, after executing:

    % ./gradlew places-library:assembleDebug

you will find:

    target/aarch64-linux-android/release/libplaces_ffi.so
    target/x86_64-linux-android/release/libplaces_ffi.so
    target/i686-linux-android/release/libplaces_ffi.so
    target/armv7-linux-androideabi/release/libplaces_ffi.so

(You will probably notice that even though as used `assembleDebug`, the directory names are `release` - this may change in the future)

You should also find the .kt files for the project somewhere there and in the right directory structure if that turns out to be useful.

# Using Windows

It's currently tricky to get some of these builds working on Windows, primarily due to our use of `sqlcipher` and `openssl`. However, by using the Windows Subsystem for Linux, it is possible to get builds working, but still have them published to your "native" local maven cache so it's available for use by a "native" Android Studio.

As above, this document may be incomplete, so please edit or open PRs where necessary.

In general, you will follow the exact same process outlined above, with one or 2 unique twists.

## Setting up the build environment

You need to install most of the build tools in WSL. This means you end up with many tools installed twice - once in WSL and once in "native" Windows - but the only cost of that is disk-space.

You will need the following tools in WSL:

* unzip - `sudo apt install unzip`

* python - `sudo apt install python`

* java - you may already have it? try `java -version`. Java ended up causing me grief (stuck at 100% CPU doing nothing), but google pointed at one popular way of installing java:

    ```
    sudo add-apt-repository ppa:webupd8team/java
    sudo apt-get update
    sudo apt-get install oracle-java8-installer
    sudo apt install oracle-java8-set-default
    ```

* tcl, used for sqlcipher builds - `sudo apt install tcl-dev`

* Android SDKs - this process is much the same as for normal Linux - roughly

  * visit https://developer.android.com/studio/, at the bottom of the page locate the current SDKs for linux
at time of writing, this is https://dl.google.com/android/repository/sdk-tools-linux-4333796.zip

    ```
    cd ~
    mkdir android-sdk
    cd android-sdk
    unzip {path-to.zip}
    export ANDROID_HOME=$HOME/android-sdk
    $ANDROID_HOME/tools/bin/sdkmanager "platforms;android-26"
    $ANDROID_HOME/tools/bin/sdkmanager --licenses
    ```

(Note - it may be necessary to execute `$ANDROID_HOME/tools/bin/sdkmanager "build-tools;26.0.2" "platform-tools" "platforms;android-26" "tools"`, but may not! See also [this gist](https://gist.github.com/fdmnio/fd42caec2e5a7e93e12943376373b7d0) which google found for me and might have useful info.

* Follow all the other steps above - eg, you still need the NDK setup in WSL and all environment variables above set.

## Configure Maven

We now want to configure maven to use the native windows maven repository - then, when doing `./gradlew install` from WSL, it ends up in the Windows maven repo.

* Execute `sudo apt install maven` - this should have created a `~/.m2` folder as the WSL maven repository. In this directory, create a file `~/.m2/settings.xml` with the content:

    ```
    <settings>
      <localRepository>/mnt/c/Users/{username}/.m2/repository</localRepository>
    </settings>
    ```

  (obviously with {username} adjusted appropriately)

* Now you should be ready to roll - `./gradlew install` should complete and publish the components to your native maven repo!

## FAQ

- **Q: Android Studio complains about Python not being found when building.**
- A: Make sure that the `python` binary is on your `PATH`. On Windows, in addition to that, 
it might be required to add its full path to the `rust.pythonCommand` entry in  `$project_root/local.properties`.
