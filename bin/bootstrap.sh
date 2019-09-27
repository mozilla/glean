#!/bin/sh

# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/. */

if [ ! -e 'Cartfile' ]; then
  ln -s tests-only-Cartfile Cartfile
fi

carthage bootstrap --platform iOS --color auto --cache-builds
