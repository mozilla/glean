# Glean Pings

Every glean ping is in JSON format and contains one or more of the [common sections](#ping-sections)
with shared information data.

If data collection is enabled, glean provides a set of built-in pings that are assembled out of the box
without any developer intervention.  The following is a list of these built-in pings:

- [`baseline` ping](baseline.md)
- `events` ping (*not provided yet*)
- `metrics` ping (*not provided yet*)

Applications can also define and send their own custom pings (*not provided yet*).

## Ping sections

There are two standard metadata sections that are added to most pings

TBD.

### The `ping_info` section

TBD.

### The `client_info` section

TBD.

## Ping submission

TBD.

### Submitted headers

TBD.
