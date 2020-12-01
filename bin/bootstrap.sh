#!/bin/sh

# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/. */

# Xcode 12 -- Carthage workaround
# See https://github.com/Carthage/Carthage/issues/3019
if xcodebuild -version | grep -q "Xcode 12."; then
  xcconfig="${PWD}/tmp.xcconfig"
  true > "$xcconfig"
  echo 'EXCLUDED_ARCHS__EFFECTIVE_PLATFORM_SUFFIX_simulator__NATIVE_ARCH_64_BIT_x86_64=arm64 arm64e armv7 armv7s armv6 armv8' >> "$xcconfig"
  echo 'EXCLUDED_ARCHS=$(inherited) $(EXCLUDED_ARCHS__EFFECTIVE_PLATFORM_SUFFIX_$(EFFECTIVE_PLATFORM_SUFFIX)__NATIVE_ARCH_64_BIT_$(NATIVE_ARCH_64_BIT))' >> "$xcconfig"
  export XCODE_XCCONFIG_FILE="${xcconfig}"
fi

carthage bootstrap --platform iOS --color auto --cache-builds
