#!/bin/bash

set -ex

cd /glean

source taskcluster/scripts/rustup-setup.sh
echo "rust.targets=linux-x86-64\n" > local.properties

./gradlew clean assembleDebugUnitTest testDebugUnitTest
