#!/usr/bin/env bash
set -euvx

# This should be invoked from inside xcode, not manually
if [ "$#" -ne 4 ]
then
    echo "Usage (note: only call inside xcode!):"
    echo "Args: $*"
    echo "path/to/build-scripts/xc-universal-binary.sh <STATIC_LIB_NAME> <FFI_TARGET> <GLEAN_ROOT_PATH> <buildvariant>"
    exit 1
fi
# e.g. libglean_ffi.a
STATIC_LIB_NAME=$1
# what to pass to cargo build -p, e.g. glean_ffi
FFI_TARGET=$2
# path to app services root
GLEAN_ROOT=$3
# buildvariant from our xcconfigs
BUILDVARIANT=$4

RELFLAG=
RELDIR="debug"
if [[ "$BUILDVARIANT" != "debug" ]]; then
    RELFLAG=--release
    RELDIR=release
fi

TARGETDIR=$GLEAN_ROOT/target

# We can't use cargo lipo because we can't link to universal libraries :(
# https://github.com/rust-lang/rust/issues/55235
IOS_TRIPLES=("x86_64-apple-ios" "aarch64-apple-ios")
for i in "${!IOS_TRIPLES[@]}"; do
    env -i PATH="$PATH" \
    $HOME/.cargo/bin/cargo build -p $FFI_TARGET --lib $RELFLAG --target ${IOS_TRIPLES[$i]}
done

UNIVERSAL_BINARY=$TARGETDIR/universal/$RELDIR/$STATIC_LIB_NAME
NEED_LIPO=

# if the universal binary doesnt exist, or if it's older than the static libs,
# we need to run `lipo` again.
if [[ ! -f "$UNIVERSAL_BINARY" ]]; then
    NEED_LIPO=1
elif [[ $(stat -f "%m" $TARGETDIR/x86_64-apple-ios/$RELDIR/$STATIC_LIB_NAME) -gt $(stat -f "%m" $UNIVERSAL_BINARY) ]]; then
    NEED_LIPO=1
elif [[ $(stat -f "%m" $TARGETDIR/aarch64-apple-ios/$RELDIR/$STATIC_LIB_NAME) -gt $(stat -f "%m" $UNIVERSAL_BINARY) ]]; then
    NEED_LIPO=1
fi
if [[ "$NEED_LIPO" = "1" ]]; then
    mkdir -p $TARGETDIR/universal/$RELDIR
    lipo -create -output "$UNIVERSAL_BINARY" \
        $TARGETDIR/x86_64-apple-ios/$RELDIR/$STATIC_LIB_NAME \
        $TARGETDIR/aarch64-apple-ios/$RELDIR/$STATIC_LIB_NAME
fi
