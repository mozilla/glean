#!/bin/bash

# Remove the temporary data path on all exit conditions
cleanup() {
  if [[ -n "$datapath" ]] && [[ -d "$datapath" ]]; then
    rm -r "$datapath"
  fi
}
trap cleanup INT ABRT TERM EXIT

TMP="${TMPDIR:-/tmp}"
RED='\033[0;31m'
GREEN='\033[0;32m'
NC='\033[0m'
WORKSPACE_ROOT="$( cd "$(dirname "$0")/../.." ; pwd -P )"
cd "$WORKSPACE_ROOT"

# This is a valid Rkv safe-mode file,
# with exactly one key-value pair: `key1=value1` in a store called `store`.
# This is NOT a valid Glean database, but that doesn't matter.
MINIMAL_DATA_SAFE_BIN="010000000000000001050000000000000073746f726500000000010000000000000004000000000000006b6579310f0000000000000007060000000000000076616c756531"
CORRUPT_DATA_SAFE_BIN="010000000000000001050000000000000073746f726500000000010000000000000004000000000000006b6579310f0000000000000007060000000000000076616c7565"

write_minbin() {
  out="$1"
  data="${2:-$MINIMAL_DATA_SAFE_BIN}"
  echo "$data" | xxd -r -p > $out
}

passed=0
failed=0
test() {
  msg="$1"
  res="$2"
  if [[ $res -eq 0 ]]; then
    passed=$(($passed+1))
    printf "    ${GREEN}PASS${NC} %s\n" "$msg"
  else
    failed=$(($failed+1))
    printf "  ${RED}FAILED${NC} %s\n" "$msg"
  fi
}

finalize() {
  total=$(($passed+$failed))
  printf "\n%d tests run, %d ${GREEN}passed${NC}, %d ${RED}failed${NC}\n" "$total" "$passed" "$failed"
  if [[ $failed -eq 0 ]]; then
    printf "${GREEN}pass${NC}: test run succeeded\n"
    exit 0
  else
    printf "${RED}error${NC}: test run failed\n"
    exit 101
  fi
}

cargo build -q -p glean-core --example rkv-open
test "building test binary" $?
cmd="target/debug/examples/rkv-open"

if [[ ! -x "$cmd" ]]; then
  test "missing test binary: $cmd" 1
  finalize
fi

export RUST_LOG=debug

# Empty directory - Glean creates a new database
{
  testname="empty dir"
  datapath=$(mktemp -d "${TMP}/glean_rkv_behavior.XXXX")
  log=$($cmd $datapath 2>&1)

  echo "$log" | ( ! grep -q "rkv failed" )
  test "$testname: no rkv failure detected" $?
  echo "$log" | grep -q "Database initialized"
  test "$testname: Database init" $?
  echo "$log" | grep -q "Glean initialized"
  test "$testname: Glean init" $?

  rm -rf "$datapath"
}

# Minimal valid Rkv database. Glean starts and uses it.
{
  testname="minimal db"
  datapath=$(mktemp -d "${TMP}/glean_rkv_behavior.XXXX")
  mkdir -p "$datapath/db"
  write_minbin "$datapath/db/data.safe.bin"

  log=$($cmd $datapath 2>&1)

  echo "$log" | ( ! grep -q "rkv failed" )
  test "$testname: no rkv failure detected" $?
  echo "$log" | grep -q "Database initialized"
  test "$testname: Database init" $?
  echo "$log" | grep -q "Glean initialized"
  test "$testname: Glean init" $?

  rm -rf "$datapath"
}

# Empty file in place of the database. Glean logs and creates a new database
{
  testname="empty db"
  datapath=$(mktemp -d "${TMP}/glean_rkv_behavior.XXXX")
  mkdir -p "$datapath/db"
  true > "$datapath/db/data.safe.bin"

  log=$($cmd $datapath 2>&1)

  echo "$log" | grep -q "rkv failed: invalid file"
  test "$testname: invalid file detected" $?
  echo "$log" | grep -q "Database initialized"
  test "$testname: Database init" $?
  echo "$log" | grep -q "Glean initialized"
  test "$testname: Glean init" $?

  rm -rf "$datapath"
}

# Corrupted Rkv file. Glean logs and creates a new database.
{
  testname="corrupted db"
  datapath=$(mktemp -d "${TMP}/glean_rkv_behavior.XXXX")
  mkdir -p "$datapath/db"
  write_minbin "$datapath/db/data.safe.bin" "$CORRUPT_DATA_SAFE_BIN"

  log=$($cmd $datapath 2>&1)

  echo "$log" | grep -q "rkv failed: invalid file"
  test "$testname: invalid file detected" $?
  echo "$log" | grep -q "Database initialized"
  test "$testname: Database init" $?
  echo "$log" | grep -q "Glean initialized"
  test "$testname: Glean init" $?

  rm -rf "$datapath"
}

