#!/bin/bash
#
# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.

# Build all docs with one command
# Documentation will be placed in `build/docs`.

# IMPORTANT: When changing this file make sure to update the
# `build-rust-docs.bat` file, too.

set -xe

CRATE_NAME=glean_core

# Add the changelog files
cp -a CHANGELOG.md docs/user/appendix/changelog/sdk.md
wget https://raw.githubusercontent.com/mozilla/glean.js/main/CHANGELOG.md -O docs/user/appendix/changelog/js.md

# Build the Glean client user book
output=$(mdbook build docs/user/ 2>&1)
if echo "$output" | grep -q "\[ERROR\]" ; then
    exit 1
fi

# Build the Glean SDK development book
output=$(mdbook build docs/dev/ 2>&1)
if echo "$output" | grep -q "\[ERROR\]" ; then
    exit 1
fi

cargo doc --no-deps

rm -rf build/docs
mkdir -p build/docs
echo '<meta http-equiv=refresh content=0;url=book/index.html>' > build/docs/index.html

# Add redirections for all pages moved on the restructuring done on Bug 1708204
# Android Specific Information
mkdir -p build/docs/book/user/android/
echo '<meta http-equiv=refresh content=0;url=../../language-bindings/android/android-build-configuration-options.html>' > build/docs/book/user/android/android-build-configuration-options.html
echo '<meta http-equiv=refresh content=0;url=../../language-bindings/android/android-offline-builds.html>' > build/docs/book/user/android/android-offline-builds.html
echo '<meta http-equiv=refresh content=0;url=../language-bindings/android/instrument-android-crashes-example.html>' > build/docs/book/user/instrument-android-crashes-example.html
echo '<meta http-equiv=refresh content=0;url=../../language-bindings/android/index.html>' > build/docs/book/user/android/index.html
# Adding Glean to your project
mkdir -p build/docs/book/user/adding-glean-to-your-project/
echo '<meta http-equiv=refresh content=0;url=./adding-glean-to-your-project/index.html>' > build/docs/book/user/adding-glean-to-your-project.html
# General API
echo '<meta http-equiv=refresh content=0;url=../reference/general/index.html>' > build/docs/book/user/general-api.html
echo '<meta http-equiv=refresh content=0;url=../reference/general/experiments-api.html>' > build/docs/book/user/experiments-api.html
# Metrics API
mkdir -p build/docs/book/user/metrics/
echo '<meta http-equiv=refresh content=0;url=../../reference/metrics/boolean.html>' > build/docs/book/user/metrics/boolean.html
echo '<meta http-equiv=refresh content=0;url=../../reference/metrics/counter.html>' > build/docs/book/user/metrics/counter.html
echo '<meta http-equiv=refresh content=0;url=../../reference/metrics/custom_distribution.html>' > build/docs/book/user/metrics/custom_distribution.html
echo '<meta http-equiv=refresh content=0;url=../../reference/metrics/datetime.html>' > build/docs/book/user/metrics/datetime.html
echo '<meta http-equiv=refresh content=0;url=../../reference/metrics/event.html>' > build/docs/book/user/metrics/event.html
echo '<meta http-equiv=refresh content=0;url=../../reference/metrics/index.html>' > build/docs/book/user/metrics/index.html
echo '<meta http-equiv=refresh content=0;url=../../reference/metrics/labeled_booleans.html>' > build/docs/book/user/metrics/labeled_booleans.html
echo '<meta http-equiv=refresh content=0;url=../../reference/metrics/labeled_counters.html>' > build/docs/book/user/metrics/labeled_counters.html
echo '<meta http-equiv=refresh content=0;url=../../reference/metrics/labeled_strings.html>' > build/docs/book/user/metrics/labeled_strings.html
echo '<meta http-equiv=refresh content=0;url=../../reference/metrics/memory_distribution.html>' > build/docs/book/user/metrics/memory_distribution.html
echo '<meta http-equiv=refresh content=0;url=../../reference/metrics/quantity.html>' > build/docs/book/user/metrics/quantity.html
echo '<meta http-equiv=refresh content=0;url=../../reference/metrics/rate.html>' > build/docs/book/user/metrics/rate.html
echo '<meta http-equiv=refresh content=0;url=../../reference/metrics/string.html>' > build/docs/book/user/metrics/string.html
echo '<meta http-equiv=refresh content=0;url=../../reference/metrics/string_list.html>' > build/docs/book/user/metrics/string_list.html
echo '<meta http-equiv=refresh content=0;url=../../reference/metrics/timespan.html>' > build/docs/book/user/metrics/timespan.html
echo '<meta http-equiv=refresh content=0;url=../../reference/metrics/timing_distribution.html>' > build/docs/book/user/metrics/timing_distribution.html
echo '<meta http-equiv=refresh content=0;url=../../reference/metrics/uuid.html>' > build/docs/book/user/metrics/uuid.html
# YAML Format
echo '<meta http-equiv=refresh content=0;url=../reference/yaml/index.html>' > build/docs/book/user/metric-parameters.html
# Metric User Guides
echo '<meta http-equiv=refresh content=0;url=metrics/adding-new-metrics.html>' > build/docs/book/user/adding-new-metrics.html
echo '<meta http-equiv=refresh content=0;url=metrics/error-reporting.html>' > build/docs/book/user/error-reporting.html
echo '<meta http-equiv=refresh content=0;url=metrics/testing-metrics.html>' > build/docs/book/user/testing-metrics.html

mkdir -p build/docs/book
cp -a docs/user/book/. build/docs/book

mkdir -p build/docs/dev
cp -a docs/dev/book/. build/docs/dev

mkdir -p build/docs/shared
cp -a docs/shared/. build/docs/shared

mkdir -p build/docs/docs
cp -a target/doc/. build/docs/docs
printf '<meta http-equiv=refresh content=0;url=%s/index.html>\n' "$CRATE_NAME" > build/docs/docs/index.html
