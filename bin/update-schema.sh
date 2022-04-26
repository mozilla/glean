#!/bin/bash

# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.

# Update the bundled schema version tests run against.
#
# The schema is pulled from mozilla-pipeline-schemas at the following URL:
#
#     https://raw.githubusercontent.com/mozilla-services/mozilla-pipeline-schemas/$HASH/schemas/glean/glean/glean.1.schema.json
#
# References in the code base are updated.
#
# Usage: update-schema.sh <commit hash>
#
# Environment:
#
# VERB    - Log commands that are run when set.

set -eo pipefail

run() {
  [ "${VERB:-0}" != 0 ] && echo "+ $*"
  "$@"
}

update() {
  COMMIT_HASH="$1"
  FULL_URL="$(printf "$SCHEMA_URL" "$COMMIT_HASH")"
  SCHEMA_PATH="${WORKSPACE_ROOT}/glean.1.schema.json"

  echo "Vendoring schema from ${FULL_URL}"
  run curl --silent --fail --show-error --location --retry 5 --retry-delay 10 "$FULL_URL" --output "$SCHEMA_PATH"
}

WORKSPACE_ROOT="$( cd "$(dirname "$0")/.." ; pwd -P )"
SCHEMA_URL="https://raw.githubusercontent.com/mozilla-services/mozilla-pipeline-schemas/%s/schemas/glean/glean/glean.1.schema.json"

if [ -z "$1" ]; then
    echo "Usage: $(basename $0) <commit hash or branch name>"
    echo
    echo "Update schema version to test"
    exit 1
fi

COMMIT_HASH="$1"
update "$COMMIT_HASH"
