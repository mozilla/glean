#!/bin/bash

for i in `seq 1 100`; do
  SEED=$((RANDOM))
  MAXN=$((RANDOM%10000))

  printf "ITER=%d SEED=%d MAXN=%d\n" "$i" "$SEED" "$MAXN"
  SEED=$SEED MAXN=$MAXN cargo test -q -p glean-tests --test disk-writes -- --ignored
  if [[ $? -ne 0 ]]; then
    exit 1
  fi
done
