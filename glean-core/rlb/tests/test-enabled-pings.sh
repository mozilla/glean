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

tmp="${TMPDIR:-/tmp}"
datapath=$(mktemp -d "${tmp}/glean_enabled_pings.XXXX")

cargo run -p glean --example enabled-pings -- $datapath

count=$(ls -1q "$datapath/sent_pings" | wc -l)
if [[ "$count" -ne 1 ]]; then
  echo "test result: FAILED."
  exit 101
fi

if ! grep -q "invalid-test-host/submit/glean-enabled-pings/one/" "$datapath/sent_pings"/*; then
  echo "test result: FAILED."
  exit 101
fi

echo "test result: ok."
exit 0
