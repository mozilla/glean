# Glean

![Glean logo](docs/glean.jpeg)

_Modern Firefox Telemetry for mobile platforms_


---

**Note: This new approach for the Glean SDK is currently in development and not yet ready for use.
The working and supported library is the [Glean SDK in android-components][glean-ac].**

---

## Documentation

All documentation is available online:

## [The Glean SDK Book][book]

## Overview

This repository is used to build the client-side cross-platform Telemetry library part of [Glean](https://docs.telemetry.mozilla.org/concepts/glean/glean.html), called the `Glean SDK`.

The code is organized as follows:

* [./glean-core/](glean-core) contains the source for the low-level Rust library.
* [./glean-core/ffi](glean-core/ffi) contains the mapping into a C FFI.
* [./glean-core/android](glean-core/android) contains the Kotlin bindings for use by Android applications.
* [./glean-core/ios](glean-core/ios) contains the Swift bindings for use by iOS applications.

This repository also hosts the documentation for the `Glean SDK`:

* [The Glean SDK book][book] - Documentation on using and developing Glean SDK.
* [Documentation of the Rust crate][rustdoc].
* [Documentation of the Kotlin library][ktdoc].

For an overview of Glean, see the [section in the Firefox data docs](https://docs.telemetry.mozilla.org/concepts/glean/glean.html).

## Contact

To contact us you can:

* Find us on the Mozilla Slack in *#glean*, on [Mozilla IRC][mozirc] in *#telemetry*.
* To report issues or request changes, file a bug in [Bugzilla in Data Platform & Tools :: Glean: SDK][newbugzilla].
* Send an email to *glean-team@mozilla.com*.
* The Glean Core team is: *:dexter*, *:janerik*, *:mdroettboom*, *:gfritzsche*

## License

    This Source Code Form is subject to the terms of the Mozilla Public
    License, v. 2.0. If a copy of the MPL was not distributed with this
    file, You can obtain one at http://mozilla.org/MPL/2.0/


[glean-ac]: https://github.com/mozilla-mobile/android-components/tree/master/components/service/glean
[mozirc]: https://wiki.mozilla.org/IRC
[newbugzilla]: https://bugzilla.mozilla.org/enter_bug.cgi?assigned_to=nobody%40mozilla.org&bug_ignored=0&bug_severity=normal&bug_status=NEW&cf_fission_milestone=---&cf_fx_iteration=---&cf_fx_points=---&cf_status_firefox65=---&cf_status_firefox66=---&cf_status_firefox67=---&cf_status_firefox_esr60=---&cf_status_thunderbird_esr60=---&cf_tracking_firefox65=---&cf_tracking_firefox66=---&cf_tracking_firefox67=---&cf_tracking_firefox_esr60=---&cf_tracking_firefox_relnote=---&cf_tracking_thunderbird_esr60=---&product=Data%20Platform%20and%20Tools&component=Glean%3A%20SDK&contenttypemethod=list&contenttypeselection=text%2Fplain&defined_groups=1&flag_type-203=X&flag_type-37=X&flag_type-41=X&flag_type-607=X&flag_type-721=X&flag_type-737=X&flag_type-787=X&flag_type-799=X&flag_type-800=X&flag_type-803=X&flag_type-835=X&flag_type-846=X&flag_type-855=X&flag_type-864=X&flag_type-916=X&flag_type-929=X&flag_type-930=X&flag_type-935=X&flag_type-936=X&flag_type-937=X&form_name=enter_bug&maketemplate=Remember%20values%20as%20bookmarkable%20template&op_sys=Unspecified&priority=P3&&rep_platform=Unspecified&status_whiteboard=%5Btelemetry%3Aglean-rs%3Am%3F%5D&target_milestone=---&version=unspecified
[book]: https://mozilla.github.io/glean/
[rustdoc]: https://mozilla.github.io/glean/docs/index.html
[ktdoc]: https://mozilla.github.io/glean/javadoc/glean/index.html
