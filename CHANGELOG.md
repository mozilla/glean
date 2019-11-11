# Unreleased changes

[Full changelog](https://github.com/mozilla/glean/compare/v20.0.0...master)

# v20.0.0 (2019-11-11)

[Full changelog](https://github.com/mozilla/glean/compare/v19.1.0...v20.0.0)

* Glean users should now use a Gradle plugin rather than a Gradle script. (#421)
  See [integrating with the build system docs](https://mozilla.github.io/glean/book/user/adding-glean-to-your-project.html#integrating-with-the-build-system) for more information.

* In Kotlin, metrics that can record errors now have a new testing method,
  `testGetNumRecordedErrors`. (#401)

# v19.1.0 (2019-10-29)

[Full changelog](https://github.com/mozilla/glean/compare/v19.0.0...v19.1.0)

* Fixed a crash calling `start` on a timing distribution metric before Glean is initialized.
  Timings are always measured, but only recorded when upload is enabled ([#400](https://github.com/mozilla/glean/pull/400))
* BUGFIX: When the Debug Activity is used to log pings, each ping is now logged only once ([#407](https://github.com/mozilla/glean/pull/407))
* New `invalid state` error, used in timespan recording ([#230](https://github.com/mozilla/glean/pull/230))
* Add an Android crash instrumentation walkthrough ([#399](https://github.com/mozilla/glean/pull/399))
* Fix crashing bug by avoiding assert-printing in lmdb ([#422](https://github.com/mozilla/glean/pull/422))
* Upgrade dependencies, including rkv ([#416](https://github.com/mozilla/glean/pull/416))

# v19.0.0 (2019-10-22)

[Full changelog](https://github.com/mozilla/glean/compare/v0.0.1-TESTING6...v19.0.0)

First stable release of Glean in Rust (aka glean-core).
This is a major milestone in using a cross-platform implementation of Glean on the Android platform.

* Fix roundtripping of timezone offsets in dates ([#392](https://github.com/mozilla/glean/pull/392))
* Handle dynamic labels in coroutine tasks ([#394](https://github.com/mozilla/glean/pull/384))

# v0.0.1-TESTING6 (2019-10-18)

[Full changelog](https://github.com/mozilla/glean/compare/v0.0.1-TESTING5...v0.0.1-TESTING6)

* Ignore dynamically stored labels if Glean is not initialized ([#374](https://github.com/mozilla/glean/pull/374))
* Make sure ProGuard doesn't remove Glean classes from the app ([#380](https://github.com/mozilla/glean/pull/380))
* Keep track of pings in all modes ([#378](https://github.com/mozilla/glean/pull/378))
* Add 'jnaTest' dependencies to the 'forUnitTest' JAR ([#382](https://github.com/mozilla/glean/pull/382))

# v0.0.1-TESTING5 (2019-10-10)

[Full changelog](https://github.com/mozilla/glean/compare/v0.0.1-TESTING4...v0.0.1-TESTING5)

* Upgrade to NDK r20 ([#365](https://github.com/mozilla/glean/pull/365))

# v0.0.1-TESTING4 (2019-10-09)

[Full changelog](https://github.com/mozilla/glean/compare/v0.0.1-TESTING3...v0.0.1-TESTING4)

* Take DST into account when converting a calendar into its items ([#359](https://github.com/mozilla/glean/pull/359))
* Include a macOS library in the `forUnitTests` builds ([#358](https://github.com/mozilla/glean/pull/358))
* Keep track of all registered pings in test mode ([#363](https://github.com/mozilla/glean/pull/363))

# v0.0.1-TESTING3 (2019-10-08)

[Full changelog](https://github.com/mozilla/glean/compare/v0.0.1-TESTING2...v0.0.1-TESTING3)

* Allow configuration of Glean through the GleanTestRule
* Bump `glean_parser` version to 1.9.2

# v0.0.1-TESTING2 (2019-10-07)

[Full changelog](https://github.com/mozilla/glean/compare/v0.0.1-TESTING1...v0.0.1-TESTING2)

* Include a Windows library in the `forUnitTests` builds

# v0.0.1-TESTING1 (2019-10-02)

[Full changelog](https://github.com/mozilla/glean/compare/95b6bcc03616c8d7c3e3e64e99ee9953aa06a474...v0.0.1-TESTING1)

### General

First testing release.
