# Plugins

The Glean JavaScript SDK accepts a plugin array as an initialization parameter.

```js
import Glean from "@mozilla/glean/webext"
// This is not a real available plugin,
// it is a hypothetical one for illustration purposes.
import HypotheticalPlugin from "@mozilla/glean/plugins/hypothetical"

Glean.initialize(
  "my.fancy.modified.app",
  uploadStatus,
  {
    plugins: [
      new HypotheticalPlugin("with", "hypothetical", "arguments")
    ]
  }
);
```

Plugins attach to specific internal events on the SDK and can modify its behavior.

A big advantage of plugins is that they can address very specific use cases of Glean without bloating the final size of the SDK or overloading Glean's configuration object.

{{#include ../../../../shared/blockquote-info.html}}

## On writing your own plugins

> It is not currently possible for users to write their own plugins,
> as this feature is still in its infancy.

## Available plugins

### [PingEncryptionPlugin](./encryption.md)

The `PingEncryptionPlugin` encrypts the pings payloads before pings are sent.
