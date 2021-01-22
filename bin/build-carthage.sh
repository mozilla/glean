#!/usr/bin/env bash
#
# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.

set -euvx

FRAMEWORK_NAME="${1:-Glean}"
CONFIGURATION="${2:-Release}"

WORKSPACE_ROOT="$( cd "$(dirname "$0")/.." ; pwd -P )"
XCODE_XCCONFIG_FILE="$WORKSPACE_ROOT/xcconfig/xcode-12-fix-carthage-lipo.xcconfig"
export XCODE_XCCONFIG_FILE

set -o pipefail && \
    carthage build --archive --platform iOS --cache-builds --verbose --configuration "${CONFIGURATION}" "${FRAMEWORK_NAME}" | \
    xcpretty

# Add dependency information
zip -u "${FRAMEWORK_NAME}.framework.zip" DEPENDENCIES.md
