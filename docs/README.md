# Glean

![Glean logo](glean.jpeg)

_Modern Firefox Telemetry for mobile platforms_

The `Glean SDK` is a modern approach for a mobile Telemetry library and is part of the [Glean project](https://docs.telemetry.mozilla.org/concepts/glean/glean.html).
It started out as an [android-components library](https://github.com/mozilla-mobile/android-components/tree/master/components/service/glean), written purely in Kotlin to be used on Android.

The `Glean SDK` is a work-in-progress to transform the concepts behind the `Glean SDK` into a cross-platform library based on a low-level [Rust](https://www.rust-lang.org/) library and the necessary platform integrations for Android and iOS (and beyond...).

**Note: The cross-platform `Glean SDK` project is in development. It's not finished, and not used. Some of this documentation refers to features that were implemented in an earlier Android-specific version of the Glean SDK, but are not yet implemented in this version.**

To contact us you can:
- Find us on the Mozilla Slack in *#glean*, on [Mozilla IRC](https://wiki.mozilla.org/IRC) in *#telemetry*.
- To report issues or request changes, file a bug in [Bugzilla in Data Platform & Tools :: Glean: SDK](https://bugzilla.mozilla.org/enter_bug.cgi?product=Data%20Platform%20and%20Tools&component=Glean%3A%20SDK).
- Send an email to *glean-team@mozilla.com*.
- The Glean SDK team is: *:janerik*, *:dexter*, *:travis*, *:mdroettboom*, *:gfritzsche*, *:chutten*

## License

This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

