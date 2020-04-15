#!/bin/bash
#
# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.

set -eo pipefail

# Build Swift with one command
# Requires jazzy from https://github.com/realm/jazzy

jazzy --version
jazzy \
    --clean \
    --output build/docs/swift \
    --sdk iphone \
    --module Glean \
    --xcodebuild-arguments -workspace,./glean-core/ios/Glean.xcodeproj/project.xcworkspace,-scheme,Glean,-destination,"generic/platform=iOS" \
    --author_url https://mozilla.github.com/glean \
    --github_url https://github.com/mozilla/glean \
    --readme README.iOS.md
