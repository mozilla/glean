#!/bin/bash
#
# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.

# A hack to not do anything when targetting darwin (macOS),
# but still correctly build everything else.
# Only to be used on Linux hosts.

unset RUSTC
if echo "$*" | grep -q "print=cfg"; then
  rustc $*
elif echo "$*" | grep -q "target x86_64-apple-darwin"; then
  true
else
  rustc $*
fi
