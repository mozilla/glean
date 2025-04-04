#!/bin/bash

# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.

# This script can be used to generate a summary of our third-party dependencies,
# including license details. Use it like this:
#
#    bin/dependency-summary.sh

set -eo pipefail

CARGO_ABOUT_MIN_VERSION="0.7.0"

WORKSPACE_ROOT="$( cd "$(dirname "$0")/.." ; pwd -P )"

MD_TEMPLATE="${WORKSPACE_ROOT}/bin/about.md.hbs"
XML_TEMPLATE="${WORKSPACE_ROOT}/bin/about.xml.hbs"

MD_OUTPUT="${WORKSPACE_ROOT}/DEPENDENCIES.md"
XML_OUTPUT="${WORKSPACE_ROOT}/glean-core/android/dependency-licenses.xml"
XML_OUTPUT_NATIVE="${WORKSPACE_ROOT}/glean-core/android-native/dependency-licenses.xml"

verlte() {
  [ "$1" = "$(printf "$1\n$2\n" | sort --version-sort | head -n1)" ]
}

command -v cargo-about >/dev/null || cargo install cargo-about
installed_version=$(cargo-about --version | awk '{print $2}')
if ! verlte "$CARGO_ABOUT_MIN_VERSION" "$installed_version"; then
  echo "WARN: Found cargo-about v${installed_version}, require at least ${CARGO_ABOUT_MIN_VERSION}" >&2
  echo "WARN: Please update cargo-about: cargo install cargo-about"
fi

cargo about generate "${MD_TEMPLATE}" > "${MD_OUTPUT}"
cargo about generate "${XML_TEMPLATE}" > "${XML_OUTPUT}"
cp "${XML_OUTPUT}" "${XML_OUTPUT_NATIVE}"
