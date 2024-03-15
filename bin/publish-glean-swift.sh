#!/bin/bash

# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.

set -xe

# Strip any `v` prefix from `vX.Y.Z`
NEW_VERSION=$(echo "$1" | sed 's/^v//')

# Get the supported iOS platform version
# See https://developer.apple.com/documentation/packagedescription/supportedplatform/iosversion for valid iOS versions
WORKSPACE_ROOT="$( cd "$(dirname "$0")/.." ; pwd -P )"
FILE=glean-core/ios/Glean.xcodeproj/project.pbxproj
IOS_PLATFORM_VERSION=v$(grep -m 1 \
    -Po "(?<=IPHONEOS_DEPLOYMENT_TARGET \= )[0-9]." \
    "${WORKSPACE_ROOT}/${FILE}")

git clone https://github.com/mozilla/glean-swift glean-swift
cd glean-swift
bin/update.sh "$NEW_VERSION" --ios-version "$IOS_PLATFORM_VERSION"
git push -q https://${GLEAN_SWIFT_GITHUB_TOKEN}@github.com/mozilla/glean-swift.git main
git push -q https://${GLEAN_SWIFT_GITHUB_TOKEN}@github.com/mozilla/glean-swift.git "$NEW_VERSION"
