#!/bin/bash

set -eux

export PATH=$PATH:/builds/worker/clang/bin

# x86_64 Darwin
export ORG_GRADLE_PROJECT_RUST_ANDROID_GRADLE_TARGET_X86_64_APPLE_DARWIN_CC=/builds/worker/clang/bin/clang
export ORG_GRADLE_PROJECT_RUST_ANDROID_GRADLE_TARGET_X86_64_APPLE_DARWIN_TOOLCHAIN_PREFIX=/builds/worker/cctools/bin
export ORG_GRADLE_PROJECT_RUST_ANDROID_GRADLE_TARGET_X86_64_APPLE_DARWIN_AR=/builds/worker/cctools/bin/x86_64-apple-darwin-ar
export ORG_GRADLE_PROJECT_RUST_ANDROID_GRADLE_TARGET_X86_64_APPLE_DARWIN_RANLIB=/builds/worker/cctools/bin/x86_64-apple-darwin-ranlib
export ORG_GRADLE_PROJECT_RUST_ANDROID_GRADLE_TARGET_X86_64_APPLE_DARWIN_LD_LIBRARY_PATH=/builds/worker/clang/lib
export ORG_GRADLE_PROJECT_RUST_ANDROID_GRADLE_TARGET_X86_64_APPLE_DARWIN_RUSTFLAGS="-C linker=/builds/worker/clang/bin/clang -C link-arg=-fuse-ld=/builds/worker/cctools/bin/x86_64-apple-darwin-ld -C link-arg=-B -C link-arg=/builds/worker/cctools/bin -C link-arg=-target -C link-arg=x86_64-apple-darwin -C link-arg=-isysroot -C link-arg=/tmp/MacOSX11.0.sdk -C link-arg=-Wl,-syslibroot,/tmp/MacOSX11.0.sdk -C link-arg=-Wl,-dead_strip"
# For ring's use of `cc`.
export ORG_GRADLE_PROJECT_RUST_ANDROID_GRADLE_TARGET_X86_64_APPLE_DARWIN_CFLAGS_x86_64_apple_darwin="-B /builds/worker/cctools/bin -target x86_64-apple-darwin -isysroot /tmp/MacOSX11.0.sdk -Wl,-syslibroot,/tmp/MacOSX11.0.sdk -Wl,-dead_strip"
# aarch64 Darwin (M1/Silicon)
export ORG_GRADLE_PROJECT_RUST_ANDROID_GRADLE_TARGET_AARCH64_APPLE_DARWIN_CC=/builds/worker/clang/bin/clang
export ORG_GRADLE_PROJECT_RUST_ANDROID_GRADLE_TARGET_AARCH64_APPLE_DARWIN_TOOLCHAIN_PREFIX=/builds/worker/cctools/bin
export ORG_GRADLE_PROJECT_RUST_ANDROID_GRADLE_TARGET_AARCH64_APPLE_DARWIN_AR=/builds/worker/cctools/bin/aarch64-apple-darwin-ar
export ORG_GRADLE_PROJECT_RUST_ANDROID_GRADLE_TARGET_AARCH64_APPLE_DARWIN_RANLIB=/builds/worker/cctools/bin/aarch64-apple-darwin-ranlib
export ORG_GRADLE_PROJECT_RUST_ANDROID_GRADLE_TARGET_AARCH64_APPLE_DARWIN_LD_LIBRARY_PATH=/builds/worker/clang/lib
export ORG_GRADLE_PROJECT_RUST_ANDROID_GRADLE_TARGET_AARCH64_APPLE_DARWIN_RUSTFLAGS="-C linker=/builds/worker/clang/bin/clang -C link-arg=-fuse-ld=/builds/worker/cctools/bin/aarch64-apple-darwin-ld -C link-arg=-B -C link-arg=/builds/worker/cctools/bin -C link-arg=-target -C link-arg=aarch64-apple-darwin -C link-arg=-isysroot -C link-arg=/tmp/MacOSX11.0.sdk -C link-arg=-Wl,-syslibroot,/tmp/MacOSX11.0.sdk -C link-arg=-Wl,-dead_strip -C link-arg=-Wl,-no_encryption"
export ORG_GRADLE_PROJECT_RUST_ANDROID_GRADLE_TARGET_AARCH64_APPLE_DARWIN_CFLAGS_aarch64_apple_darwin="-B /builds/worker/cctools/bin -target aarch64-apple-darwin -isysroot /tmp/MacOSX11.0.sdk -Wl,-syslibroot,/tmp/MacOSX11.0.sdk -Wl,-dead_strip"

