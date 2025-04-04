# The General API

The Glean SDKs have a minimal API available on their top-level `Glean` object called the General API.
This API allows, among other things, to enable and disable upload, register [custom pings][custom-pings] and set [experiment data][experiments-api].

[custom-pings]: ../../user/pings/custom.md
[experiments-api]: ./experiments-api.md

{{#include ../../../shared/blockquote-warning.html}}

##### Only initialize in the main application!

> Glean should only be initialized from the main application, not individual libraries.
> If you are adding Glean support to a library, you can safely skip this section.

## The API

The Glean SDKs provide a general API that supports the following operations. See API reference pages for SDK-specific details.

| Operation | Description | Notes |
| --------- | ----------- | ----- |
| `initialize` | Configure and initialize the Glean SDK. | [Initializing the Glean SDK](./initializing.md) |
| `setCollectionEnabled` | Enable or disable Glean collection and upload. | [Toggling collection status](./toggling-collection-status.md) |
| `registerPings` | Register custom pings generated from `pings.yaml`. | [Custom pings][custom-pings] |
| `setExperimentActive` | Indicate that an experiment is running. | [Using the Experiments API][experiments-api] |
| `setExperimentInactive` | Indicate that an experiment is no longer running.. | [Using the Experiments API][experiments-api] |
| `registerEventListener` | Register a callback by which a consumer can be notified of all event metrics being recorded. | [Glean Event Listener](./glean-event-listener.md) |
| `update{Attribution|Distribution}` | Update the product attribution or distribution `client_info` fields. | [Reporting Product Attribution and Distribution][attribution-api] |
