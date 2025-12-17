#!/usr/bin/env bash
#
# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.

# Run a benchmark comparison between a previous commit and the current one.

set -eo pipefail

run() {
    [ "${VERB:-0}" != 0 ] && echo "+ $*"
    "$@"
}

WORKSPACE_ROOT="$( cd "$(dirname "$0")/.." ; pwd -P )"

if [ -z "$1" ]; then
    echo "Usage: $(basename "$0") <start commit> [end commit]"
    echo
    echo "Run a benchmark on both the <start commit> and [end commit] for comparison."
    echo "[end commit] defaults to HEAD"
    exit 1
fi

START_COMMIT="$1"
START_COMMIT=$(git rev-parse "$START_COMMIT")
END_COMMIT="${2:-HEAD}"
END_COMMIT=$(git rev-parse "$END_COMMIT")

BASELINE_NAME="bn-${END_COMMIT}"

stash_commit=$(git stash create)

run git checkout "$START_COMMIT"
run cargo benchmark --bench dispatcher -- --save-baseline "$BASELINE_NAME"

run git checkout "$END_COMMIT"
run cargo benchmark --bench dispatcher -- --baseline "$BASELINE_NAME"

if [[ -n "$stash_commit" ]]; then
  git stash apply "$stash_commit"
fi

run echo "You can rerun the benchmark against the baseline using:"
run echo "cargo benchmark --bench dispatcher -- --baseline $BASELINE_NAME"
