# Shut down

Provides a way for users to gracefully shut down Glean,
by blocking until it is finished performing pending tasks
such as recording metrics and uploading pings.

{{#include ../../../shared/blockquote-info.html}}

## How the Glean SDKs execute tasks

> Most calls to Glean APIs are dispatched[^1]. This strategy is adopted because most tasks
> performed by the Glean SDKs involve file system read or write operations, HTTP requests and
> other time consuming actions.
>
> Each Glean SDK has an internal structure called "Dispatcher" which makes sure API calls
> get executed in the order they were called, while not requiring the caller to block on the
> completion of each of these tasks.
>
> [^1]: Here, this term indicates the tasks are run asynchronously in JavaScript or in a different
> thread for all other SDKs.

## API

### `shutdown`

{{#include ../../../shared/tab_header.md}}
<div data-lang="Kotlin" class="tab" data-info="No blocking tasks will be performed on application shutdown, pending pings will be sent on startup."></div>
<div data-lang="Java" class="tab" data-info="No blocking tasks will be performed on application shutdown, pending pings will be sent on startup."></div>
<div data-lang="Swift" class="tab" data-info="No blocking tasks will be performed on application shutdown, pending pings will be sent on startup."></div>
<div data-lang="Python" class="tab" data-info="On application shutdown Glean will wait for up to 30 seconds to finish upload tasks."></div>
<div data-lang="Rust" class="tab">


```Rust
fn main() {
    let cfg = Configuration {
        // ...
    };
    let client_info = /* ... */;
    glean::initialize(cfg, client_info);

    // Ensure the dispatcher thread winds down
    glean::shutdown();
}
```

</div>
<div data-lang="JavaScript" class="tab">

```js
import Glean from "@mozilla/glean/webext";

async function onUninstall() {
  // Flips Glean upload status to `false`,
  // which triggers sending of a `deletion-request` ping.
  Glean.setUploadEnabled(false);

  // Block on shut down to guarantee all pending pings
  // (including the `deletion-request` sent above)
  // are sent before the extension is uninstalled.
  await Glean.shutdown();

  // Uninstall browser extension without asking for user approval before doing so.
  await browser.management.uninstallSelf({ showConfirmDialog: false });
}
```

The `shutdown` API is available for all JavaScript targets, even though the above example
is explicitly using the `webext` target.
</div>
<div data-lang="Firefox Desktop" class="tab" data-info="On application shutdown Glean will automatically wait for the dispatcher thread to finish."></div>
{{#include ../../../shared/tab_footer.md}}

## Reference

* [Rust API docs](../../../docs/glean/fn.shutdown.html)
* [JavaScript API docs](https://mozilla.github.io/glean.js/classes/core_glean.default.html#shutdown)
