#!/usr/bin/env bash

set -euvx

set -o pipefail && \
xcodebuild \
  -workspace ./glean-core/ios/Glean.xcodeproj/project.xcworkspace \
  -scheme Glean \
  -sdk iphonesimulator \
  -destination 'platform=iOS Simulator,name=iPhone 8' \
  test | \
tee raw_xcodetest.log | \
xcpretty && exit "${PIPESTATUS[0]}"
