# Breaking Changes

The Glean SDK generally follows [Semantic Versioning 2.0.0](https://semver.org/).
This applies to:

* The Rust SDK ([glean](https://crates.io/crates/glean), [glean-core](https://crates.io/crates/glean-core), [glean-sym](https://crates.io/crates/glean-sym))
* The Kotlin SDK ([Maven](https://maven.mozilla.org/maven2?prefix=maven2/org/mozilla/telemetry/glean/))
* The Swift SDK ([glean-swift](https://github.com/mozilla/glean-swift))
* The Python SDK ([glean-sdk](https://pypi.org/project/glean-sdk/))

All SDKs are versioned in lockstep, so any breaking change in any part will require a major version bump for all SDKs.

[glean-parser](https://github.com/mozilla/glean_parser/) is versioned separately.
It also follows Semantic Versioning.

## How to detect breaking changes?

* Breaking changes in the Rust SDK can be detected with [cargo-semver-checks](https://github.com/obi1kenobi/cargo-semver-checks/)
  * This tooling is not integrated into the development workflow at the moment
* Test `firefox-main` with [a local Glean build](https://firefox-source-docs.mozilla.org/toolkit/components/glean/dev/local_glean.html).
  If this requires changes to FOG it most likely is a breaking change

## What is considered a breaking change?

This list is not exhaustive.

* Any changes to any metric API
  * This includes changes to methods as well as the layout of objects in the respective language
* Publicly visible changes to the existing general API

## What is NOT a breaking change in the Glean SDK?

This list is not exhaustive.

* Adding new methods to the general API
* Adding a new metric type
* New defaulted parameters to existing methods of the general API

## What is considered a breaking change in glean-parser?

This list is not exhaustive.

* New error-level lints
* Changes to the generated code (in any language) that also require SDK changes

## What is NOT a breaking change in glean-parser?

This list is not exhaustive.

* New warning-level lints
  * Consumers might still break due to the new lint triggering and warning being treated as errors.
    This is considered okay and the lints can be addressed or silenced.
* Fixing bugs in lints
  * Consumers might still break due to the lint now triggering and warning being treated as errors.
    This is considered okay and the lints can be addressed or silenced.
*

## Landing major version bumps

Landing a major SDK version bump in [firefox-main](https://github.com/mozilla-firefox/firefox) will always require
coordination with [application-services](https://github.com/mozilla/application-services) and a nightly build.  
See [a previous bump in application-services](https://github.com/mozilla/application-services/pull/7688).  
See [Try runs with released versions of Glean](android/development-with-mozilla-central.md#try-runs-with-released-versions-of-glean)
for how to run Firefox CI with a new Glean release.

Landing a major SDK version bump in [Firefox iOS](https://github.com/mozilla-mobile/firefox-ios) also requires
coordination with application-services and a nightly build.
See [a previous bump in Firefox iOS](https://github.com/mozilla-mobile/firefox-ios/pull/34270).
