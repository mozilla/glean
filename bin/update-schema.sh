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
# DRY_RUN - Do not modify files or run destructive commands when set.
# VERB    - Log commands that are run when set.

set -eo pipefail

run() {
  [ "${VERB:-0}" != 0 ] && echo "+ $*"
  if [ "$DOIT" = y ]; then
      "$@"
  else
      true
  fi
}

update() {
  COMMIT_HASH="$1"
  FULL_URL="$(printf "$SCHEMA_URL" "$COMMIT_HASH")"
  SCHEMA_PATH="${WORKSPACE_ROOT}/glean.1.schema.json"

  echo "Vendoring schema from ${FULL_URL}"
  run curl --silent --fail --show-error --location --retry 5 --retry-delay 10 "$FULL_URL" --output "$SCHEMA_PATH"
}

get_latest() {
  API_URL="https://api.github.com/repos/mozilla-services/mozilla-pipeline-schemas/commits?path=schemas%2Fglean%2Fglean%2Fglean.1.schema.json&page=1&per_page=1"
  SHA="$(curl --silent --fail --show-error --location --retry 5 --retry-delay 10 "$API_URL" | grep --max-count=1 sha)"

  echo "$SHA" | $SED -E -e 's/.+: "([^"]+)".*/\1/'
}

SED=sed
if command -v gsed >/dev/null; then
    SED=gsed
fi

DOIT=y
if [[ -n "$DRY_RUN" ]]; then
    echo "Dry-run. Not modifying files."
    DOIT=n
fi

WORKSPACE_ROOT="$( cd "$(dirname "$0")/.." ; pwd -P )"
SCHEMA_URL="https://raw.githubusercontent.com/mozilla-services/mozilla-pipeline-schemas/%s/schemas/glean/glean/glean.1.schema.json"

if [ -z "$1" ]; then
    echo "Usage: $(basename $0) <commit hash>"
    echo
    echo "Update schema version to test"
    exit 1
fi

COMMIT_HASH="$1"

if [ "$COMMIT_HASH" = "latest" ]; then
  COMMIT_HASH="$(get_latest)"
fi

update "$COMMIT_HASH"
