#!/usr/bin/env bash
#
# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.

set -euvx

set -o pipefail && \
xcodebuild \
  -workspace ./glean-core/ios/Glean.xcodeproj/project.xcworkspace \
  -scheme Glean \
  -sdk iphonesimulator \
  -destination 'platform=iOS Simulator,name=iPhone 11' \
  test | \
tee raw_xcodetest.log | \
xcpretty && exit "${PIPESTATUS[0]}"