# x86_64 Windows
# The wrong linker gets used otherwise: https://github.com/rust-lang/rust/issues/33465.
export ORG_GRADLE_PROJECT_RUST_ANDROID_GRADLE_TARGET_X86_64_PC_WINDOWS_GNU_RUSTFLAGS="-C linker=x86_64-w64-mingw32-gcc"

# Ensure we're compiling dependencies in non-debug mode.
# This is required for rkv/lmdb to work correctly on Android targets and not link to unavailable symbols.
export TARGET_CFLAGS="-DNDEBUG"

# Install clang, a port of cctools, and the macOS SDK into /tmp.
# If it weren't for the size we could do it in the Dockerfile directly to cache it.
#
# To update:
# * Go to https://firefox-ci-tc.services.mozilla.com/tasks/index/gecko.cache.level-3.toolchains.v3
# * Find the tasks for `clang-dist-toolchain` and `linux64-cctools-port`
# * Per task, follow the link to the latest indexed task
# * In the detail view, click "View Task"
# * In the task view, click "See more"
# * Find the "Routes" list
# * Pick the "index.*.hash.*" route
# * Use that in the URLs below
#   (drop the "index." prefix, ensure the "public/build" path matches the artifacts of the TC task)
pushd /builds/worker
curl -sfSL --retry 5 --retry-delay 10 \
    https://firefox-ci-tc.services.mozilla.com/api/index/v1/task/gecko.cache.level-3.toolchains.v3.linux64-cctools-port.pushdate.2024.07.23.20240723071212/artifacts/public%2Fbuild%2Fcctools.tar.zst > cctools.tar.zst
tar -I zstd -xf cctools.tar.zst
rm cctools.tar.zst
curl -sfSL --retry 5 --retry-delay 10 \
    https://firefox-ci-tc.services.mozilla.com/api/index/v1/task/gecko.cache.level-3.toolchains.v3.clang-dist-toolchain.pushdate.2024.07.30.20240730145721/artifacts/public%2Fbuild%2Fclang-dist-toolchain.tar.xz > clang-dist-toolchain.tar.xz
tar -xf clang-dist-toolchain.tar.xz
mv builds/worker/toolchains/clang clang
rm clang-dist-toolchain.tar.xz

# Fixup symlink
rm /builds/worker/clang/bin/clang
ln -s /builds/worker/clang/bin/clang-18 /builds/worker/clang/bin/clang

popd

pushd /tmp || exit

tooltool.py \
  --url=http://taskcluster/tooltool.mozilla-releng.net/ \
  --manifest="/builds/worker/checkouts/vcs/taskcluster/scripts/macos.manifest" \
  fetch
# tooltool doesn't know how to unpack zstd-files,
# so we do it manually.
tar -I zstd -xf "MacOSX11.0.sdk.tar.zst"

popd || exit

rustup target add x86_64-apple-darwin
rustup target add aarch64-apple-darwin
rustup target add x86_64-pc-windows-gnu

# Verify paths after extraction and permission changes
echo "Verifying paths after extraction"
ls -la /builds/worker/clang/bin
file /builds/worker/clang/bin/clang
file /builds/worker/clang/bin/clang-18

set +eu
