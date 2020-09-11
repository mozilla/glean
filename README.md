# Glean SDK

![Glean logo](docs/glean.jpeg)

[![glean-core on crates.io](http://meritbadge.herokuapp.com/glean-core)](https://crates.io/crates/glean-core)
[![License: MPL-2.0](https://img.shields.io/crates/l/glean-core)](https://github.com/mozilla/glean/blob/main/LICENSE)
[![The Glean SDK book](https://img.shields.io/badge/Docs-Glean%20SDK-brightgreen)][book]
[![Build Status](https://img.shields.io/circleci/build/github/mozilla/glean/main)](https://circleci.com/gh/mozilla/glean)

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

**Note: The Glean SDK requires at least [Rust 1.43.0](https://blog.rust-lang.org/2020/04/23/Rust-1.43.0.html). Older versions are untested.**

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


[newbugzilla]: https://bugzilla.mozilla.org/enter_bug.cgi?product=Data+Platform+and+Tools&component=Glean%3A+SDK&priority=P3&status_whiteboard=%5Btelemetry%3Aglean-rs%3Am%3F%5D
[book]: https://mozilla.github.io/glean/
[rustdoc]: https://mozilla.github.io/glean/docs/index.html
[ktdoc]: https://mozilla.github.io/glean/javadoc/glean/index.html
