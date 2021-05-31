# The General API

The Glean SDK has a minimal API available on its top-level `Glean` object called the General API.
This API allows, among other things, to enable and disable upload, register [custom pings][custom-pings] and set [experiment data][experiments-api].

[custom-pings]: ../../user/pings/custom.md
[experiments-api]: ../../user/experiments-api.md

{{#include ../../../shared/blockquote-warning.html}}

##### Only initialize in the main application!

> The Glean SDK should only be initialized from the main application, not individual libraries.
> If you are adding Glean SDK support to a library, you can safely skip this section.

## The API

The Glean SDK provides a general API that supports the following operations. See below for language-specific details.

| Operation | Description | Notes |
| --------- | ----------- | ----- |
| `initialize` | Configure and initialize the Glean SDK. | [Initializing the Glean SDK](./initializing.md) |
| `setUploadEnabled` | Enable or disable Glean collection and upload. | [Enabling and disabling Metrics](#enabling-and-disabling-metrics) |
| `registerPings` | Register custom pings generated from `pings.yaml`. | [Custom pings][custom-pings] |
| `setExperimentActive` | Indicate that an experiment is running. | [Using the Experiments API][experiments-api] |
| `setExperimentInactive` | Indicate that an experiment is no longer running.. | [Using the Experiments API][experiments-api] |

## Enabling and disabling metrics

{{#include ../../../shared/tab_header.md}}

<div data-lang="Kotlin" class="tab">

`Glean.setUploadEnabled()` should be called in response to the user enabling or disabling telemetry.

**Note**:
If called before `Glean.initialize()` the call to `Glean.setUploadEnabled()` will be ignored.
Set the initial state using `uploadEnabled` on `Glean.initialize()`.

</div>

<div data-lang="Swift" class="tab">

`Glean.shared.setUploadEnabled()` should be called in response to the user enabling or disabling telemetry.

**Note**:
If called before `Glean.shared.initialize()` the call to `Glean.shared.setUploadEnabled()` will be ignored.
Set the initial state using `uploadEnabled` on `Glean.shared.initialize()`.

</div>

<div data-lang="Python" class="tab">

`Glean.set_upload_enabled()` should be called in response to the user enabling or disabling telemetry.

**Note**:
If called before `Glean.initialize()` the call to `Glean.set_upload_enabled()` will be ignored.
Set the initial state using `upload_enabled` on `Glean.initialize()`.

</div>

<div data-lang="C#" class="tab">

`GleanInstance.SetUploadEnabled()` should be called in response to the user enabling or disabling telemetry.

**Note**:
If called before `GleanInstance.initialize()` the call to `GleanInstance.SetUploadEnabled()` will be ignored.
Set the initial state using `uploadEnabled` on `GleanInstance.initialize()`.

</div>

{{#include ../../../shared/tab_footer.md}}

The application should provide some form of user interface to call this method.

When going from enabled to disabled, all pending events, metrics and pings are cleared, except for [`first_run_date` and `first_run_hour`](../../user/pings/index.html#the-client_info-section).
When re-enabling, core Glean metrics will be recomputed at that time.
