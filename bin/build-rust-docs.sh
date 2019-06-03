#!/bin/bash

# Build all docs with one command
# Documentation will be placed in `build/docs`.

set -e

CRATE_NAME=glean_core

pushd docs &&
    mdbook build &&
    popd

cargo doc --no-deps

rm -rf build/docs
mkdir -p build/docs
echo '<meta http-equiv=refresh content=0;url=book/index.html>' > build/docs/index.html

mkdir -p build/docs/book
cp -a docs/book/. build/docs/book

mkdir -p build/docs/docs
cp -a target/doc/. build/docs/docs
printf '<meta http-equiv=refresh content=0;url=%s/index.html>\n' "$CRATE_NAME" > build/docs/docs/index.html
