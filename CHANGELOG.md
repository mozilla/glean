# Unreleased changes

[Full changelog](https://github.com/mozilla/glean/compare/v32.1.0...main)

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
