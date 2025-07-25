#!/bin/bash

# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.

# Update the Xcode version used to build Glean iOS bindings.
#
# Usage: update-xcode-version.sh <Xcode version>
#
# Environment:
#
# VERB    - Log commands that are run when set.

set -eo pipefail

run() {
  [ "${VERB:-0}" != 0 ] && echo "+ $*"
  "$@"
}

# All sed commands below work with either
# GNU sed (standard on Linux distrubtions) or BSD sed (standard on macOS)
SED="sed"

WORKSPACE_ROOT="$( cd "$(dirname "$0")/.." ; pwd -P )"

if [ -z "$1" ]; then
    echo "Usage: $(basename "$0") <new version>"
    echo
    echo "Update the Xcode version used to build Glean iOS bindings"
    exit 1
fi

NEW_VERSION="$1"
NEW_VERSION_MAJOR_MINOR="$(echo "$NEW_VERSION" | awk -F'.' '{print $1"."$2}')"

# Update the version in .circleci/config.yml
FILE=.circleci/config.yml
run $SED -i.bak -E \
    -e "s/xcode: \"[0-9.]+\"/xcode: \"${NEW_VERSION_MAJOR_MINOR}\"/" \
    "${WORKSPACE_ROOT}/${FILE}"
run rm "${WORKSPACE_ROOT}/${FILE}.bak"

# Update the version in docs/dev/ios/setup-ios-build-environment.md
FILE=docs/dev/ios/setup-ios-build-environment.md
run $SED -i.bak -E \
    -e "s/Install Xcode [0-9.]+/Install Xcode ${NEW_VERSION_MAJOR_MINOR}/" \
    "${WORKSPACE_ROOT}/${FILE}"
run rm "${WORKSPACE_ROOT}/${FILE}.bak"

# Update the version in samples/ios/app/README.md
FILE=samples/ios/app/README.md
run $SED -i.bak -E \
    -e "s/Install Xcode [0-9.]+/Install Xcode ${NEW_VERSION_MAJOR_MINOR}/" \
    "${WORKSPACE_ROOT}/${FILE}"
run rm "${WORKSPACE_ROOT}/${FILE}.bak"

# Reminder to add a changelog entry
run echo "Updated Xcode to version ${NEW_VERSION_MAJOR_MINOR}"
run echo "Please add the following to the CHANGELOG.md under the unrelased section for iOS:"
echo
run echo "* Glean for iOS is now being built with Xcode ${NEW_VERSION} ([<pull request #>](<pull request URL>))'"
