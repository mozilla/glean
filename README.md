# Glean SDK

![Glean logo](docs/glean.jpeg)

[![glean-core on crates.io](http://meritbadge.herokuapp.com/glean-core)](https://crates.io/crates/glean-core)
[![License: MPL-2.0](https://img.shields.io/crates/l/glean-core)](https://github.com/mozilla/glean/blob/master/LICENSE)
[![The Glean SDK book](https://img.shields.io/badge/Docs-Glean%20SDK-brightgreen)][book]
[![Build Status](https://img.shields.io/circleci/build/github/mozilla/glean/master)](https://circleci.com/gh/mozilla/glean)
[![Code coverage](https://img.shields.io/codecov/c/github/mozilla/glean)](https://codecov.io/gh/mozilla/glean)

## Documentation

All documentation is available online:

## [The Glean SDK Book][book]

## Overview

Refer to the documentation for [using and developing the Glean SDK][book].

For an overview of Glean beyond just the SDK, see the [section in the Firefox data docs](https://docs.telemetry.mozilla.org/concepts/glean/glean.html).

The code in this repository is organized as follows:

* [./glean-core/](glean-core) contains the source for the low-level Rust library.
* [./glean-core/ffi](glean-core/ffi) contains the mapping into a C FFI.
* [./glean-core/android](glean-core/android) contains the Kotlin bindings for use by Android applications.
* [./glean-core/ios](glean-core/ios) contains the Swift bindings for use by iOS applications.
* [./glean-core/python](glean-core/python) contains Python bindings.

**Note: The Glean SDK requires at least [Rust 1.41.0](https://blog.rust-lang.org/2020/01/30/Rust-1.41.0.html). Older versions are untested.**

## Contact

To contact us you can:

* Find us in the [#glean channel on chat.mozilla.org](https://chat.mozilla.org/#/room/#glean:mozilla.org).
* To report issues or request changes, file a bug in [Bugzilla in Data Platform & Tools :: Glean: SDK][newbugzilla].
* Send an email to *glean-team@mozilla.com*.
* The Glean Core team is: *:dexter*, *:janerik*, *:mdroettboom*, *:travis_*, *:gfritzsche*, *:chutten*, *:brizental*.

## Credits

The [Glean logo artwork](https://dianaciufo.wordpress.com/2019/10/11/glean-graphic-identity-for-mozilla-firefox/) was contributed by [Diana Ciufo](https://dianaciufo.wordpress.com/).
It's licensed under MPL.

## License

    This Source Code Form is subject to the terms of the Mozilla Public
    License, v. 2.0. If a copy of the MPL was not distributed with this
    file, You can obtain one at http://mozilla.org/MPL/2.0/


[newbugzilla]: https://bugzilla.mozilla.org/enter_bug.cgi?assigned_to=nobody%40mozilla.org&bug_ignored=0&bug_severity=normal&bug_status=NEW&cf_fission_milestone=---&cf_fx_iteration=---&cf_fx_points=---&cf_status_firefox65=---&cf_status_firefox66=---&cf_status_firefox67=---&cf_status_firefox_esr60=---&cf_status_thunderbird_esr60=---&cf_tracking_firefox65=---&cf_tracking_firefox66=---&cf_tracking_firefox67=---&cf_tracking_firefox_esr60=---&cf_tracking_firefox_relnote=---&cf_tracking_thunderbird_esr60=---&product=Data%20Platform%20and%20Tools&component=Glean%3A%20SDK&contenttypemethod=list&contenttypeselection=text%2Fplain&defined_groups=1&flag_type-203=X&flag_type-37=X&flag_type-41=X&flag_type-607=X&flag_type-721=X&flag_type-737=X&flag_type-787=X&flag_type-799=X&flag_type-800=X&flag_type-803=X&flag_type-835=X&flag_type-846=X&flag_type-855=X&flag_type-864=X&flag_type-916=X&flag_type-929=X&flag_type-930=X&flag_type-935=X&flag_type-936=X&flag_type-937=X&form_name=enter_bug&maketemplate=Remember%20values%20as%20bookmarkable%20template&op_sys=Unspecified&priority=P3&&rep_platform=Unspecified&status_whiteboard=%5Btelemetry%3Aglean-rs%3Am%3F%5D&target_milestone=---&version=unspecified
[book]: https://mozilla.github.io/glean/
[rustdoc]: https://mozilla.github.io/glean/docs/index.html
[ktdoc]: https://mozilla.github.io/glean/javadoc/glean/index.html
