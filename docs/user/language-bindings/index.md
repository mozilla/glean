# Overview

The Glean SDK provides multiple language bindings for integration into different platforms and with different programming languages

## Rust

The Rust bindings can be used with any Rust application.
It can serve a wide variety of usage patterns,
such as short-lived CLI applications as well as longer-running desktop or server applications.
It can optionally send builtin pings at startup.
It does not assume an interaction model and the integrating application is responsible to connect the respective hooks.

It is available as the [`glean` crate][glean crate] on crates.io.

[glean crate]: https://crates.io/crates/glean

## Kotlin

The Kotlin bindings are primarily used for integration with Android applications.
It assumes a common interaction model for mobile applications.
It sends builtin pings at startup of the integrating application.

It is available standalone as `org.mozilla.telemetry:glean`
or via [Android Components][ac] as `org.mozilla.components:service-glean`
from the [Mozilla Maven instance][maven].

> *Note*: The Kotlin bindings can also be used from Java.

[ac]: https://github.com/mozilla-mobile/android-components/
[maven]: https://maven.mozilla.org/?prefix=maven2

See [Android](android/) for more on integrating Glean on Android.

## Swift

The Swift bindings are primarily used for integration with iOS applications.
It assumes a common interaction model for mobile applications.
It sends builtin pings at startup of the integrating application.

It is available as a standalone Xcode framework from the [Glean releases page][releases] or bundled with the [AppServices framework][as-releases].

[releases]: https://github.com/mozilla/glean/releases
[as-releases]: https://github.com/mozilla/application-services/releases

## Python

The Python bindings allow integration with any Python application.
It can serve a wide variety of usage patterns,
such as short-lived CLI applications as well as longer-running desktop or server applications.

It is available as [`glean-sdk` on PyPI][pypi].

[pypi]: https://pypi.org/project/glean-sdk/
