#!/usr/bin/env bash

set -euvx

set -o pipefail && \
xcodebuild \
  -workspace ./samples/ios/app/glean-sample-app.xcodeproj/project.xcworkspace \
  -scheme glean-sample-app \
  -sdk iphonesimulator \
  -destination 'platform=iOS Simulator,name=iPhone 8' \
  build | \
tee raw_sample_xcodebuild.log | \
xcpretty && exit "${PIPESTATUS[0]}"
