# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.

# We use this specific version because our decision task also runs on this one.
# We also use that same version in decisionlib.py
FROM ubuntu:bionic-20180821

LABEL maintainer="aplacitelli@mozilla.com"

# Add worker user for TaskCluster.

RUN mkdir /builds && \
    useradd -d /builds/worker -s /bin/bash -m worker && \
    chown worker:worker /builds/worker && \
    mkdir /builds/worker/artifacts && \
    chown worker:worker /builds/worker/artifacts

WORKDIR /builds/worker/

# Configuration
ENV DOTNET_CLI_TELEMETRY_OPTOUT true

# Set up the language variables to avoid problems (we run locale-gen later).
ENV LANG en_US.UTF-8
ENV LANGUAGE en_US:en
ENV LC_ALL en_US.UTF-8

# Do not use fancy output on taskcluster
ENV TERM dumb

# Used to detect in scripts whether we are running on taskcluster
ENV CI_TASKCLUSTER true

ENV \
    # Some APT packages like 'tzdata' wait for user input on install by default.
    # https://stackoverflow.com/questions/44331836/apt-get-install-tzdata-noninteractive
    DEBIAN_FRONTEND=noninteractive

# System.
RUN apt-get update -qq \
    && apt-get install -qy --no-install-recommends \
        # Required for the C# dependencies installation
        apt-transport-https \
        # Required for the C# dependencies installation
        wget \
        # To fetch Glean.
        git \
        # To fetch tooltool et al.
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
        # <TODO: Explain why we have this dependency>.
        clang \
        python3 \
        python3-pip \
        # taskcluster > mohawk > setuptools.
        python3-setuptools \
        # Required by tooltool to extract tar.xz archives.
        xz-utils \
        # Required to build libs/.
        make \
        # Required to build sqlcipher.
        tclsh \
        # Required in libs/ by some scripts patching the source they download.
        patch \
        # For windows cross-compilation.
        mingw-w64 \
    && apt-get clean

RUN pip3 install --upgrade pip
RUN pip3 install \
    'taskcluster>=4,<5' \
    pyyaml

# Install the C# dependencies.
RUN \
    wget https://packages.microsoft.com/config/ubuntu/18.04/packages-microsoft-prod.deb -O packages-microsoft-prod.deb \
    dpkg -i packages-microsoft-prod.deb

RUN apt-get update -qq \
    && apt-get install -qy --no-install-recommends \
    dotnet-sdk-3.1 \
    && apt-get clean

# Compile the UTF-8 english locale files (required by Python).
RUN locale-gen en_US.UTF-8

# sccache
RUN \
    curl -sfSL --retry 5 --retry-delay 10 \
        https://github.com/mozilla/sccache/releases/download/0.2.13/sccache-0.2.13-x86_64-unknown-linux-musl.tar.gz \
        | tar -xz --strip-components=1 -C /usr/local/bin/ \
            sccache-0.2.13-x86_64-unknown-linux-musl/sccache

# tooltool
RUN \
    curl -sfSL --retry 10 --retry-delay 10 \
         -o /usr/local/bin/tooltool.py \
         https://raw.githubusercontent.com/mozilla/build-tooltool/36511dae0ead6848017e2d569b1f6f1b36984d40/tooltool.py && \
         chmod +x /usr/local/bin/tooltool.py


# %include-run-task

ENV SHELL=/bin/bash \
    HOME=/builds/worker \
    PATH=/builds/worker/.local/bin:$PATH


VOLUME /builds/worker/checkouts
VOLUME /builds/worker/.cache


# run-task expects to run as root
USER root
