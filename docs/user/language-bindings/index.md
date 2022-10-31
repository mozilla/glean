# Overview

The Glean SDKs are available for several programming languages and development environments.

Although there are different SDKs, each of them is based off of either
a JavaScript core or a Rust core. These cores contain the bulk of the logic
of the client libraries. Thin wrappers around them expose APIs for the target platforms.
Each SDK may also send a different set of [default pings](../user/pings/sent-by-glean.html#available-pings-per-platform)
and collect a different set of [default metrics](../user/collected-metrics/metrics.html).

Finally, each SDK may also be[^1] accompanied by `glean_parser`, a Python command line utility
that provides a list of useful development tools for developers instrumenting a project using Glean.

[^1]: Some SDKs are not bundled with `glean_parser` and it is left to the user to install it separately.

<!-- toc -->

## Rust Core-based SDKs

The code for the Rust Core-based SDKs is available on the
[`mozilla/glean`](https://github.com/mozilla/glean) repository.

{{#include ../../shared/blockquote-info.html}}

> These group of SDKs were previously referred to as "language bindings" i.e.
> "the Kotlin language bindings" or "the Python language bindings".

### Rust

The Glean Rust SDK can be used with any Rust application.
It can serve a wide variety of usage patterns,
such as short-lived CLI applications as well as longer-running desktop or server applications.
It can optionally send builtin pings at startup.
It does not assume an interaction model and the integrating application is responsible to connect the respective hooks.

It is available as the [`glean` crate][glean crate] on crates.io.

[glean crate]: https://crates.io/crates/glean

### Kotlin

The Glean Kotlin SDK is primarily used for integration with Android applications.
It assumes a common interaction model for mobile applications.
It sends builtin pings at startup of the integrating application.

It is available standalone as `org.mozilla.telemetry:glean`
or via [Android Components][ac] as `org.mozilla.components:service-glean`
from the [Mozilla Maven instance][maven].

The Kotlin SDK can also be used from Java.

[ac]: https://github.com/mozilla-mobile/firefox-android/tree/main/android-components/
[maven]: https://maven.mozilla.org/?prefix=maven2

See [Android](android/) for more on integrating Glean on Android.

### Swift

The Glean Swift SDK is primarily used for integration with iOS applications.
It assumes a common interaction model for mobile applications.
It sends builtin pings at startup of the integrating application.

It is available as a standalone Xcode framework from the [Glean releases page][releases] or bundled with the [AppServices framework][as-releases].

[releases]: https://github.com/mozilla/glean/releases
[as-releases]: https://github.com/mozilla/application-services/releases

### Python

The Glean Python SDK allows integration with any Python application.
It can serve a wide variety of usage patterns,
such as short-lived CLI applications as well as longer-running desktop or server applications.

It is available as [`glean-sdk` on PyPI][pypi].

[pypi]: https://pypi.org/project/glean-sdk/

<!-- ### Firefox Desktop SDK

TODO -->

## JavaScript Core-based SDKs

The code for the JavaScript Core-based SDKs is available on the
[`mozilla/glean.js`](https://github.com/mozilla/glean.js) repository.

This collection of SDKs is commonly referred to as **Glean.js**.

### JavaScript

The Glean JavaScript SDK allows integration with three distint JavaScript environments: websites,
web extension and Node.js.

It is available as [`@mozilla/glean` on npm](https://www.npmjs.com/package/@mozilla/glean).
This package has different entry points to access the different SDKs.

- `@mozilla/glean/web` gives access to the websites SDK
- `@mozilla/glean/webext` gives access to the web extension SDK
- `@mozilla/glean/node` gives access to the Node.js SDK

### QML

The Glean QML SDK allows integration with Qt/QML applications and libraries.

It is available as a compressed file attached to each new Glean.js release
and may be downloaded from the project
[GitHub releases page](https://github.com/mozilla/glean.js/releases/).
