#!/bin/bash

TMP="${TMPDIR:-/tmp}"

# Remove the temporary data path on all exit conditions
cleanup() {
  if [[ -n "$datapath" ]] && [[ -d "$datapath" ]]; then
    rm -r "$datapath"
  fi
}
trap cleanup INT ABRT TERM EXIT

maxn=${1:-100}
shift
seed=${2:-100}
shift

function runrapid() {
  extraargs="$*"
  datapath=$(mktemp -d "${TMP}/glean_rapid_metrics.XXXXXXXXXX")
  logout=$(mktemp)

  fsnotifywatch -r "${datapath}" > "$logout" 2>&1 &
  watchpid=$!

  echo target/release/rapid-metrics --maxn "$maxn" --seed "$seed" "$datapath" $extraargs
  target/release/rapid-metrics --maxn "$maxn" --seed "$seed" "$datapath" $extraargs

  kill $watchpid
  wait $watchpid

  grep -E "total|${datapath}/db" "$logout"
  echo
  rm -r "$datapath" "$logout"
}

cargo build -p rapid-metrics --release

echo "===== Full write mode (Rkv) ====="
runrapid

echo "===== delay-ping-lifetime mode (Rkv) ====="
runrapid --delay

cargo build -p rapid-metrics --release --features sqlite

echo "===== Full write mode (SQLite) ====="
runrapid

echo "===== delay-ping-lifetime mode (SQLite) ====="
runrapid --delay