# Existing but inaccessible database file. Glean fails to initialize.
{
  testname="inaccessible db"
  datapath=$(mktemp -d "${TMP}/glean_rkv_behavior.XXXX")
  mkdir -p "$datapath/db"
  dbfile="$datapath/db/data.safe.bin"
  true > "$dbfile"
  chmod ugo-rw "$dbfile"

  log=$($cmd $datapath 2>&1)

  echo "$log" | grep -qE "Failed to initialize Glean.+Permission denied"
  test "$testname: Glean fails to initialize (Permission denied error)" $?

  rm -rf "$datapath"
}

# Existing but inaccessible temporary database file. Glean initializes, but can't write metrics.
{
  testname="inaccessible tmp db"
  datapath=$(mktemp -d "${TMP}/glean_rkv_behavior.XXXX")
  mkdir -p "$datapath/db"
  dbfile="$datapath/db/data.safe.tmp"
  true > "$dbfile"
  chmod ugo-rw "$dbfile"

  log=$($cmd $datapath 2>&1)

  echo "$log" | ( ! grep -q "rkv failed" )
  test "$testname: no rkv failure detected" $?
  echo "$log" | grep -q "Database initialized"
  test "$testname: Database init" $?
  echo "$log" | grep -q "Glean initialized"
  test "$testname: Glean init" $?
  echo "$log" | grep -qE "Failed to record metric.+Permission denied"
  test "$testname: Can't record metric (permission denied)" $?

  rm -rf "$datapath"
}

# Database directory is a file. Glean fails to initialize.
{
  testname="db path is a file"
  datapath=$(mktemp -d "${TMP}/glean_rkv_behavior.XXXX")
  true > "$datapath/db"

  log=$($cmd $datapath 2>&1)

  echo "$log" | grep -qE "Failed to initialize Glean.+File exists"
  test "$testname: Glean fails to initialize (File exists error)" $?

  rm -rf "$datapath"
}

# Database directory is not writable. Glean initializes, but can't record metrics.
{
  testname="db dir is not writable"
  datapath=$(mktemp -d "${TMP}/glean_rkv_behavior.XXXX")
  mkdir "$datapath/db"
  chmod -w "$datapath/db"

  log=$($cmd $datapath 2>&1)

  echo "$log" | ( ! grep -q "rkv failed" )
  test "$testname: no rkv failure detected" $?
  echo "$log" | grep -q "Database initialized"
  test "$testname: Database init" $?
  echo "$log" | grep -q "Glean initialized"
  test "$testname: Glean init" $?
  echo "$log" | grep -qE "Failed to record metric.+Permission denied"
  test "$testname: Can't record metric (permission denied)" $?

  rm -rf "$datapath"
}
#
# Minimal valid Rkv database in tmp location. Glean starts.
{
  testname="minimal tmp db"
  datapath=$(mktemp -d "${TMP}/glean_rkv_behavior.XXXX")
  mkdir -p "$datapath/db"
  write_minbin "$datapath/db/data.safe.tmp"

  log=$($cmd $datapath 2>&1)

  echo "$log" | ( ! grep -q "rkv failed" )
  test "$testname: no rkv failure detected" $?
  echo "$log" | grep -q "Database initialized"
  test "$testname: Database init" $?
  echo "$log" | grep -q "Glean initialized"
  test "$testname: Glean init" $?

  rm -rf "$datapath"
}

# Empty file in place of the database. Glean logs and creates a new database
{
  testname="empty tmp db"
  datapath=$(mktemp -d "${TMP}/glean_rkv_behavior.XXXX")
  mkdir -p "$datapath/db"
  true > "$datapath/db/data.safe.tmp"

  log=$($cmd $datapath 2>&1)

  echo "$log" | ( ! grep -q "rkv failed: invalid file" )
  test "$testname: no invalid file detected" $?
  echo "$log" | grep -q "Database initialized"
  test "$testname: Database init" $?
  echo "$log" | grep -q "Glean initialized"
  test "$testname: Glean init" $?

  rm -rf "$datapath"
}

# Corrupted Rkv file. Glean logs and creates a new database.
{
  testname="corrupted tmp db"
  datapath=$(mktemp -d "${TMP}/glean_rkv_behavior.XXXX")
  mkdir -p "$datapath/db"
  write_minbin "$datapath/db/data.safe.tmp" "$CORRUPT_DATA_SAFE_BIN"

  log=$($cmd $datapath 2>&1)

  echo "$log" | ( ! grep -q "rkv failed: invalid file" )
  test "$testname: no invalid file detected" $?
  echo "$log" | grep -q "Database initialized"
  test "$testname: Database init" $?
  echo "$log" | grep -q "Glean initialized"
  test "$testname: Glean init" $?

  rm -rf "$datapath"
}

finalize
