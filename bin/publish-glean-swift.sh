#!/bin/bash

# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.

set -xe

# Strip any `v` prefix from `vX.Y.Z`
NEW_VERSION=$(echo "$1" | sed 's/^v//')

git clone https://github.com/mozilla/glean-swift glean-swift
cd glean-swift
bin/update.sh "$NEW_VERSION"
git push -q https://${GLEAN_SWIFT_GITHUB_TOKEN}@github.com/mozilla/glean-swift.git main
git push -q https://${GLEAN_SWIFT_GITHUB_TOKEN}@github.com/mozilla/glean-swift.git "$NEW_VERSION"
