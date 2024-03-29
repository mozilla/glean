#!/usr/bin/env bash
#
# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.

set -euvx

set -o pipefail && \
xcodebuild \
  -workspace ./samples/ios/app/glean-sample-app.xcodeproj/project.xcworkspace \
  -scheme glean-sample-app \
  -sdk iphonesimulator \
  -destination 'platform=iOS Simulator,name=iPhone 15' \
  build | \
tee raw_sample_xcodebuild.log | \
xcpretty && exit "${PIPESTATUS[0]}"
