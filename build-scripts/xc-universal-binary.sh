#!/usr/bin/env bash

# This should be invoked from inside xcode, not manually
if [ "$#" -ne 2 ]
then
    echo "Usage (note: only call inside xcode!):"
    echo "Args: $*"
    echo "path/to/build-scripts/xc-universal-binary.sh <FFI_TARGET> <GLEAN_ROOT_PATH>"
    exit 1
fi

# what to pass to cargo build -p, e.g. glean_ffi
FFI_TARGET=$1
# path to app services root
GLEAN_ROOT=$2

if [ -d "$HOME/.cargo/bin" ]; then
  export PATH="$HOME/.cargo/bin:$PATH"
fi

if ! command -v cargo-lipo 2>/dev/null >/dev/null;
then
    echo "$(basename $0) failed."
    echo "Requires cargo-lipo to build universal library."
    echo "Install it with:"
    echo
    echo "   cargo install cargo-lipo"
    exit 1
fi

# Ease testing of this script by assuming something about the environment.
if [ -z "$ACTION" ]; then
  export ACTION=build
fi

# Always build both architectures on x86_64.
export ARCHS="arm64 x86_64"

set -euvx

if [[ -n "${DEVELOPER_SDK_DIR:-}" ]]; then
  # Assume we're in Xcode, which means we're probably cross-compiling.
  # In this case, we need to add an extra library search path for build scripts and proc-macros,
  # which run on the host instead of the target.
  # (macOS Big Sur does not have linkable libraries in /usr/lib/.)
  export LIBRARY_PATH="${DEVELOPER_SDK_DIR}/MacOSX.sdk/usr/lib:${LIBRARY_PATH:-}"
fi

# Force correct target for dependencies compiled with `cc`.
# Required for M1 MacBooks (Arm target).
# Without this some dependencies might be compiled for the wrong target.
export CFLAGS_x86_64_apple_ios="-target x86_64-apple-ios"

cargo lipo --xcode-integ --manifest-path "$GLEAN_ROOT/Cargo.toml" --package "$FFI_TARGET"
