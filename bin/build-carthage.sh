#!/usr/bin/env bash
#
# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.

set -euvx

FRAMEWORK_NAME="${1:-Glean}"
CONFIGURATION="${2:-Release}"

set -o pipefail && \
    carthage build --no-skip-current --platform iOS --cache-builds --verbose --configuration "${CONFIGURATION}" "${FRAMEWORK_NAME}" | \
    xcpretty

# Add dependency information
zip -r "${FRAMEWORK_NAME}.framework.zip" "Carthage/Build/iOS/Static" DEPENDENCIES.md
