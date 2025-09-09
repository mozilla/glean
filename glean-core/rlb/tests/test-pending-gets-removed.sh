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

set -e

tmp="${TMPDIR:-/tmp}"
datapath=$(mktemp -d "${tmp}/pending-gets-removed.XXXX")

# Build it once
cargo build -p glean --example pending-gets-removed

cmd="cargo run -q -p glean --example pending-gets-removed -- $datapath"

$cmd 1
count=$(find "$datapath/pending_pings" -name "*-*" -exec grep -e "submit.*\(validation\|nofollows\)" {} ';' | wc -l)
if [[ "$count" -ne 2 ]]; then
  echo "1: test result: FAILED. Expected: 2; Actual: $count"
  exit 101
fi

$cmd 2
count=$(find "$datapath/pending_pings" -name "*-*" -exec grep -e "submit.*\(validation\|nofollows\)" {} ';' | wc -l)
if [[ "$count" -ne 1 ]]; then
  echo "2: test result: FAILED. Expected: 1; Actual: $count"
  exit 101
fi

if ! grep -q "/submit/glean-pending-removed/nofollows/" "$datapath/pending_pings"/*; then
  echo "3: test result: FAILED."
  exit 101
fi

$cmd 3
count=$(find "$datapath/pending_pings" -name "*-*" -exec grep -e "submit.*\(validation\|nofollows\)" {} ';' | wc -l)
if [[ "$count" -ne 0 ]]; then
  echo "4: test result: FAILED. Expected: 0; Actual: $count"
  exit 101
fi

echo "test result: ok."
exit 0
