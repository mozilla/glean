#!/usr/bin/env bash
#
# This is a small wrapper for running `cargo` inside of an XCode build,
# which unfortunately doesn't seem to work quite right out-of-the-box.
set -eEuvx

# Xcode tries to be helpful and overwrites the PATH. Reset that.
export PATH="${HOME}/.cargo/bin:$PATH"
export LIBRARY_PATH="${DEVELOPER_SDK_DIR}/MacOSX.sdk/usr/lib:${LIBRARY_PATH:-}"

"${@}"
