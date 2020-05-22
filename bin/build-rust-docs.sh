#!/bin/bash
#
# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.

# Build all docs with one command
# Documentation will be placed in `build/docs`.

set -xe

CRATE_NAME=glean_core

# Add the changelog file
cp -a CHANGELOG.md docs/appendix/changelog.md

output=$(mdbook build docs/ 2>&1)
if echo "$output" | grep -q "\[ERROR\]" ; then
    exit 1
fi

cargo doc --no-deps

rm -rf build/docs
mkdir -p build/docs
echo '<meta http-equiv=refresh content=0;url=book/index.html>' > build/docs/index.html

mkdir -p build/docs/book
cp -a docs/book/. build/docs/book

mkdir -p build/docs/docs
cp -a target/doc/. build/docs/docs
printf '<meta http-equiv=refresh content=0;url=%s/index.html>\n' "$CRATE_NAME" > build/docs/docs/index.html
