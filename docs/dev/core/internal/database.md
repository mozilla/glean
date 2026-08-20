# Database format

The Glean SDK stores all recorded data in a [SQLite] database for persistence.
This data is read, written and transformed by the core implementation.

Internal metrics are stored similar to user-defined metrics.

We guarantee backwards-compatibility of already stored data.
If necessary an old database will be converted to the new format.

## Database tables

The Glean SDK will store all metric data in a table called `telemetry`.
This table has the following schema:

```
CREATE TABLE telemetry(
  id TEXT NOT NULL,
  ping TEXT NOT NULL,
  lifetime TEXT NOT NULL,
  labels TEXT NOT NULL,
  value BLOB,
  UNIQUE(id, ping, labels)
);",
```

| Column | Type | Description |
| ------ | ---- | ----------- |
| `id`   | `TEXT` | A full metric identifier: `category.name`. |
| `ping` | `TEXT` | The ping this value is recorded for. |
| `lifetime` | `TEXT` | The lifetime of the stored value, one of `ping`, `app` or `user`. |
| `labels` | `TEXT` | The label or labels for this value. Multiple labels are separated by the record separator (`\x1E`). Empty string when no labels are specified. |
| `value` | `BLOB` | The encoded value. |

### Indices

`UNIQUE(id, ping, labels)`

Every row is unique by the id, ping and associated labels.
This allows for efficient fetching and updating of those values.
Recorded values for a specific `id` can go into multiple pings.
For a specific `id` values can be recorded for different labels.

{{#include ../../../shared/blockquote-info.html}}

#### Additional characters in Glean.js

> Glean.js uses a different format to store data.
> Additional information is encoded into database keys, using the `+` (plus) character to separate parts of data.
> Watch [Bug 1720476](https://bugzilla.mozilla.org/show_bug.cgi?id=1720476) for details.

## Value

The value is stored in an implementation-defined format to encode the value's data.
It can be read, modified and serialized into the [Payload format].

[SQLite]: https://sqlite.org/
[Payload format]: payload.md
