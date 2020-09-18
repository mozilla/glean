#!/usr/bin/env bash
#
# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.

# Update the glean_parser version

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
    echo "Update the glean_parser version"
    exit 1
fi

NEW_VERSION="$1"

# Update the version in glean-core/ios/sdk_generator.sh
FILE=glean-core/ios/sdk_generator.sh
run $SED -i.bak -E \
    -e "s/^GLEAN_PARSER_VERSION=[0-9.]+/GLEAN_PARSER_VERSION=${NEW_VERSION}/" \
    "${WORKSPACE_ROOT}/${FILE}"
run rm "${WORKSPACE_ROOT}/${FILE}.bak"

# Update the version in glean-core/python/setup.py
FILE=glean-core/python/setup.py
run $SED -i.bak -E \
    -e "s/\"glean_parser==[0-9.]+\"/\"glean_parser==${NEW_VERSION}\"/" \
    "${WORKSPACE_ROOT}/${FILE}"
run rm "${WORKSPACE_ROOT}/${FILE}.bak"

# Update the version in glean-core/python/glean/__init__.py
FILE=glean-core/python/glean/__init__.py
run $SED -i.bak -E \
    -e "s/^GLEAN_PARSER_VERSION = \"[0-9.]+\"/GLEAN_PARSER_VERSION = \"${NEW_VERSION}\"/" \
    "${WORKSPACE_ROOT}/${FILE}"
run rm "${WORKSPACE_ROOT}/${FILE}.bak"

# update the version in gradle-plugin/src/main/groovy/mozilla/telemetry/glean-gradle-plugin/GleanGradlePlugin.groovy
FILE=gradle-plugin/src/main/groovy/mozilla/telemetry/glean-gradle-plugin/GleanGradlePlugin.groovy
run $SED -i.bak -E \
    -e "s/GLEAN_PARSER_VERSION = \"[0-9.]+\"/GLEAN_PARSER_VERSION = \"${NEW_VERSION}\"/" \
    "${WORKSPACE_ROOT}/${FILE}"
run rm "${WORKSPACE_ROOT}/${FILE}.bak"

# update the version in glean-core/csharp/Glean/GleanParser.cs
FILE=glean-core/csharp/Glean/GleanParser.cs
run $SED -i.bak -E \
    -e "s/GleanParserVersion = \"[0-9.]+\"/GleanParserVersion = \"${NEW_VERSION}\"/" \
    "${WORKSPACE_ROOT}/${FILE}"
run rm "${WORKSPACE_ROOT}/${FILE}.bak"

# update the version in glean-core/Cargo.toml
FILE=glean-core/Cargo.toml
run $SED -i.bak -E \
    -e "s/glean-parser = \"[0-9.]+\"/glean-parser = \"${NEW_VERSION}\"/" \
    "${WORKSPACE_ROOT}/${FILE}"
run rm "${WORKSPACE_ROOT}/${FILE}.bak"

# update the version in the Makefile
FILE=Makefile
run $SED -i.bak -E \
    -e "s/glean_parser==[0-9.]+/glean_parser==${NEW_VERSION}/" \
    "${WORKSPACE_ROOT}/${FILE}"
run rm "${WORKSPACE_ROOT}/${FILE}.bak"
