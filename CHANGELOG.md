# Unreleased changes

[Full changelog](https://github.com/mozilla/glean/compare/v25.0.0...master)

* Python:
  * The boolean metric type is now supported in Python.

# v25.0.0 (2020-02-17)

[Full changelog](https://github.com/mozilla/glean/compare/v24.2.0...v25.0.0)

* General:
  * `ping_type` is not included in the `ping_info` any more ([#653](https://github.com/mozilla/glean/pull/653)), the pipeline takes the value from the submission URL.
  * The version of `glean_parser` has been upgraded to 1.18.2:
    * **Breaking Change (Java API)** Have the metrics names in Java match the names in Kotlin.
      See [Bug 1588060](https://bugzilla.mozilla.org/show_bug.cgi?id=1588060).
    * The reasons a ping are sent are now included in the generated markdown documentation.
* Android:
  * The `Glean.initialize` method runs mostly off the main thread ([#672](https://github.com/mozilla/glean/pull/672)).
  * Labels in labeled metrics now have a correct, and slightly stricter, regular expression.
    See [label format](https://mozilla.github.io/glean/user/metrics/index.html#label-format) for more information.
* iOS:
  * The baseline ping will now include `reason` codes that indicate why it was
    submitted. If an unclean shutdown is detected (e.g. due to force-close), this
    ping will be sent at startup with `reason: dirty_startup`.
  * Per [Bug 1614785](https://bugzilla.mozilla.org/show_bug.cgi?id=1614785), the
    clearing of application lifetime metrics now occurs after the metrics ping is
    sent in order to preserve values meant to be included in the startup metrics
    ping.
  * `initialize()` now performs most of its work in a background thread.
* Python:
  * When the pre-init task queue overruns, this is now recorded in the metric
    `glean.error.preinit_tasks_overflow`.
  * glinter warnings are printed to `stderr` when loading `metrics.yaml` and
    `pings.yaml` files.

# v24.2.0 (2020-02-11)

[Full changelog](https://github.com/mozilla/glean/compare/v24.1.0...v24.2.0)

* General:
  * Add `locale` to `client_info` section.
  * **Deprecation Warning** Since `locale` is now in the `client_info` section, the one
    in the baseline ping ([`glean.baseline.locale`](https://github.com/mozilla/glean/blob/c261205d6e84d2ab39c50003a8ffc3bd2b763768/glean-core/metrics.yaml#L28-L42))
    is redundant and will be removed by the end of the quarter.
  * Drop the Glean handle and move state into glean-core ([#664](https://github.com/mozilla/glean/pull/664))
  * If an experiment includes no `extra` fields, it will no longer include `{"extra": null}` in the JSON payload.
  * Support for ping `reason` codes was added.
  * The metrics ping will now include `reason` codes that indicate why it was
    submitted.
  * The version of `glean_parser` has been upgraded to 1.17.3
* Android:
  * Collections performed before initialization (preinit tasks) are now dispatched off
    the main thread during initialization.
  * The baseline ping will now include `reason` codes that indicate why it was
    submitted. If an unclean shutdown is detected (e.g. due to force-close), this
    ping will be sent at startup with `reason: dirty_startup`.
* iOS:
  * Collections performed before initialization (preinit tasks) are now dispatched off
    the main thread and not awaited during initialization.
  * Added recording of `glean.error.preinit_tasks_overflow` to report when
    the preinit task queue overruns, leading to data loss. See [bug
    1609734](https://bugzilla.mozilla.org/show_bug.cgi?id=1609734)

# v24.1.0 (2020-01-16)

[Full changelog](https://github.com/mozilla/glean/compare/v24.0.0...v24.1.0)

* General:
  * Stopping a non started measurement in a timing distribution will now be reported
    as an `invalid_state` error.
* Android:
  * A new metric `glean.error.preinit_tasks_overflow` was added to report when
    the preinit task queue overruns, leading to data loss. See [bug
    1609482](https://bugzilla.mozilla.org/show_bug.cgi?id=1609482)

# v24.0.0 (2020-01-14)

[Full changelog](https://github.com/mozilla/glean/compare/v23.0.1...v24.0.0)

* General:
  * **Breaking Change** An `enableUpload` parameter has been added to the `initialize()`
    function. This removes the requirement to call `setUploadEnabled()` prior to calling
    the `initialize()` function.
* Android:
  * The metrics ping scheduler will now only send metrics pings while the
    application is running. The application will no longer "wake up" at 4am
    using the Work Manager.
  * The code for migrating data from Glean SDK before version 19 was removed.
  * When using the `GleanTestLocalServer` rule in instrumented tests, pings are
    immediately flushed by the `WorkManager` and will reach the test endpoint as
    soon as possible.
* Python:
  * The Python bindings now support Python 3.5 - 3.7.
  * The Python bindings are now distributed as a wheel on Linux, macOS and
    Windows.

# v23.0.1 (2020-01-08)

[Full changelog](https://github.com/mozilla/glean/compare/v23.0.0...v23.0.1)

* Android:
  * BUGFIX: The Glean Gradle plugin will now work if an app or library doesn't
    have a metrics.yaml or pings.yaml file.
* iOS:
  * The released iOS binaries are now built with Xcode 11.3.

# v23.0.0 (2020-01-07)

[Full changelog](https://github.com/mozilla/glean/compare/v22.1.0...v23.0.0)

* Python bindings:
  * Support for events and UUID metrics was added.
* Android:
  * The Glean Gradle Plugin correctly triggers docs and API updates when registry files
    change, without requiring them to be deleted.
  * `parseISOTimeString` has been made 4x faster. This had an impact on Glean
    migration and initialization.
  * Metrics with `lifetime: application` are now cleared when the application is started,
    after startup Glean SDK pings are generated.
* All platforms:
  * The public method `PingType.send()` (in all platforms) have been deprecated
    and renamed to `PingType.submit()`.
  * Rename `deletion_request` ping to `deletion-request` ping after glean_parser update

# v22.1.0 (2019-12-17)

[Full changelog](https://github.com/mozilla/glean/compare/v22.0.0...v22.1.0)

* Add `InvalidOverflow` error to `TimingDistribution`s ([#583](https://github.com/mozilla/glean/pull/583))

# v22.0.0 (2019-12-05)

[Full changelog](https://github.com/mozilla/glean/compare/v21.3.0...v22.0.0)

* Add option to defer ping lifetime metric persistence ([#530](https://github.com/mozilla/glean/pull/530))
* Add a crate for the nice control API ([#542](https://github.com/mozilla/glean/pull/542))
* Pending `deletion_request` pings are resent on start ([#545](https://github.com/mozilla/glean/pull/545))

# v21.3.0 (2019-12-03)

[Full changelog](https://github.com/mozilla/glean/compare/v21.2.0...v21.3.0)

* Timers are reset when disabled. That avoids recording timespans across disabled/enabled toggling ([#495](https://github.com/mozilla/glean/pull/495)).
* Add a new flag to pings: `send_if_empty` ([#528](https://github.com/mozilla/glean/pull/528))
* Upgrade `glean_parser` to v1.12.0
* Implement the deletion request ping in Glean ([#526](https://github.com/mozilla/glean/pull/526))

# v21.2.0 (2019-11-21)

[Full changelog](https://github.com/mozilla/glean/compare/v21.1.1...v21.2.0)

* All platforms

  * The experiments API is no longer ignored before the Glean SDK initialized. Calls are
    recorded and played back once the Glean SDK is initialized.

  * String list items were being truncated to 20, rather than 50, bytes when using
    `.set()` (rather than `.add()`). This has been corrected, but it may result
    in changes in the sent data if using string list items longer than 20 bytes.

# v21.1.1 (2019-11-20)

[Full changelog](https://github.com/mozilla/glean/compare/v21.1.0...v21.1.1)

* Android:

  * Use the `LifecycleEventObserver` interface, rather than the `DefaultLifecycleObserver`
    interface, since the latter isn't compatible with old SDK targets.

# v21.1.0 (2019-11-20)

[Full changelog](https://github.com/mozilla/glean/compare/v21.0.0...v21.1.0)

* Android:

  * Two new metrics were added to investigate sending of metrics and baseline pings.
    See [bug 1597980](https://bugzilla.mozilla.org/show_bug.cgi?id=1597980) for more information.

  * Glean's two lifecycle observers were refactored to avoid the use of reflection.

* All platforms:

  * Timespans will now not record an error if stopping after setting upload enabled to false.

# v21.0.0 (2019-11-18)

[Full changelog](https://github.com/mozilla/glean/compare/v20.2.0...v21.0.0)

* Android:

  * The `GleanTimerId` can now be accessed in Java and is no longer a `typealias`.

  * Fixed a bug where the metrics ping was getting scheduled twice on startup.
* All platforms

  * Bumped `glean_parser` to version 1.11.0.

# v20.2.0 (2019-11-11)

[Full changelog](https://github.com/mozilla/glean/compare/v20.1.0...v20.2.0)

* In earlier 20.x.x releases, the version of glean-ffi was incorrectly built
  against the wrong version of glean-core.

# v20.1.0 (2019-11-11)

[Full changelog](https://github.com/mozilla/glean/compare/v20.0.0...v20.1.0)

* The version of Glean is included in the Glean Gradle plugin.

* When constructing a ping, events are now sorted by their timestamp. In practice,
  it rarely happens that event timestamps are unsorted to begin with, but this
  guards against a potential race condition and incorrect usage of the lower-level
  API.

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
* Add an Android crash instrumentation walk-through ([#399](https://github.com/mozilla/glean/pull/399))
* Fix crashing bug by avoiding assert-printing in LMDB ([#422](https://github.com/mozilla/glean/pull/422))
* Upgrade dependencies, including rkv ([#416](https://github.com/mozilla/glean/pull/416))

# v19.0.0 (2019-10-22)

[Full changelog](https://github.com/mozilla/glean/compare/v0.0.1-TESTING6...v19.0.0)

First stable release of Glean in Rust (aka glean-core).
This is a major milestone in using a cross-platform implementation of Glean on the Android platform.

* Fix round-tripping of timezone offsets in dates ([#392](https://github.com/mozilla/glean/pull/392))
* Handle dynamic labels in coroutine tasks ([#394](https://github.com/mozilla/glean/pull/384))

# v0.0.1-TESTING6 (2019-10-18)

[Full changelog](https://github.com/mozilla/glean/compare/v0.0.1-TESTING5...v0.0.1-TESTING6)

* Ignore dynamically stored labels if Glean is not initialized ([#374](https://github.com/mozilla/glean/pull/374))
* Make sure ProGuard doesn't remove Glean classes from the app ([#380](https://github.com/mozilla/glean/pull/380))
* Keep track of pings in all modes ([#378](https://github.com/mozilla/glean/pull/378))
* Add `jnaTest` dependencies to the `forUnitTest` JAR ([#382](https://github.com/mozilla/glean/pull/382))

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

* Allow configuration of Glean through the `GleanTestRule`
* Bump `glean_parser` version to 1.9.2

# v0.0.1-TESTING2 (2019-10-07)

[Full changelog](https://github.com/mozilla/glean/compare/v0.0.1-TESTING1...v0.0.1-TESTING2)

* Include a Windows library in the `forUnitTests` builds

# v0.0.1-TESTING1 (2019-10-02)

[Full changelog](https://github.com/mozilla/glean/compare/95b6bcc03616c8d7c3e3e64e99ee9953aa06a474...v0.0.1-TESTING1)

### General

First testing release.
