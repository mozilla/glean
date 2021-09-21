#!/bin/bash

# Run the Android build in Docker.
# This should run exactly the same as the Taskcluster build, using a local Docker setup.
# It only runs the Android Arm build & test.
# The Docker image is rebuild everytime (Docker is smart enough to skip cached steps though).
# Note: the Docker image is ~15 GB.

WORKSPACE_ROOT="$( cd "$(dirname "$0")/../../.." ; pwd -P )"

# Build docker image to use
pushd "${WORKSPACE_ROOT}/taskcluster/docker/linux"
docker build -t gleanlinux .

# Run the Android build
pushd "${WORKSPACE_ROOT}"
docker run --rm -v "$(pwd):/glean" gleanlinux /glean/taskcluster/docker/linux/runbuild.sh
