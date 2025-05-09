#!/bin/bash

git clone --depth=1 https://github.com/mozilla/glean ~/project

PATH=$HOME/.cargo/bin:$PATH \
  rustc tools/patches/bcryptprimitives.rs -Copt-level=3 -Clto=fat --out-dir ~/project/wine_shims --target x86_64-pc-windows-gnu

exec bash -l
