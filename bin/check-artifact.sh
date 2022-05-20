#!/usr/bin/env bash
#
# A simple check that our Maven release files contain the shared libraries as expected.
# If this fails then something has gone wrong with the build process,
# such as miscompilation or failure to package the libraries correctly.

set -eu

if [[ "$#" -ne 2 ]]
then
    echo "Usage:"
    echo "./bin/check-artifact.sh <buildDir> <artifactId>"
    exit 1
fi

BUILD_DIR="$1"
ARTIFACT_ID="$2"

REQUIRED_FILES_AAR=(
    jni/arm64-v8a/libxul.so
    jni/armeabi-v7a/libxul.so
    jni/x86/libxul.so
    jni/x86_64/libxul.so
)
REQUIRED_FILES_TEST=(
    darwin-aarch64/libxul.dylib
    darwin-x86-64/libxul.dylib
    linux-x86-64/libxul.so
    win32-x86-64/xul.dll
)

check_files() {
    local artifact
    local files
    local content
    local missing

    artifact="$1"
    echo "Artifact: $artifact"

    if [[ -z "$artifact" ]]; then
        echo "No artifact found. Skipping."
        return
    fi

    shift
    files=("$@")
    content="$(unzip -l "$artifact")"
    missing=0

    for file in "${files[@]}"; do
        if printf "%s" "$content" | grep -q "$file"; then
            true
        else
            echo "File missing in '${artifact}': ${file}" >&2
            missing=1
        fi
    done

    if [ "$missing" -eq 1 ]; then
        echo "Files missing. Abort." >&2
        exit 1
    fi
}


case "$ARTIFACT_ID" in
    glean-native)
        ARTIFACT="$(find "${BUILD_DIR}" -path "*/${ARTIFACT_ID}/*" -name "*.aar")"
        check_files "$ARTIFACT" "${REQUIRED_FILES_AAR[@]}"
        ;;
    glean-native-forUnitTests)
        ARTIFACT="$(find "${BUILD_DIR}" -path "*/${ARTIFACT_ID}/*" -name "*.jar")"
        check_files "$ARTIFACT" "${REQUIRED_FILES_TEST[@]}"
        ;;
    *)
        echo "Unknown Artifact ID"
        ;;
esac
