# Unreleased changes

[Full changelog](https://github.com/mozilla/glean/compare/v65.2.2...main)

* General
  * BREAKING: Remove infill 0 buckets from custom distributions ([#3246](https://github.com/mozilla/glean/pull/3246))
  * Add a new `oneOf` type to Glean's object metric type structure ([#3273](https://github.com/mozilla/glean/pull/3273))
  * Updated to `glean_parser` v18.0.2 ([#3273](https://github.com/mozilla/glean/pull/3273), [#3289](https://github.com/mozilla/glean/pull/3289))
* Rust
  *  Use an associated type for `TestGetValue` ([#3259](https://github.com/mozilla/glean/pull/3259))
* Swift
  * Make `EventMetricType`, `ObjectMetricType`, `URLMetricType` and `Ping` `Sendable`  ([#3255](https://github.com/mozilla/glean/pull/3255))
  * Glean for iOS is now being built with Xcode 16.4 ([#3270](https://github.com/mozilla/glean/pull/3270))
* Android
  * BREAKING CHANGE: Updated the minimum Android API level to 26 (Android 8) ([#3287](https://github.com/mozilla/glean/pull/3287))
  * Updated Android Gradle Plugin to 8.13.0 ([#3287](https://github.com/mozilla/glean/pull/3287))
  * Updated Kotlin to 2.2.20 ([#3287](https://github.com/mozilla/glean/pull/3287))

# v65.2.2 (2025-10-02)

[Full changelog](https://github.com/mozilla/glean/compare/v65.2.1...v65.2.2)

* General
  * Report db record counts during init phases ([bug 1992024](https://bugzilla.mozilla.org/show_bug.cgi?id=1992024))

# v65.2.1 (2025-09-26)

[Full changelog](https://github.com/mozilla/glean/compare/v65.2.0...v65.2.1)

* Python
  * BUGFIX: Fix macOS python wheel deploy

# v65.2.0 (2025-09-26)

[Full changelog](https://github.com/mozilla/glean/compare/v65.1.1...v65.2.0)

* Swift
  * Glean for iOS is now being built with Xcode 16.2 ([#3189](https://github.com/mozilla/glean/pull/3189))
* General
  * Report running count of initializations in "health" ping ([bug 1990624](https://bugzilla.mozilla.org/show_bug.cgi?id=1990624))
  * Report db file sizes during init phases ([bug 1990627](https://bugzilla.mozilla.org/show_bug.cgi?id=1990627))

# v65.1.1 (2025-09-16)

[Full changelog](https://github.com/mozilla/glean/compare/v65.1.0...v65.1.1)

* General
  * Remove newly added call to set test-mode in `test_reset_glean`, instead setting test-mode only in necessary tests.

# v65.1.0 (2025-09-09)

[Full changelog](https://github.com/mozilla/glean/compare/v65.0.3...v65.1.0)

* General
  * Added a Glean Health ping which collects telemetry health data into a single ping sent before and after initialization in order to track issues with Glean storage files and other telemetry health characteristics. ([#3221](https://github.com/mozilla/glean/pull/3221))
* Python
  * Ship a Python wheel for aarch64-windows ([#3245](https://github.com/mozilla/glean/pull/3245))

# v65.0.3 (2025-09-02)

[Full changelog](https://github.com/mozilla/glean/compare/v65.0.2...v65.0.3)

* Swift
  * BUGFIX: Refactor uploader interfaces to permit customization ([#3235](https://github.com/mozilla/glean/pull/3235), [#3239](https://github.com/mozilla/glean/pull/3239))
  * Make objects `BuildInfo`, `DatetimeMetricType`, `EventMetricType`, and `UuidMetricType` `sendable` ([#3241](https://github.com/mozilla/glean/pull/3241))

# v65.0.2 (2025-08-25)

[Full changelog](https://github.com/mozilla/glean/compare/v65.0.1...v65.0.2)

* General
  * Pings on a ping schedule now get submitted even if the related ping might be disabled ([#3226](https://github.com/mozilla/glean/pull/3226))
  * Gracefully handle inability to open the event storage file ([#3232](https://github.com/mozilla/glean/pull/3232))

# v65.0.1 (2025-08-21)

[Full changelog](https://github.com/mozilla/glean/compare/v65.0.0...v65.0.1)

* Kotlin
  * Lock access to the internal dispatcher to ensure correct order of operations.
    Previously a race condition could lead to early metric recordings throwing an exception and crashing the application ([#3228](https://github.com/mozilla/glean/pull/3228))

# v65.0.0 (2025-08-18)

[Full changelog](https://github.com/mozilla/glean/compare/v64.5.4...v65.0.0)

* General
  * Performance improvement: Reduce file system operations when recording events ([#3179](https://github.com/mozilla/glean/pull/3179))
  * `LabeledMetric` improvement: Added `testGetValue` as a test method on all labeled metric types ([#3190](https://github.com/mozilla/glean/pull/3190))
  * `DualLabeledCounter` improvement: Added `testGetValue` as a test method ([#3209](https://github.com/mozilla/glean/pull/3209))
  * Improvement: Updated all remaining metrics to implement the `TestGetValue` trait ([#3209](https://github.com/mozilla/glean/pull/3209))
  * New metric: `glean.ping.uploader_capabilities` reporting the requested uploader capabilities for a ping ([#3188](https://github.com/mozilla/glean/pull/3188))
* Android
  * Updated Android Gradle Plugin to 8.12.0 ([#3208](https://github.com/mozilla/glean/pull/3208))
  * Updated Android NDK to r28c ([#3199](https://github.com/mozilla/glean/pull/3199))
  * Updated Android SDK target to version 36 ([#3180](https://github.com/mozilla/glean/pull/3180))
  * Updated Gradle to 8.14.3 ([#3180](https://github.com/mozilla/glean/pull/3180))
  * Updated Kotlin to 2.2.10 ([#3219](https://github.com/mozilla/glean/pull/3219))
  * BREAKING CHANGE: Dispatch most metric recordings on a Kotlin dispatcher to avoid calling into glean-core early.
    This does not change any behavior: The dispatch queue is worked on right after initialization ([#3183](https://github.com/mozilla/glean/pull/3183))
  * The `testBeforeNextSubmit` now returns a job to be awaited. This allows to wait for the callback
    and properly handles exceptions ([#3218](https://github.com/mozilla/glean/pull/3218))
* Python
  * Bump minimum required Python version to 3.9 ([#3164](https://github.com/mozilla/glean/issues/3164))
  * Report `client_info.architecture` as reported from Python again ([#3185](https://github.com/mozilla/glean/issues/3185))
* Swift
  * Expose an interface by which to supply an external uploader on iOS ([Bug 1950143](https://bugzilla.mozilla.org/show_bug.cgi?id=1950143))
* Rust
  * New feature `gecko`. If enabled spawned threads are registered with the Gecko profiler (non-Android only) ([#3212](https://github.com/mozilla/glean/pull/3212))

# v64.5.5 (2025-08-25)

[Full changelog](https://github.com/mozilla/glean/compare/v64.5.4...v64.5.5)

* General
  * Pings on a ping schedule now get submitted even if the related ping might be disabled ([#3226](https://github.com/mozilla/glean/pull/3226))
    (Backported changes)

# v64.5.4 (2025-07-29)

[Full changelog](https://github.com/mozilla/glean/compare/v64.5.3...v64.5.4)

* General
  * Update the compiler used on CI

# v64.5.3 (2025-07-29)

[Full changelog](https://github.com/mozilla/glean/compare/v64.5.2...v64.5.3)

* General
  * BUGFIX: Avoid accidental rapid rescheduling of the `metrics` ping on startup ([#3201](https://github.com/mozilla/glean/pull/3201))

# v64.5.2 (2025-07-01)

[Full changelog](https://github.com/mozilla/glean/compare/v64.5.1...v64.5.2)

* Rust
  * Added `MetricType` implementation for `DualLabeledCounter` ([Bug 1973287](https://bugzilla.mozilla.org/show_bug.cgi?id=1973287))
  * Update `time` dependency to v0.3 ([#3165](https://github.com/mozilla/glean/pull/3165))
* Python
  * Ship a Python wheel for aarch64-linux ([#3173](https://github.com/mozilla/glean/pull/3173))

# v64.5.1 (2025-06-23)

[Full changelog](https://github.com/mozilla/glean/compare/v64.5.0...v64.5.1)

* Swift
  * No change dot-release to fix failed Swift package deploy to `mozilla/glean-swift`

# v64.5.0 (2025-06-18)

[Full changelog](https://github.com/mozilla/glean/compare/v64.4.0...v64.5.0)

* Python
  * Stop building wheels for Windows i686 ([#3144](https://github.com/mozilla/glean/pull/3144))
* Rust
  * Added new metric type (`DualLabeledCounter`) to support migration of legacy keyed-categorical histograms ([bug 1957085](https://bugzilla.mozilla.org/show_bug.cgi?id=1957085))
* General
  * BUGFIX: Count distinct labels, not total labels, before spilling into `__other__` bucket ([#3157](https://github.com/mozilla/glean/pull/3157))
  * Updated to `glean_parser` v17.2.0 ([#3166](https://github.com/mozilla/glean/pull/3166))

# v64.4.0 (2025-05-30)

[Full changelog](https://github.com/mozilla/glean/compare/v64.3.1...v64.4.0)

* General
  * Permit non-ASCII in dynamic labels ([bug 1968533](https://bugzilla.mozilla.org/show_bug.cgi?id=1968533))
* Android
  * Build release artifacts on Rust 1.86.0 to avoid broken files ([#3149](https://github.com/mozilla/glean/pull/3149))

# v64.3.1 (2025-05-23)

[Full changelog](https://github.com/mozilla/glean/compare/v64.3.0...v64.3.1)

* Android
  * Reverted JNA to version 5.14.0 due crashes on Android 5 & 6 ([#3136](https://github.com/mozilla/glean/pull/3136))

# v64.3.0 (2025-05-21)

[Full changelog](https://github.com/mozilla/glean/compare/v64.2.0...v64.3.0)

* Android
  * Updated to Android NDK r28b ([#3118](https://github.com/mozilla/glean/pull/3118))
* Kotlin
  * Added support for labeled quantity metric type ([#3121](https://github.com/mozilla/glean/pull/3121))
  * BUGFIX: Ensure the inner ping object is initialized before accessing it ([#3132](https://github.com/mozilla/glean/pull/3132))
* Swift
  * Added support for labeled quantity metric type ([#3121](https://github.com/mozilla/glean/pull/3121))
* Python
  * Added support for labeled quantity metric type ([#3121](https://github.com/mozilla/glean/pull/3121))
  * Make rate metric actually usable ([#3131](https://github.com/mozilla/glean/pull/3131))
  * Make text metric actually usable ([#3131](https://github.com/mozilla/glean/pull/3131))

# v64.2.0 (2025-04-28)

[Full changelog](https://github.com/mozilla/glean/compare/v64.1.1...v64.2.0)

* Rust
  * Apply `malloc_size_of` to most types to gather heap-allocated memory ([#2794](https://github.com/mozilla/glean/pull/2794))

# v64.1.1 (2025-04-10)

[Full changelog](https://github.com/mozilla/glean/compare/v64.1.0...v64.1.1)

* General
  * Increase the maximum label length to 111 ([#3108](https://github.com/mozilla/glean/pull/3108))
  * Allow `test_get_{attribution|distribution}` to wait on init if in progress ([bug 1959515](https://bugzilla.mozilla.org/show_bug.cgi?id=1959515))
  * Fix race where setting ping enabled, ping registered, server knobs, debug tag, source tag, logging, ping submission, attribution, or distribution could crash if it comes in between init being called and the Global Glean being setup. ([bug 1959771](https://bugzilla.mozilla.org/show_bug.cgi?id=1959771))

# v64.1.0 (2025-04-07)

[Full changelog](https://github.com/mozilla/glean/compare/v64.0.1...v64.1.0)

* General
  * FEATURE: New client-wide attribution and distribution fields ([bug 1955428](https://bugzilla.mozilla.org/show_bug.cgi?id=1955428))
  * Update to `glean_parser` v17.1.0
* Kotlin
  * Updated Android Gradle Plugin to 8.9.1 ([#3098](https://github.com/mozilla/glean/pull/3098))
  * Updated Kotlin to version 2.1.20 ([#3098](https://github.com/mozilla/glean/pull/3098))
  * Dispatch ping API on the task queue ([#3101](https://github.com/mozilla/glean/pull/3101))

# v64.0.1 (2025-04-01)

[Full changelog](https://github.com/mozilla/glean/compare/v64.0.0...v64.0.1)

* Android
  * Revert changes that tried to fix `StrictMode` violations in Fenix ([bug 1946133](https://bugzilla.mozilla.org/show_bug.cgi?id=1946133))

# v64.0.0 (2025-03-18)

[Full changelog](https://github.com/mozilla/glean/compare/v63.1.0...v64.0.0)

* General
  * BREAKING CHANGE: Pings now pass required uploader capabilities during upload ([bug 1920732](https://bugzilla.mozilla.org/show_bug.cgi?id=1920732))
  * BREAKING CHANGE: Glean won't clear `client_info` fields when collection gets disabled. The `client_id` will still be cleared. ([#3068](https://github.com/mozilla/glean/pull/3068))
* Android
  * Updated Gradle to 8.13 ([#3074](https://github.com/mozilla/glean/pull/3080))
  * Updated to Android NDK 28 and SDK 35 ([#3074](https://github.com/mozilla/glean/pull/3080))
  * Updated Kotlin to version 2.1.10 ([#3074](https://github.com/mozilla/glean/pull/3080))
  * Updated Android Gradle Plugin to 8.8.2 ([#3074](https://github.com/mozilla/glean/pull/3080))
  * Updated JNA to version 5.17.0 ([#3081](https://github.com/mozilla/glean/pull/3081))
* Rust
  * Report more desktop architectures in `client_info.architecture` ([bug 1944694](https://bugzilla.mozilla.org/show_bug.cgi?id=1944694))

# v63.1.0 (2025-01-30)

[Full changelog](https://github.com/mozilla/glean/compare/v63.0.0...v63.1.0)

* General
  * The `glean.validation.pings_submitted` metric will now only record counts for built-in pings ([#3010](https://github.com/mozilla/glean/pull/3010))
* Rust
  * Provide a public interface so that consumers of RLB can access metric identifiers ([#3054](https://github.com/mozilla/glean/pull/3054))
* Kotlin
  * Updated `rust-android-gradle` to avoid problems with Python 3.13+ ([#3031](https://github.com/mozilla/glean/pull/3031))
  * Update Glean plugin to be configuration-cache friendly ([#3041](https://github.com/mozilla/glean/pull/3041))
  * Dispatch experiment API on the task queue ([#3032](https://github.com/mozilla/glean/pull/3032))

# v63.0.0 (2024-11-28)

[Full changelog](https://github.com/mozilla/glean/compare/v62.0.0...v63.0.0)

* General
  * Add methods to access current Glean debugging settings and the list of currently registered pings([Bug 1921976](https://bugzilla.mozilla.org/show_bug.cgi?id=1921976)).
  * Require `glean_parser` v16.1.0 ([#3006](https://github.com/mozilla/glean/pull/3006))
  * BREAKING CHANGE: Add new `collection-enabled` mode (and `follows_collection_enabled` setting for pings).
    This allows to control a subset of pings independently from the Glean-wide `upload-enabled` flag.
    This deprecates the `setUploadEnabled` API in favor of `setCollectionEnabled`. ([#3006](https://github.com/mozilla/glean/pull/3006))
* Rust
  * Permit Glean shutdown to interrupt UploadManager Wait tasks ([bug 1928288](https://bugzilla.mozilla.org/show_bug.cgi?id=1928288))

# v62.0.0 (2024-11-05)

[Full changelog](https://github.com/mozilla/glean/compare/v61.2.0...v62.0.0)

* General
  * **BREAKING**: Remove LMDB-to-safe-mode migration.
    Safe-mode became the default in Glean v51. ([bug 1780370](https://bugzilla.mozilla.org/show_bug.cgi?id=1780370))
  * **BREAKING**: Stop sending buckets with 0 counts in memory_distribution and timing_distribution metric payloads ([bug 1898336](https://bugzilla.mozilla.org/show_bug.cgi?id=1898336))
  * Require `glean_parser` v15.2.0 ([bug 1925346](https://bugzilla.mozilla.org/show_bug.cgi?id=1925346))
  * Disabled the `glean.database.write_time` metric as the instrumented behavior was triggering metrics pings to be sent containing only that metric ([Bug 1928168](https://bugzilla.mozilla.org/show_bug.cgi?id=1928168))
* Rust
  * New Metric Type: `labeled_quantity` ([bug 1925346](https://bugzilla.mozilla.org/show_bug.cgi?id=1925346))

# v61.2.0 (2024-10-07)

[Full changelog](https://github.com/mozilla/glean/compare/v61.1.0...v61.2.0)

* Kotlin
  * Accept a ping schedule map on initialize ([#2967](https://github.com/mozilla/glean/pull/2967))
* Swift
  * Accept a ping schedule map on initialize ([#2967](https://github.com/mozilla/glean/pull/2967))

# v61.1.0 (2024-09-24)

[Full changelog](https://github.com/mozilla/glean/compare/v61.0.0...v61.1.0)

* Kotlin
  * Change Metrics Ping Scheduler to use daemon threads ([#2930](https://github.com/mozilla/glean/pull/2930))
  * Dispatch metric recording for event, object and timing distribution on the task queue ([#2942](https://github.com/mozilla/glean/pull/2942))
* Rust
  * **Experimental**: Buffered API for timing, memory and custom distribution ([#2948](https://github.com/mozilla/glean/pull/2948))

# v61.0.0 (2024-08-21)

[Full changelog](https://github.com/mozilla/glean/compare/v60.5.0...v61.0.0)

* General
  * BREAKING CHANGE: Updated to UniFFI 0.28.0 ([#2920](https://github.com/mozilla/glean/pull/2920))
  * BREAKING CHANGE: Update to `glean_parser` v15.0.0 ([release notes](https://github.com/mozilla/glean_parser/releases/tag/v15.0.0))
* Kotlin
  * BREAKING CHANGE: Remove now obsolete type alias ([#2935](https://github.com/mozilla/glean/issues/2935))

# v60.5.0 (2024-08-06)

[Full changelog](https://github.com/mozilla/glean/compare/v60.4.0...v60.5.0)

* General
  * Make auto-flush behavior configurable and time-based ([#2871](https://github.com/mozilla/glean/pull/2871))
  * Require `glean_parser` v14.5.0 ([#2916](https://github.com/mozilla/glean/pull/2916))
* Android
  * Update to Gradle v8.9 ([#2909](https://github.com/mozilla/glean/pull/2909))
  * Fixed `GleanTestLocalServer` test rule to prevent leaking between tests([Bug 1787234](https://bugzilla.mozilla.org/show_bug.cgi?id=1787234))
* Rust
  * Remove cargo feature `preinit_million_queue` and set the default pre-init queue size to 10^6 for all consumers ([Bug 1909246](https://bugzilla.mozilla.org/show_bug.cgi?id=1909246))

# v60.4.0 (2024-07-23)

[Full changelog](https://github.com/mozilla/glean/compare/v60.3.0...v60.4.0)

* General
  * Bump the string length limit to 255 characters ([#2857](https://github.com/mozilla/glean/pull/2857))
  * New metric `glean.database.write_time` to measure database writes ([#2845](https://github.com/mozilla/glean/pull/2845))
  * Require glean_parser v14.3.0 ([bug 1909244](https://bugzilla.mozilla.org/show_bug.cgi?id=1909244))
* Android
  * Delay log init until Glean is getting initialized ([#2858](https://github.com/mozilla/glean/pull/2858))
  * Update to Gradle v8.8 ([#2860](https://github.com/mozilla/glean/pull/2860))
  * Updated Kotlin to version 1.9.24 ([#2861](https://github.com/mozilla/glean/pull/2861))
  * Default-enable `delayPingLifetimeIo` ([#2863](https://github.com/mozilla/glean/issues/2863))
  * Preparing Glean to be able to remove `service-glean` from Android Components ([#2891](https://github.com/mozilla/glean/pull/2891))
  * Gradle Plugin: Support for using an external Python environment ([#2889](https://github.com/mozilla/glean/pull/2889))
* Rust
  * New Metric Types: `labeled_custom_distribution`, `labeled_memory_distribution`, and `labeled_timing_distribution` ([bug 1657947](https://bugzilla.mozilla.org/show_bug.cgi?id=1657947))

# v60.3.0 (2024-05-31)

[Full changelog](https://github.com/mozilla/glean/compare/v60.2.0...v60.3.0)

* Android
  * Allow configuring `delayPingLifetimeIo` in Kotlin and auto-flush this data after 1000 writes.
    It is also auto-flushed on background. ([#2851](https://github.com/mozilla/glean/pull/2851))

# v60.2.0 (2024-05-23)

[Full changelog](https://github.com/mozilla/glean/compare/v60.1.0...v60.2.0)

* Rust
  * Accept a ping schedule map on initialize ([#2839](https://github.com/mozilla/glean/pull/2839))

# v60.1.1 (2024-05-31)

[Full changelog](https://github.com/mozilla/glean/compare/v60.1.0...v60.1.1)

* Android
  * Allow configuring `delayPingLifetimeIo` in Kotlin and auto-flush this data after 1000 writes.
    It is also auto-flushed on background. ([#2851](https://github.com/mozilla/glean/pull/2851))
    (Backported changes)

# v60.1.0 (2024-05-06)

[Full changelog](https://github.com/mozilla/glean/compare/v60.0.0...v60.1.0)

* Rust
  * New `TimingDistribution` API for no-allocation single-duration accumulation. ([bug 1892097](https://bugzilla.mozilla.org/show_bug.cgi?id=1892097))
* Python
  * Replace use of deprecated functionality (and make installs work on Python 3.12) ([#2820](https://github.com/mozilla/glean/pull/2820))

# v60.0.1 (2024-05-31)

[Full changelog](https://github.com/mozilla/glean/compare/v60.0.0...v60.0.1)

* Android
  * Allow configuring `delayPingLifetimeIo` in Kotlin and auto-flush this data after 1000 writes.
    It is also auto-flushed on background. ([#2851](https://github.com/mozilla/glean/pull/2851))
    (Backported changes)

# v60.0.0 (2024-04-22)

[Full changelog](https://github.com/mozilla/glean/compare/v59.0.0...v60.0.0)

* General
  * BREAKING CHANGE: Server Knobs API changes requiring changes to consuming applications which make use of Server Knobs ([Bug 1889114](https://bugzilla.mozilla.org/show_bug.cgi?id=1889114))
  * BREAKING CHANGE: Deprecated Server Knobs API `setMetricsDisabled` has been removed from all bindings. ([#2792](https://github.com/mozilla/glean/pull/2792))
  * Added support for `ping_schedule` metadata property so that pings can be scheduled to be sent when other pings are sent. ([#2791](https://github.com/mozilla/glean/pull/2791))
* Android
  * Updated Kotlin to version 1.9.23 ([#2737](https://github.com/mozilla/glean/pull/2737))
  * New metric type: Object ([#2796](https://github.com/mozilla/glean/pull/2796))
* iOS
  * New metric type: Object ([#2796](https://github.com/mozilla/glean/pull/2796))
* Python
  * New metric type: Object ([#2796](https://github.com/mozilla/glean/pull/2796))

# v59.0.0 (2024-03-28)

[Full changelog](https://github.com/mozilla/glean/compare/v58.1.0...v59.0.0)

* General
  * Hide `glean_timestamp` from event extras in tests ([#2776](https://github.com/mozilla/glean/pull/2776))
  * Timing Distribution's timer ids now begin at 1, rather than 0, to make some multi-language use cases easier. ([2777](https://bugzilla.mozilla.org/show_bug.cgi?id=1882584))
  * Add a configuration option to disable internal pings ([#2786](https://github.com/mozilla/glean/pull/2786/))
  * Updated to UniFFI 0.27.0 ([#2762](https://github.com/mozilla/glean/pull/2762))

# v58.1.0 (2024-03-12)

[Full changelog](https://github.com/mozilla/glean/compare/v58.0.0...v58.1.0)

* General
  * Enable wall clock timestamp on all events by default ([#2767](https://github.com/mozilla/glean/issues/2767))
* Rust
  * Timing distribution and Custom distributions now expose `accumulate_single_sample`. This includes their traits and consumers that make use of them will need to implement the new functions ([Bug 1881297](https://bugzilla.mozilla.org/show_bug.cgi?id=1881297))
* Android
  * Timing and Custom Distributions now have a `accumulate_single_sample` API that don't require use of a collection ([Bug 1881297](https://bugzilla.mozilla.org/show_bug.cgi?id=1881297))
* Python
  * Timing Distributions now have both a `accumulate_samples` and `accumulate_single_sample` ([Bug 1881297](https://bugzilla.mozilla.org/show_bug.cgi?id=1881297))

# v58.0.0 (2024-02-29)

[Full changelog](https://github.com/mozilla/glean/compare/v57.0.0...v58.0.0)

* General
  * Update `glean_parser` to v13.0.0 ([release notes](https://github.com/mozilla/glean_parser/releases/tag/v13.0.0))
* Rust
  * New metric type: Object ([#2489](https://github.com/mozilla/glean/pull/2489))
  * BREAKING CHANGE: Support pings without `{client|ping}_info` sections ([#2756](https://github.com/mozilla/glean/pull/2756))
* Android
  * Upgrade Android NDK to r26c ([#2745](https://github.com/mozilla/glean/pull/2745))

# v57.0.0 (2024-02-12)

[Full changelog](https://github.com/mozilla/glean/compare/v56.1.0...v57.0.0)

* General
  * Added an experimental event listener API ([#2719](https://github.com/mozilla/glean/pull/2719))
* Android
  * BREAKING CHANGE: Update JNA to version 5.14.0. Projects using older JNA releases may encounter errors until they update. ([#2727](https://github.com/mozilla/glean/pull/2727))
  * Set the target Android SDK to version 34 ([#2709](https://github.com/mozilla/glean/pull/2709))
  * Fixed an incorrectly named method. The method is now correctly named `setExperimentationId`.
  * Update to Gradle v8.6 ([#2721](https://github.com/mozilla/glean/pull/2721)/[#2731](https://github.com/mozilla/glean/pull/2731))
  
# v56.1.0 (2024-01-16)

[Full changelog](https://github.com/mozilla/glean/compare/v56.0.0...v56.1.0)

* General
  * Errors are now recorded in cases where we had to create a new data store for Glean due to a failure ([bug 1815253](https://bugzilla.mozilla.org/show_bug.cgi?id=1815253))
  * Update `glean_parser` to v11.0.0 ([release notes](https://github.com/mozilla/glean_parser/releases/tag/v11.0.0))
  * Event metrics can now record a maximum of 50 keys in the event extra object ([Bug 1869429](https://bugzilla.mozilla.org/show_bug.cgi?id=1869429))
* iOS
  * Glean for iOS is now being built with Xcode 15.1 ([#2669](https://github.com/mozilla/glean/pull/2669))
* Android
  * Replaced `whenTaskAdded` with `configureEach` in `GleanGradlePlugin` to avoid unnecessary configuration. ([#2697](https://github.com/mozilla/glean/pull/2697))

# v56.0.0 (2023-11-30)

[Full changelog](https://github.com/mozilla/glean/compare/v55.0.0...v56.0.0)

* General
  * Updated to UniFFI 0.25.2 ([#2678](https://github.com/mozilla/glean/pull/2678))
* iOS
  * Dropped support for iOS < 15 ([#2681](https://github.com/mozilla/glean/pull/2681))

# v55.0.0 (2023-10-23)

* Python
  * BREAKING CHANGE: Dropped support for Python 3.7 ([#]())

[Full changelog](https://github.com/mozilla/glean/compare/v54.0.0...v55.0.0)

* General
  * BREAKING CHANGE: Adding `0` to a `counter` or `labeled_counter` metric will be silently ignored instead of raising an `invalid_value` error ([bug 1762859](https://bugzilla.mozilla.org/show_bug.cgi?id=1762859))
  * Trigger the uploader thread after scanning the pending pings directory ([bug 1847950](https://bugzilla.mozilla.org/show_bug.cgi?id=1847950))
  * Extend start/stop time of a ping to millisecond precision. Custom pings can opt-out using `precise_timestamps: false` ([#2456](https://github.com/mozilla/glean/pull/2456))
  * Update `glean_parser` to v10.0.0. Disallow `unit` field for anything but quantity, disallows `ping` lifetime metrics on the events ping, allows to configure precise timestamps in pings ([release notes](https://github.com/mozilla/glean_parser/releases/tag/v10.0.0))
  * Add an API to set an Experimentation ID that will be annotated to all pings ([Bug 1848201](https://bugzilla.mozilla.org/show_bug.cgi?id=1848201))

# v54.0.0 (2023-09-12)

[Full changelog](https://github.com/mozilla/glean/compare/v53.2.0...v54.0.0)

* General
  * Experimental: Add configuration to add a wall clock timestamp to all events ([#2513](https://github.com/mozilla/glean/issues/2513))
* Python
  * Switched the build system to maturin. This should not have any effect on consumers. ([#2345](https://github.com/mozilla/glean/pull/2345))
  * BREAKING CHANGE: Dropped support for Python 3.6 ([#2345](https://github.com/mozilla/glean/pull/2345))
* Kotlin
  * Update to Gradle v8.2.1 ([#2516](https://github.com/mozilla/glean/pull/2516))
  * Increase Android compile SDK to version 34 ([#2614](https://github.com/mozilla/glean/pull/2614))

# v53.2.0 (2023-08-02)

[Full changelog](https://github.com/mozilla/glean/compare/v53.1.0...v53.2.0)

* General
  * Update `glean_parser` to v8.1.0. Subsequently, metric names now have a larger limit of 70 characters ([release notes](https://github.com/mozilla/glean_parser/releases/tag/v8.1.0))
* Rust
  * The Ping Rate Limit type is now accessible in the Rust Language Binding ([#2528](https://github.com/mozilla/glean/pull/2528))
  * Gracefully handle a failure when starting the upload thread. Glean no longer crashes in that case. ([#2545](https://github.com/mozilla/glean/pull/2545))
  * `locale` now exposed through the RLB so it can be set by consumers ([#2531](https://github.com/mozilla/glean/pull/2531))
* Python
  * Added the shutdown API for Python to ensure orderly shutdown and waiting for uploader processes ([#2538](https://github.com/mozilla/glean/pull/2538))
* Kotlin
  * Move running of upload task when Glean is running in a background service to use the internal Glean Dispatchers rather than WorkManager. [Bug 1844533](https://bugzilla.mozilla.org/show_bug.cgi?id=1844533)

# v53.1.0 (2023-06-28)

[Full changelog](https://github.com/mozilla/glean/compare/v53.0.0...v53.1.0)

* General
  * Gracefully handle the waiting thread going away during shutdown ([#2503](https://github.com/mozilla/glean/pull/2503))
  * Updated to UniFFI 0.24.1 ([#2510](https://github.com/mozilla/glean/pull/2510))
  * Try blocking shutdown 10s for init to complete ([#2518](https://github.com/mozilla/glean/pull/2518))
* Android
  * Update minimum supported Java byte code generation to 17 ([#2498](https://github.com/mozilla/glean/pull/2498/))

# v53.0.0 (2023-06-07)

[Full changelog](https://github.com/mozilla/glean/compare/v52.7.0...v53.0.0)

* General
  * Adds the capability to merge remote metric configurations, enabling multiple Nimbus Features or components to share this functionality ([Bug 1833381](https://bugzilla.mozilla.org/show_bug.cgi?id=1833381))
  * StringList metric type limits have been increased. The length of strings allowed has been increased from 50 to 100 to match the String metric type, and the list length has been increased from 20 to 100 ([Bug 1833870](https://bugzilla.mozilla.org/show_bug.cgi?id=1833870))
  * Make ping rate limiting configurable on Glean init. ([bug 1647630](https://bugzilla.mozilla.org/show_bug.cgi?id=1647630))
* Rust
  * Timing distribution traits now expose `accumulate_samples` and `accumulate_raw_samples_nanos`. This is a breaking change for consumers that make use of the trait as they will need to implement the new functions ([Bug 1829745](https://bugzilla.mozilla.org/show_bug.cgi?id=1829745))
* iOS
  * Make debugging APIs available on Swift ([#2470](https://github.com/mozilla/glean/pull/2470))
  * Added a shutdown API for Swift. This should only be necessary for when Glean is running in a process other than the main process (like in the VPN daemon, for instance)([Bug 1832324](https://bugzilla.mozilla.org/show_bug.cgi?id=1832324))
  * Glean for iOS is now being built with Xcode 14.3 ([#2253](https://github.com/mozilla/glean/pull/2253))

# v52.7.0 (2023-05-10)

[Full changelog](https://github.com/mozilla/glean/compare/v52.6.0...v52.7.0)

* General
  * Allow user to configure how verbose the internal logging is ([#2459](https://github.com/mozilla/glean/pull/2459))
  * Added a timeout waiting for the dispatcher at shutdown ([#2461](https://github.com/mozilla/glean/pull/2461))
  * Added a new Glean metric `glean.validation.shutdown_dispatcher_wait` measuring the wait time at shutdown ([#2461](https://github.com/mozilla/glean/pull/2461))
* Kotlin
  * Update Kotlin to version 1.8.21 ([#2462](https://github.com/mozilla/glean/pull/2462))
  * Make debugging APIs available on Android ([Bug 1830937](https://bugzilla.mozilla.org/show_bug.cgi?id=1830937))

# v52.6.0 (2023-04-20)

[Full changelog](https://github.com/mozilla/glean/compare/v52.5.0...v52.6.0)

* Rust
  * The Text metric type is now available in the Rust language bindings ([#2451](https://github.com/mozilla/glean/pull/2451))

# v52.5.0 (2023-04-11)

[Full changelog](https://github.com/mozilla/glean/compare/v52.4.3...v52.5.0)

* General
  * On Rkv detecting a corrupted database delete the old RKV, create a new one and log the error ([#2425](https://github.com/mozilla/glean/pull/2425))
  * Add the Date header as late as possible before the uploader acts ([#2436](https://github.com/mozilla/glean/pull/2436))
  * The logic of the Server Knobs API has been flipped. Instead of applying a list of metrics and their _disabled_ state, the API now accepts a list of metrics and their _enabled_ state ([bug 1811253](https://bugzilla.mozilla.org/show_bug.cgi?id=1811253))
* Kotlin
  * Adds the ability to record metrics on a non-main process. This is enabled by setting a `dataPath` in the Glean configuration ([bug 1815233](https://bugzilla.mozilla.org/show_bug.cgi?id=1815233))
* iOS
  * Adds the ability to record metrics on a non-main process. This is enabled by setting a `dataPath` in the Glean configuration ([bug 1815233](https://bugzilla.mozilla.org/show_bug.cgi?id=1815233))

# v52.4.3 (2023-03-24)

[Full changelog](https://github.com/mozilla/glean/compare/v52.4.2...v52.4.3)

* General
  * Expose Server Knobs functionality via UniFFI for use on mobile
* iOS
  * BUGFIX: Prevent another test-only issue: The storage going away when the uploader reports back its status ([#2430](https://github.com/mozilla/glean/pull/2430))

# v52.4.2 (2023-03-15)

[Full changelog](https://github.com/mozilla/glean/compare/v52.4.1...v52.4.2)

* Rust
  * Revert to libstd's `remove_dir_all` instead of external crate ([#2415](https://github.com/mozilla/glean/pull/2415))
* Python
  * BUGFIX: Implement an empty shutdown function ([#2417](https://github.com/mozilla/glean/pull/2417))

# v52.4.1 (2023-03-10)

[Full changelog](https://github.com/mozilla/glean/compare/v52.4.0...v52.4.1)

* General
  * Update `tempfile` crate to remove dependency on potentially vulnerable version of `remove_dir_all@0.5.3`

# v52.4.0 (2023-03-09)

[Full changelog](https://github.com/mozilla/glean/compare/v52.3.1...v52.4.0)

* General
  * Update `glean_parser` to v7.1.0 ([release notes](https://github.com/mozilla/glean_parser/releases/tag/v7.1.0))
* Kotlin
  * Upgrade Android NDK to r25c ([#2399](https://github.com/mozilla/glean/pull/2399))
* iOS
  * BUGFIX: Reworking the HTTP uploader to avoid background uploading issues ([Bug 1819161](https://bugzilla.mozilla.org/show_bug.cgi?id=1819161))

# v52.3.1 (2023-03-01)

[Full changelog](https://github.com/mozilla/glean/compare/v52.3.0...v52.3.1)

* General
  * No functional change from v52.3.0, just CI updates.

# v52.3.0 (2023-02-23)

[Full changelog](https://github.com/mozilla/glean/compare/v52.2.0...v52.3.0)

* General
  * Loosen label restrictions to "at most 71 characters of printable ASCII" ([bug 1672273](https://bugzilla.mozilla.org/show_bug.cgi?id=1672273))
  * Introduced 2 new Glean health metrics: `glean.upload.send_failure` and `glean.upload.send_success` to measure the time for sending a ping ([#2365](https://github.com/mozilla/glean/pull/2365))
  * Introduced a new Glean metric: `glean.validation.shutdown_wait` to measure the time Glean waits for the uploader on shutdown ([#2365](https://github.com/mozilla/glean/pull/2365))
* Rust
  * On shutdown wait up to 30s on the uploader to finish work ([#2232](https://github.com/mozilla/glean/pull/2332))
* iOS
  * BUGFIX: Avoid an invalid state (double-starting) for `baseline.duration` when Glean is first initialized ([#2368](https://github.com/mozilla/glean/pull/2368))

# v52.2.0 (2023-01-30)

[Full changelog](https://github.com/mozilla/glean/compare/v52.1.1...v52.2.0)

* General
  * Update to UniFFI 0.23 ([#2338](https://github.com/mozilla/glean/pull/2338))

# v52.1.1 (2023-01-26)

[Full changelog](https://github.com/mozilla/glean/compare/v52.1.0...v52.1.1)

* General
  * BUGFIX: Properly invoke the windows build number function from whatsys ([bug 1812672](https://bugzilla.mozilla.org/show_bug.cgi?id=1812672))

# v52.1.0 (2023-01-26)

[Full changelog](https://github.com/mozilla/glean/compare/v52.0.1...v52.1.0)

* General
  * BUGFIX: Custom Pings with events should no longer erroneously post `InvalidState` errors ([bug 1811872](https://bugzilla.mozilla.org/show_bug.cgi?id=1811872))
  * Upgrade to `glean_parser` v7.0.0 ([#2346](https://github.com/mozilla/glean/pull/2346))
* Kotlin
  * Update to Gradle v7.6 ([#2317](https://github.com/mozilla/glean/pull/2317))
* Rust
  * Added a new `client_info` field `windows_build_number` (Windows only) ([#2325](https://github.com/mozilla/glean/pull/2325))
  * A new `ConfigurationBuilder` allows to create the Glean configuration before initialization ([#2313](https://github.com/mozilla/glean/pull/2313))
  * Drop dependency on `env_logger` for regular builds ([#2312](https://github.com/mozilla/glean/pull/2312))

# v52.0.1 (2023-01-19)

[Full changelog](https://github.com/mozilla/glean/compare/v52.0.0...v52.0.1)

* Android
  * The `GleanDebugActivity` can run without Glean being initialized ([#2336](https://github.com/mozilla/glean/pull/2336))
* Python
  * Ship `universal2` (`aarch64` + `x86_64` in one) wheels ([#2340](https://github.com/mozilla/glean/pull/2340))

# v52.0.0 (2022-12-13)

[Full changelog](https://github.com/mozilla/glean/compare/v51.8.3...v52.0.0)

* General
  * Remove the metric `glean.validation.first_run_hour`. Note that this will mean no `reason=upgrade` metrics pings from freshly installed clients anymore. ([#2271](https://github.com/mozilla/glean/pull/2271))
  * BEHAVIOUR CHANGE: Events in Custom Pings no longer trigger their submission. ([bug 1716725](https://bugzilla.mozilla.org/show_bug.cgi?id=1716725))
    * Custom Pings with unsent events will no longer be sent at startup with reason `startup`.
    * `glean.restarted` events will be included in Custom Pings with other events to rationalize event timestamps across restarts.
  * `test_reset_glean` will remove all previous data if asked to clear stores, even if Glean never has been initialized ([#2294](https://github.com/mozilla/glean/pull/2294))
  * Upgrade to `glean_parser` v6.5.0, with support for `Cow` in Rust code ([#2300](https://github.com/mozilla/glean/issues/2300))
  * API REMOVED: The deprecated-since-v38 `event` metric `record(map)` API has been removed ([bug 1802550](https://bugzilla.mozilla.org/show_bug.cgi?id=1802550))
  * BEHAVIOUR CHANGE: "events" pings will no longer be sent if they have metrics but no events ([bug 1803513](https://bugzilla.mozilla.org/show_bug.cgi?id=1803513))
  * *_Experimental:_* Add functionality necessary to remotely configure the metric `disabled` property ([bug 1798919](https://bugzilla.mozilla.org/show_bug.cgi?id=1798919))
    * This change has no effect when the API is not used and is transparent to consumers. The API is currently experimental because it is not stable and may change.
* Rust
  * Static labels for labeled metrics are now `Cow<'static, str>` to reduce heap allocations ([#2272](https://github.com/mozilla/glean/pull/2272))
  * NEW INTERNAL CONFIGURATION OPTION: `trim_data_to_registered_pings` will trim event storage to just the registered pings. Consult with the Glean Team before using. ([bug 1804915](https://bugzilla.mozilla.org/show_bug.cgi?id=1804915))

# v51.8.3 (2022-11-25)

[Full changelog](https://github.com/mozilla/glean/compare/v51.8.2...v51.8.3)

* General
  * Upgrade to rkv 0.18.3. This comes with a bug fix that ensures that interrupted database writes don't corrupt/truncate the database file ([#2288](https://github.com/mozilla/glean/pull/2288))
* iOS
  * Avoid building a dynamic library ([#2285](https://github.com/mozilla/glean/pull/2285)).
    Note: v51.8.1 and 51.8.2 are **not** working on iOS and will break the build due to accidentally including a link to a dynamic library.

# v51.8.2 (2022-11-17)

[Full changelog](https://github.com/mozilla/glean/compare/v51.8.1...v51.8.2)

* General
  * BUGFIX: Reliably clear pending pings and events on Windows using `remove_dir_all` crate ([bug 1801128](https://bugzilla.mozilla.org/show_bug.cgi?id=1801128))
  * Update to rkv v0.18.2 ([#2270](https://github.com/mozilla/glean/pull/2270))

# v51.8.1 (2022-11-15)

[Full changelog](https://github.com/mozilla/glean/compare/v51.8.0...v51.8.1)

* General
  * Do not serialize `count` field in distribution payload ([#2267](https://github.com/mozilla/glean/pull/2267))
  * BUGFIX: The glean-core "metrics" ping scheduler will now schedule and send "upgrade"-reason pings. ([bug 1800646](https://bugzilla.mozilla.org/show_bug.cgi?id=1800646))

# v51.8.0 (2022-11-03)

[Full changelog](https://github.com/mozilla/glean/compare/v51.7.0...v51.8.0)

* General
  * Upgrade to `glean_parser` v6.3.0, increases the event extra limit to 15 ([#2255](https://github.com/mozilla/glean/issues/2255))
  * Increase event extras value limit to 500 bytes ([#2255](https://github.com/mozilla/glean/issues/2255))
* Kotlin
  * Increase to Android target/compile SDK version 33 ([#2246](https://github.com/mozilla/glean/pull/2246))
* iOS
  * Try to avoid a crash by not invalidating upload sessions ([#2254](https://github.com/mozilla/glean/pull/2254))

# v51.7.0 (2022-10-25)

[Full changelog](https://github.com/mozilla/glean/compare/v51.6.0...v51.7.0)

* iOS
  * Glean for iOS is now being built with Xcode 13.4 again ([#2242](https://github.com/mozilla/glean/pull/2242))
* Rust
  * Add cargo feature `preinit_million_queue` to up the preinit queue length from 10^3 to 10^6 ([bug 1796258](https://bugzilla.mozilla.org/show_bug.cgi?id=1796258))

# v51.6.0 (2022-10-24)

[Full changelog](https://github.com/mozilla/glean/compare/v51.5.0...v51.6.0)

* General
  * The internal glean-core dispatch queue changed from `bounded` to `unbounded`, while still behaving as a bounded queue.
* iOS
  * BUGFIX: Additional work to address an iOS crash due to an invalidated session ([#2235](https://github.com/mozilla/glean/pull/2235))

# v51.5.0 (2022-10-18)

[Full changelog](https://github.com/mozilla/glean/compare/v51.4.0...v51.5.0)

* General
  * Add `count` to `DistributionData` payload ([#2196](https://github.com/mozilla/glean/pull/2196))
  * Update to UniFFI 0.21.0 ([#2229](https://github.com/mozilla/glean/pull/2229))
* Android
  * Synchronize AndroidX dependencies with AC ([#2219](https://github.com/mozilla/glean/pull/2219))
  * Bump `jna` to 5.12.1 #2221 ([#2221](https://github.com/mozilla/glean/pull/2221))
* iOS
  * Glean for iOS is now being built with Xcode 14.0 ([#2188](https://github.com/mozilla/glean/pull/2188))

# v51.4.0 (2022-10-04)

[Full changelog](https://github.com/mozilla/glean/compare/v51.3.0...v51.4.0)

* Kotlin
  * Update Kotlin and Android Gradle Plugin to the latest releases ([#2211](https://github.com/mozilla/glean/pull/2211))
* Swift
  * Fix for iOS startup crash caused by Glean ([#2206](https://github.com/mozilla/glean/pull/2206))

# v51.3.0 (2022-09-28)

[Full changelog](https://github.com/mozilla/glean/compare/v51.2.0...v51.3.0)

* General
  * Update URL metric character limit to 8k to support longer URLs. URLs that are too long now are truncated to `MAX_URL_LENGTH` and still recorded along with an Overflow error. ([#2199](https://github.com/mozilla/glean/pull/2199))
* Kotlin
  * Gradle plugin: Fix quoting issue in Python wrapper code ([#2193](https://github.com/mozilla/glean/pull/2193))
  * Bumped the required Android NDK to version 25.1.8937393 ([#2195](https://github.com/mozilla/glean/pull/2195))

# v51.2.0 (2022-09-08)

[Full changelog](https://github.com/mozilla/glean/compare/v51.1.0...v51.2.0)

* General
  * Relax `glean_parser` version requirement. All "compatible releases" are now allowed ([#2086](https://github.com/mozilla/glean/pull/2086))
  * Uploaders can now signal that they can't upload anymore ([#2136](https://github.com/mozilla/glean/pull/2136))
  * Update UniFFI to version 0.19.6 ([#2175](https://github.com/mozilla/glean/pull/2175))
* Kotlin
  * BUGFIX: Re-enable correctly collecting `glean.validation.foreground_count` again ([#2153](https://github.com/mozilla/glean/pull/2153))
  * BUGFIX: Gradle plugin: Correctly remove the version conflict check. Now the consuming module need to ensure it uses a single version across all dependencies ([#2155](https://github.com/mozilla/glean/pull/2155))
  * Upgrade dependencies and increase to Android target/compile SDK version 32 ([#2150](https://github.com/mozilla/glean/pull/2150))
  * Upgrade Android NDK to r25 ([#2159](https://github.com/mozilla/glean/pull/2159))
  * BUGFIX: Correctly set `os_version` and `architecture` again ([#2174](https://github.com/mozilla/glean/pull/2174))
* iOS
  * BUGFIX: Correctly set `os_version` and `architecture` again ([#2174](https://github.com/mozilla/glean/pull/2174))
* Python
  * BUGFIX: Correctly handle every string that represents a UUID, including non-hyphenated random 32-character strings ([#2182](https://github.com/mozilla/glean/pull/2182))

# v51.1.0 (2022-08-08)

[Full changelog](https://github.com/mozilla/glean/compare/v51.0.1...v51.1.0)

* General
  * BUGFIX: Handle that Glean might be uninitialized when an upload task is requested ([#2131](https://github.com/mozilla/glean/pull/2131))
  * Updated the glean_parser to version 6.1.2
* Kotlin
  * BUGFIX: When setting a local endpoint in testing check for testing mode, not initialization ([#2145](https://github.com/mozilla/glean/pull/2145/))
  * Gradle plugin: Remove the version conflict check. Now the consuming module need to ensure it uses a single version across all dependencies ([#2143](https://github.com/mozilla/glean/pull/2143))

# v51.0.1 (2022-07-26)

[Full changelog](https://github.com/mozilla/glean/compare/v51.0.0...v51.0.1)

* General
  * BUGFIX: Set the following `client_info` fields correctly again: `android_sdk_version`, `device_manufacturer`, `device_model`, `locale`. These were never set in Glean v50.0.0 to v51.0.0 ([#2131](https://github.com/mozilla/glean/pull/2131))

# v51.0.0 (2022-07-22)

[Full changelog](https://github.com/mozilla/glean/compare/v50.1.2...v51.0.0)

* General
  * Remove `testHasValue` from all implementations.
    `testGetValue` always returns a null value
    (`null`, `nil`, `None` depending on the language) and does not throw an exception ([#2087](https://github.com/mozilla/glean/pull/2087)).
  * BREAKING CHANGE: Dropped `ping_name` argument from all `test_get_num_recorded_errors` methods ([#2088](https://github.com/mozilla/glean/pull/2088))  
    Errors default to the `metrics` ping, so that's what is queried internally.
  * BREAKING: Disable `safe-mode` everywhere. This causes all clients to migrate from LMDB to safe-mode storage ([#2123](https://github.com/mozilla/glean/pull/2123))
* Kotlin
  * Fix the Glean Gradle Plugin to work with Android Gradle Plugin v7.2.1 ([#2114](https://github.com/mozilla/glean/pull/2114))
* Rust
  * Add a method to construct an Event with runtime-known allowed extra keys. ([bug 1767037](https://bugzilla.mozilla.org/show_bug.cgi?id=1767037))

# v50.1.4 (2022-08-01)

[Full changelog](https://github.com/mozilla/glean/compare/v50.1.3...v50.1.4)

* General
  * BUGFIX: Handle that Glean might be uninitialized when an upload task is requested ([#2131](https://github.com/mozilla/glean/pull/2131))

# v50.1.3 (2022-07-26)

[Full changelog](https://github.com/mozilla/glean/compare/v50.1.2...v50.1.3)

* General
  * BUGFIX: Set the following `client_info` fields correctly again: `android_sdk_version`, `device_manufacturer`, `device_model`, `locale`. These were never set in Glean v50.0.0 to v51.0.0 ([#2131](https://github.com/mozilla/glean/pull/2131))


# v50.1.2 (2022-07-08)

[Full changelog](https://github.com/mozilla/glean/compare/v50.1.1...v50.1.2)

* General
  * Update UniFFI to version 0.19.3
  * Fix rust-beta-tests linting

# v50.1.1 (2022-06-17)

[Full changelog](https://github.com/mozilla/glean/compare/v50.1.0...v50.1.1)

* Kotlin
  * Fix bug in Glean Gradle plugin by using correct quoting in embedded Python script ([#2097](https://github.com/mozilla/glean/pull/2097))
  * Fix bug in Glean Gradle plugin by removing references to Linux paths ([#2098](https://github.com/mozilla/glean/pull/2098))

# v50.1.0 (2022-06-15)

[Full changelog](https://github.com/mozilla/glean/compare/v50.0.1...v50.1.0)

* General
  * Updated to `glean_parser` v6.1.1 ([#2092](https://github.com/mozilla/glean/pull/2092))
* Swift
  * Dropped usage of Carthage for internal dependencies ([#2089](https://github.com/mozilla/glean/pull/2089))
  * Implement the text metric ([#2073](https://github.com/mozilla/glean/pull/2073))
* Kotlin
  * Implement the text metric ([#2073](https://github.com/mozilla/glean/pull/2073))
* Rust
  * Derive `serde::{Deserialize, Serialize}` on `Lifetime` and `CommonMetricData` ([bug 1772156](https://bugzilla.mozilla.org/show_bug.cgi?id=1772156))

# v50.0.1 (2022-05-25)

[Full changelog](https://github.com/mozilla/glean/compare/v50.0.0...v50.0.1)

* General
  * Updated to `glean_parser` v6.0.1
* Python
  * Remove duplicate log initialization and prevent crash ([#2064](https://github.com/mozilla/glean/pull/2064))

# v50.0.0 (2022-05-20)

[Full changelog](https://github.com/mozilla/glean/compare/v44.2.0...v50.0.0)

This release is a major refactoring of the internals and contains several breaking changes to exposed APIs.
Exposed functionality should be unaffected.
See below for details.

* General
  * Switch to UniFFI-defined and -generated APIs for all 3 foreign-language SDKs
  * The task dispatcher has been moved to Rust for all foreign-language SDKs
  * Updated to `glean_parser` v6.0.0
* Swift
  * `testGetValue` on all metric types now returns `nil` when no data is recorded instead of throwing an exception.
  * `testGetValue` on metrics with more complex data now return new objects for inspection.
    See the respective documentation for details.
  * `testHasValue` on all metric types is deprecated.
    It is currently still available as extension methods.
    Use `testGetValue` with not-null checks.
* Kotlin
  * `testGetValue` on all metric types now returns `null` when no data is recorded instead of throwing an exception.
  * `testGetValue` on metrics with more complex data now return new objects for inspection.
    See the respective documentation for details.
  * `testHasValue` on all metric types is deprecated.
    It is currently still available as extension methods and thus require an additional import. Use `testGetValue` with not-null checks.
  * On `TimingDistributionMetric`, `CustomDistributionMetric`, `MemoryDistributionMetric` the `accumulateSamples` method now takes a `List<Long>` instead of `LongArray`.
    Use `listOf` instead of `longArrayOf` or call `.toList`
 * `TimingDistributionMetricType.start` now always returns a valid `TimerId`, `TimingDistributionMetricType.stopAndAccumulate` always requires a `TimerId`.
* Python
  * `test_get_value` on all metric types now returns `None` when no data is recorded instead of throwing an exception.
  * `test_has_value` on all metric types was removed.
    Use `test_get_value` with not-null checks.

# v44.2.0 (2022-05-16)

[Full changelog](https://github.com/mozilla/glean/compare/v44.1.1...v44.2.0)

* General
  * The `glean.error.preinit_tasks_overflow` metric now reports only the number of overflowing tasks.
    It is marked as version 1 in the definition now. ([#2026](https://github.com/mozilla/glean/pull/2026))
* Kotlin
  * (Development only) Allow to override the used `glean_parser` in the Glean Gradle Plugin ([#2029](https://github.com/mozilla/glean/pull/2029))
  * `setSourceTags` is now a public API ([#2035](https://github.com/mozilla/glean/pull/2035)))
* iOS
  * `setSourceTags` is now a public API ([#2035](https://github.com/mozilla/glean/pull/2035))
* Rust
  * Implemented `try_get_num_recorded_errors` for Boolean in Rust Language Bindings ([#2049](https://github.com/mozilla/glean/pull/2049))

# v44.1.1 (2022-04-14)

[Full changelog](https://github.com/mozilla/glean/compare/v44.1.0...v44.1.1)

* Rust
  * Raise the global dispatcher queue limit from 100 to 1000 tasks. ([bug 1764549](https://bugzilla.mozilla.org/show_bug.cgi?id=1764549))
* iOS
  * Enable expiry by version in the `sdk_generator.sh` script ([#2013](https://github.com/mozilla/glean/pull/2013))

# v44.1.0 (2022-04-06)

[Full changelog](https://github.com/mozilla/glean/compare/v44.0.0...v44.1.0)

* Android
  * The `glean-native-forUnitTests` now ships with separate libraries for macOS x86_64 and macOS aarch64 ([#1967](https://github.com/mozilla/glean/pull/1967))
* Rust
  * Glean will no longer overwrite the `User-Agent` header, but instead send that information as `X-Telemetry-Agent` ([bug 1711928](https://bugzilla.mozilla.org/show_bug.cgi?id=1711928))

# v44.0.0 (2022-02-09)

* General
  * BREAKING CHANGE: Updated `glean_parser` version to 5.0.1 ([#1852](https://github.com/mozilla/glean/pull/1852)).
    This update drops support for generating C# specific metrics API.
* Rust
  * Ensure test-only `destroy_glean()` handles `initialize()` having started but not completed ([bug 1750235](https://bugzilla.mozilla.org/show_bug.cgi?id=1750235))
* Swift
  * Dropping support of the Carthage-compatible framework archive ([#1943](https://github.com/mozilla/glean/pull/1943)).
    The Swift Package (https://github.com/mozilla/glean-swift) is the recommended way of consuming Glean iOS.
* Python
  * BUGFIX: Datetime metrics now correctly record the local timezone ([#1953](https://github.com/mozilla/glean/pull/1953)).

[Full changelog](https://github.com/mozilla/glean/compare/v43.0.2...v44.0.0)

# v43.0.2 (2022-01-17)

[Full changelog](https://github.com/mozilla/glean/compare/v43.0.1...v43.0.2)

* General
  * Fix artifact publishing properly ([#1930](https://github.com/mozilla/glean/pull/1930))

# v43.0.1 (2022-01-17)

[Full changelog](https://github.com/mozilla/glean/compare/v43.0.0...v43.0.1)

* General
  * Fix artifact publishing ([#1930](https://github.com/mozilla/glean/pull/1930))

# v43.0.0 (2022-01-17)

[Full changelog](https://github.com/mozilla/glean/compare/v42.3.2...v43.0.0)

* General
  * Removed `invalid_timezone_offset` metric ([#1923](https://github.com/mozilla/glean/pull/1923))
* Python
  * It is now possible to emit log messages from the networking subprocess by using the new `log_level` parameter to `Glean.initialize`. ([#1918](https://github.com/mozilla/glean/pull/1918))
* Kotlin
  * Automatically pass build date as part of the build info ([#1917](https://github.com/mozilla/glean/pull/1917))
* iOS
  * BREAKING CHANGE: Pass build info into `initialize`, which contains the build date ([#1917](https://github.com/mozilla/glean/pull/1917)).
    A suitable instance is generated by `glean_parser` in `GleanMetrics.GleanBuild.info`.

# v42.3.2 (2021-12-15)

[Full changelog](https://github.com/mozilla/glean/compare/v42.3.1...v42.3.2)

* Python
  * Reuse existing environment when launching subprocess ([#1908](https://github.com/mozilla/glean/pull/1908))

# v42.3.1 (2021-12-07)

[Full changelog](https://github.com/mozilla/glean/compare/v42.3.0...v42.3.1)

* iOS
  * Fix Carthage archive release ([#1891](https://github.com/mozilla/glean/pull/1891))

# v42.3.0 (2021-12-07)

[Full changelog](https://github.com/mozilla/glean/compare/v42.2.0...v42.3.0)

* Rust
  * BUGFIX: Correct category & name for `preinit_tasks_overflow` metric. Previously it would have been wrongly recorded as `preinit_tasks_overflow.glean.error` ([#1887](https://github.com/mozilla/glean/pull/1887))
  * BUGFIX: Fix to name given to the events ping when instantiated ([#1885](https://github.com/mozilla/glean/pull/1885))
* iOS
  * BUGFIX: Make fields of `RecordedEventData` publicly accessible ([#1867](https://github.com/mozilla/glean/pull/1867))
  * Skip code generation in `indexbuild` build ([#1889](https://github.com/mozilla/glean/pull/1889))
* Python
  * Don't let environment affect subprocess module search path ([#1542](https://github.com/mozilla/glean/pull/1542))

# v42.2.0 (2021-11-03)

[Full changelog](https://github.com/mozilla/glean/compare/v42.1.0...v42.2.0)

* General
  * Updated `glean_parser` version to 4.3.1 ([#1852](https://github.com/mozilla/glean/pull/1852))
* Android
  * Automatic detection of `tags.yaml` files ([#1852](https://github.com/mozilla/glean/pull/1852))

# v42.1.0 (2021-10-18)

[Full changelog](https://github.com/mozilla/glean/compare/v42.0.1...v42.1.0)

* Rust
  * Backwards-compatible API Change: Make experiment test APIs public. ([#1834](https://github.com/mozilla/glean/pull/1834))

# v42.0.1 (2021-10-11)

[Full changelog](https://github.com/mozilla/glean/compare/v42.0.0...v42.0.1)

* General
  * BUGFIX: Avoid a crash when accessing labeled metrics by caching created objects ([#1823](https://github.com/mozilla/glean/pull/1823)).
* Python
  * Glean now officially supports Python 3.10 ([#1818](https://github.com/mozilla/glean/pull/1818))

# v42.0.0 (2021-10-06)

[Full changelog](https://github.com/mozilla/glean/compare/v41.1.1...v42.0.0)

* Android
  * Updated to Gradle 7, Android Gradle Plugin 7 and Rust Android Plugin 0.9 as well as building with Java 11 ([#1801](https://github.com/mozilla/glean/pull/1801))
* iOS
  * Add support for the URL metric type ([#1791](https://github.com/mozilla/glean/pull/1791))
  * Remove reliance on `Operation` for uploading and instead use the background capabilities of `URLSession` ([#1783](https://github.com/mozilla/glean/pull/1783))
  * Glean for iOS is now being built with Xcode 13.0.0 ([#1802](https://github.com/mozilla/glean/pull/1802)).
* Rust
  * BUGFIX: No panic if trying to flush ping-lifetime data after shutdown ([#1800](https://github.com/mozilla/glean/pull/1800))
  * BREAKING CHANGE: `glean::persist_ping_lifetime_data` is now async ([#1812](https://github.com/mozilla/glean/pull/1812))

# v41.1.1 (2021-09-29)

[Full changelog](https://github.com/mozilla/glean/compare/v41.1.0...v41.1.1)

* Android
  * BUGFIX: Limit logging to Glean crates ([#1808](https://github.com/mozilla/glean/pull/1808))

# v41.1.0 (2021-09-16)

[Full changelog](https://github.com/mozilla/glean/compare/v41.0.0...v41.1.0)

* Rust
  * BUGFIX: Ensure RLB persists ping lifetime data on shutdown ([#1793](https://github.com/mozilla/glean/pull/1793))
  * Expose `persist_ping_lifetime_data` in the RLB. Consumers can call this to persist data at convenient times, data is also persisted on shutdown ([#1793](https://github.com/mozilla/glean/pull/1793))

# v41.0.0 (2021-09-13)

[Full changelog](https://github.com/mozilla/glean/compare/v40.2.0...v41.0.0)

* General
  * BUGFIX: Only clear specified storage in delayed ping io mode ([#1782](https://github.com/mozilla/glean/pull/1782))
  * Require Rust >= 1.53.0 ([#1782](https://github.com/mozilla/glean/pull/1782))
* Android
  * `Glean.initialize` now requires a `buildInfo` parameter to pass in build time version information. A suitable instance is generated by `glean_parser` in `${PACKAGE_ROOT}.GleanMetrics.GleanBuildInfo.buildInfo`. Support for not passing in a `buildInfo` object has been removed. ([#1752](https://github.com/mozilla/glean/pull/1752))

# v40.2.0 (2021-09-08)

[Full changelog](https://github.com/mozilla/glean/compare/v40.1.1...v40.2.0)

* General
  * Updated `glean_parser` version to 4.0.0
* Android
  * Add support for the URL metric type ([#1778](https://github.com/mozilla/glean/pull/1778))
* Rust
  * Add support for the URL metric type ([#1778](https://github.com/mozilla/glean/pull/1778))
* Python
  * Add support for the URL metric type ([#1778](https://github.com/mozilla/glean/pull/1778))

# v40.1.1 (2021-09-02)

[Full changelog](https://github.com/mozilla/glean/compare/v40.1.0...v40.1.1)

* iOS
  * Use 'Unknown' value if system data can't be decoded as UTF-8 ([#1769](https://github.com/mozilla/glean/pull/1769))
  * BUGFIX: Add quantity metric type to the build ([#1774](https://github.com/mozilla/glean/pull/1774)). Previous builds are unable to use quantity metrics

# v40.1.0 (2021-08-25)

[Full changelog](https://github.com/mozilla/glean/compare/v40.0.0...v40.1.0)

* Android
  * Updated to Kotlin 1.5, Android Gradle Plugin 4.2.2 and Gradle 6.7.1 ([#1747](https://github.com/mozilla/glean/pull/1747))
  * The `glean-gradle-plugin` now forces a compile failure when multiple Glean versions are detected in the build ([#1756](https://github.com/mozilla/glean/pull/1756))
  * The `glean-gradle-plugin` does not enable a `glean-native` capability on GeckoView anymore. That will be done by GeckoView directly ([#1759](https://github.com/mozilla/glean/pull/1759))

# v40.0.0 (2021-07-28)

[Full changelog](https://github.com/mozilla/glean/compare/v39.1.0...v40.0.0)

* Android
  * **Breaking Change**: Split the Glean Kotlin SDK into two packages: `glean` and `glean-native` ([#1595](https://github.com/mozilla/glean/pull/1595)).
    Consumers will need to switch to `org.mozilla.telemetry:glean-native-forUnitTests`.
    Old code in `build.gradle`:

    ```
    testImplementation "org.mozilla.telemetry:glean-forUnitTests:${project.ext.glean_version}"
    ```

    New code in `build.gradle`:

    ```
    testImplementation "org.mozilla.telemetry:glean-native-forUnitTests:${project.ext.glean_version}"
    ```
  * The `glean-gradle-plugin` now automatically excludes the `glean-native` dependency if `geckoview-omni` is also part of the build.
    Glean native functionality will be provided by the `geckoview-omni` package.
* Rust
  * The `glean-ffi` is no longer compiled as a `cdylib`. Other language SDKs consume `glean-bundle` instead as a `cdylib`.
    This doesn't affect consumers.

# v39.1.0 (2021-07-26)

[Full changelog](https://github.com/mozilla/glean/compare/v39.0.4...v39.1.0)

* General
  * Updated `glean_parser` version to 3.6.0
  * Allow Custom Distribution metric type on all platforms ([#1679](https://github.com/mozilla/glean/pull/1679))

# v39.0.4 (2021-07-26)

[Full changelog](https://github.com/mozilla/glean/compare/v39.0.3...v39.0.4)

* General
  * Extend `invalid_timezone_offset` metric until the end of the year ([#1697](https://github.com/mozilla/glean/pull/1697))

# v39.0.3 (2021-06-09)

[Full changelog](https://github.com/mozilla/glean/compare/v39.0.2...v39.0.3)

* Android
  * Unbreak Event#record API by accepting `null` on the deprecated API.
    The previous 39.0.0 release introduced the new API, but accidentally broke certain callers that just forward arguments.
    This restores passing `null` (or nothing) when using the old API. It remains deprecated.

# v39.0.2 (2021-06-07)

[Full changelog](https://github.com/mozilla/glean/compare/v39.0.1...v39.0.2)

* iOS
  * Fix iOS release build ([#1668](https://github.com/mozilla/glean/pull/1668), [#1669](https://github.com/mozilla/glean/pull/1669))

# v39.0.1 (2021-06-04)

[Full changelog](https://github.com/mozilla/glean/compare/v39.0.0...v39.0.1)

* iOS
  * Build and release Glean as an xcframework ([#1663](https://github.com/mozilla/glean/pull/1663))
    This will now also auto-update the Glean package at https://github.com/mozilla/glean-swift.

# v39.0.0 (2021-05-31)

[Full changelog](https://github.com/mozilla/glean/compare/v38.0.1...v39.0.0)

* General
  * Add new event extras API to all implementations. See below for details ([#1603](https://github.com/mozilla/glean/pull/1603))
  * Updated `glean_parser` version to 3.4.0 ([#1603](https://github.com/mozilla/glean/pull/1603))
* Rust
  * **Breaking Change**: Allow event extras to be passed as an object.
    This replaces the old `HashMap`-based API.
    Values default to `string`.
    See [the event documentation](https://mozilla.github.io/glean/book/reference/metrics/event.html#recordobject) for details.
    ([#1603](https://github.com/mozilla/glean/pull/1603))
    Old code:

    ```
    let mut extra = HashMap::new();
    extra.insert(SomeExtra::Key1, "1".into());
    extra.insert(SomeExtra::Key2, "2".into());
    metric.record(extra);
    ```

    New code:

    ```
    let extra = SomeExtra {
        key1: Some("1".into()),
        key2: Some("2".into()),
    };
    metric.record(extra);
    ```
* Android
  * **Deprecation**: The old event recording API is replaced by a new one, accepting a typed object ([#1603](https://github.com/mozilla/glean/pull/1603)).
    See [the event documentation](https://mozilla.github.io/glean/book/reference/metrics/event.html#recordobject) for details.
  * Skip build info generation for libraries ([#1654](https://github.com/mozilla/glean/pull/1654))
* Python
  * **Deprecation**: The old event recording API is replaced by a new one, accepting a typed object ([#1603](https://github.com/mozilla/glean/pull/1603)).
    See [the event documentation](https://mozilla.github.io/glean/book/reference/metrics/event.html#recordobject) for details.
* Swift
  * **Deprecation**: The old event recording API is replaced by a new one, accepting a typed object ([#1603](https://github.com/mozilla/glean/pull/1603)).
    See [the event documentation](https://mozilla.github.io/glean/book/reference/metrics/event.html#recordobject) for details.

# v38.0.1 (2021-05-17)

[Full changelog](https://github.com/mozilla/glean/compare/v38.0.0...v38.0.1)

* General
  * BUGFIX: Invert the lock order in glean-core's metrics ping scheduler ([#1637](https://github.com/mozilla/glean/pull/1637))

# v38.0.0 (2021-05-12)

[Full changelog](https://github.com/mozilla/glean/compare/v37.0.0...v38.0.0)

* General
  * Update documentation to recommend using Glean Dictionary instead of metrics.md ([#1604](https://github.com/mozilla/glean/pull/1604))
* Rust
  * **Breaking Change**: Don't return a result from `submit_ping`. The boolean return value indicates whether a ping was submitted ([#1613](https://github.com/mozilla/glean/pull/1613))
  * **Breaking Change**: Glean now schedules "metrics" pings, accepting a new Configuration parameter. ([#1599](https://github.com/mozilla/glean/pull/1599))
  * Dispatch setting the source tag to avoid a potential crash ([#1614](https://github.com/mozilla/glean/pull/1614))
  * Testing mode will wait for init & upload tasks to finish ([#1628](https://github.com/mozilla/glean/pull/1628))
* Android
  * Set required fields for `client_info` before optional ones ([#1633](https://github.com/mozilla/glean/pull/1633))
  * Provide forward-compatibility with Gradle 6.8 ([#1616](https://github.com/mozilla/glean/pull/1633))

# v37.0.0 (2021-04-30)

[Full changelog](https://github.com/mozilla/glean/compare/v36.0.1...v37.0.0)

* General
  * **Breaking Change**: "deletion-request" pings now include the reason upload was disabled: `at_init` (Glean detected a change between runs) or `set_upload_enabled` (Glean was told of a change as it happened). ([#1593](https://github.com/mozilla/glean/pull/1593)).
  * Attempt to upload a ping even in the face of IO Errors ([#1576](https://github.com/mozilla/glean/pull/1576)).
  * Implement an additional check to avoid crash due to faulty timezone offset ([#1581](https://github.com/mozilla/glean/pull/1581))
    * This now records a new metric `glean.time.invalid_timezone_offset`, counting how often we failed to get a valid timezone offset.
  * Use proper paths throughout to hopefully handle non-UTF-8 paths more gracefully ([#1596](https://github.com/mozilla/glean/pull/1596))
  * Updated `glean_parser` version to 3.2.0 ([#1609](https://github.com/mozilla/glean/pull/1608))
* iOS
  * Code generator: Ensure at least pip 20.3 is available in iOS build ([#1590](https://github.com/mozilla/glean/pull/1590))

# v36.0.1 (2021-04-09)

[Full changelog](https://github.com/mozilla/glean/compare/v36.0.0...v36.0.1)

* RLB
  * Provide an internal-use-only API to pass in raw samples for timing distributions ([#1561](https://github.com/mozilla/glean/pull/1561)).
  * Expose Timespan's `set_raw` to Rust ([#1578](https://github.com/mozilla/glean/pull/1578)).
* Android
  * BUGFIX: `TimespanMetricType.measure` and `TimingDistributionMetricType.measure` won't get inlined anymore ([#1560](https://github.com/mozilla/glean/pull/1560)).
    This avoids a potential bug where a `return` used inside the closure would end up not measuring the time.
    Use `return@measure <val>` for early returns.
* Python
  * The Glean Python bindings now use rkv's safe mode backend. This should avoid intermittent segfaults in the LMDB backend.

# v36.0.0 (2021-03-16)

[Full changelog](https://github.com/mozilla/glean/compare/v35.0.0...v36.0.0)

* General
  * Introduce a new API `Ping#test_before_next_submit` to run a callback right before a custom ping is submitted ([#1507](https://github.com/mozilla/glean/pull/1507)).
    * The new API exists for all language bindings (Kotlin, Swift, Rust, Python).
  * Updated `glean_parser` version to 2.5.0
  * Change the `fmt-` and `lint-` make commands for consistency ([#1526](https://github.com/mozilla/glean/pull/1526))
  * The Glean SDK can now produce testing coverage reports for your metrics ([#1482](https://github.com/mozilla/glean/pull/1482/files)).
* Python
  * Update minimal required version of `cffi` dependency to 1.13.0 ([#1520](https://github.com/mozilla/glean/pull/1520)).
  * Ship wheels for arm64 macOS ([#1534](https://github.com/mozilla/glean/pull/1534)).
* RLB
  * Added `rate` metric type ([#1516](https://github.com/mozilla/glean/pull/1516)).
  * Set `internal_metrics::os_version` for MacOS, Windows and Linux ([#1538](https://github.com/mozilla/glean/pull/1538))
  * Expose a function `get_timestamp_ms` to get a timestamp from a monotonic clock on all supported operating systems, to be used for event timestamps ([#1546](https://github.com/mozilla/glean/pull/1546)).
  * Expose a function to record events with an externally provided timestamp.
* iOS
  * **Breaking Change**: Event timestamps are now correctly recorded in milliseconds ([#1546](https://github.com/mozilla/glean/pull/1546)).
    * Since the first release event timestamps were erroneously recorded with nanosecond precision ([#1549](https://github.com/mozilla/glean/pull/1549)).
      This is now fixed and event timestamps are in milliseconds.
      This is equivalent to how it works in all other language bindings.

# v35.0.0 (2021-02-22)

[Full changelog](https://github.com/mozilla/glean/compare/v34.1.0...v35.0.0)

* Android
  * `Glean.initialize` can now take a `buildInfo` parameter to pass in build time version information, and avoid calling out to the Android package manager at runtime. A suitable instance is generated by `glean_parser` in `${PACKAGE_ROOT}.GleanMetrics.GleanBuildInfo.buildInfo` ([#1495](https://github.com/mozilla/glean/pull/1495)). Not passing in a `buildInfo` object is still supported, but is deprecated.
  * The `testGetValue` APIs now include a message on the `NullPointerException` thrown when the value is missing.
  * **Breaking change:** `LEGACY_TAG_PINGS` is removed from `GleanDebugActivity` ([#1510](https://github.com/mozilla/glean/pull/1510))
* RLB
  * **Breaking change:** `Configuration.data_path` is now a `std::path::PathBuf`([#1493](https://github.com/mozilla/glean/pull/1493)).

# v34.1.0 (2021-02-04)

[Full changelog](https://github.com/mozilla/glean/compare/v34.0.0...v34.1.0)

* General
  * A new metric `glean.validation.pings_submitted` tracks the number of pings sent. It is included in both the `metrics` and `baseline` pings.
* iOS
  * The metric `glean.validation.foreground_count` is now sent in the metrics ping ([#1472](https://github.com/mozilla/glean/pull/1472)).
  * BUGFIX: baseline pings with reason `dirty_startup` are no longer sent if Glean did not full initialize in the previous run ([#1476](https://github.com/mozilla/glean/pull/1476)).
* Python
  * Expose the client activity API ([#1481](https://github.com/mozilla/glean/pull/1481)).
  * BUGFIX: Publish a macOS wheel again. The previous release failed to build a Python wheel for macOS platforms ([#1471](https://github.com/mozilla/glean/pull/1471)).
* RLB
  * BUGFIX: baseline pings with reason `dirty_startup` are no longer sent if Glean did shutdown cleanly ([#1483](https://github.com/mozilla/glean/pull/1483)).

# v34.0.0 (2021-01-29)

[Full changelog](https://github.com/mozilla/glean/compare/v33.10.3...v34.0.0)

* General
  * Other bindings detect when RLB is used and try to flush the RLB dispatcher to unblock the Rust API ([#1442](https://github.com/mozilla/glean/pull/1442)).
    * This is detected automatically, no changes needed for consuming code.
  * Add support for the client activity API ([#1455](https://github.com/mozilla/glean/pull/1455)). This API is either automatically used or exposed by the language bindings.
  * Rename the reason `background` to `inactive` for both the `baseline` and `events` ping. Rename the reason `foreground` to `active` for the `baseline` ping.
* RLB
  * When the pre-init task queue overruns, this is now recorded in the metric `glean.error.preinit_tasks_overflow` ([#1438](https://github.com/mozilla/glean/pull/1438)).
  * Expose the client activity API ([#1455](https://github.com/mozilla/glean/pull/1455)).
  * Send the `baseline` ping with reason `dirty_startup`, if needed, at startup.
  * Expose all required types directly ([#1452](https://github.com/mozilla/glean/pull/1452)).
    * Rust consumers will not need to depend on `glean-core` anymore.
* Android
  * BUGFIX: Don't crash the ping uploader when throttled due to reading too large wait time values ([#1454](https://github.com/mozilla/glean/pull/1454)).
  * Use the client activity API ([#1455](https://github.com/mozilla/glean/pull/1455)).
  * Update `AndroidX` dependencies ([#1441](https://github.com/mozilla/glean/pull/1441)).
* iOS
  * Use the client activity API ([#1465](https://github.com/mozilla/glean/pull/1465)).
    Note: this now introduces a baseline ping with reason `active` on startup.

# v33.10.3 (2021-01-18)

[Full changelog](https://github.com/mozilla/glean/compare/v33.10.2...v33.10.3)

* Rust
  * Upgrade rkv to 0.17 ([#1434](https://github.com/mozilla/glean/pull/1434))

# v33.10.2 (2021-01-15)

[Full changelog](https://github.com/mozilla/glean/compare/v33.10.1...v33.10.2)

* General:
  * A new metric `glean.error.io` has been added, counting the times an IO error happens when writing a pending ping to disk ([#1428](https://github.com/mozilla/glean/pull/1428))
* Android
  * A new metric `glean.validation.foreground_count` was added to the metrics ping ([#1418](https://github.com/mozilla/glean/pull/1418)).
* Rust
  * BUGFIX: Fix lock order inversion in RLB Timing Distribution ([#1431](https://github.com/mozilla/glean/pull/1431)).
  * Use RLB types instead of glean-core ones for RLB core metrics. ([#1432](https://github.com/mozilla/glean/pull/1432)).

# v33.10.1 (2021-01-06)

[Full changelog](https://github.com/mozilla/glean/compare/v33.10.0...v33.10.1)

No functional changes. v33.10.0 failed to generated iOS artifacts due to broken tests ([#1421](https://github.com/mozilla/glean/pull/1421)).

# v33.10.0 (2021-01-06)

[Full changelog](https://github.com/mozilla/glean/compare/v33.9.1...v33.10.0)

* General
  * A new metric `glean.validation.first_run_hour`, analogous to the existing `first_run_date` but with hour resolution, has been added. Only clients running the app for the first time after this change will report this metric ([#1403](https://github.com/mozilla/glean/pull/1403)).
* Rust
  * BUGFIX: Don't require mutable references in RLB traits ([#1417](https://github.com/mozilla/glean/pull/1417)).
* Python
  * Building the Python package from source now works on musl-based Linux distributions, such as Alpine Linux ([#1416](https://github.com/mozilla/glean/pull/1416)).

# v33.9.1 (2020-12-17)

[Full changelog](https://github.com/mozilla/glean/compare/v33.9.0...v33.9.1)

* Rust
  * BUGFIX: Don't panic on shutdown and avoid running tasks if uninitialized ([#1398](https://github.com/mozilla/glean/pull/1398)).
  * BUGFIX: Don't fail on empty database files ([#1398](https://github.com/mozilla/glean/pull/1398)).
  * BUGFIX: Support ping registration before Glean initializes ([#1393](https://github.com/mozilla/glean/pull/1393)).

# v33.9.0 (2020-12-15)

[Full changelog](https://github.com/mozilla/glean/compare/v33.8.0...v33.9.0)

* Rust
  * Introduce the String List metric type in the RLB. ([#1380](https://github.com/mozilla/glean/pull/1380)).
  * Introduce the `Datetime` metric type in the RLB ([#1384](https://github.com/mozilla/glean/pull/1384)).
  * Introduce the `CustomDistribution` and `TimingDistribution` metric type in the RLB ([#1394](https://github.com/mozilla/glean/pull/1394)).

# v33.8.0 (2020-12-10)

[Full changelog](https://github.com/mozilla/glean/compare/v33.7.0...v33.8.0)

* Rust
  * Introduce the Memory Distribution metric type in the RLB. ([#1376](https://github.com/mozilla/glean/pull/1376)).
  * Shut down Glean in tests before resetting to make sure they don't mistakenly init Glean twice in parallel ([#1375](https://github.com/mozilla/glean/pull/1375)).
  * BUGFIX: Fixing 2 `lock-order-inversion` bugs found by TSan ([#1378](https://github.com/mozilla/glean/pull/1378)).
    * TSan runs on mozilla-central tests, which found two (potential) bugs where 2 different locks were acquired in opposite order in different code paths,
      which could lead to deadlocks in multi-threaded code. As RLB uses multiple threads (e.g. for init and the dispatcher) by default, this can easily become an actual issue.
* Python
  * All log messages from the Glean SDK are now on the `glean` logger, obtainable through `logging.getLogger("glean")`.  (Prior to this, each module had its own logger, for example `glean.net.ping_upload_worker`).

# v33.7.0 (2020-12-07)

[Full changelog](https://github.com/mozilla/glean/compare/v33.6.0...v33.7.0)

* Rust
  * Upgrade rkv to 0.16.0 (no functional changes) ([#1355](https://github.com/mozilla/glean/pull/1355)).
  * Introduce the Event metric type in the RLB ([#1361](https://github.com/mozilla/glean/pull/1361)).
* Python
  * Python Linux wheels no longer work on Linux distributions released before 2014 (they now use the manylinux2014 ABI) ([#1353](https://github.com/mozilla/glean/pull/1353)).
  * Unbreak Python on non-Linux ELF platforms (BSD, Solaris/illumos) ([#1363](https://github.com/mozilla/glean/pull/1363)).

# v33.6.0 (2020-12-02)

[Full changelog](https://github.com/mozilla/glean/compare/v33.5.0...v33.6.0)

* Rust
  * BUGFIX: Negative timespans for the timespan metric now correctly record an `InvalidValue` error ([#1347](https://github.com/mozilla/glean/pull/1347)).
  * Introduce the Timespan metric type in the RLB ([#1347](https://github.com/mozilla/glean/pull/1347)).
* Python
  * BUGFIX: Network slowness or errors will no longer block the main dispatcher thread, leaving work undone on shutdown ([#1350](https://github.com/mozilla/glean/pull/1350)).
  * BUGFIX: Lower sleep time on upload waits to avoid being stuck when the main process ends ([#1349](https://github.com/mozilla/glean/pull/1349)).

# v33.5.0 (2020-12-01)

[Full changelog](https://github.com/mozilla/glean/compare/v33.4.0...v33.5.0)

* Rust
  * Introduce the UUID metric type in the RLB.
  * Introduce the Labeled metric type in the RLB ([#1327](https://github.com/mozilla/glean/pull/1327)).
  * Introduce the Quantity metric type in the RLB.
  * Introduce the `shutdown` API.
  * Add Glean debugging APIs.
* Python
  * BUGFIX: Setting a UUID metric to a value that is not in the expected UUID format will now record an error with the Glean error reporting system.

# v33.4.0 (2020-11-17)

[Full changelog](https://github.com/mozilla/glean/compare/v33.3.0...v33.4.0)

* General
  * When Rkv's safe mode is enabled (`features = ["rkv-safe-mode"]` on the `glean-core` crate) LMDB data is migrated at first start ([#1322](https://github.com/mozilla/glean/pull/1322)).
* Rust
  * Introduce the Counter metric type in the RLB.
  * Introduce the String metric type in the RLB.
  * BUGFIX: Track the size of the database directory at startup ([#1304](https://github.com/mozilla/glean/pull/1304)).
* Python
  * BUGFIX: Fix too-long sleep time in uploader due to unit mismatch ([#1325](https://github.com/mozilla/glean/pull/1325)).
* Swift
  * BUGFIX: Fix too-long sleep time in uploader due to unit mismatch ([#1325](https://github.com/mozilla/glean/pull/1325)).

# v33.3.0 (2020-11-12)

[Full changelog](https://github.com/mozilla/glean/compare/v33.2.0...v33.3.0)

* General
  * Do not require default-features on rkv and downgrade bincode ([#1317](https://github.com/mozilla/glean/pull/1317))
  * Do not require default-features on `rkv` and downgrade `bincode` ([#1317](https://github.com/mozilla/glean/pull/1317))
* Rust
  * Implement the experiments API ([#1314](https://github.com/mozilla/glean/pull/1314))

# v33.2.0 (2020-11-10)

[Full changelog](https://github.com/mozilla/glean/compare/v33.1.2...v33.2.0)

* Python
  * Fix building of Linux wheels ([#1303](https://github.com/mozilla/glean/pull/1303))
      * Python Linux wheels no longer work on Linux distributions released before 2010. (They now use the manylinux2010 ABI, rather than the manylinux1 ABI.)
* Rust
  * Introduce the RLB `net` module ([#1292](https://github.com/mozilla/glean/pull/1292))

# v33.1.2 (2020-11-04)

[Full changelog](https://github.com/mozilla/glean/compare/v33.1.1...v33.1.2)

* No changes.  v33.1.1 was tagged incorrectly.

# v33.1.1 (2020-11-04)

[Full changelog](https://github.com/mozilla/glean/compare/v33.1.0...v33.1.1)

* No changes.  v33.1.0 was tagged incorrectly.

# v33.1.0 (2020-11-04)

[Full changelog](https://github.com/mozilla/glean/compare/v33.0.4...v33.1.0)

* General
  * Standardize throttle backoff time throughout all bindings. ([#1240](https://github.com/mozilla/glean/pull/1240))
  * Update `glean_parser` to 1.29.0
    * Generated code now includes a comment next to each metric containing the name of the metric in its original `snake_case` form.
  * Expose the description of the metric types in glean_core using traits.
* Rust
  * Add the `BooleanMetric` type.
  * Add the `dispatcher` module (copied over from [mozilla-central](https://hg.mozilla.org/mozilla-central/rev/fbe0ea62f4bb50bfc5879a56667945697b2c90e7)).
  * Allow consumers to specify a custom uploader.
* Android
  * Update the JNA dependency from 5.2.0 to 5.6.0
  * The `glean-gradle-plugin` now makes sure that only a single Miniconda installation will happen at the same time to avoid a race condition when multiple components within the same project are using Glean.

# v33.0.4 (2020-09-28)

[Full changelog](https://github.com/mozilla/glean/compare/v33.0.3...v33.0.4)

Note: Previous 33.0.z releases were broken. This release now includes all changes from 33.0.0 to 33.0.3.

* General
  * Update `glean_parser` to 1.28.6
    * BUGFIX: Ensure Kotlin arguments are deterministically ordered
* Android
  * **Breaking change:** Updated to the Android Gradle Plugin v4.0.1 and Gradle 6.5.1. Projects using older versions of these components will need to update in order to use newer versions of the Glean SDK.
  * Update the Kotlin Gradle Plugin to version 1.4.10.
  * Fixed the building of `.aar` releases on Android so they include the Rust shared objects.

# v33.0.3 (2020-09-25)

[Full changelog](https://github.com/mozilla/glean/compare/v33.0.2...v33.0.3)

* General
  * v33.0.2 was tagged incorrectly. This release is just to correct that mistake.

# v33.0.2 (2020-09-25)

[Full changelog](https://github.com/mozilla/glean/compare/v33.0.1...v33.0.2)

* Android
  * Fixed the building of `.aar` releases on Android so they include the Rust shared objects.

# v33.0.1 (2020-09-24)

[Full changelog](https://github.com/mozilla/glean/compare/v33.0.0...v33.0.1)

* General
  * Update `glean_parser` to 1.28.6
    * BUGFIX: Ensure Kotlin arguments are deterministically ordered
* Android
  * Update the Kotlin Gradle Plugin to version 1.4.10.

# v33.0.0 (2020-09-22)

[Full changelog](https://github.com/mozilla/glean/compare/v32.4.0...v33.0.0)

* Android
  * **Breaking change:** Updated to the Android Gradle Plugin v4.0.1 and Gradle 6.5.1. Projects using older versions of these components will need to update in order to use newer versions of the Glean SDK.

# v32.4.1 (2020-10-01)

[Full changelog](https://github.com/mozilla/glean/compare/v32.4.0...v32.4.1)

* General
  * Update `glean_parser` to 1.28.6
    * BUGFIX: Ensure Kotlin arguments are deterministically ordered
  * BUGFIX: Transform ping directory size from bytes to kilobytes before accumulating to `glean.upload.pending_pings_directory_size` ([#1236](https://github.com/mozilla/glean/pull/1236)).

# v32.4.0 (2020-09-18)

[Full changelog](https://github.com/mozilla/glean/compare/v32.3.2...v32.4.0)

* General
  * Allow using quantity metric type outside of Gecko ([#1198](https://github.com/mozilla/glean/pull/1198))
  * Update `glean_parser` to 1.28.5
    * The `SUPERFLUOUS_NO_LINT` warning has been removed from the glinter. It likely did more harm than good, and makes it hard to make metrics.yaml files that pass across different versions of `glean_parser`.
    * Expired metrics will now produce a linter warning, `EXPIRED_METRIC`.
    * Expiry dates that are more than 730 days (~2 years) in the future will produce a linter warning, `EXPIRATION_DATE_TOO_FAR`.
    * Allow using the Quantity metric type outside of Gecko.
    * New parser configs `custom_is_expired` and `custom_validate_expires` added. These are both functions that take the expires value of the metric and return a bool. (See `Metric.is_expired` and `Metric.validate_expires`). These will allow FOG to provide custom validation for its version-based `expires` values.
  * Add a limit of 250 pending ping files. ([#1217](https://github.com/mozilla/glean/pull/1217)).
* Android
  * Don't retry the ping uploader when waiting, sleep instead. This avoids a never-ending increase of the backoff time ([#1217](https://github.com/mozilla/glean/pull/1217)).

# v32.3.2 (2020-09-11)

[Full changelog](https://github.com/mozilla/glean/compare/v32.3.1...v32.3.2)

* General
  * Track the size of the database file at startup ([#1141](https://github.com/mozilla/glean/pull/1141)).
  * Submitting a ping with upload disabled no longer shows an error message ([#1201](https://github.com/mozilla/glean/pull/1201)).
  * BUGFIX: scan the pending pings directories **after** dealing with upload status on initialization. This is important, because in case upload is disabled we delete any outstanding non-deletion ping file, and if we scan the pending pings folder before doing that we may end up sending pings that should have been discarded. ([#1205](https://github.com/mozilla/glean/pull/1205))
* iOS
  * Disabled code coverage in release builds ([#1195](https://github.com/mozilla/glean/issues/1195)).
* Python
  * Glean now ships a source package to pip install on platforms where wheels aren't provided.

# v32.3.1 (2020-09-09)

[Full changelog](https://github.com/mozilla/glean/compare/v32.3.0...v32.3.1)

* Python
    * Fixed the release process to generate all wheels ([#1193](https://github.com/mozilla/glean/pull/1193)).

# v32.3.0 (2020-08-27)

[Full changelog](https://github.com/mozilla/glean/compare/v32.2.0...v32.3.0)

* Android
  * Handle ping registration off the main thread. This removes a potential blocking call ([#1132](https://github.com/mozilla/glean/pull/1132)).
* iOS
  * Handle ping registration off the main thread. This removes a potential blocking call ([#1132](https://github.com/mozilla/glean/pull/1132)).
  * Glean for iOS is now being built with Xcode 12.0.0 (Beta 5) ([#1170](https://github.com/mozilla/glean/pull/1170)).

# v32.2.0 (2020-08-25)

[Full changelog](https://github.com/mozilla/glean/compare/v32.1.1...v32.2.0)

* General
  * Move logic to limit the number of retries on ping uploading "recoverable failures" to glean-core. ([#1120](https://github.com/mozilla/glean/pull/1120))
    * The functionality to limit the number of retries in these cases was introduced to the Glean SDK in `v31.1.0`. The work done now was to move that logic to the glean-core in order to avoid code duplication throughout the language bindings.
  * Update `glean_parser` to `v1.28.3`
    * BUGFIX: Generate valid C# code when using Labeled metric types.
    * BUGFIX: Support `HashSet` and `Dictionary` in the C# generated code.
  * Add a 10MB quota to the pending pings storage. ([#1100](https://github.com/mozilla/glean/pull/1110))
* C#
  * Add support for the String List metric type ([#1108](https://github.com/mozilla/glean/pull/1108)).
  * Enable generating the C# APIs using the glean_parser ([#1092](https://github.com/mozilla/glean/pull/1092)).
  * Add support for the `EventMetricType` in C# ([#1129](https://github.com/mozilla/glean/pull/1129)).
  * Add support for the `TimingDistributionMetricType` in C# ([#1131](https://github.com/mozilla/glean/pull/1131)).
  * Implement the experiments API in C# ([#1145](https://github.com/mozilla/glean/pull/1145)).
  * This is the last release with C# language bindings changes. Reach out to the Glean SDK team if you want to use the C# bindings in a new product and require additional features.
* Python
  * BUGFIX: Limit the number of retries for 5xx server errors on ping uploads ([#1120](https://github.com/mozilla/glean/pull/1120)).
    * This kinds of failures yield a "recoverable error", which means the ping gets re-enqueued. That can cause infinite loops on the ping upload worker. For python we were incorrectly only limiting the number of retries for I/O errors, another type of "recoverable error".
  * `kebab-case` ping names are now converted to `snake_case` so they are available on the object returned by `load_pings` ([#1122](https://github.com/mozilla/glean/pull/1122)).
  * For performance reasons, the `glinter` is no longer run as part of `glean.load_metrics()`. We recommend running `glinter` as part of your project's continuous integration instead ([#1124](https://github.com/mozilla/glean/pull/1124)).
  * A `measure` context manager for conveniently measuring runtimes has been added to `TimespanMetricType` and `TimingDistributionMetricType` ([#1126](https://github.com/mozilla/glean/pull/1126)).
  * Networking errors have changed from `ERROR` level to `DEBUG` level so they aren't displayed by default ([#1166](https://github.com/mozilla/glean/pull/1166)).
* iOS
  * Changed logging to use [`OSLog`](https://developer.apple.com/documentation/os/logging) rather than a mix of `NSLog` and `print`. ([#1133](https://github.com/mozilla/glean/pull/1133))

# v32.1.1 (2020-08-24)

[Full changelog](https://github.com/mozilla/glean/compare/v32.1.0...v32.1.1)

* Android
  * Support installing glean_parser in offline mode ([#1065](https://github.com/mozilla/glean/pull/1065)).
  * Fix a startup crash on some Android 8 (SDK=25) devices, due to a [bug in the Java compiler](https://issuetracker.google.com/issues/110848122#comment17) ([#1135](https://github.com/mozilla/glean/pull/1135)).

# v32.1.0 (2020-08-17)

[Full changelog](https://github.com/mozilla/glean/compare/v32.0.0...v32.1.0)

* General
  * The upload rate limiter has been changed from 10 pings per minute to 15 pings per minute.

# v32.0.0 (2020-08-03)

[Full changelog](https://github.com/mozilla/glean/compare/v31.6.0...v32.0.0)

* General
  * Limit ping request body size to 1MB. ([#1098](https://github.com/mozilla/glean/pull/1098))
* iOS
  * Implement ping tagging (i.e. the `X-Source-Tags` header) through custom URL ([#1100](https://github.com/mozilla/glean/pull/1100)).
* C#
  * Add support for Labeled Strings and Labeled Booleans.
  * Add support for the Counter metric type and Labeled Counter.
  * Add support for the `MemoryDistributionMetricType`.
* Python
  * **Breaking change:** `data_dir` must always be passed to `Glean.initialize`. Prior to this, a missing value would store Glean data in a temporary directory.
  * Logging messages from the Rust core are now sent through Python's standard library `logging` module. Therefore all logging in a Python application can be controlled through the `logging` module interface.
* Android
  * BUGFIX: Require activities executed via `GleanDebugView` to be exported.

# v31.6.0 (2020-07-24)

[Full changelog](https://github.com/mozilla/glean/compare/v31.5.0...v31.6.0)

* General
  * Implement JWE metric type ([#1073](https://github.com/mozilla/glean/pull/1073), [#1062](https://github.com/mozilla/glean/pull/1062)).
  * DEPRECATION: `getUploadEnabled` is deprecated (respectively `get_upload_enabled` in Python) ([#1046](https://github.com/mozilla/glean/pull/1046))
    * Due to Glean's asynchronous initialization the return value can be incorrect.
      Applications should not rely on Glean's internal state.
      Upload enabled status should be tracked by the application and communicated to Glean if it changes.
      Note: The method was removed from the C# and Python implementation.
  * Update `glean_parser` to `v1.28.1`
    * The `glean_parser` linting was leading consumers astray by incorrectly suggesting that `deletion-request` be instead `deletion_request` when used for `send_in_pings`. This was causing metrics intended for the `deletion-request` ping to not be included when it was collected and submitted. Consumers that are sending metrics in the `deletion-request` ping will need to update the `send_in_pings` value in their metrics.yaml to correct this.
    * Fixes a bug in doc rendering.

# v31.5.0 (2020-07-22)

[Full changelog](https://github.com/mozilla/glean/compare/v31.4.1...v31.5.0)

* General
  * Implement ping tagging (i.e. the `X-Source-Tags` header) ([#1074](https://github.com/mozilla/glean/pull/1074)). Note that this is not yet implemented for iOS.
  * String values that are too long now record `invalid_overflow` rather than `invalid_value` through the Glean error reporting mechanism. This affects the string, event and string list metrics.
  * `metrics.yaml` files now support a `data_sensitivity` field to all metrics for specifying the type of data collected in the field.
* Python
  * The Python unit tests no longer send telemetry to the production telemetry endpoint.
  * BUGFIX: If an `application_version` isn't provided to `Glean.initialize`, the `client_info.app_display_version` metric is set to `"Unknown"`, rather than resulting in invalid pings.
* Android
  * Allow defining which `Activity` to run next when using the `GleanDebugActivity`.
* iOS
  * BUGFIX: The memory unit is now correctly set on the `MemoryDistribution` metric type in Swift in generated metrics code.
* C#
  * Metrics can now be generated from the `metrics.yaml` files.

# v31.4.1 (2020-07-20)

[Full changelog](https://github.com/mozilla/glean/compare/v31.4.0...v31.4.1)

* General
  * BUGFIX: fix `int32` to `ErrorType` mapping. The `InvalidOverflow` had a value mismatch between glean-core and the bindings. This would only be a problem in unit tests. ([#1063](https://github.com/mozilla/glean/pull/1063))
* Android
  * Enable propagating options to the main product Activity when using the `GleanDebugActivity`.
  * BUGFIX: Fix the metrics ping collection for startup pings such as `reason=upgrade` to occur in the same thread/task as Glean initialize. Otherwise, it gets collected after the application lifetime metrics are cleared such as experiments that should be in the ping. ([#1069](https://github.com/mozilla/glean/pull/1069))

# v31.4.0 (2020-07-16)

[Full changelog](https://github.com/mozilla/glean/compare/v31.3.0...v31.4.0)

* General
  * Enable debugging features through environment variables. ([#1058](https://github.com/mozilla/glean/pull/1058))

# v31.3.0 (2020-07-10)

[Full changelog](https://github.com/mozilla/glean/compare/v31.2.3...v31.3.0)

* General
    * Remove locale from baseline ping. ([1609968](https://bugzilla.mozilla.org/show_bug.cgi?id=1609968), [#1016](https://github.com/mozilla/glean/pull/1016))
    * Persist X-Debug-ID header on store ping. ([1605097](https://bugzilla.mozilla.org/show_bug.cgi?id=1605097), [#1042](https://github.com/mozilla/glean/pull/1042))
    * BUGFIX: raise an error if Glean is initialized with an empty string as the `application_id` ([#1043](https://github.com/mozilla/glean/pull/1043)).
* Python
    * BUGFIX: correctly set the `app_build` metric to the newly provided `application_build_id` initialization option ([#1031](https://github.com/mozilla/glean/pull/1031)).
    * The Python bindings now report networking errors in the `glean.upload.ping_upload_failure` metric (like all the other bindings) ([#1039](https://github.com/mozilla/glean/pull/1039)).
    * Python default upgraded to Python 3.8 ([#995](https://github.com/mozilla/glean/pull/995))
* iOS
    * BUGFIX: Make `LabeledMetric` subscript public, so consuming applications can actually access it ([#1027](https://github.com/mozilla/glean/pull/1027))

# v31.2.3 (2020-06-29)

[Full changelog](https://github.com/mozilla/glean/compare/v31.2.2...v31.2.3)

* General
    * Move debug view tag management to the Rust core. ([1640575](https://bugzilla.mozilla.org/show_bug.cgi?id=1640575), [#998](https://github.com/mozilla/glean/pull/998))
    * BUGFIX: Fix mismatch in `event`s keys and values by using `glean_parser` version 1.23.0.

# v31.2.2 (2020-06-26)

[Full changelog](https://github.com/mozilla/glean/compare/v31.2.1...v31.2.2)

* Android
    * BUGFIX: Compile dependencies with `NDEBUG` to avoid linking unavailable symbols.
      This fixes a crash due to a missing `stderr` symbol on older Android ([#1020](https://github.com/mozilla/glean/pull/1020))

# v31.2.1 (2020-06-25)

[Full changelog](https://github.com/mozilla/glean/compare/v31.2.0...v31.2.1)

* Python
    * BUGFIX: Core metrics are now present in every ping, even if submit is called before initialize has a chance to complete. ([#1012](https://github.com/mozilla/glean/pull/1012))

# v31.2.0 (2020-06-24)

[Full changelog](https://github.com/mozilla/glean/compare/v31.1.2...v31.2.0)

* General
    * Add rate limiting capabilities to the upload manager. ([1543612](https://bugzilla.mozilla.org/show_bug.cgi?id=1543612), [#974](https://github.com/mozilla/glean/pull/974))
* Android
    * BUGFIX: baseline pings with reason "dirty startup" are no longer sent if Glean did not full initialize in the previous run ([#996](https://github.com/mozilla/glean/pull/996)).
* Python
    * Support for Python 3.5 was dropped ([#987](https://github.com/mozilla/glean/pull/987)).
    * Python wheels are now shipped with Glean release builds, resulting in much smaller libraries ([#1002](https://github.com/mozilla/glean/pull/1002))
    * The Python bindings now use `locale.getdefaultlocale()` rather than `locale.getlocale()` to determine the locale ([#1004](https://github.com/mozilla/glean/pull/1004)).

# v31.1.2 (2020-06-23)

[Full changelog](https://github.com/mozilla/glean/compare/v31.1.1...v31.1.2)

* General
    * BUGFIX: Correctly format the date and time in the `Date` header ([#993](https://github.com/mozilla/glean/pull/993)).
* Python
    * BUGFIX: Additional time is taken at shutdown to make sure pings are sent and telemetry is recorded. ([1646173](https://bugzilla.mozilla.org/show_bug.cgi?id=1646173), [#983](https://github.com/mozilla/glean/pull/983))
    * BUGFIX: Glean will run on the main thread when running in a `multiprocessing` subprocess ([#986](https://github.com/mozilla/glean/pull/986)).

# v31.1.1 (2020-06-12)

[Full changelog](https://github.com/mozilla/glean/compare/v31.1.0...v31.1.1)

* Android
    * Dropping the version requirement for lifecycle extensions down again. Upping the required version caused problems in A-C.

# v31.1.0 (2020-06-11)

[Full changelog](https://github.com/mozilla/glean/compare/v31.0.2...v31.1.0)

* General:
    * The `regex` crate is no longer required, making the Glean binary smaller ([#949](https://github.com/mozilla/glean/pull/949))
    * Record upload failures into a new metric ([#967](https://github.com/mozilla/glean/pull/967))
    * Log FFI errors as actual errors ([#935](https://github.com/mozilla/glean/pull/935))
    * Limit the number of upload retries in all implementations ([#953](https://github.com/mozilla/glean/pull/953), [#968](https://github.com/mozilla/glean/pull/968))
* Python
    * Additional safety guarantees for applications that use Python `threading` ([#962](https://github.com/mozilla/glean/pull/962))

# v31.0.2 (2020-05-29)

[Full changelog](https://github.com/mozilla/glean/compare/v31.0.1...v31.0.2)

* Rust
    * Fix list of included files in published crates

# v31.0.1 (2020-05-29)

[Full changelog](https://github.com/mozilla/glean/compare/v31.0.0...v31.0.1)

* Rust
    * Relax version requirement for `flate2` for compatibility reasons

# v31.0.0 (2020-05-28)

[Full changelog](https://github.com/mozilla/glean/compare/v30.1.0...v31.0.0)

* General:
  * The version of `glean_parser` has been upgraded to v1.22.0
    * A maximum of 10 `extra_keys` is now enforced for event metric types.
    * **Breaking change**: (Swift only) Combine all metrics and pings into a single generated file Metrics.swift.
      * For Swift users this requires to change the list of output files for the `sdk_generator.sh` script.
        It now only needs to include the single file `Generated/Metrics.swift`.

* Python:
  * BUGFIX: `lifetime: application` metrics are no longer recorded as `lifetime: user`.
  * BUGFIX: glean-core is no longer crashing when calling `uuid.set` with invalid UUIDs.
  * Refactor the ping uploader to use the new upload mechanism and add gzip compression.
  * Most of the work in `Glean.initialize` happens on a worker thread and no longer blocks
    the main thread.
* Rust:
  * Expose `Datetime` types to Rust consumers.

# v30.1.0 (2020-05-22)

[Full changelog](https://github.com/mozilla/glean/compare/v30.0.0...v30.1.0)

* Android & iOS
  * Ping payloads are now compressed using gzip.
* iOS
  * `Glean.initialize` is now a no-op if called from an embedded extension. This means that Glean will only run in the base application process in order to prevent extensions from behaving like separate applications with different client ids from the base application. Applications are responsible for ensuring that extension metrics are only collected within the base application.
* Python
  * `lifetime: application` metrics are now cleared after the Glean-owned pings are sent,
    after the product starts.
  * Glean Python bindings now build in a native Windows environment.
  * BUGFIX: `MemoryDistributionMetric` now parses correctly in `metrics.yaml` files.
  * BUGFIX: Glean will no longer crash if run as part of another library's coverage testing.

# v30.0.0 (2020-05-13)

[Full changelog](https://github.com/mozilla/glean/compare/v29.1.0...v30.0.0)

* General:
  * We completely replaced how the upload mechanism works.
    glean-core (the Rust part) now controls all upload and coordinates the platform side with its own internals.
    All language bindings implement ping uploading around a common API and protocol.
    There is no change for users of Glean, the language bindings for Android and iOS have been adopted to the new mechanism already.
  * Expose `RecordedEvent` and `DistributionData` types to Rust consumers ([#876](https://github.com/mozilla/glean/pull/876))
  * Log crate version at initialize ([#873](https://github.com/mozilla/glean/pull/873))
* Android:
  * Refactor the ping uploader to use the new upload mechanism.
* iOS:
  * Refactor the ping uploader to use the new upload mechanism.

# v29.1.2 (2021-01-26)

[Full changelog](https://github.com/mozilla/glean/compare/v29.1.1...v29.1.2)

**This is an iOS release only, built with Xcode 11.7**

Otherwise no functional changes.

* iOS
  * Build with Xcode 11.7 ([#1457](https://github.com/mozilla/glean/pull/1457))

# v29.1.1 (2020-05-22)

[Full changelog](https://github.com/mozilla/glean/compare/v29.1.0...v29.1.1)

* Android
  * BUGFIX: Fix a race condition that leads to a `ConcurrentModificationException`. [Bug 1635865](https://bugzilla.mozilla.org/1635865)

# v29.1.0 (2020-05-11)

[Full changelog](https://github.com/mozilla/glean/compare/v29.0.0...v29.1.0)

* General:
  * The version of glean_parser has been upgraded to v1.20.4
    * BUGFIX: `yamllint` errors are now reported using the correct file name.
  * The minimum and maximum values of a timing distribution can now be controlled by the `time_unit` parameter. See [bug 1630997](https://bugzilla.mozilla.org/show_bug.cgi?id=1630997) for more details.

# v29.0.0 (2020-05-05)

[Full changelog](https://github.com/mozilla/glean/compare/v28.0.0...v29.0.0)

* General:
  * The version of glean_parser has been upgraded to v1.20.2 ([#827](https://github.com/mozilla/glean/pull/827)):
    * **Breaking change:** glinter errors found during code generation will now return an error code.
    * `glean_parser` now produces a linter warning when `user` lifetime metrics are set to expire. See [bug 1604854](https://bugzilla.mozilla.org/show_bug.cgi?id=1604854) for additional context.
* Android:
  * The `PingType.submit()` can now be called without a `null` by Java consumers ([#853](https://github.com/mozilla/glean/pull/853)).
* Python:
  * BUGFIX: Fixed a race condition in the `atexit` handler, that would have resulted in the message "No database found" ([#854](https://github.com/mozilla/glean/pull/854)).
  * The Glean FFI header is now parsed at build time rather than runtime. Relevant for packaging in `PyInstaller`, the wheel no longer includes `glean.h` and adds `_glean_ffi.py` ([#852](https://github.com/mozilla/glean/pull/852)).
  * The minimum versions of many secondary dependencies have been lowered to make the Glean SDK compatible with more environments.
  * Dependencies that depend on the version of Python being used are now specified using the [Declaring platform specific dependencies syntax in setuptools](https://setuptools.readthedocs.io/en/latest/setuptools.html#declaring-platform-specific-dependencies). This means that more recent versions of dependencies are likely to be installed on Python 3.6 and later, and unnecessary backport libraries won't be installed on more recent Python versions.
* iOS:
  * Glean for iOS is now being built with Xcode 11.4.1 ([#856](https://github.com/mozilla/glean/pull/856))

# v28.0.0 (2020-04-23)

[Full changelog](https://github.com/mozilla/glean/compare/v27.1.0...v28.0.0)

* General:
  * The baseline ping is now sent when the application goes to foreground, in addition to background and dirty-startup.
* Python:
  * BUGFIX: The ping uploader will no longer display a trace back when the upload fails due to a failed DNS lookup, network outage, or related issues that prevent communication with the telemetry endpoint.
  * The dependency on `inflection` has been removed.
  * The Python bindings now use `subprocess` rather than `multiprocessing` to perform ping uploading in a separate process. This should be more compatible on all of the platforms Glean supports.

# v27.1.0 (2020-04-09)

[Full changelog](https://github.com/mozilla/glean/compare/v27.0.0...v27.1.0)

* General:
  * BUGFIX: baseline pings sent at startup with the `dirty_startup` reason will now include application lifetime metrics ([#810](https://github.com/mozilla/glean/pull/810))
* iOS:
  * **Breaking change:** Change Glean iOS to use Application Support directory [#815](https://github.com/mozilla/glean/pull/815). No migration code is included. This will reset collected data if integrated without migration. Please [contact the Glean SDK team](https://github.com/mozilla/glean#contact) if this affects you.
* Python
  * BUGFIX: Fixed a race condition between uploading pings and deleting the temporary directory on shutdown of the process.

# v27.0.0 (2020-04-08)

[Full changelog](https://github.com/mozilla/glean/compare/v26.0.0...v27.0.0)

* General
  * Glean will now detect when the upload enabled flag changes outside of the application, for example due to a change in a config file. This means that if upload is disabled while the application wasn't running (e.g. between the runs of a Python command using the Glean SDK), the database is correctly cleared and a deletion request ping is sent. See [#791](https://github.com/mozilla/glean/pull/791).
  * The `events` ping now includes a reason code: `startup`, `background` or `max_capacity`.
* iOS:
  * BUGFIX: A bug where the metrics ping is sent immediately at startup on the last day of the month has been fixed.
  * Glean for iOS is now being built with Xcode 11.4.0
  * The `measure` convenience function on timing distributions and time spans will now cancel the timing if the measured function throws, then rethrow the exception ([#808](https://github.com/mozilla/glean/pull/808))
  * Broken doc generation has been fixed ([#805](https://github.com/mozilla/glean/pull/805)).
* Kotlin
  * The `measure` convenience function on timing distributions and time spans will now cancel the timing if the measured function throws, then rethrow the exception ([#808](https://github.com/mozilla/glean/pull/808))
* Python:
  * Glean will now wait at application exit for up to one second to let its worker thread complete.
  * Ping uploading now happens in a separate child process by default. This can be disabled with the `allow_multiprocessing` configuration option.

# v26.0.0 (2020-03-27)

[Full changelog](https://github.com/mozilla/glean/compare/v25.1.0...v26.0.0)

* General:
  * The version of `glean_parser` has been updated to 1.19.0:
    * **Breaking change:** The regular expression used to validate labels is
      stricter and more correct.
    * Add more information about pings to markdown documentation:
      * State whether the ping includes client id;
      * Add list of data review links;
      * Add list of related bugs links.
    * `glean_parser` now makes it easier to write external translation functions for
      different language targets.
    * BUGFIX: glean_parser now works on 32-bit Windows.
* Android:
  * `gradlew clean` will no longer remove the Miniconda installation in
    `~/.gradle/glean`. Therefore `clean` can be used without reinstalling
    Miniconda afterward every time.
* Python:
  * **Breaking Change**: The `glean.util` and `glean.hardware` modules, which
    were unintentionally public, have been made private.
  * Most Glean work and I/O is now done on its own worker thread. This brings the parallelism Python in line with the other platforms.
  * The timing distribution, memory distribution, string list, labeled boolean and labeled string metric types are now supported in Python ([#762](https://github.com/mozilla/glean/pull/762), [#763](https://github.com/mozilla/glean/pull/763), [#765](https://github.com/mozilla/glean/pull/765), [#766](https://github.com/mozilla/glean/pull/766))

# v25.1.0 (2020-02-26)

[Full changelog](https://github.com/mozilla/glean/compare/v25.0.0...v25.1.0)

* Python:
  * The Boolean, Datetime and Timespan metric types are now supported in Python ([#731](https://github.com/mozilla/glean/pull/731), [#732](https://github.com/mozilla/glean/pull/732), [#737](https://github.com/mozilla/glean/pull/737))
  * Make public, document and test the debugging features ([#733](https://github.com/mozilla/glean/pull/733))

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
