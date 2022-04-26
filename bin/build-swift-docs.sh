#!/bin/bash
#
# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.

set -eo pipefail

# Build Swift with one command
# Requires jazzy from https://github.com/realm/jazzy

WORKSPACE_ROOT="$( cd "$(dirname "$0")/.." ; pwd -P )"
cd "$WORKSPACE_ROOT"

jazzy --version
jazzy \
  --clean \
  --config "$WORKSPACE_ROOT/.circleci/jazzy.yml" \
  --output "$WORKSPACE_ROOT/build/docs/swift"
