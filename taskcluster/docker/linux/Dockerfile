# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.

# We use this specific version because our decision task also runs on this one.
# We also use that same version in decisionlib.py
FROM ubuntu:22.04

MAINTAINER Jan-Erik Rediger "janerik@mozilla.com"

RUN echo "trigger rebuild"

# Add worker user

RUN mkdir /builds && \
    useradd -d /builds/worker -s /bin/bash -m worker && \
    chown worker:worker /builds/worker && \
    mkdir /builds/worker/artifacts && \
    chown worker:worker /builds/worker/artifacts

WORKDIR /builds/worker/

# Configuration

ENV ANDROID_BUILD_TOOLS "36.0.0"
ENV ANDROID_TOOLS_VERSION "13114758"
ENV ANDROID_PLATFORM_VERSION "36"
ENV ANDROID_NDK_VERSION "28.2.13676358"

# Set up the language variables to avoid problems (we run locale-gen later).
ENV LANG en_US.UTF-8
ENV LANGUAGE en_US:en
ENV LC_ALL en_US.UTF-8

# Do not use fancy output on taskcluster
ENV TERM dumb

ENV GRADLE_OPTS -Xmx4096m -Dorg.gradle.daemon=false

# Used to detect in scripts whether we are running on taskcluster
ENV CI_TASKCLUSTER true

ENV \
    # Some APT packages like 'tzdata' wait for user input on install by default.
    # https://stackoverflow.com/questions/44331836/apt-get-install-tzdata-noninteractive
    DEBIAN_FRONTEND=noninteractive

# System.

RUN apt-get update -qq \
    && apt-get install -qy --no-install-recommends \
        # To compile Android stuff.
        openjdk-17-jdk \
        git \
        curl \
        # Required by symbolstore.py.
        file \
        # Will set up the timezone to UTC (?).
        tzdata \
        # To install UTF-8 locales.
        locales \
        # For `cc` crates; see https://github.com/jwilm/alacritty/issues/1440.
        # <TODO: Is this still true?>.
        g++ \
        libxml2-dev \
        python3 \
        python3-pip \
        python3-venv \
        # taskcluster > mohawk > setuptools.
        python3-setuptools \
        # Required to extract the Android SDK/NDK.
        unzip \
        # Required to extract tar.bz2 archives.
        bzip2 \
        # Required by tooltool to extract tar.xz archives.
        xz-utils \
        # Required to unpack compiler
        zstd \
        # Required to build libs/.
        make \
        # For windows cross-compilation.
        mingw-w64 \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

RUN pip3 install --upgrade pip
RUN pip3 install \
    pyyaml \
    'taskcluster'

# Compile the UTF-8 english locale files (required by Python).
RUN locale-gen en_US.UTF-8

# Android SDK
RUN mkdir -p /builds/worker/android-sdk
WORKDIR /builds/worker

ENV ANDROID_HOME /builds/worker/android-sdk
ENV ANDROID_SDK_HOME /builds/worker/android-sdk
ENV PATH ${PATH}:${ANDROID_SDK_HOME}/cmdline-tools/latest/bin:${ANDROID_SDK_HOME}/platform-tools:/opt/tools:${ANDROID_SDK_HOME}/build-tools/${ANDROID_BUILD_TOOLS}

# Download the Android SDK tools, unzip them to ${ANDROID_SDK_HOME}/cmdline-tools/latest/, accept all licenses
# The download link comes from https://developer.android.com/studio/#downloads
RUN curl -sfSL --retry 5 --retry-delay 10 https://dl.google.com/android/repository/commandlinetools-linux-${ANDROID_TOOLS_VERSION}_latest.zip > sdk.zip \
    && unzip -q sdk.zip \
    && mkdir $ANDROID_SDK_HOME/cmdline-tools \
    && mv cmdline-tools $ANDROID_HOME/cmdline-tools/latest \
    && rm sdk.zip \
    && mkdir -p /builds/worker/android-sdk/.android/ \
    && touch /builds/worker/android-sdk/.android/repositories.cfg \
    && yes | sdkmanager --licenses \
    && sdkmanager --verbose "platform-tools" \
        "platforms;android-${ANDROID_PLATFORM_VERSION}" \
        "build-tools;${ANDROID_BUILD_TOOLS}" \
        "extras;android;m2repository" \
        "extras;google;m2repository" \
        "ndk;${ANDROID_NDK_VERSION}"

RUN chown -R worker:worker /builds/worker/android-sdk

# tooltool
RUN \
    curl -sfSL --retry 5 --retry-delay 10 \
         -o /usr/local/bin/tooltool.py \
         https://raw.githubusercontent.com/mozilla-releng/tooltool/master/client/tooltool.py && \
         chmod +x /usr/local/bin/tooltool.py

# %include-run-task

ENV SHELL=/bin/bash \
    HOME=/builds/worker \
    PATH=/builds/worker/.local/bin:$PATH

VOLUME /builds/worker/checkouts
VOLUME /builds/worker/.cache

# run-task expects to run as root
USER root
