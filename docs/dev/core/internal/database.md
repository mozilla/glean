# Database format

The Glean SDK stores all recorded data in a database for persistence.
This data is read, written and transformed by the core implementation.

Some internal metrics are stored similar to user-defined metrics,
but encode additional implementation-defined information in the key or value of the entry.

We guarantee backwards-compatibility of already stored data.
If necessary an old database will be converted to the new format.

## Database stores

The Glean SDK will use one store per metric lifetime:
`user`, `application` and `ping`.
This allows to separately read and clear metrics based on their respective lifetimes.

## Key

The key of a database entry uniquely identifies the stored metric data.
It encodes additional information about the stored data in the key name using special characters.
The full list of special characters in use is:

`. # / +`

These characters cannot be used in a user-defined ping name, metric category,  metric name or label.

A key will usually look like:

```
ping#category.name[/label]
```

where:

| Field | Description | Allowed characters | Maximum length\* | Note |
| ----- | ----------- | ------ | ----- | ----- |
| `ping` | The ping name this data is stored for | `[a-z0-9-]` | 30 |
| `category` | The metric's category | `[a-z0-9._]` | 40 | Empty string possible. |
| `name` | The metric's name | `[a-z0-9._#]` | 70 |
| `label` | The label (optional) | `[a-z0-9._-]` | 111 |

_\* The maximum length is not enforced for internal metrics, but is enforced for user metrics as per schema definition._

{{#include ../../../shared/blockquote-info.html}}

#### Additional characters in Glean.js

> Glean.js uses a different format to store data.
> Additional information is encoded into database keys, using the `+` (plus) character to separate parts of data.
> Watch [Bug 1720476](https://bugzilla.mozilla.org/show_bug.cgi?id=1720476) for details.

## Value

The value is stored in an implementation-defined format to encode the value's data.
It can be read, modified and serialized into the [Payload format].

[Payload format]: payload.md
