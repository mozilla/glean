#!/usr/bin/env bash
#
# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.

set -euvx

pushd samples/ios/GleanSampleFramework

set -o pipefail && \
    carthage build --archive --platform iOS --cache-builds --verbose --configuration "Debug" "GleanSampleFramework" | \
    xcpretty

popd

# Add dependency information
zip -u "GleanSampleFramework.framework.zip" DEPENDENCIES.md
