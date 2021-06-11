# Toggling upload status

{{#include ../../../shared/tab_header.md}}

<div data-lang="Kotlin" class="tab">

`Glean.setUploadEnabled()` should be called in response to the user enabling or disabling telemetry.

{{#include ../../../shared/blockquote-warning.html}}

##### Do not call `setUploadEnabled` before initializing

> If called before `Glean.initialize()` the call to `Glean.setUploadEnabled()` will be ignored.
> Set the initial state using `uploadEnabled` on `Glean.initialize()`.

</div>

<div data-lang="Swift" class="tab">

`Glean.shared.setUploadEnabled()` should be called in response to the user enabling or disabling telemetry.

{{#include ../../../shared/blockquote-warning.html}}

##### Do not call `setUploadEnabled` before initializing

> If called before `Glean.shared.initialize()` the call to `Glean.shared.setUploadEnabled()` will be ignored.
> Set the initial state using `uploadEnabled` on `Glean.shared.initialize()`.

</div>

<div data-lang="Python" class="tab">

`Glean.set_upload_enabled()` should be called in response to the user enabling or disabling telemetry.

{{#include ../../../shared/blockquote-warning.html}}

##### Do not call `set_upload_enabled` before initializing

> If called before `Glean.initialize()` the call to `Glean.set_upload_enabled()` will be ignored.
> Set the initial state using `upload_enabled` on `Glean.initialize()`.

</div>

<div data-lang="JavaScript" class="tab">

`Glean.setUploadEnabled()` should be called in response to the user enabling or disabling telemetry.

{{#include ../../../shared/blockquote-warning.html}}

##### Do not call `setUploadEnabled` before initializing

> If called before `Glean.initialize()` the call to `Glean.setUploadEnabled()` will be ignored.
> Set the initial state using `uploadEnabled` on `Glean.initialize()`.

</div>

{{#include ../../../shared/tab_footer.md}}

The application should provide some form of user interface to call this method.

When going from enabled to disabled, all pending events, metrics and pings are cleared, except for [`first_run_date` and `first_run_hour`](../../user/pings/index.html#the-client_info-section).
When re-enabling, core Glean metrics will be recomputed at that time.
