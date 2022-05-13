# Source Tags

Tag all subsequent outgoing pings with a maximum of 5 comma-separated tags.

"To tag" a ping with Source Tags means that the ping request
will contain the `X-Source-Tags` header with a comma separated list of the given tags.

Once enabled, the only way to disable this feature is to restart or manually reset the application.

## Limits

- Any valid HTTP header value is a valid source tag (e.g. any value that matches the
regex `[a-zA-Z0-9-]{1,20}`) and values starting with the substring `glean` are reserved for internal
Glean usage and thus are also considered invalid. If any value in the list of source tags is invalid,
the whole list will be ignored.
- If the list of tags has more than five members, the whole list will be ignored.
- The special value `automation` is meant for tagging pings generated on automation: such pings will
be specially handled on the pipeline (i.e. discarded from [non-live views](https://docs.telemetry.mozilla.org/cookbooks/bigquery/querying.html#table-layout-and-naming)).

## API

### `setSourceTags`

Sets the Source Tags to a given value.

This API can safely be called before `Glean.initialize`.
The tag will be applied upon initialization in this case.

{{#include ../../../shared/tab_header.md}}
<div data-lang="Kotlin" class="tab">

```Kotlin
import mozilla.telemetry.glean.Glean

Glean.setSourceTags(setOf("my-tag", "your-tag", "our-tag"))
```

</div>
<div data-lang="Java" class="tab">

```Java
Glean.INSTANCE.setSourceTags(setOf("my-tag", "your-tag", "our-tag"))
```

</div>
<div data-lang="Swift" class="tab">

```Swift
import Glean

Glean.shared.setSourceTags(["my-tag", "your-tag", "our-tag"])
```
</div>
<div data-lang="Python" class="tab"></div>
<div data-lang="Rust" class="tab">

```Rust
use glean;

glean.set_source_tags(["my-tag", "your-tag", "our-tag"]);
```
</div>
<div data-lang="JavaScript" class="tab">

```js
import Glean from "@mozilla/glean/<platform>";

Glean.setSourceTags(["my-tag", "your-tag", "our-tag"]);
```
</div>
<div data-lang="Firefox Desktop" class="tab"></div>
{{#include ../../../shared/tab_footer.md}}

## Environment variable

### `GLEAN_SOURCE_TAGS`

It is also  possible to set the debug view tag through
the `GLEAN_SOURCE_TAGS` environment variable.

This variable must be set at runtime, not at compile time.
It will be checked upon Glean initialization.

{{#include ../../../shared/tab_header.md}}
<div data-lang="Kotlin" class="tab"
  data-info="Although it is technically possible to use this environment variable in Android, the Glean team is not aware of a proper way to set environment variables in Android devices or emulators. When in this environment, enable debugging features through the <a href='../../user/debugging/android.html'>GleanDebugActivity</a>"></div>
<div data-lang="Java" class="tab"
  data-info="Although it is technically possible to use this environment variable in Android, the Glean team is not aware of a proper way to set environment variables in Android devices or emulators. When in this environment, enable debugging features through the <a href='../../user/debugging/android.html'>GleanDebugActivity</a>"></div>
<div data-lang="Swift" class="tab">

  ![Xcode IDE scheme editor popup screenshot](./screenshots/source_tags_screenshot_swift.png "GLEAN_SOURCE_TAGS")

  To set environment variables to the process running your app in an iOS device or emulator you need to edit the scheme for your app. In the Xcode IDE, use the shortcut `Cmd + <` to open the scheme editor popup. The environment variables editor is under the `Arguments` tab on this popup.
</div>
<div data-lang="Python" class="tab">

```bash
$ GLEAN_SOURCE_TAGS=my-tag,your-tag,our-tag python my_application.py
```
</div>
<div data-lang="Rust" class="tab">

```bash
$ GLEAN_SOURCE_TAGS=my-tag,your-tag,our-tag cargo run
```
</div>
<div data-lang="JavaScript" class="tab" data-info="It is not possible to access environment variables from the currently supported JavaScript platforms: Qt and browsers."></div>
<div data-lang="Firefox Desktop" class="tab">

```bash
$ GLEAN_SOURCE_TAGS=my-tag,your-tag,our-tag ./mach run
```
</div>
{{#include ../../../shared/tab_footer.md}}
