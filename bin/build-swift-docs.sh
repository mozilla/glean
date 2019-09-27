#!/bin/bash

set -eo pipefail

# Build Swift with one command
# Requires jazzy from https://github.com/realm/jazzy

jazzy --version
jazzy \
    --output build/docs/swift \
    --sdk iphone \
    --module Glean \
    --xcodebuild-arguments -workspace,./glean-core/ios/Glean.xcodeproj/project.xcworkspace,-scheme,Glean \
    --author_url https://mozilla.github.com/glean \
    --github_url https://github.com/mozilla/glean
