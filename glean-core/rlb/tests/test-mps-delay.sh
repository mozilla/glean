#!/bin/bash

# Test harness for testing the RLB processes from the outside.
#
# Some behavior can only be observed when properly exiting the process running Glean,
# e.g. when an uploader runs in another thread.
# On exit the threads will be killed, regardless of their state.

# Remove the temporary data path on all exit conditions
cleanup() {
  if [ -n "$datapath" ]; then
    rm -r "$datapath"
  fi
}
trap cleanup INT ABRT TERM EXIT

WORKSPACE_ROOT="$( cd "$(dirname "$0")/../../.." ; pwd -P )"
cd "$WORKSPACE_ROOT"

tmp="${TMPDIR:-/tmp}"
datapath=$(mktemp -d "${tmp}/glean_mps_delay.XXXX")
# Build it once
cargo build -p glean --example mps-delay || exit 1

cmd="target/debug/examples/mps-delay $datapath"

# Set a timezone (Los Angeles = -07:00)
export TZ=America/Los_Angeles
export RUST_LOG=debug

timeout 5s faketime --exclude-monotonic -f "2025-07-27 04:05:00" $cmd init
count=$(find "$datapath/sent_pings" -name "*.json" -exec grep -e "url.*metrics" {} ';' | wc -l)
if [[ "$count" -ne 0 ]]; then
  echo "1/3 test result: FAILED. Expected 0, got $count pings"
  exit 101
fi

timeout 5s faketime --exclude-monotonic -f "2025-07-28 22:27:00" $cmd second
count=$(find "$datapath/sent_pings" -name "*.json" -exec grep -e "url.*metrics" {} ';' | wc -l)
if [[ "$count" -ne 1 ]]; then
  echo "2/3 test result: FAILED. Expected 1, got $count pings"
  exit 101
fi

timeout 5s faketime --exclude-monotonic -f "2025-07-28 22:30:00" $cmd third
count=$(find "$datapath/sent_pings" -name "*.json" -exec grep -e "url.*metrics" {} ';' | wc -l)
if [[ "$count" -ne 1 ]]; then
  echo "3/3 test result: FAILED. Expected 1, got $count pings"
  exit 101
fi

echo "test result: PASSED."
exit 0
