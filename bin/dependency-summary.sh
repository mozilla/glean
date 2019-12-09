#!/bin/bash

# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.

# This script can be used to generate a summary of our third-party dependencies,
# including license details. Use it like this:
#
#    bin/dependency-summary.sh

set -eo pipefail

WORKSPACE_ROOT="$( cd "$(dirname "$0")/.." ; pwd -P )"

MD_TEMPLATE="${WORKSPACE_ROOT}/bin/about.md.hbs"
XML_TEMPLATE="${WORKSPACE_ROOT}/bin/about.xml.hbs"

MD_OUTPUT="${WORKSPACE_ROOT}/DEPENDENCIES.md"
XML_OUTPUT="${WORKSPACE_ROOT}/glean-core/android/dependency-licenses.xml"

command -v cargo-about >/dev/null || cargo install cargo-about
cargo about generate "${MD_TEMPLATE}" > "${MD_OUTPUT}"
cargo about generate "${XML_TEMPLATE}" > "${XML_OUTPUT}"
