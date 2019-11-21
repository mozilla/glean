#!/usr/bin/env bash

set -euvx

FRAMEWORK_NAME="${1:-Glean}"
CONFIGURATION="${2:-Release}"

set -o pipefail && \
    carthage build --archive --platform iOS --cache-builds --verbose --configuration "${CONFIGURATION}" "${FRAMEWORK_NAME}" | \
    xcpretty
