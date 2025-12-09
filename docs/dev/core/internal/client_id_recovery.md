# Client ID recovery

Currently (2025-10-31, Glean v66) we see some unexplained Glean SDK database resets.
These are noticeable in data as client ID regenerations:
A client application with telemetry enabled, which previously already sent data,
regenerates its client ID on initialize and thus looks like a new client.

That's undesirable and a bug.
However we have yet to track down the actual faulty code path.
Until that bug is found and fixed, the Glean SDK provides an extra mitigation.

From Glean v66.1.0 on the SDK will store the client ID in a `client_id.txt` in the provided data path.
Any inconsistencies in that data compared to the database will be reported
and, if applicable, the client ID restored.

**Note:** Glean v66.1.0 will only report the inconsistency, but will not restore a recovered client ID.
From Glean v66.2.0 on we apply the mitigation and restore the client ID.

The exact flow of decisions is depicted in the chart below.
The implementation is in [`glean-core/src/core/mod.rs`](https://github.com/mozilla/glean/blob/HEAD/glean-core/src/core/mod.rs#L264)

```mermaid
flowchart TD
    A["Glean.init"] -->B
    B{client_id.txt exists?} -->|yes| C
    B -->|no| D
    C["(a) load file ID"] --> E
    D["load DB ID"] --> D3
    D3{DB ID empty} -->|yes| D4
    D3 -->|no| S
    D4["generate DB ID"] --> S
    E{valid file and ID?} -->|yes| H
    E -->|no| G
    G["(b) record file read error"] --> H
    H{"(c) DB size <= 0"} -->|yes| J
    H -->|no| F
    J["(d) record empty DB error
    report recovered ID: file ID"] --> Q
    F["load DB ID"] --> N
    L{file ID == DB ID} --> |yes| Z
    L -->|no| T
    N{DB ID empty?} -->|yes| O
    N -->|no| L
    O["(f) record regen error"] --> Q
    P["(g) record mismatch error
    report recovered ID: file ID"] --> S
    Q["(e) mitigation:
    set DB ID = file ID"] --> Z
    S["(h) write DB ID to file"] --> Z
    T{"DB ID == 'c0ffee'"}
    T -->|yes| U
    T -->|no| P
    U["(i) record c0ffee error
    report recovered ID: file ID"] --> Q
    
    Z(normal operation)
```
