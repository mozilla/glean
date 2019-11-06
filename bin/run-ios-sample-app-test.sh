#!/usr/bin/env bash

set -euvx

set -o pipefail && \
xcodebuild \
  -workspace ./samples/ios/app/glean-sample-app.xcodeproj/project.xcworkspace \
  -scheme glean-sample-appUITests \
  -sdk iphonesimulator \
  -destination 'platform=iOS Simulator,name=iPhone 8' \
  test | \
tee raw_sample_xcodetest.log | \
xcpretty && exit "${PIPESTATUS[0]}"
