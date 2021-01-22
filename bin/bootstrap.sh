#!/bin/sh

# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/. */

WORKSPACE_ROOT="$( cd "$(dirname "$0")/.." ; pwd -P )"
XCODE_XCCONFIG_FILE="$WORKSPACE_ROOT/xcconfig/xcode-12-fix-carthage-lipo.xcconfig"
export XCODE_XCCONFIG_FILE

carthage bootstrap --platform iOS --color auto --cache-builds
