#!/bin/bash

# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.

# Glean SDK metrics build script.
#
# More about Glean at https://mozilla.github.io/glean
#
# This script generates metrics and pings as defined in user-provided files
# and generates Swift code to be included in the final build.
# It uses the `glean_parser`.
# See https://mozilla.github.io/glean_parser/ for details.
#
# To use it in a Swift project, follow these steps:
# 1. Import the `sdk_generator.sh` script into your project.
# 2. Add your `metrics.yaml` and (optionally) `pings.yaml` to your project.
# 3. Add a new "Run Script" build step and set the command to `bash $PWD/sdk_generator.sh`
# 4. Add your definition files (`metrics.yaml`, `pings.yaml`) as Input Files for the "Run Script" step.
# 5. Run the build.
# 6. Add the files in the `Generated` folder to your project.
# 7. Add the same files from the `Generated` folder as Output Files of the newly created "Run SCript" step.
# 8. Start using the generated metrics.

set -e

GLEAN_PARSER_VERSION=1.9.5

# When the special argument "internal" is passed, don't add a namespace import and also allow all reserved items.
NAMESPACE=Glean
INTERNAL=""
if [ "$1" = "internal" ]; then
  NAMESPACE=""
  INTERNAL="--allow-reserved"
fi

VENVDIR="${SOURCE_ROOT}/.venv"

[ -x "${VENVDIR}/bin/python" ] || python3 -m venv "${VENVDIR}"
${VENVDIR}/bin/pip install --upgrade glean_parser==$GLEAN_PARSER_VERSION
${VENVDIR}/bin/python -m glean_parser \
    translate \
    -f "swift" \
    -o "${SOURCE_ROOT}/${PROJECT}/Generated" \
    -s glean_namespace=$NAMESPACE \
    $INTERNAL \
    "${SOURCE_ROOT}/metrics.yaml" \
    "${SOURCE_ROOT}/pings.yaml"
