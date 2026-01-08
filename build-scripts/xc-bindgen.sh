#!/bin/bash

# what to pass to cargo build -p, e.g. glean_ffi
FFI_TARGET=$1
# path to Glean root
GLEAN_ROOT=$2
# buildvariant from our xcconfigs
BUILDVARIANT=$3

RELMODE=debug
if [[ "$BUILDVARIANT" != "debug" ]]; then
    RELMODE=release
fi

set -euvx

IS_SIMULATOR=0
if [ "${LLVM_TARGET_TRIPLE_SUFFIX-}" = "-simulator" ]; then
  IS_SIMULATOR=1
fi

TARGET=

for arch in $ARCHS; do
  case "$arch" in
    x86_64)
      # Intel iOS simulator
      TARGET="x86_64-apple-ios"
      break
      ;;

    arm64)
      if [ $IS_SIMULATOR -eq 0 ]; then
        # Hardware iOS targets
        TARGET=aarch64-apple-ios
      else
        # M1 iOS simulator
        TARGET=aarch64-apple-ios-sim
      fi
      break
      ;;
  esac
done

if [[ -z "$TARGET" ]]; then
  echo "Missing TARGET. No suitable arch in ${ARCHS}"
  exit 2
fi

LIBRARY_PATH="${GLEAN_ROOT}/target/${TARGET}/${RELMODE}/lib${FFI_TARGET}.a"

bash "${GLEAN_ROOT}/build-scripts/xc-cargo.sh" cargo run --package uniffi-bindgen -- \
  generate \
  --language swift \
  --config "${GLEAN_ROOT}/glean-core/bundle/uniffi.toml" \
  --out-dir "${GLEAN_ROOT}/glean-core/ios/Glean/Generated/uniffi" \
  --library "$LIBRARY_PATH" \
  --no-format
