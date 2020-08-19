#!/bin/bash

# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.

export RUST_BACKTRACE='1'
export RUSTFLAGS='-Dwarnings'
export CARGO_INCREMENTAL='0'
export CI='1'
export CCACHE='sccache'
export RUSTC_WRAPPER='sccache'
export SCCACHE_IDLE_TIMEOUT='1200'
export SCCACHE_CACHE_SIZE='40G'
export SCCACHE_ERROR_LOG='/builds/worker/sccache.log'
export RUST_LOG='sccache=info,glean_core=debug,glean_ffi=debug'

# Rust
set -eux; \
    RUSTUP_PLATFORM='x86_64-unknown-linux-gnu'; \
    RUSTUP_VERSION='1.21.1'; \
    RUSTUP_SHA256='ad1f8b5199b3b9e231472ed7aa08d2e5d1d539198a15c5b1e53c746aad81d27b'; \
    curl -sfSL --retry 5 --retry-delay 10 -O "https://static.rust-lang.org/rustup/archive/${RUSTUP_VERSION}/${RUSTUP_PLATFORM}/rustup-init"; \
    echo "${RUSTUP_SHA256} *rustup-init" | sha256sum -c -; \
    chmod +x rustup-init; \
    ./rustup-init -y --no-modify-path --default-toolchain none; \
    rm rustup-init
export PATH=$HOME/.cargo/bin:$PATH

TOOLCHAIN="${1:-stable}"

# No argument -> default stable install
if [ "${TOOLCHAIN}" = "stable" ]; then
    echo "Installing Rust stable & Android targets"
    rustup toolchain install stable
    rustup default stable
    rustup target add x86_64-linux-android i686-linux-android armv7-linux-androideabi aarch64-linux-android
else
    echo "Installing Rust ${TOOLCHAIN}"
    rustup toolchain add "${TOOLCHAIN}" --profile minimal
    rustup default "${TOOLCHAIN}"
fi
