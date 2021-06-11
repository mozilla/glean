# PingEncryptionPlugin

The `PingEncryptionPlugin` encrypts the pings payloads before pings are sent.

The encryption happens after a ping is collected and before it is sent, the scope
of this plugin does not include data encryption _before_ collection. In other words,
Glean will still store plain data on the user's machine and the data will only be
encrypted during transit.

Encrypted pings will not be compliant with the usual [Glean ping schema](https://github.com/mozilla-services/mozilla-pipeline-schemas/blob/main/schemas/glean/glean/glean.1.schema.json).
Instead they will follow a specific encrypted ping schema that looks like this:

```js
{
  "payload": "eyJhbGciOiJFQ0RILUVTI..."
}
```

The schema has one field, `payload`, which is an unbounded string containing the output of the
encryption of the common Glean payload. Since JSON is the supported transport for our data ingestion
platform, the encrypted payload will be in the [JWE Compact Serialization](https://tools.ietf.org/html/rfc7516#section-3.1)
format and must include a key id (`kid`) header identifying the public key used for encryption.
A matching key is derived from the document namespace that uniquely identifies the application
instead of the key id derived from the public key.

## Requesting an encryption key

The encryption key is provisioned by Data SRE and must be generated before new pings can be
successfully ingested into a data store. **Without a valid encryption key, the Glean pipeline will**
**not be able to parse the pings and they will be thrown into the error stream.**

The encryption key should be requested as part of the process of adding a Glean application id to the ingestion pipeline (see [this checklist](../../../user/adding-glean-to-your-project/index.md)).

## Usage

### Entry point

```js
//esm
import PingEncryptionPlugin from "@mozilla/glean/plugins/encryption"
// cjs
const { default: PingEncryptionPlugin } = require("@mozilla/glean/plugins/encryption");
```

### Instantiating

The `PingEncryptionPlugin` constructor expects a
[JWK](https://datatracker.ietf.org/doc/html/rfc7516#section-4.1.5) as a parameter.
This JWK is the key provided by Data SRE during the
["Requesting an encryption key"](#requesting-an-encryption-key) step.

```js
Glean.initialize({
  "my.fancy.encrypted.app",
  uploadStatus,
  {
    plugins: [
         new PingEncryptionPlugin({
              "crv": "P-256",
              "kid": "fancy",
              "kty": "EC",
              "x": "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",
              "y": "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"
         })
    ]
  }
});
```

{{#include ../../../../shared/blockquote-info.html}}

#### Note on `logPings`

> The [logPings](../../../reference/debug/logPings.md) debug feature can still be used if
> ping encryption is turned on. Pings will be logged in their plain format,
> right before encryption happens.
