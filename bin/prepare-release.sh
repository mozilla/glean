#!/usr/bin/env bash
#
# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.

# Prepare a new release by updating the version numbers in all related files,
# updating the changelog to include the released version.
#
# Optionally, it can create the release commit and tag it.
#
# Usage: prepare-release.sh <new version>
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

SED=sed
if command -v gsed >/dev/null; then
    SED=gsed
fi

WORKSPACE_ROOT="$( cd "$(dirname "$0")/.." ; pwd -P )"

if [ -z "$1" ]; then
    echo "Usage: $(basename $0) <new version>"
    echo
    echo "Prepare for a new release by setting the version number"
    exit 1
fi

NEW_VERSION="$1"
DATE=$(date +%Y-%m-%d)

if ! echo "$NEW_VERSION" | grep --quiet --extended-regexp '^[0-9]+\.[0-9]+\.[0-9]+(-[a-z0-9.-]+)?'; then
    echo "error: Specified version '${NEW_VERSION}' doesn't match the Semantic Versioning pattern."
    echo "error: Use MAJOR.MINOR.PATCH versioning."
    echo "error: See https://semver.org/"
    exit 1
fi

echo "Preparing update to v${NEW_VERSION} (${DATE})"
echo "Workspace root: ${WORKSPACE_ROOT}"
echo

GIT_STATUS_OUTPUT=$(git status --untracked-files=no --porcelain)
if [ -z "$ALLOW_DIRTY" ] && [ -n "${GIT_STATUS_OUTPUT}" ]; then
    lines=$(echo "$GIT_STATUS_OUTPUT" | wc -l | tr -d '[:space:]')
    echo "error: ${lines} files in the working directory contain changes that were not yet committed into git:"
    echo
    echo "${GIT_STATUS_OUTPUT}"
    echo
    echo 'To proceed despite this and include the uncommited changes, set the `ALLOW_DIRTY` environment variable.'
    exit 1

fi

DOIT=y
if [[ -n "$DRY_RUN" ]]; then
    echo "Dry-run. Not modifying files."
    DOIT=n
fi

### GLEAN-CORE ###

# Update the glean-core version

FILE=glean-core/Cargo.toml
run $SED -i.bak -E \
    -e "s/^version = \"[0-9a-z.-]+\"/version = \"${NEW_VERSION}\"/" \
    "${WORKSPACE_ROOT}/${FILE}"
run rm "${WORKSPACE_ROOT}/${FILE}.bak"

### GLEAN-FFI ###

# Update the glean-ffi version, and its glean-core dependency

FILE=glean-core/ffi/Cargo.toml
run $SED -i.bak -E \
    -e "s/^version = \"[0-9a-z.-]+\"/version = \"${NEW_VERSION}\"/" \
    -e "/glean-core/!b;n;n;s/version = \"[0-9a-z.-]+\"/version = \"${NEW_VERSION}\"/" \
    "${WORKSPACE_ROOT}/${FILE}"
run rm "${WORKSPACE_ROOT}/${FILE}.bak"

### GLEAN-PREVIEW ###

# Update the version of the glean-core dependency

FILE=glean-core/preview/Cargo.toml
run $SED -i.bak -E \
    -e "/glean-core/!b;n;n;s/version = \"[0-9a-z.-]+\"/version = \"${NEW_VERSION}\"/" \
    "${WORKSPACE_ROOT}/${FILE}"
run rm "${WORKSPACE_ROOT}/${FILE}.bak"

### Update Cargo.lock

cargo update -p glean-core -p glean-ffi

### KOTLIN PACKAGES ###

FILE=.buildconfig.yml
run $SED -i.bak -E \
    -e "s/libraryVersion: [0-9A-Z.-]+/libraryVersion: ${NEW_VERSION}/" \
    "${WORKSPACE_ROOT}/${FILE}"
run rm "${WORKSPACE_ROOT}/${FILE}.bak"

### GLEAN GRADLE PLUGIN ###

FILE=gradle-plugin/src/main/groovy/mozilla/telemetry/glean-gradle-plugin/GleanGradlePlugin.groovy
run $SED -i.bak -E \
    -e "s/project.ext.glean_version = \"[0-9A-Z.-]+\"/project.ext.glean_version = \"${NEW_VERSION}\"/" \
    "${WORKSPACE_ROOT}/${FILE}"
run rm "${WORKSPACE_ROOT}/${FILE}.bak"

### CHANGELOG ###

FILE=CHANGELOG.md
run $SED -i.bak -E \
    -e "s/# Unreleased changes/# v${NEW_VERSION} (${DATE})/" \
    -e "s/\.\.\.master/...v${NEW_VERSION}/" \
    "${WORKSPACE_ROOT}/${FILE}"
run rm "${WORKSPACE_ROOT}/${FILE}.bak"

if [ "$DOIT" = y ]; then
    CHANGELOG=$(cat "${WORKSPACE_ROOT}/${FILE}")
    cat > "${WORKSPACE_ROOT}/${FILE}" <<EOL
# Unreleased changes

[Full changelog](https://github.com/mozilla/glean/compare/v${NEW_VERSION}...master)

${CHANGELOG}
EOL
fi

echo "Everything prepared for v${NEW_VERSION}"
echo
echo "Changed files:"
git status --untracked-files=no --porcelain || true
echo
echo "Create release commit v${NEW_VERSION} now? [y/N]"
read -r RESP
echo
if [ "$RESP" != "y" ] && [ "$RESP" != "Y" ]; then
    echo "No new commit. No new tag. Proceed manually."
    exit 0
fi

run git add --update "${WORKSPACE_ROOT}"
run git commit --message "Release version ${NEW_VERSION}"
